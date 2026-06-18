# 🦀 Apache Iggy — Rust-Native Message Streaming

*Subtitle: produce and consume on a thread-per-core message broker built in Rust.*

> **Test-driven approach**: This project includes a Cargo project with progressive
> unit tests. Each function in `src/lib.rs` starts as a `todo!()` stub. As you
> follow each section, replace `todo!()` with real code and run `cargo test` to
> watch the pass count grow. Your goal: **all 15 tests pass**.

---

## What Is Apache Iggy?

Rust-native message streaming — thread-per-core, Kafka-style consumer groups, no ZooKeeper.

### Python equivalent

```python
from kafka import KafkaProducer, KafkaConsumer

producer = KafkaProducer(bootstrap_servers="localhost:9092")
producer.send("events", b"hello")

consumer = KafkaConsumer("events", group_id="my-group")
for msg in consumer:
    print(msg.value)
``` Lighter alternatives (NATS, Redis
Streams) lack Kafka's consumer-group semantics.

**Rust fix:** Apache Iggy (incubating, project-of-record since Feb
2025) is a Rust-native message broker with:

- **Thread-per-core** I/O (`io_uring` on Linux)
- **Kafka-style consumer groups** without ZooKeeper
- **HTTP + TCP + QUIC** protocols
- **Disk-backed storage** (mmap'd segment files)
- Single ~30 MB binary, 10-second startup

For data engineers who already know Kafka, Iggy is the "what if
Kafka were a single Rust binary?" question. This project lets you
benchmark and validate that path.

## At a Glance

| # | Concept | Rust | Python | Why it matters |
|---|---------|------|--------|----------------|
| 1 | TCP/HTTP/QUIC | `tcp_url` / `http_url` | `kafka-python` | Three protocols, one API |
| 2 | Domain envelope | `IggyMessage` | `dict` | Self-describing payload |
| 3 | Stream/topic/partition | `stream_for` / `topic_for` | `f"stream:{x}"` | Centralized naming |
| 4 | Stable partition | `partition_for` (FNV-1a) | `hash(key) % N` | Same key -> same partition |
| 5 | Thread-per-core | `consumer_parallelism` | n/a | Match consumers to cores |
| 6 | Offset cursor | `OffsetCursor` | `kafka.consumer` | Manual commit semantics |
| 7 | In-memory dedup | `IggyDedup` | `deque` | At-least-once -> effectively-once |
| 8 | Wire format | `encode` / `decode` (JSON) | `json.dumps` | Cross-language readable |

---

## Table of Contents
1. [Introduction](#1-introduction)
2. [Prerequisites](#2-prerequisites)
3. [Setup: Create the Project from Scratch](#3-setup-create-the-project-from-scratch)
4. [Concept: Iggy vs Kafka](#4-concept-iggy-vs-kafka)
5. [Concept: Thread-per-Core](#5-concept-thread-per-core)
6. [Concept: Consumer Groups in Iggy](#6-concept-consumer-groups-in-iggy)
7. [Step-by-Step Implementation](#7-step-by-step-implementation)
8. [Running End-to-End](#8-running-end-to-end)
9. [Summary](#9-summary)

---

## 1. Introduction

Apache Iggy is the **alternative broker** in our data infrastructure
section. The library here is intentionally close to the
Kafka project's API surface (`encode` / `decode` / `partition_for`)
so switching from one broker to the other is a search-and-replace.

Key features exposed:

- `IggyMessage` — UUID-keyed envelope with optional routing key + headers
- `stream_for` / `topic_for` / `partition_for` — naming helpers
- `default_partition_count` / `consumer_parallelism` — sizing helpers
- `OffsetCursor` — manual-commit cursor
- `IggyDedup` — in-memory FIFO dedup

## 2. Prerequisites

- Rust 1.75+ (1.96 recommended)
- Docker Compose: `docker compose up -d iggy`
- Concept: Kafka (Project 01) — the parallel structure makes this
  a breeze if you have done Kafka first
- Concept: FNV-1a hash (Project 01, step 7)

## 3. Setup: Create the Project from Scratch

```bash
cargo new --lib apache_iggy
cd apache_iggy
# Replace Cargo.toml with workshop/Cargo.toml
cargo test   # 15 tests should fail with "not yet implemented"
```

## 4. Concept: Iggy vs Kafka

| | **Kafka** | **Iggy** |
|---|---|---|
| Language | Java/Scala | Rust |
| Coordination | KRaft (or ZooKeeper) | Built-in (no controller) |
| Protocols | TCP | TCP, HTTP, QUIC |
| Storage | Log segments (`.log`) | Log segments (`.bin` mmap) |
| Throughput target | 1M+ msg/s | 5M+ msg/s (single node) |
| Consumer groups | Yes | Yes (single-broker) |
| Footprint | 2 GB heap | 30 MB binary |

If your data engineering team is allergic to JVM, Iggy is the
"zero-JVM Kafka" answer. We keep both in this section so you can
A/B benchmark.

## 5. Concept: Thread-per-Core

Iggy uses a thread-per-core architecture (similar to `glommio`,
`seastar`). Each CPU core has a dedicated thread that owns its
sockets and runs its share of partitions. The implication:

- One consumer per partition is fine; *two* consumers per partition
  is wasteful (one will be parked).
- `consumer_parallelism` caps at the number of physical cores —
  see `step_8_parallelism`.

## 6. Concept: Consumer Groups in Iggy

Unlike Kafka, Iggy has a built-in coordinator (no external metadata
service). You create a group with `iggy::client::Client::create_group(...)`,
then each consumer polls its assigned partition. The broker
balances partitions to consumers on join/leave.

## 7. Step-by-Step Implementation

Work through the tests in order:

```bash
cargo test step_01_connection   # 2 tests
cargo test step_02_domain       # 2 tests
cargo test step_03_naming       # 3 tests
cargo test step_04_partition    # 2 tests
cargo test step_05_serde        # 1 test
cargo test step_06_cursor       # 3 tests
cargo test step_07_dedup        # 1 test
cargo test step_08_parallelism  # 2 tests
```

All `todo!()` bodies live in `src/lib.rs`.

## 8. Running End-to-End

```bash
# From the section root
docker compose up -d iggy
cargo run --release --manifest-path 05-ApacheIggy/workshop/Cargo.toml
```

You should see:

```
INFO iggy plan stream=stream:orders topic=order.order partitions=N consumers=<=N
INFO encoded message id=... key=order:... partition=N bytes=...
INFO round-trip ok
INFO consumer state offset=1 dedup_size=1
```

## 9. Summary

| Concept | Where used | Next project |
|---------|-----------|--------------|
| `IggyMessage` | `main.rs` | 06-DuckLakeCatalog (lake writer) |
| `partition_for` | `main.rs` | 07-CdcPipeline (Debezium routing) |
| `OffsetCursor` | `main.rs` | 08-UnifiedPipeline (multi-broker cursor) |
| `IggyDedup` | `main.rs` | 01-KafkaRdkafka (parity) |

## Further Reading

- [Apache Iggy (incubating)](https://github.com/apache/iggy) — main repo
- [iggy crate docs](https://docs.rs/iggy) — Rust SDK
- [Iggy architecture deep-dive](https://iggy.rs/blog) — thread-per-core + io_uring
- [Iggy vs Kafka benchmark](https://iggy.rs/blog) — single-node throughput

## Exercises

* **Easy** – modify the existing function to handle an extra edge case.
* **Medium** – extend the project with a new helper function that re‑uses the core logic.

