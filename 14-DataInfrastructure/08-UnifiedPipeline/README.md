# 🦀 Unified Pipeline — Compose Kafka, ClickHouse, DuckLake, Redis

*Subtitle: one orchestrator, many sinks, retries, dead-letters, and live counters.*

> **Test-driven approach**: This project includes a Cargo project with progressive
> unit tests. Each function in `src/lib.rs` starts as a `todo!()` stub. As you
> follow each section, replace `todo!()` with real code and run `cargo test` to
> watch the pass count grow. Your goal: **all 14 tests pass**.

---

## Why a unified pipeline?

**Python pain:** Each Python data service reinvents the same
plumbing: pull from Kafka, transform, write to ClickHouse, retry
on failure, dead-letter on poison messages, checkpoint for
restarts. Every team writes its own `airbyte/`, `prefect/`,
`dagster/`, `kafka-connect/`, or `custom-script.py`. None of them
share a common shape.

**Rust fix:** A single `PipelineConfig` + `PipelineEvent` +
`SinkOutcome` triad gives every team the same skeleton. The
**orchestrator** is a small `for event in stream: fanout(event)`
loop; the **sinks** are independent `async` tasks. You can
swap ClickHouse for DuckHouse, Kafka for Iggy, or add a Redis
cache layer, without rewriting the pipeline.

This is the capstone of Section 14 — it composes Projects 01–07.

## At a Glance

| # | Concept | Rust | Python | Why it matters |
|---|---------|------|--------|----------------|
| 1 | Pipeline config | `PipelineConfig` | dataclass | Single source of truth |
| 2 | Canonical event | `PipelineEvent` | dict | Type-safe handoff |
| 3 | Sink outcome | `SinkOutcome` | tuple | Per-sink success/error |
| 4 | Fan-out | `fanout_targets` | `if "kafka" in sinks:` | Pluggable routing |
| 5 | Live counters | `WindowCounters` | `Counter` | Per-type metrics |
| 6 | Backoff | `sink_backoff_ms` | `tenacity` | Retry policy |
| 7 | Dead-letter | `DeadLetter` | dict | Poison-message capture |
| 8 | Pipeline stats | `PipelineStats` | dict | Success rate / throughput |

---

## Table of Contents
1. [Introduction](#1-introduction)
2. [Prerequisites](#2-prerequisites)
3. [Setup: Create the Project from Scratch](#3-setup-create-the-project-from-scratch)
4. [Concept: Sinks as Async Tasks](#4-concept-sinks-as-async-tasks)
5. [Concept: Fan-out vs Fan-in](#5-concept-fan-out-vs-fan-in)
6. [Concept: Dead-Letter Strategy](#6-concept-dead-letter-strategy)
7. [Step-by-Step Implementation](#7-step-by-step-implementation)
8. [Running End-to-End](#8-running-end-to-end)
9. [Summary](#9-summary)

---

## 1. Introduction

This is the **capstone** project. It composes every earlier
project into a single orchestrator:

- **Source**: Project 07's CDC pipeline (Postgres logical replication)
- **Transform**: this project (renames, validates, fans out)
- **Sinks**:
  - Project 01 (Kafka) — event log for downstream consumers
  - Project 04 (ClickHouse) — operational analytics
  - Project 06 (DuckLake) — historical lakehouse
  - Project 03 (Redis) — hot cache

The library exposes:

- `PipelineConfig` — name, source, sinks, batch sizes
- `PipelineEvent` — canonical event
- `SinkOutcome` — per-sink success / error
- `fanout_targets` — routing policy
- `WindowCounters` — live per-type metrics
- `sink_backoff_ms` — retry policy
- `DeadLetter` — poison-message capture
- `PipelineStats` — run-level summary

## 2. Prerequisites

- Rust 1.75+ (1.96 recommended)
- All previous projects (01-07) — they are the *sinks*
- Concept: async-trait (Project 07)
- Concept: futures, tokio (Section 05/02-Futures)

## 3. Setup: Create the Project from Scratch

```bash
cargo new --lib unified_pipeline
cd unified_pipeline
# Replace Cargo.toml with workshop/Cargo.toml
cargo test   # 14 tests should fail with "not yet implemented"
```

## 4. Concept: Sinks as Async Tasks

Each sink is an independent `async` task. The orchestrator:

```rust
let sinks: Vec<Box<dyn Sink>> = vec![
    Box::new(KafkaSink::new(...)),
    Box::new(ClickHouseSink::new(...)),
    Box::new(DuckLakeSink::new(...)),
    Box::new(RedisSink::new(...)),
];
for event in source {
    for sink in &sinks {
        sink.send(&event).await?;   // can run in parallel with futures::join!
    }
}
```

The `Sink` trait from Project 07 makes this trivial; the
unified pipeline doesn't need to know *how* each sink writes —
only that it implements `send` / `flush`.

## 5. Concept: Fan-out vs Fan-in

- **Fan-out**: one source → many sinks. This project.
- **Fan-in**: many sources → one sink. Project 07 (multiple
  Postgres tables → one Kafka topic).
- **Shuffle**: one source → one sink with routing. Project 01's
  `routing_key` + `partition_for`.

Most production pipelines are fan-out. The unified pipeline is
the canonical fan-out skeleton.

## 6. Concept: Dead-Letter Strategy

When a sink fails N times in a row, the event is moved to a
dead-letter queue (DLQ). DLQs serve three purposes:

1. **Stop the bleeding** — the pipeline keeps moving instead of
   being stuck on a poison message
2. **Diagnose** — operators inspect the DLQ to see what went
   wrong (bad schema? broker outage? bug?)
3. **Replay** — once the root cause is fixed, the DLQ is drained
   back into the main pipeline

`DeadLetter` captures all of that metadata.

## 7. Step-by-Step Implementation

Work through the tests in order:

```bash
cargo test step_01_config        # 2 tests
cargo test step_02_event         # 2 tests
cargo test step_03_outcome       # 2 tests
cargo test step_04_fanout
cargo test step_05_counters      # 2 tests
cargo test step_06_retry         # 2 tests
cargo test step_07_dlq
cargo test step_08_stats         # 2 tests
```

All `todo!()` bodies live in `src/lib.rs`.

## 8. Running End-to-End

```bash
# From the section root
docker compose up -d postgres kafka clickhouse minio redis
cargo run --release --manifest-path 08-UnifiedPipeline/workshop/Cargo.toml
```

You should see:

```
INFO pipeline config name=orders-pipeline source=dataeng.orders sinks=4
INFO window counters counters_total=5 errors=1 types=2
WARN sink=kafka error="broker timeout" retry_in_ms=Some(100) sink failed, would retry
INFO pipeline stats in=5 out=19 fails=1 dlq=1 success_rate=0.95
```

(Exact numbers depend on the simulated failure pattern.)

## 9. Summary

This is the end of Section 14. By working through the eight
projects you have built:

- **Wave 1** (foundations):
  1. Kafka with rdkafka — event streaming
  2. PostgreSQL with sqlx — transactional outbox
  3. Redis with redis-rs — cache + streams
  4. ClickHouse with clickhouse-rs — OLAP sink
- **Wave 2** (composition):
  5. Apache Iggy — alternative Rust-native broker
  6. DuckLake — SQL-on-Parquet lakehouse
  7. CDC pipeline — Debezium-style change capture
  8. Unified pipeline — fan-out orchestrator

The composition diagram:

```
PostgreSQL ─CDC→ Project 07 ─→ Project 08 (orchestrator) ─→ Kafka (Project 01)
                                       │
                                       ├──→ ClickHouse (Project 04)
                                       │
                                       ├──→ DuckLake (Project 06)
                                       │
                                       ├──→ Apache Iggy (Project 05)
                                       │
                                       └──→ Redis (Project 03)
```

## Further Reading

- [Designing Data-Intensive Applications](https://dataintensive.net/) — Ch. 11 (Stream Processing)
- [Apache Airflow vs Rust pipelines](https://www.prefect.io/blog/data-pipeline-orchestration-rust) — when to use which
- [The Log: What every software engineer should know](https://engineering.linkedin.com/distributed-systems/log-what-every-software-engineer-should-know-about-real-time-datas-unifying) — LinkedIn's log philosophy
- [Tokio tutorial](https://tokio.rs/tokio/tutorial) — async pipelines

## Exercises

* **Easy** – modify the existing function to handle an extra edge case.
* **Medium** – extend the project with a new helper function that re‑uses the core logic.

