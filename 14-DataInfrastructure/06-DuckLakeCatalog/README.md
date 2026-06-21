# 🦀 DuckLake Catalog — SQL-on-Parquet Lakehouse Layer

*Subtitle: build a Parquet-backed analytics lake with SQL catalog, time-travel, and merge-upsert.*

> **Test-driven approach**: This project includes a Cargo project with progressive
> unit tests. Each function in `src/lib.rs` starts as a `todo!()` stub. As you
> follow each section, replace `todo!()` with real code and run `cargo test` to
> watch the pass count grow. Your goal: **all 14 tests pass**.

---

## What Is DuckLake?

SQL-on-Parquet lakehouse layer — catalog, time-travel, and merge-upsert without JVM.

### Python equivalent

```python
# PyIceberg / Delta Lake on Spark — JVM-heavy, slow to start
from pyspark.sql import SparkSession
spark = SparkSession.builder.getOrCreate()
```

- **Catalog**: SQL tables in DuckDB/Postgres/SQLite
- **Data**: Parquet files on local disk or S3
- **API**: Plain SQL (`SELECT ... AT (VERSION => N)`)
- **No JVM**, no Spark, no Hive Metastore

The Rust SDK (`ducklake` crate, pre-1.0) is incubating. This project
exercises the *SQL* surface directly via the `duckdb` crate so you
get the same queries whether you use the SDK or the SQL CLI.

### Topics covered

| # | Concept | Why it matters |
|---|---------|----------------|
| 1 | DuckLake attach | SQL catalog + Parquet data |
| 2 | Time-travel | Reproducible queries |
| 3 | Merge-upsert | Atomic batched writes |
| 4 | Compaction | Bounded small-file count |
| 5 | Domain types | Wire-format rows |
| 6 | SQL builders | Testable SQL fragments |

---

## Table of Contents
1. [Introduction](#1-introduction)
2. [Prerequisites](#2-prerequisites)
3. [Setup: Create the Project from Scratch](#3-setup-create-the-project-from-scratch)
4. [Concept: DuckLake Architecture](#4-concept-ducklake-architecture)
5. [Concept: SQL Catalog vs HMS](#5-concept-sql-catalog-vs-hms)
6. [Concept: Time-Travel Queries](#6-concept-time-travel-queries)
7. [Concept: Merge-Upsert](#7-concept-merge-upsert)
8. [Step-by-Step Implementation](#8-step-by-step-implementation)
9. [Running End-to-End](#9-running-end-to-end)
10. [Summary](#10-summary)

---

## 1. Introduction

DuckLake is the **lakehouse** layer in our data infrastructure section.
Unlike Iceberg/Delta, there is no JVM, no service to start, no
`spark-submit`. The "catalog" is a SQL database; the "data" is
Parquet files. You query it with plain SQL.

In our pipeline:

- Project 04-ClickHouseIngestion writes *operational* analytics
- Project 06 (this one) writes *historical* lake data with time-travel
- Project 08-UnifiedPipeline composes both

The library exposes:

- Connection URL builders (`catalog_url`, `in_memory_url`, `s3_data_path`)
- `LakeOrder` and `TableMetadata` types
- DDL/SQL fragment builders (so they're testable)
- `snapshot_id_for` / `format_snapshot`
- `merge_upsert_statement` (a generic SQL template)
- `should_compact` heuristic

`main.rs` opens a DuckDB connection, runs the canonical ATTACH
statement, exercises the SQL builders, and prints stats.

## 2. Prerequisites

- Rust 1.75+ (1.96 recommended)
- Docker Compose: optional (MinIO if you want S3-backed data)
- Concept: Arrow (Section 12)
- Concept: sqlx (Project 02)

## 3. Setup: Create the Project from Scratch

```bash
cargo new --lib ducklake_catalog
cd ducklake_catalog
# Replace Cargo.toml with workshop/Cargo.toml
cargo test   # 14 tests should fail with "not yet implemented"
```

## 4. Concept: DuckLake Architecture

```
┌────────────────────────────┐         ┌────────────────────────────┐
│  SQL Catalog (DuckDB file) │         │  Data (Parquet files)      │
│  - tables                  │         │  - data/orders/            │
│  - snapshots               │         │    snap-001/abc.parquet    │
│  - schema versions         │         │    snap-002/def.parquet    │
│  - file inventory          │         │  - data/events/...         │
└────────────────────────────┘         └────────────────────────────┘
         ▲                                       ▲
         │           SQL on Parquet             │
         └───────────────────────────────────────┘
                       DuckDB
```

The catalog is small (KB–MB) and tracks *which Parquet files belong
to which table at which snapshot*. Data is the actual bytes.

## 5. Concept: SQL Catalog vs HMS

The Hive Metastore is a Thrift service that requires its own
process and its own database. DuckLake's SQL catalog is a few
tables in your existing database — no extra service, no extra DB.

## 6. Concept: Time-Travel Queries

Every write to a DuckLake table creates a new snapshot. You can
read the table "as of" any past snapshot:

```sql
SELECT * FROM lake.orders AT (VERSION => 1700000000000);
```

This is gold for reproducible ML training and for debugging "what
did the data look like yesterday?"

## 7. Concept: Merge-Upsert

DuckLake supports standard SQL `MERGE INTO ... USING` for upserts.
You stage incoming rows in `staging.orders_incoming` and merge into
`lake.orders` keyed by `id`. This is how late-arriving data is
applied without rewriting the whole table.

## 8. Step-by-Step Implementation

Work through the tests in order:

```bash
cargo test step_01_connection   # 3 tests
cargo test step_02_domain       # 2 tests
cargo test step_03_ddl          # 2 tests
cargo test step_04_snapshot     # 2 tests
cargo test step_05_time_travel
cargo test step_06_merge
cargo test step_07_compaction   # 2 tests
cargo test step_08_stats
```

All `todo!()` bodies live in `src/lib.rs`.

## 9. Running End-to-End

```bash
# From the section root
cargo run --release --manifest-path 06-DuckLakeCatalog/workshop/Cargo.toml
```

You should see:

```
INFO opening duckdb catalog: duckdb://:memory:
INFO ensured orders table
INFO inserted 5 rows
INFO time-travel query sql=SELECT * FROM lake.orders AT (VERSION => ...)
INFO merge statement: ... chars
INFO compaction plan compact=false target_bytes=134217728
INFO stats rows=5 revenue=149.85 customers=5
```

## 10. Summary

| Concept | Where used | Next project |
|---------|-----------|--------------|
| `ATTACH 'ducklake:...'` | `lib.rs` constant | 08-UnifiedPipeline (lake writer) |
| Time-travel SQL | `select_at_snapshot` | 08-UnifiedPipeline (reproducible ML) |
| `MERGE INTO` | `merge_upsert_statement` | 07-CdcPipeline (apply CDC) |
| `should_compact` | `lib.rs` | 08-UnifiedPipeline (background job) |
| `compute_stats` | `main.rs` | 08-UnifiedPipeline (live dashboard) |

## Further Reading

- [DuckLake announcement](https://duckdb.org/2025/05/13/announcing-ducklake.html) — design rationale
- [duckdb crate docs](https://docs.rs/duckdb) — embedded DuckDB for Rust
- [DuckDB time travel](https://duckdb.org/docs/stable/sql/time_travel.html) — `AT (VERSION => N)` syntax
- [ducklake-sdk (incubating)](https://github.com/borchero/ducklake-sdk) — pre-1.0 Rust SDK

## Exercises

* **Easy** – modify the existing function to handle an extra edge case.
* **Medium** – extend the project with a new helper function that re‑uses the core logic.

