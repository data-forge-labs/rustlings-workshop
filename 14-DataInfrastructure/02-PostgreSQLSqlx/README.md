# 🦀 PostgreSQL Sqlx — Transactional Outbox with sqlx

*Subtitle: write orders + outbox events in one PostgreSQL transaction with compile-time-checked SQL.*

> **Test-driven approach**: This project includes a Cargo project with progressive
> unit tests. Each function in `src/lib.rs` starts as a `todo!()` stub. As you
> follow each section, replace `todo!()` with real code and run `cargo test` to
> watch the pass count grow. Your goal: **all 13 tests pass**.

---

## What Is This Project?

Transactional outbox pattern with sqlx — compile-time-checked SQL queries.

### Python equivalent

```python
from sqlalchemy import create_engine, Column, Integer, String

engine = create_engine("postgresql://localhost/mydb")
# ORM: convenient but typos surface in production, not at build time
```

**Rust fix:** `sqlx` with the `query!` macro does **compile-time SQL
verification** against a live database (or `sqlx-cli`-managed cache). You
*cannot* ship a query with a typo or a missing column. And `sqlx` is
async-first, so it composes with `tokio` and `axum` without a GIL.

### Topics covered

| # | Concept | Why it matters |
|---|---------|----------------|
| 1 | Async connection pool | Bounded concurrency, auto-reconnect |
| 2 | Compile-time SQL check | Type-safe queries |
| 3 | Transaction | Atomic business writes |
| 4 | Outbox table | Transactional event publishing |
| 5 | Row converters | Domain types from DB rows |
| 6 | Exponential backoff | Retry policy for failed publishes |

---

## Table of Contents
1. [Introduction](#1-introduction)
2. [Prerequisites](#2-prerequisites)
3. [Setup: Create the Project from Scratch](#3-setup-create-the-project-from-scratch)
4. [Concept: sqlx vs SQLAlchemy](#4-concept-sqlx-vs-sqlalchemy)
5. [Concept: Transactional Outbox](#5-concept-transactional-outbox)
6. [Concept: Batching + Retry](#6-concept-batching--retry)
7. [Step-by-Step Implementation](#7-step-by-step-implementation)
8. [Running End-to-End](#8-running-end-to-end)
9. [Summary](#9-summary)

---

## 1. Introduction

This is the **source database** of the data infrastructure section. The
PostgreSQL container is initialized with two schemas:

- `dataeng.orders` — business table
- `dataeng.outbox` — transactional outbox
- `dataeng.events` — high-volume event log

The Rust library:

- Builds connection URLs from discrete env vars.
- Exposes domain types (`Order`, `OutboxRow`).
- Provides pure SQL fragments (no I/O) — so they can be unit-tested.
- Implements an `OutboxBatcher` and retry policy.

`main.rs` does the I/O: open a pool, run a `BEGIN/COMMIT` block, insert
an order + outbox row, then read and mark the outbox.

## 2. Prerequisites

- Rust 1.75+ (1.96 recommended)
- Docker Compose: `docker compose up -d postgres`
- Concept: `tokio` async (Section 05/02-Futures)
- Concept: Result + `?` (Section 02/03-TicketV2)
- Concept: `serde` (Section 04/02-CSVWriter)

## 3. Setup: Create the Project from Scratch

```bash
cargo new --lib postgres_sqlx
cd postgres_sqlx
# Replace Cargo.toml with workshop/Cargo.toml
# Optionally: cargo sqlx prepare --workspace
cargo test   # 13 tests should fail with "not yet implemented"
```

The `workshop/Cargo.toml` adds:

- `sqlx` with `runtime-tokio-rustls`, `postgres`, `uuid`, `chrono`, `json`, `migrate`
- `tokio`, `serde`, `serde_json`, `uuid`, `chrono`
- `tracing`, `anyhow`, `thiserror`

## 4. Concept: sqlx vs SQLAlchemy

**In Python/SQLAlchemy:**

```python
result = await session.execute(select(Order).where(Order.quantity > 0))
orders = result.scalars().all()
```

The query string is built *at runtime*. A typo or a schema drift won't
fail until the query is actually executed.

**In Rust/sqlx (compile-time-checked):**

```rust
let order = sqlx::query_as!(
    Order,
    r#"SELECT id, customer_id, product_id, quantity, unit_price,
              total_price, status, created_at, updated_at
       FROM dataeng.orders
       WHERE quantity > $1"#,
    0_i32,
)
.fetch_one(&pool).await?;
```

`query!`/`query_as!` reach into a `.sqlx` cache directory and fail the
**build** if the column is missing or the types don't match. We avoid the
macro in this project to keep tests offline; the function-based
`sqlx::query()` is the runtime alternative and you get the same pool /
async ergonomics.

## 5. Concept: Transactional Outbox

Two-phase commit between Postgres and Kafka doesn't exist. The
**outbox pattern** writes the business row and the *intent-to-publish*
row in the same database transaction:

```
BEGIN
  INSERT INTO orders (...) VALUES (...);          -- business write
  INSERT INTO outbox (id, aggregate_type, ...)   -- event intent
  VALUES (...);
COMMIT
-- (separately) SELECT * FROM outbox WHERE processed_at IS NULL;
-- (separately) publish each row to Kafka; on success UPDATE outbox SET processed_at = now();
```

This gives you **exactly-once semantics from the database's point of
view**: a row only appears in the outbox if the business write
succeeded.

## 6. Concept: Batching + Retry

Publishing row-by-row is fine for low volume; at 10k events/second you
need batching to amortize the Kafka round-trip. `OutboxBatcher` is a
*pure* accumulator: push rows, get a `Vec<OutboxRow>` back when the
batch is full, then flush manually with `flush()`.

When publishing fails, exponential backoff with jitter avoids
thundering-herd retries.

## 7. Step-by-Step Implementation

Work through the tests in order:

```bash
cargo test step_01_connection   # 3 tests
cargo test step_02_domain       # 1 test
cargo test step_03_sql          # 2 tests
cargo test step_04_row_converters
cargo test step_05_validate     # 4 tests
cargo test step_06_batcher      # 2 tests
cargo test step_07_retry        # 2 tests
cargo test step_08_kafka_event_type
```

All `todo!()` bodies live in `src/lib.rs`.

## 8. Running End-to-End

```bash
# From the section root
docker compose up -d postgres
cargo run --release --manifest-path 02-PostgreSQLSqlx/workshop/Cargo.toml
```

You should see:

```
INFO connected to postgres
INFO wrote order+outbox order_id=... outbox_id=...
INFO unprocessed outbox rows count=...
INFO marked outbox processed kafka_event=order.created
```

## 9. Summary

| Concept | Where used | Next project |
|---------|-----------|--------------|
| `PgPool` | `main.rs` | 03-RedisAsync (cache layer) |
| Transactional outbox | `main.rs` | 07-CdcPipeline (Debezium alternative) |
| `OutboxBatcher` | `OutboxBatcher::push` | 08-UnifiedPipeline (multi-sink) |
| `next_retry_delay_ms` | `OutboxBatcher` | 07-CdcPipeline (Kafka backpressure) |
| NUMERIC -> f64 | `main.rs` insert | 04-ClickHouseIngestion (sink) |

## Further Reading

- [sqlx crate docs](https://docs.rs/sqlx) — pool, query, query!, query_as!
- [Transactional Outbox Pattern](https://microservices.io/patterns/data/transactional-outbox.html) — pattern rationale
- [Debezium Outbox SMT](https://debezium.io/documentation/reference/stable/transformations/outbox-event-router.html) — Kafka-native outbox
- [PostgreSQL logical replication](https://www.postgresql.org/docs/current/logical-replication.html) — used by Debezium

## Exercises

* **Easy** – modify the existing function to handle an extra edge case.
* **Medium** – extend the project with a new helper function that re‑uses the core logic.

