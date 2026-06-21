# 🦀 ClickHouse Ingestion — Columnar OLAP Sink

*Subtitle: write millions of rows to ClickHouse with batched inserts, retry, and per-minute aggregations.*

> **Test-driven approach**: This project includes a Cargo project with progressive
> unit tests. Each function in `src/lib.rs` starts as a `todo!()` stub. As you
> follow each section, replace `todo!()` with real code and run `cargo test` to
> watch the pass count grow. Your goal: **all 14 tests pass**.

---

## What Is This Project?

ClickHouse ingestion with batched inserts, retry, and backpressure control.

### Python equivalent

```python
from clickhouse_driver import Client

client = Client("localhost")
client.execute("INSERT INTO events VALUES", [(1, "a"), (2, "b")])
# Manual batching, no built-in retry or backpressure
``` A small Python service can accidentally DoS a ClickHouse
cluster with thousands of small inserts per second.

**Rust fix:** `clickhouse-rs` (the official client) gives you a
`Client` whose `insert(...).await` is *one batch*. We wrap it in an
`IngestBatcher` that flushes on row count **or** byte size, and a
`ClickHouseRetry` that backs off exponentially. The result is
predictable throughput that the rest of the section (Project 08's
unified pipeline) can rely on.

In this project you'll learn to build this in Rust — and along the way
you'll discover **ClickHouse**, **batched inserts**, and **exponential backoff**.

### Topics covered

| # | Concept | Why it matters |
|---|---------|----------------|
| 1 | Row structs | Wire-format types |
| 2 | Status enum | Prevent string drift |
| 3 | Batcher | Bounded memory + throughput |
| 4 | Retry policy | Predictable failure recovery |
| 5 | Aggregations | Local rollup for testing |

---

## Table of Contents
1. [Introduction](#1-introduction)
2. [Prerequisites](#2-prerequisites)
3. [Setup: Create the Project from Scratch](#3-setup-create-the-project-from-scratch)
4. [Concept: ClickHouse vs Postgres](#4-concept-clickhouse-vs-postgres)
5. [Concept: Batch Sizing](#5-concept-batch-sizing)
6. [Concept: MergeTree + ORDER BY](#6-concept-mergetree--order-by)
7. [Step-by-Step Implementation](#7-step-by-step-implementation)
8. [Running End-to-End](#8-running-end-to-end)
9. [Summary](#9-summary)

---

## 1. Introduction

This is the **OLAP destination** of the data infrastructure section.
ClickHouse is a columnar database optimized for analytical queries over
billions of rows. In our pipeline:

- Project 02-PostgreSQLSqlx produces `orders` rows
- Project 01-KafkaRdkafka streams them as events
- Project 04 (this one) writes them into `analytics.orders`
- Project 08-UnifiedPipeline composes the above

The library:

- Builds HTTP/native URLs.
- Exposes row types matching the schema.
- Provides DDL/INSERT SQL fragments.
- Implements a row-count *and* byte-size aware batcher.
- Implements a retry policy with exponential backoff capped at 30s.
- Projects rows into per-minute buckets for local testing.

## 2. Prerequisites

- Rust 1.75+ (1.96 recommended)
- Docker Compose: `docker compose up -d clickhouse`
- Concept: sqlx (Project 02)
- Concept: serde + chrono (Section 04/02-CSVWriter, 01-Foundations)

## 3. Setup: Create the Project from Scratch

```bash
cargo new --lib clickhouse_ingestion
cd clickhouse_ingestion
# Replace Cargo.toml with workshop/Cargo.toml
cargo test   # 14 tests should fail with "not yet implemented"
```

## 4. Concept: ClickHouse vs Postgres

- **Postgres**: row-oriented OLTP. Each `SELECT *` returns a full
  tuple. Good for thousands of writes per second.
- **ClickHouse**: column-oriented OLAP. Each column is stored
  contiguously. Compression is 5–10x better; aggregations over
  billions of rows return in milliseconds.

In our pipeline Postgres is the *source of truth*; ClickHouse is the
*queryable projection*. We never delete from ClickHouse; we replace
partitions (Project 08).

## 5. Concept: Batch Sizing

A `clickhouse-rs` `insert` opens one HTTP request per call. Per-row
inserts at 10k rows/sec means 10k HTTP requests — your TLS handshake
alone is the bottleneck. The cure:

```
Batcher:
  push row
  if rows >= 10_000 OR bytes >= 10_000_000:
    flush
```

The `IngestBatcher` in this project exposes both thresholds.

## 6. Concept: MergeTree + ORDER BY

```sql
ENGINE = MergeTree()
PARTITION BY toYYYYMM(created_at)
ORDER BY (created_at, id)
```

- `MergeTree` is the default engine; it stores data sorted by
  `ORDER BY` and merges parts in the background.
- `PARTITION BY toYYYYMM(created_at)` creates a new part per month
  → fast `DROP PARTITION` for retention.
- `ORDER BY (created_at, id)` is the primary key; queries that
  filter or sort by `created_at` are 10–100x faster.

## 7. Step-by-Step Implementation

Work through the tests in order:

```bash
cargo test step_01_connection   # 2 tests
cargo test step_02_domain       # 1 test
cargo test step_03_ddl          # 2 tests
cargo test step_04_status       # 2 tests
cargo test step_05_aggregates   # 2 tests
cargo test step_06_batcher      # 2 tests
cargo test step_07_retry        # 2 tests
cargo test step_08_aggregations
```

All `todo!()` bodies live in `src/lib.rs`.

## 8. Running End-to-End

```bash
# From the section root
docker compose up -d clickhouse
cargo run --release --manifest-path 04-ClickHouseIngestion/workshop/Cargo.toml
```

You should see:

```
INFO connecting to http://localhost:8123/analytics
INFO ensured analytics.orders exists
INFO inserted batch
INFO ingested 50 rows total_qty=1275
INFO readback=50 revenue=12745.0 buckets=1
```

## 9. Summary

| Concept | Where used | Next project |
|---------|-----------|--------------|
| `Client::insert` | `main.rs::insert_batch` | 08-UnifiedPipeline (multi-sink) |
| `IngestBatcher` | `main.rs` | 05-ApacheIggy (stream buffer) |
| `ClickHouseRetry` | `insert_batch` | 07-CdcPipeline (Kafka sink) |
| `OrderStatus` enum | `main.rs` | 02-PostgreSQLSqlx (source) |
| `project_minute_buckets` | `main.rs` | 08-UnifiedPipeline (live dashboard) |

## Further Reading

- [clickhouse-rs async docs](https://docs.rs/clickhouse/0.13) — `Client`, `insert`, `fetch_all`
- [ClickHouse MergeTree](https://clickhouse.com/docs/en/engines/table-engines/mergetree-family/mergetree) — engine reference
- [Batching best practices](https://clickhouse.com/docs/en/cloud/bestpractices/batching-inserts) — when and how to batch
- [ClickHouse Kafka table engine](https://clickhouse.com/docs/en/engines/table-engines/integrations/kafka) — direct Kafka sink (Project 07 alternative)

## Exercises

* **Easy** – modify the existing function to handle an extra edge case.
* **Medium** – extend the project with a new helper function that re‑uses the core logic.

