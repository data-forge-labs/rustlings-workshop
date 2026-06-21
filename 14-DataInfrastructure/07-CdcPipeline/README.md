# 🦀 CDC Pipeline — Debezium-style Change Data Capture in Rust

*Subtitle: stream Postgres row-level changes through a typed, async, trait-based pipeline.*

> **Test-driven approach**: This project includes a Cargo project with progressive
> unit tests. Each function in `src/lib.rs` starts as a `todo!()` stub. As you
> follow each section, replace `todo!()` with real code and run `cargo test` to
> watch the pass count grow. Your goal: **all 15 tests pass**.

---

## What Is This Project?

Change Data Capture pipeline — streaming Postgres row-level changes through a typed async pipeline.

### Python equivalent

```python
# Debezium (JVM) + Kafka Connect + Python consumer
# JVM alone is 4 GB, Python GIL limits parallelism
from confluent_kafka import Consumer

consumer = Consumer({"bootstrap.servers": "localhost:9092"})
consumer.subscribe(["dbserver1.public.orders"])
```

- Replicate the same envelope shape (`before`, `after`, `op`,
  `ts_ms`, `tx_id`)
- Use a `Sink` trait so we can swap Kafka/ClickHouse/DuckLake
  without changing pipeline code
- Run on Tokio with no GIL

This project ships the **envelope shape** and the **plumbing**
(leader claim, checkpoint, filter, batching). The actual
`wal2json` / `pgoutput` decoder is a thin addition in `main.rs`.

### Topics covered

| # | Concept | Why it matters |
|---|---------|----------------|
| 1 | CDC envelope | Debezium-compatible shape |
| 2 | Op codes & routing | Compact serial form, partition by row id |
| 3 | Leader claim | Single active pipeline per table |
| 4 | Sink trait | Pluggable destinations |
| 5 | Checkpoint | Restart-safe progress |
| 6 | Batch decision | Size / age / tx boundary |

---

## Table of Contents
1. [Introduction](#1-introduction)
2. [Prerequisites](#2-prerequisites)
3. [Setup: Create the Project from Scratch](#3-setup-create-the-project-from-scratch)
4. [Concept: Debezium Envelope](#4-concept-debezium-envelope)
5. [Concept: Logical Replication](#5-concept-logical-replication)
6. [Concept: Sink Trait](#6-concept-sink-trait)
7. [Step-by-Step Implementation](#7-step-by-step-implementation)
8. [Running End-to-End](#8-running-end-to-end)
9. [Summary](#9-summary)

---

## 1. Introduction

This is the **integration** project. It ties the previous six
together:

- Project 02 (PostgreSQL) → row-level changes stream out via
  `wal2json` or `pgoutput`
- Project 01 (Kafka) or 05 (Iggy) → row events land on a topic
- Project 04 (ClickHouse) or 06 (DuckLake) → row events land
  in a columnar store

The library models:

- `CdcEvent` — Debezium-style envelope (`before`, `after`, `op`,
  `ts_ms`, `tx_id`)
- `CdcOp` — operation enum with single-char serialization
- `topic_for` / `routing_key` — partition-by-row-id
- `should_forward` — drop `Read` (snapshot) and soft-delete events
- `LeaderClaim` — TTL-based leader election
- `Sink` trait + `InMemorySink` — pluggable destinations
- `Checkpoint` — restart-safe progress
- `batch_ready` — size / age / tx-boundary decision

## 2. Prerequisites

- Rust 1.75+ (1.96 recommended)
- Docker Compose: `docker compose up -d postgres kafka` (or `iggy`)
- Concept: async traits (Section 05/02-Futures)
- Concept: serde (Section 04/02-CSVWriter)
- Concept: Outbox pattern (Project 02)

## 3. Setup: Create the Project from Scratch

```bash
cargo new --lib cdc_pipeline
cd cdc_pipeline
# Replace Cargo.toml with workshop/Cargo.toml
cargo test   # 15 tests should fail with "not yet implemented"
```

## 4. Concept: Debezium Envelope

A Debezium CDC event has the shape:

```json
{
  "before": { "id": "...", "status": "pending" },
  "after":  { "id": "...", "status": "paid" },
  "op": "u",
  "ts_ms": 1700000000000,
  "source": { "db": "dataeng", "table": "orders" },
  "transaction": { "id": "..." }
}
```

We strip the `source` and `transaction` wrappers to keep our
`CdcEvent` flat — the routing helpers fill the same role.

## 5. Concept: Logical Replication

Postgres 10+ ships **logical replication** as a first-class
feature. Setting `wal_level=logical` (we did in `docker-compose.yml`)
turns the WAL into a stream of `INSERT/UPDATE/DELETE` row
events. The `pgoutput` plugin (default) emits Debezium-compatible
JSON. Our pipeline ingests that stream, drops `Read` (snapshot)
events, and forwards everything else to the sink.

## 6. Concept: Sink Trait

The `Sink` trait abstracts the destination:

```rust
#[async_trait]
trait Sink: Send + Sync {
    async fn send(&self, event: &CdcEvent) -> anyhow::Result<()>;
    async fn flush(&self) -> anyhow::Result<()>;
}
```

A real `KafkaSink` would call `produce_one`; a `ClickHouseSink`
would batch into `IngestBatcher`; a `DuckLakeSink` would run
`MERGE INTO`. They all share the same pipeline code.

## 7. Step-by-Step Implementation

Work through the tests in order:

```bash
cargo test step_01_envelope     # 3 tests
cargo test step_02_routing      # 2 tests
cargo test step_03_filter       # 3 tests
cargo test step_04_leader       # 2 tests
cargo test step_05_06_sink      # 2 tests
cargo test step_07_checkpoint   # 2 tests
cargo test step_08_batching     # 4 tests
```

All `todo!()` bodies live in `src/lib.rs`.

## 8. Running End-to-End

```bash
# From the section root
docker compose up -d postgres kafka
cargo run --release --manifest-path 07-CdcPipeline/workshop/Cargo.toml
```

You should see:

```
INFO forwarded topic=orders.cdc.c key=order:... op=c
INFO forwarded topic=orders.cdc.u key=order:... op=u
INFO filtered op=Read
INFO leader claim holder=worker-1 valid=true
INFO batch decision in_memory=2 batch_ready=true
INFO checkpoint processed=2 last_lsn=1
```

## 9. Summary

| Concept | Where used | Next project |
|---------|-----------|--------------|
| `CdcEvent` | `main.rs` | 08-UnifiedPipeline (consumes CDC) |
| `Sink` trait | `InMemorySink` | 08 (KafkaSink, ClickHouseSink, DuckLakeSink) |
| `Checkpoint` | `main.rs` | 08 (resumable pipelines) |
| `LeaderClaim` | `main.rs` | 08 (single active worker) |
| `batch_ready` | `main.rs` | 08 (multi-sink batching) |

## Further Reading

- [Debezium documentation](https://debezium.io/documentation/reference/stable/index.html) — envelope format
- [Postgres logical replication](https://www.postgresql.org/docs/current/logical-replication.html) — `wal2json`, `pgoutput`
- [async-trait crate](https://docs.rs/async-trait) — async methods in traits
- [At-least-once vs exactly-once](https://martin.kleppmann.com/blog/2016/02/08/how-to-do-distributed-locking.html) — why dedup matters

## Exercises

* **Easy** – modify the existing function to handle an extra edge case.
* **Medium** – extend the project with a new helper function that re‑uses the core logic.

