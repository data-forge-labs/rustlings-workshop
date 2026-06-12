# 🦀 Apache DataFusion Query Engine — Python to Rust Workshop

*Subtitle: Embed a SQL query engine — used by Ballista, InfluxDB IOx, and Cube.js — in your Rust app.*

> **Test-driven approach**: This project includes a Cargo project with progressive
> unit tests. Each function in `src/lib.rs` starts as a `todo!()` stub. As you
> follow each section, replace `todo!()` with real code and run `cargo test` to
> watch the pass count grow. Your goal: **all 7 tests pass**.

---

## Why DataFusion for Custom Query Engines?

**Python pain:** You need to expose SQL to your application's users, but Postgres is overkill and SQLite is too slow for analytical queries. You start writing a hand-rolled query parser and discover it's a year-long project.

**Rust fix:** Apache DataFusion is a fully-featured SQL query engine built on Arrow. It handles parsing, planning, optimization, and execution. You write 5 lines of Rust to expose a SQL endpoint:

```rust
let ctx = SessionContext::new();
ctx.register_csv("orders", "data/orders.csv", CsvReadOptions::new()).await?;
let batches = ctx.sql("SELECT * FROM orders WHERE amount > 100").await?.collect().await?;
```

DataFusion is the query layer of distributed systems like **Ballista** (distributed DataFusion), **InfluxDB IOx** (the time-series database), and **Cube.js** (the analytics API platform). If your system needs to answer SQL queries, DataFusion is the kernel.

## At a Glance

| # | Concept | Rust | Python | Why it matters |
|---|---------|------|--------|----------------|
| 1 | Query engine | `SessionContext` | `pyspark.sql.SparkSession` | Embed a SQL planner in your app |
| 2 | CSV registration | `ctx.register_csv` | `spark.read.csv` | Turn a file into a queryable table |
| 3 | SQL execution | `ctx.sql("SELECT ...") | `spark.sql("...")` | Full SQL standard support |
| 4 | Async API | `async/await` | n/a (sync in pyspark) | Composable with tokio |
| 5 | Result collection | `.collect().await?` | `.toPandas()` | Returns Arrow `RecordBatch`es |
| 6 | Parquet write | `ArrowWriter` | `df.write.parquet(...)` | Zero-copy Parquet output |

---

## Setup: Create the Project from Scratch

This is a hands-on workshop. You will write the code yourself following the steps below.

### 1. Create the new Cargo project

```bash
cargo new --lib datafusion_workshop
cd datafusion_workshop
```

### 2. Add the dependencies

Open `Cargo.toml` and replace whatever is there with this:

```toml
[package]
name = "datafusion_workshop"
version = "0.1.0"
edition = "2024"

[dependencies]
datafusion = "43"
tokio = { version = "1", features = ["rt-multi-thread", "macros"] }

```

### 3. Copy the test stubs as your starting point

This project follows a **test-driven** approach. Each function in `src/lib.rs` starts as a `todo!()` stub, and progressive tests guide your implementation.

```bash
cp "12-DataEngAnalytics/03-DataFusion/workshop/src/lib.rs" src/lib.rs
cp "12-DataEngAnalytics/03-DataFusion/workshop/src/main.rs" src/main.rs
```

### 4. Run the tests to see them fail (this is expected!)

```bash
cargo test
```

You should see all tests fail with the message "not yet implemented". That's the starting point — you are about to make them pass.

### 5. Follow the step-by-step sections below

Each section below corresponds to a step module in the test file. Implement the function(s) described, then run:

```bash
cargo test step_XX_name
```

to watch the pass count grow. The test module names match the section headings.

## Setup: Create the Project from Scratch

This is a hands-on workshop. You will write the code yourself following the steps below.

### 1. Create the new Cargo project

```bash
cargo new --lib datafusion_workshop
cd datafusion_workshop
```

### 2. Add the dependencies

Open `Cargo.toml` and replace whatever is there with this:

```toml
[package]
name = "datafusion_workshop"
version = "0.1.0"
edition = "2024"

[dependencies]
datafusion = "43"
tokio = { version = "1", features = ["rt-multi-thread", "macros"] }

```

### 3. Copy the test stubs as your starting point

This project follows a **test-driven** approach. Each function in `src/lib.rs` starts as a `todo!()` stub, and progressive tests guide your implementation.

```bash
cp "12-DataEngAnalytics/03-DataFusion/workshop/src/lib.rs" src/lib.rs
cp "12-DataEngAnalytics/03-DataFusion/workshop/src/main.rs" src/main.rs
```

### 4. Run the tests to see them fail (this is expected!)

```bash
cargo test
```

You should see all tests fail with the message "not yet implemented". That's the starting point — you are about to make them pass.

### 5. Follow the step-by-step sections below

Each section below corresponds to a step module in the test file. Implement the function(s) described, then run:

```bash
cargo test step_XX_name
```

to watch the pass count grow. The test module names match the section headings.

## Table of Contents
1. [Introduction](#1-introduction)
2. [Prerequisites](#2-prerequisites)
3. [Concept: `SessionContext` and Async SQL](#3-concept-sessioncontext-and-async-sql)
4. [Concept: Registering CSV as a Table](#4-concept-registering-csv-as-a-table)
5. [Concept: Aggregations and Filters](#5-concept-aggregations-and-filters)
6. [Concept: Parquet Write](#6-concept-parquet-write)
7. [Putting It All Together](#7-putting-it-all-together)
8. [Complete Code Reference](#8-complete-code-reference)
9. [Summary](#9-summary)

## 1. Introduction

DataFusion is the **Apache** query engine. It has been a top-level Apache project since 2020 and is one of the most active Apache projects by commit count. It powers:

- **Ballista** — distributed DataFusion (similar to Spark, but smaller and faster)
- **InfluxDB IOx** — the time-series database's storage engine
- **Cube Store** — the caching layer for Cube.js
- **GlareDB** — a distributed SQL database built on DataFusion

**Python to Rust:** The Python `datafusion` package wraps the Rust crate. The native Rust API gives you direct access to the planner, optimizer, and physical operators — useful when you want to build a custom query engine or expose SQL inside an existing application.

**Data-engineering motivation:** When you build a feature-store, metrics layer, or query API, you need a SQL planner. DataFusion is the lightest-weight option that still has the full SQL standard.

## 2. Prerequisites

- Completed [04-FileIO/04-Arrow](../../../04-FileIO/04-Arrow/README.md) — comfortable with Arrow `RecordBatch`es.
- Familiar with `async/await` from [05-Concurrency/02-Futures](../../../05-Concurrency/02-Futures/README.md) (or [12-DataEngAnalytics/02-DuckDB](../../02-DuckDB/README.md)).
- Comfortable with `Result` and `Box<dyn Error>`.

## 3. Concept: `SessionContext` and Async SQL

DataFusion's entry point is `SessionContext`. It owns the catalog (tables), the SQL parser, and the query optimizer. Every SQL statement goes through the context:

```rust
use datafusion::prelude::*;

let ctx = SessionContext::new();
let df = ctx.sql("SELECT 1 AS one").await?;
let batches = df.collect().await?;
```

The `sql()` method returns a `DataFrame` (lazy, like Polars). The `collect()` method executes the query and returns `Vec<RecordBatch>`.

**Key difference from Polars/DuckDB:** DataFusion is **fully async** because it's designed to be embedded in async runtimes like tokio. Every method is `async fn`. The `await?` chains are common.

**In Python (`pyspark`):**

```python
from pyspark.sql import SparkSession
spark = SparkSession.builder.getOrCreate()
df = spark.sql("SELECT 1 AS one")
df.collect()
```

The structure is the same. DataFusion is what `pyspark` would be if it were a small Rust library, not a JVM-based distributed system.

## 4. Concept: Registering CSV as a Table

DataFusion can read CSV, Parquet, JSON, and Avro. The simplest is CSV:

```rust
ctx.register_csv("orders", "data/orders.csv", CsvReadOptions::new()).await?;
```

After registration, the table is queryable like any other SQL table:

```sql
SELECT * FROM orders WHERE amount > 100;
```

DataFusion's CSV reader does **predicate pushdown** — if you have an `amount > 100` filter, it skips rows where `amount <= 100` at the read stage, not after loading the full file. This is the same optimization Parquet gets, but it works on CSV too (in a limited form).

**In Polars**, the equivalent is `df.lazy().scan_csv(path)`. The difference: DataFusion's lazy is async and triggers on `collect().await`; Polars' triggers on `collect()` (sync).

## 5. Concept: Aggregations and Filters

Aggregation is SQL:

```sql
SELECT COUNT(*) AS n, AVG(amount) AS avg_amt FROM orders;
```

The Rust side walks the result `RecordBatch` and extracts typed columns:

```rust
let batches = ctx.sql(sql).await?.collect().await?;
let batch = &batches[0];
let n = batch.column(0).as_any().downcast_ref::<Int64Array>().unwrap().value(0);
let avg = batch.column(1).as_any().downcast_ref::<Float64Array>().unwrap().value(0);
```

The `as_any().downcast_ref::<T>()` pattern is the standard Arrow idiom for typed access. Compare to the simpler Polars `df.column("n")?.i64()?.get(0)` — DataFusion gives you the underlying Arrow API, which is one level lower.

## 6. Concept: Parquet Write

DataFusion itself doesn't have a built-in `df.write.parquet()` like Polars does, but you can write `RecordBatch`es to Parquet using the `parquet::arrow::ArrowWriter` from the `parquet` crate (covered in [04-FileIO/03-Parquet](../../../04-FileIO/03-Parquet/README.md)):

```rust
use parquet::arrow::ArrowWriter;
use std::fs::File;

let batch = &batches[0];
let file = File::create(path)?;
let mut writer = ArrowWriter::try_new(file, batch.schema(), Default::default())?;
writer.write(batch)?;
writer.close()?;
```

This is the **zero-copy bridge** between DataFusion and the Parquet file format. The same `RecordBatch` that DataFusion produces goes directly into Parquet without re-serialization.

## 7. Putting It All Together

`lib.rs` is organized in five progressive steps:

1. **Step 1 (`step_01_context`)** — `SessionContext` + simple `SELECT 1`.
2. **Step 2 (`step_02_csv`)** — register CSV, count rows.
3. **Step 3 (`step_03_aggregations`)** — SUM, COUNT with WHERE, return names.
4. **Step 4 (`step_04_sql`)** — ad-hoc SQL returning `RecordBatch`es.
5. **Step 5 (`step_05_parquet`)** — write Parquet to disk.

`main.rs` ties it all together: register the CSV, run a few aggregations, write Parquet.

## 8. Complete Code Reference

See [`workshop/src/lib.rs`](workshop/src/lib.rs) and [`workshop/src/main.rs`](workshop/src/main.rs). Sample CSV is at [`workshop/data/orders.csv`](workshop/data/orders.csv).

## 9. Summary

| Concept | Used In |
|---------|---------|
| `SessionContext::new()` | `create_context` |
| `register_csv` | `register_csv` |
| Async SQL execution | All functions |
| Arrow `Int64Array`/`Float64Array` | `count_rows`, `total_amount` |
| `Arc<dyn Array>` downcasting | `names_above_amount` |
| `ArrowWriter` from parquet crate | `write_parquet` |

## Further Reading

- [Apache DataFusion docs](https://arrow.apache.org/datafusion/)
- [DataFusion GitHub](https://github.com/apache/datafusion)
- Alex Merced, "DataFusion 53/54 release notes" (Medium, 2026)
- [Ballista: Distributed DataFusion](https://github.com/apache/arrow-ballista)
- [InfluxDB IOx architecture](https://www.influxdata.com/blog/announcing-influxdb-iox/)

## Exercises

1. **Easy**: Add `count_distinct_names(ctx, table) -> Result<i64>` that runs `SELECT COUNT(DISTINCT name) FROM <table>` and 1 test.
2. **Medium**: Add a UDF with `create_udf` that uppercases a string column, register it as `uppercase`, run `SELECT uppercase(name) FROM orders` and verify.
3. **Hard**: Add a `join_orders_with_sales(ctx, orders_table, sales_table) -> Result<Vec<RecordBatch>>` that joins on `id` and groups by `name` to get total units sold.

---

**Goal**: Implement all functions in `src/lib.rs` to pass all 7 tests.

## Functions to Implement

### Step 1 — SessionContext

#### `create_context`
- **Signature**: `pub async fn create_context() -> Result<SessionContext>`
- **Task**: `SessionContext::new()`

### Step 2 — CSV registration

#### `register_csv`
- **Signature**: `pub async fn register_csv(ctx: &SessionContext, table: &str, path: &str) -> Result<()>`
- **Task**: `ctx.register_csv(table, path, CsvReadOptions::new()).await?`

#### `count_rows`
- **Signature**: `pub async fn count_rows(ctx: &SessionContext, table: &str) -> Result<i64>`
- **Task**: `ctx.sql(&format!("SELECT COUNT(*) FROM {}", table)).await?.collect().await?` and extract the `Int64Array`.

### Step 3 — Aggregations

#### `total_amount`
- **Signature**: `pub async fn total_amount(ctx: &SessionContext, table: &str) -> Result<f64>`
- **Task**: `SELECT SUM(amount) FROM <table>` and extract the `Float64Array`.

#### `rows_above_amount`
- **Signature**: `pub async fn rows_above_amount(ctx: &SessionContext, table: &str, threshold: f64) -> Result<usize>`
- **Task**: `SELECT COUNT(*) FROM <table> WHERE amount > <threshold>` and return the count.

#### `names_above_amount`
- **Signature**: `pub async fn names_above_amount(ctx: &SessionContext, table: &str, threshold: f64) -> Result<Vec<String>>`
- **Task**: `SELECT name FROM <table> WHERE amount > <threshold>` and collect names.

### Step 4 — Ad-hoc SQL

#### `run_sql`
- **Signature**: `pub async fn run_sql(ctx: &SessionContext, sql: &str) -> Result<Vec<RecordBatch>>`
- **Task**: `ctx.sql(sql).await?.collect().await?`

### Step 5 — Parquet write

#### `write_parquet`
- **Signature**: `pub async fn write_parquet(ctx: &SessionContext, table: &str, path: &str) -> Result<()>`
- **Task**: Use `ctx.sql(&format!("SELECT * FROM {}", table))` then `datafusion::arrow::ipc::writer` or `parquet` crate's `ArrowWriter` directly. Alternatively use `datafusion::datasource::file_format::parquet::ParquetSink` via `df.execute` and write.

> Note: The simplest path is `ctx.sql(...).await?.collect().await?` → take the first batch → `ArrowWriter::try_new(File::create(path)?, batch.schema(), Default::default())?` → write & close.

## Test Modules

| Module | Tests | What It Tests |
|--------|-------|---------------|
| step_01_context | 1 | SessionContext + simple SQL |
| step_02_csv | 1 | Register CSV and count rows |
| step_03_aggregations | 3 | Sum, count above threshold, names above threshold |
| step_04_sql | 1 | Ad-hoc SQL with COUNT + AVG |
| step_05_parquet | 1 | Write Parquet to disk |

## How to Run Tests
```bash
cargo test
```
