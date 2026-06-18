# 🦀 Kafka Rdkafka — Produce & Consume Events in Rust

*Subtitle: ship a single-broker Kafka pipeline with idempotent producer, manual commit, and a FIFO dedup cache.*

> **Test-driven approach**: This project includes a Cargo project with progressive
> unit tests. Each function in `src/lib.rs` starts as a `todo!()` stub. As you
> follow each section, replace `todo!()` with real code and run `cargo test` to
> watch the pass count grow. Your goal: **all 13 tests pass**.

---

## What Is This Project?

Kafka produce/consume with `rdkafka` — idempotent producer, manual commit, FIFO dedup cache.

### Python equivalent

```python
from confluent_kafka import Consumer, Producer

consumer = Consumer({"bootstrap.servers": "localhost:9092"})
msg = consumer.poll(1.0)
if msg:
    process(msg.value())  # may duplicate on rebalance
    consumer.commit(msg)
``` You end
up wrapping every handler in a "have-I-seen-this-id" Redis check or buying a
dedup library:

```python
# At-least-once + naive handler == duplicates
def handle(msg):
    db.execute(...)            # called twice on rebalance
    consumer.commit(msg)       # commits anyway
```

**Rust fix:** `rdkafka` (librdkafka bindings) gives you idempotent producers
and manual-commit consumers with a typed `FutureProducer` / `StreamConsumer`.
A `Future<Output = Result<(i32, i64), (KafkaError, OwnedMessage)>>` return
type forces the caller to handle the error path, and a `DedupCache` struct
keeps idempotency logic in your hands — not in a global.

## At a Glance

| # | Concept | Rust | Python | Why it matters |
|---|---------|------|--------|----------------|
| 1 | rdkafka client | `rdkafka::FutureProducer` / `StreamConsumer` | `confluent_kafka.Producer` / `Consumer` | Production-grade broker client |
| 2 | Idempotent producer | `enable.idempotence=true` | `config["enable.idempotence"]="true"` | No duplicate writes on retry |
| 3 | Manual commit | `commit_message(..., CommitMode::Async)` | `consumer.commit(msg)` | You control when offsets advance |
| 4 | Tokio async | `tokio::main`, `.await` | `asyncio.run` | One runtime, no callback hell |
| 5 | Streaming consumer | `StreamConsumer` + `recv().await` | poll-loop | Backpressure-aware |
| 6 | FIFO dedup | `VecDeque<Uuid>` | `collections.deque` | At-least-once -> effectively-once |
| 7 | FNV-1a partitioner | `partition_for(&Uuid, n)` | hashlib.fnv1a_64 | Stable key->partition mapping |
| 8 | Outbox pattern | `OutboxRow -> EventEnvelope` | row + dict | Transactional event publishing |
| 9 | Tracing | `tracing::info!` | `logging.info` | Structured logs in async land |
| 10 | Docker broker | `kafka:9092` | localhost:9092 | Reproducible infra |

---

## Table of Contents
1. [Introduction](#1-introduction)
2. [Prerequisites](#2-prerequisites)
3. [Setup: Create the Project from Scratch](#3-setup-create-the-project-from-scratch)
4. [Concept: rdkafka Producer/Consumer](#4-concept-rdkafka-producerconsumer)
5. [Concept: At-least-once vs Effectively-once](#5-concept-at-least-once-vs-effectively-once)
6. [Concept: Tokio + StreamConsumer](#6-concept-tokio--streamconsumer)
7. [Concept: Outbox Pattern](#7-concept-outbox-pattern)
8. [Step-by-Step Implementation](#8-step-by-step-implementation)
9. [Running End-to-End](#9-running-end-to-end)
10. [Summary](#10-summary)

---

## 1. Introduction

We build a Rust client for a single-broker Kafka cluster running in Docker
(see `../docker-compose.yml`). The library exposes:

- `EventEnvelope` — the canonical event format (UUIDv4 id, aggregate id,
  event type, JSON payload, UTC timestamp).
- `producer_config` / `consumer_config` — pure builders returning an
  rdkafka `ClientConfig` (no I/O).
- `produce_one` — async wrapper that returns the broker's
  `(partition, offset)` tuple.
- `DedupCache` — FIFO id-remembering cache to collapse redeliveries.
- `outbox_row_to_envelope` — bridge from a PostgreSQL outbox row to a
  Kafka event.

This project is the first half of the Data Infrastructure section.
Project 02-PostgreSQLSqlx will give us the outbox **table** to read from;
Project 07-CdcPipeline will use Debezium to stream that table *into* Kafka
so we don't have to poll it.

## 2. Prerequisites

- Rust 1.75+ (rustup default stable is fine; we use 1.96 in WSL)
- Docker Compose: `cd .. && docker compose up -d kafka`
- Topics: `auto.create.topics.enable=true` is on in our compose file,
  so no `kafka-topics` calls are needed.
- Concept: Tokio async (Section 05/02-Futures) — read that first if
  `.await` looks foreign.

## 3. Setup: Create the Project from Scratch

```bash
cargo new --lib kafka_rdkafka
cd kafka_rdkafka
# Replace Cargo.toml with the one in workshop/Cargo.toml
cargo test   # 13 tests should fail with "not yet implemented"
```

The `workshop/Cargo.toml` adds:

- `rdkafka` (librdkafka via cmake build)
- `tokio` (rt-multi-thread + macros + time + signal)
- `serde`, `serde_json`, `uuid`, `chrono`
- `tracing` + `tracing-subscriber`
- `anyhow`, `thiserror`

## 4. Concept: rdkafka Producer/Consumer

**In Python** you build a `confluent_kafka.Producer(config)` from a dict and
call `producer.produce(topic, value=..., key=..., callback=...)`. The
callback runs on the librdkafka poll thread.

**In Rust** `rdkafka` exposes:

- `FutureProducer<C = DefaultProducerContext>` — an async producer whose
  `send` returns a `Future<Output = Result<(i32, i64), (KafkaError, OwnedMessage)>>`.
  The error is **paired with the message** so you can re-send or log it.
- `StreamConsumer` — a `tokio::sync::mpsc` of messages, so you `await` on
  `consumer.recv().await` like any other async source.

```rust
// Spawn a one-shot produce
let producer: FutureProducer = ClientConfig::new()
    .set("bootstrap.servers", "kafka:9092")
    .set("enable.idempotence", "true")
    .create()?;

let record: FutureRecord<'_, Vec<u8>, Vec<u8>> =
    FutureRecord::to("order.order")
        .key(b"order:550e8400-...")
        .payload(&serde_json::to_vec(&event)?);
let (partition, offset) = producer.send(record, Duration::from_secs(5))
    .await
    .map_err(|(e, _)| e)?;
```

## 5. Concept: At-least-once vs Effectively-once

**At-least-once** is the *broker's* guarantee: it never loses a message,
but it can redeliver. **Effectively-once** is what the *application*
must enforce: dedup by event id.

`enable.idempotence=true` on the producer prevents broker-side
duplicates within a single producer session (no two records share
the same `(producer_id, sequence_number)`), but cross-session
duplicates (rebalance, restart) still need an application-side
dedup. That is `DedupCache` in this project.

## 6. Concept: Tokio + StreamConsumer

`StreamConsumer` implements `Stream<Item = Result<BorrowedMessage, KafkaError>>`
when wrapped in `tokio::pin!` — so you can use `next().await` or the
shorter `recv().await`. Combined with `tokio::time::timeout`, you can
poll for "drain whatever's in the buffer in the next 100 ms" patterns
common in batch jobs.

## 7. Concept: Outbox Pattern

Writing to a database row and publishing to Kafka in the same
transaction is impossible without a distributed transaction. The
outbox pattern writes a row to `outbox` in the **same DB transaction**
as the business write; a separate poller (us in Project 07 via
Debezium, here directly) reads `outbox` and publishes to Kafka.

`OutboxRow` mirrors the table created by
`14-DataInfrastructure/init/postgres/01-init.sql`.

## 8. Step-by-Step Implementation

Work through the tests in order. Run after each step:

```bash
cargo test step_01_domain_types       # 2 tests
cargo test step_02_client_config      # 2 tests
cargo test step_03_topic_naming       # 2 tests
cargo test step_04_serde              # 3 tests
cargo test step_05_outbox             # 1 test
cargo test step_06_dedup              # 2 tests
cargo test step_07_partition          # 2 tests
cargo test step_08_produce            # 2 tests
```

All `todo!()` bodies live in `src/lib.rs` — replace each with the
implementation shown in the matching "Concept" section above.

## 9. Running End-to-End

```bash
# From the section root
docker compose up -d kafka
cargo run --release --manifest-path 01-KafkaRdkafka/workshop/Cargo.toml
```

You should see:

```
INFO connected producer to kafka:9092
INFO produced order.created topic=order.order partition=0 offset=0
INFO produced order.paid via outbox topic=order.order partition=0 offset=1
INFO consumed (new) event_type=order.created
INFO consumed (new) event_type=order.paid
```

Open http://localhost:8080 (Kafka UI) to inspect the `order.order` topic.

## 10. Summary

| Concept | Where used | Next project |
|---------|-----------|--------------|
| `EventEnvelope` (UUIDv4, aggregate id) | All data infra projects | 02-PostgreSQLSqlx (writes the outbox) |
| Idempotent `FutureProducer` | `produce_one` | 04-ClickHouseIngestion (sink) |
| `DedupCache` | `main.rs` consumer loop | 07-CdcPipeline (Debezium redelivery) |
| Outbox row | `OutboxRow` | 02-PostgreSQLSqlx (table source) |
| FNV-1a partitioner | `partition_for` | 05-ApacheIggy (alternative broker) |

## Further Reading

- [rdkafka crate docs](https://docs.rs/rdkafka/0.36) — full API surface
- [Apache Kafka — Idempotent Producer](https://kafka.apache.org/documentation/#producerconfigs_enable.idempotence) — broker-side guarantees
- [Debezium Outbox Pattern](https://debezium.io/documentation/reference/stable/transformations/outbox-event-router.html) — production outbox
- [Tokio `StreamConsumer` example](https://github.com/fede1024/rust-rdkafka/blob/master/examples/asynchronous_processing.rs) — async producer/consumer pattern

## Exercises

* **Easy** – modify the existing function to handle an extra edge case.
* **Medium** – extend the project with a new helper function that re‑uses the core logic.

