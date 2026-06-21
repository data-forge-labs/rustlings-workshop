# 🦀 Polars DataFrame Library — Python to Rust Workshop

*Subtitle: Lightning-fast DataFrames with lazy evaluation, group-by, joins, and Parquet I/O.*

> **Test-driven approach**: This project includes a Cargo project with progressive
> unit tests. Each function in `src/lib.rs` starts as a `todo!()` stub. As you
> follow each section, replace `todo!()` with real code and run `cargo test` to
> watch the pass count grow. Your goal: **all 10 tests pass**.

---

## What Is Polars?

A from-scratch DataFrame library — 5-30x faster than pandas, parallel by default, built on Arrow.

### Python equivalent

```python
import pandas as pd

df = pd.read_csv("sales.csv")
result = df[df["amount"] > 100].groupby("region")["revenue"].sum()
``` The lazy query planner applies the same optimizations as a database (predicate pushdown, projection pushdown, query fusion):

```rust
let df = LazyFrame::scan_csv("data/sales.csv", ScanArgsCsv::default())?
    .filter(col("amount").gt(lit(100.0)))
    .group_by([col("region")])
    .agg([col("revenue").sum()])
    .collect()?;
```

The lazy API builds a **query plan** that the optimizer transforms before execution. You can `.explain()` the plan to see what will run.

In this project you'll learn to build this in Rust — and along the way
you'll discover **lazy evaluation**, **predicate pushdown**, and **Arrow integration**.

### Topics covered

| # | Concept | Why it matters |
|---|---------|----------------|
| 1 | Eager & lazy API | Direct access vs query optimization |
| 2 | Group-by | Optimized aggregation |
| 3 | Predicate pushdown | Skip rows that don't match |
| 4 | Parquet I/O | Native Arrow format |
| 5 | Column expressions | Composable, type-checked |
| 6 | Sort | Multi-key, descending |

---

## Setup: Create the Project from Scratch

This is a hands-on workshop. You will write the code yourself following the steps below.

### 1. Create the new Cargo project

```bash
cargo new --lib polars_workshop
cd polars_workshop
```

### 2. Add the dependencies

Open `Cargo.toml` and replace whatever is there with this:

```toml
[package]
name = "polars_workshop"
version = "0.1.0"
edition = "2024"

[dependencies]
polars = { version = "0.46", features = ["lazy", "parquet", "csv"] }

```

### 3. Copy the test stubs as your starting point

This project follows a **test-driven** approach. Each function in `src/lib.rs` starts as a `todo!()` stub, and progressive tests guide your implementation.

```bash
cp "12-DataEngAnalytics/01-Polars/workshop/src/lib.rs" src/lib.rs
cp "12-DataEngAnalytics/01-Polars/workshop/src/main.rs" src/main.rs
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
cargo new --lib polars_workshop
cd polars_workshop
```

### 2. Add the dependencies

Open `Cargo.toml` and replace whatever is there with this:

```toml
[package]
name = "polars_workshop"
version = "0.1.0"
edition = "2024"

[dependencies]
polars = { version = "0.46", features = ["lazy", "parquet", "csv"] }

```

### 3. Copy the test stubs as your starting point

This project follows a **test-driven** approach. Each function in `src/lib.rs` starts as a `todo!()` stub, and progressive tests guide your implementation.

```bash
cp "12-DataEngAnalytics/01-Polars/workshop/src/lib.rs" src/lib.rs
cp "12-DataEngAnalytics/01-Polars/workshop/src/main.rs" src/main.rs
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
3. [Concept: DataFrame vs LazyFrame](#3-concept-dataframe-vs-lazyframe)
4. [Concept: Loading CSV and Schema Inference](#4-concept-loading-csv-and-schema-inference)
5. [Concept: Aggregations and Expressions](#5-concept-aggregations-and-expressions)
6. [Concept: Lazy Query Plans and Optimization](#6-concept-lazy-query-plans-and-optimization)
7. [Concept: Parquet I/O](#7-concept-parquet-i-o)
8. [Putting It All Together](#8-putting-it-all-together)
9. [Complete Code Reference](#9-complete-code-reference)
10. [Summary](#10-summary)

## 1. Introduction

Polars is the de-facto DataFrame library in Rust. Used in production at:
- **Apple** (data pipelines for Siri and App Store analytics)
- **Shopify** (Shopify Sidekick ML pipelines)
- **Netflix** (data quality checks)
- **Bump.sh** (API analytics)

**Python to Rust:** The `polars` Python package is built on the same Rust crate, so the API is nearly identical. The Rust version gives you `Result<T, PolarsError>` instead of Python exceptions, full type-safety, and zero-overhead FFI.

**Data-engineering motivation:** When your pandas pipeline becomes the bottleneck, Polars is the simplest drop-in replacement. The lazy mode often gives a 10x speedup for free, just by adding `.lazy()` and `.collect()`.

## 2. Prerequisites

- Completed [04-FileIO/04-Arrow](../../../04-FileIO/04-Arrow/README.md) — comfortable with Arrow, the underlying format.
- Familiar with `Result` and error handling.
- Knows what a DataFrame is (even if only in pandas).

## 3. Concept: DataFrame vs LazyFrame

Polars has two execution modes:

- **Eager (`DataFrame`)** — operations execute immediately. Like pandas.
- **Lazy (`LazyFrame`)** — operations build a query plan, executed on `.collect()`. Like dask, but with a real optimizer.

```rust
use polars::prelude::*;

// Eager
let df = CsvReader::from_path("data/sales.csv")?.has_header(true).finish()?;
let total = df.column("units")?.sum::<i64>()?;

// Lazy
let lf = LazyFrame::scan_csv("data/sales.csv", ScanArgsCsv::default())?;
let result = lf
    .filter(col("amount").gt(lit(100.0)))
    .select([col("units").sum().alias("total_units")])
    .collect()?;
```

The eager mode is fine for small data. For any serious work, use the lazy mode — Polars will apply the same optimizations as a SQL engine.

**In Python:**

```python
import polars as pl
df = pl.read_csv("data/sales.csv")
total = df["units"].sum()

# Lazy
lf = pl.scan_csv("data/sales.csv")
result = lf.filter(pl.col("amount") > 100).select(pl.col("units").sum()).collect()
```

The two are nearly identical. The Rust version uses method-style expressions (`col("x").gt(lit(100))`); the Python version uses operator overloading (`pl.col("x") > 100`).

## 4. Concept: Loading CSV and Schema Inference

`CsvReader::from_path(path)?.has_header(true).finish()` reads a CSV and infers the schema from the first N rows:

```rust
let df = CsvReader::from_path("data/sales.csv")?
    .has_header(true)
    .with_dtype_overwrite(None)
    .finish()?;
```

The inferred schema:
- `id: Int64`
- `name: Utf8` (Rust's name for `String`)
- `amount: Float64`
- `units: Int64`

**Schema overrides** are common in production: you can pass `.with_dtype_overwrite(Some(schema))` to force a specific type if the inference is wrong.

**In pandas:**

```python
df = pd.read_csv("data/sales.csv")
```

Same result, but pandas's inference is slower and less accurate (it falls back to `object` for ambiguous columns, which kills performance).

## 5. Concept: Aggregations and Expressions

Aggregations use the `.agg()` method with expressions:

```rust
let revenue_per_product = df
    .clone()
    .lazy()
    .group_by([col("name")])
    .agg([(col("amount") * col("units")).sum().alias("revenue")])
    .collect()?;
```

The expression `(col("amount") * col("units"))` is a **column expression** — Polars composes it lazily, fuses it with the aggregation, and runs it as a single SIMD-optimized kernel.

**In pandas:**

```python
df.groupby("name").apply(lambda g: (g["amount"] * g["units"]).sum())
```

The pandas version is slow because of the Python-level lambda. The Polars version is fast because the expression is compiled to a typed query plan.

## 6. Concept: Lazy Query Plans and Optimization

The killer feature of Polars is the **lazy query optimizer**. After you build a `LazyFrame`, you can call `.explain()` to see the optimized plan:

```rust
let lf = LazyFrame::scan_csv("data/sales.csv", ScanArgsCsv::default())?
    .filter(col("amount").gt(lit(100.0)))
    .group_by([col("name")])
    .agg([col("units").sum()]);

println!("{}", lf.explain()?);
// ANTI projection: [col("name"), col("units")]
// SELECTION: [(col("amount")) > (100.0)]
//   ParquetSCAN [data/sales.csv]
//     PROJECTION: [name, amount, units]
//   AGGREGATE
//     [col("name")]
//     [col("units").sum()]
```

The optimizer sees:
- Only `name`, `amount`, `units` are needed → drop other columns at scan time
- The filter `amount > 100` can be pushed down to the scan → skip rows that don't match

**In dask:**

```python
import dask.dataframe as dd
df = dd.read_csv("data/sales.csv")
result = df[df["amount"] > 100].groupby("name")["units"].sum().compute()
```

Dask has fewer optimizations than Polars, and the API is more cumbersome. Polars is the more polished lazy DataFrame.

## 7. Concept: Parquet I/O

Polars reads and writes Parquet natively (no PyArrow dependency):

```rust
// Write
let mut file = std::fs::File::create("data.parquet")?;
ParquetWriter::new(&mut file).finish(&mut df.clone())?;

// Read
let file = std::fs::File::open("data.parquet")?;
let df = ParquetReader::new(&mut file).finish()?;
```

The format is the **same Apache Parquet** that Spark, DuckDB, and pandas use. A Parquet file written by Polars is readable by any of them.

**In pandas:**

```python
df.to_parquet("data.parquet")
df = pd.read_parquet("data.parquet")
```

Same code, different language. The Rust version is faster (5-10x) and uses less memory (columnar, no Python object overhead).

## 8. Putting It All Together

`lib.rs` is organized in six progressive steps:

1. **Step 1 (`step_01_load`)** — read CSV, inspect shape and columns.
2. **Step 2 (`step_02_aggregations`)** — total units, total revenue.
3. **Step 3 (`step_03_filter_select`)** — filter rows by amount.
4. **Step 4 (`step_04_group_by`)** — group-by with revenue expression, threshold filter.
5. **Step 5 (`step_05_parquet`)** — write & read Parquet round-trip.
6. **Step 6 (`step_06_lazy`)** — `LazyFrame` filter and group-by.

`main.rs` loads the CSV, computes totals, filters, groups, and writes Parquet.

## 9. Complete Code Reference

See [`workshop/src/lib.rs`](workshop/src/lib.rs) and [`workshop/src/main.rs`](workshop/src/main.rs). Sample CSV is at [`workshop/data/sales.csv`](workshop/data/sales.csv).

## 10. Summary

| Concept | Used In |
|---------|---------|
| `CsvReader::from_path` | `load_sales_csv` |
| Eager aggregation | `total_units`, `total_revenue` |
| Lazy filter | `filter_expensive` |
| Lazy group-by | `revenue_per_product`, `high_revenue_products` |
| `ParquetWriter` | `write_parquet` |
| `ParquetReader` | `read_parquet` |
| `LazyFrame::scan_csv` | `lazy_filter_expensive`, `lazy_group_by_total` |

## Further Reading

- [Polars user guide](https://pola-rs.github.io/polars/)
- [Polars Rust API docs](https://docs.rs/polars/)
- Ritchie Vink, "Polars — DataFrame library built for performance" (PyData talk, 2023)
- dasroot.net, "Polars vs DataFusion" (Medium, 2026)

## Exercises

1. **Easy**: Add `count_products(sales: &DataFrame) -> Result<usize>` that returns the row count, and 1 test.
2. **Medium**: Add a join function `join_sales_with_products(sales, products) -> Result<DataFrame>` that joins on `product_id` and 1 test that uses a small embedded `products` DataFrame.
3. **Hard**: Add a UDF with `apply` that uppercases the `name` column, and 1 test verifying all names are uppercased.

---

**Goal**: Implement all functions in `src/lib.rs` to pass all 10 tests.

## Functions to Implement

### Step 1 — Load CSV

#### `load_sales_csv`
- **Signature**: `pub fn load_sales_csv(path: &str) -> Result<DataFrame, PolarsError>`
- **Task**: `CsvReader::from_path(path)?.has_header(true).finish()`

### Step 2 — Aggregations

#### `total_units`
- **Signature**: `pub fn total_units(sales: &DataFrame) -> Result<i64, PolarsError>`
- **Task**: `sales.column("units")?.sum::<i64>()`

#### `total_revenue`
- **Signature**: `pub fn total_revenue(sales: &DataFrame) -> Result<f64, PolarsError>`
- **Task**: `sales.lazy().select([(col("amount") * col("units")).sum().alias("revenue")]).collect()?.column("revenue")?.f64()?.get(0).unwrap_or(0.0)`

### Step 3 — Filter

#### `filter_expensive`
- **Signature**: `pub fn filter_expensive(sales: &DataFrame, min_amount: f64) -> Result<DataFrame, PolarsError>`
- **Task**: `sales.clone().lazy().filter(col("amount").gt_eq(lit(min_amount))).collect()`

### Step 4 — Group-by

#### `revenue_per_product`
- **Signature**: `pub fn revenue_per_product(sales: &DataFrame) -> Result<DataFrame, PolarsError>`
- **Task**: `sales.lazy().group_by([col("name")]).agg([(col("amount") * col("units")).sum().alias("revenue")]).sort("revenue", SortOptions::default().with_order_desc(true)).collect()`

#### `high_revenue_products`
- **Signature**: `pub fn high_revenue_products(sales: &DataFrame, min_revenue: f64) -> Result<DataFrame, PolarsError>`
- **Task**: First compute revenue per product, then filter rows where revenue >= min_revenue.

### Step 5 — Parquet I/O

#### `write_parquet` / `read_parquet`
- **Task**: `ParquetWriter::new(File::create(path)?).finish(df)` and `ParquetReader::new(File::open(path)?).finish()`.

### Step 6 — Lazy

#### `lazy_filter_expensive`
- **Signature**: `pub fn lazy_filter_expensive(min_amount: f64) -> Result<DataFrame, PolarsError>`
- **Task**: Read CSV via `LazyFrame::scan_csv`, filter, collect.

#### `lazy_group_by_total`
- **Signature**: `pub fn lazy_group_by_total() -> Result<DataFrame, PolarsError>`
- **Task**: Read CSV via `LazyCsvReader`, group by `name`, sum `units`, collect.

## Test Modules

| Module | Tests | What It Tests |
|--------|-------|---------------|
| step_01_load | 2 | CSV → DataFrame shape and column names |
| step_02_aggregations | 2 | Total units and total revenue |
| step_03_filter_select | 1 | Filter rows by amount |
| step_04_group_by | 2 | Group-by and threshold filter |
| step_05_parquet | 1 | Parquet roundtrip |
| step_06_lazy | 2 | LazyFrame filter and group-by |

## How to Run Tests
```bash
cargo test
```
