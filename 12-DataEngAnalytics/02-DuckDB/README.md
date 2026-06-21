# 🦀 DuckDB In-Process OLAP — Python to Rust Workshop

*Subtitle: Embedded analytical SQL — "SQLite for analytics" — directly from Rust.*

> **Test-driven approach**: This project includes a Cargo project with progressive
> unit tests. Each function in `src/lib.rs` starts as a `todo!()` stub. As you
> follow each section, replace `todo!()` with real code and run `cargo test` to
> watch the pass count grow. Your goal: **all 7 tests pass**.

> **Compile-heavy workshop**: This project uses the `duckdb` crate with the
> `bundled` feature, which compiles the full DuckDB C++ engine. The first
> `cargo test` may take 10-20 minutes and needs a C++ toolchain. Subsequent
> runs are cached.

---

## What Is DuckDB?

"SQLite for analytics" — embedded analytical SQL with columnar storage, directly from Rust.

### Python equivalent

```python
import duckdb

conn = duckdb.connect(":memory:")
conn.execute("CREATE TABLE t (x INTEGER, y TEXT)")
result = conn.execute("SELECT COUNT(*) FROM t").fetchone()
print(result[0])
```

```rust
let conn = Connection::open_in_memory()?;
conn.execute("CREATE TABLE t (x INTEGER, y TEXT)", [])?;
let count: i64 = conn.query_row("SELECT COUNT(*) FROM t", [], |r| r.get(0))?;
```

In this project you'll learn to build this in Rust — and along the way
you'll discover **DuckDB**, **in-process OLAP**, and **CSV import**.

### Topics covered

| # | Concept | Why it matters |
|---|---------|----------------|
| 1 | In-process DB | No server, no config |
| 2 | DDL & DML | Standard SQL with parameterized queries |
| 3 | CSV import | Zero-config CSV ingestion |
| 4 | Prepared statements | Pre-compiled, reusable |
| 5 | Raw SQL | Full SQL standard support |

---

## Setup: Create the Project from Scratch

This is a hands-on workshop. You will write the code yourself following the steps below.

### 1. Create the new Cargo project

```bash
cargo new --lib duckdb_workshop
cd duckdb_workshop
```

### 2. Add the dependencies

Open `Cargo.toml` and replace whatever is there with this:

```toml
[package]
name = "duckdb_workshop"
version = "0.1.0"
edition = "2024"

[dependencies]
duckdb = { version = "1.1", features = ["bundled"] }

```

### 3. Copy the test stubs as your starting point

This project follows a **test-driven** approach. Each function in `src/lib.rs` starts as a `todo!()` stub, and progressive tests guide your implementation.

```bash
cp "12-DataEngAnalytics/02-DuckDB/workshop/src/lib.rs" src/lib.rs
cp "12-DataEngAnalytics/02-DuckDB/workshop/src/main.rs" src/main.rs
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
cargo new --lib duckdb_workshop
cd duckdb_workshop
```

### 2. Add the dependencies

Open `Cargo.toml` and replace whatever is there with this:

```toml
[package]
name = "duckdb_workshop"
version = "0.1.0"
edition = "2024"

[dependencies]
duckdb = { version = "1.1", features = ["bundled"] }

```

### 3. Copy the test stubs as your starting point

This project follows a **test-driven** approach. Each function in `src/lib.rs` starts as a `todo!()` stub, and progressive tests guide your implementation.

```bash
cp "12-DataEngAnalytics/02-DuckDB/workshop/src/lib.rs" src/lib.rs
cp "12-DataEngAnalytics/02-DuckDB/workshop/src/main.rs" src/main.rs
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
3. [Concept: Connection and In-Memory Mode](#3-concept-connection-and-in-memory-mode)
4. [Concept: DDL and Parameterized DML](#4-concept-ddl-and-parameterized-dml)
5. [Concept: Queries and Group-By](#5-concept-queries-and-group-by)
6. [Concept: CSV Import with `read_csv_auto`](#6-concept-csv-import-with-read_csv_auto)
7. [Concept: Prepared Statements](#7-concept-prepared-statements)
8. [Putting It All Together](#8-putting-it-all-together)
9. [Complete Code Reference](#9-complete-code-reference)
10. [Summary](#10-summary)

## 1. Introduction

DuckDB is **the** analytical database for embedded use. It's used in:
- **dbt-duckdb** for local dbt runs
- **Evidence.dev** for BI dashboards
- **DuckDB-Spatial** for GIS analytics
- **Apache Arrow** ecosystem tools

**Python to Rust:** The `duckdb` Rust crate gives you the same engine as the Python `duckdb` package, but with a typed `Result<T, duckdb::Error>` return type, zero-copy Arrow interop, and no GIL.

**Data-engineering motivation:** For one-off analytical queries, training ML models on local data, or building ad-hoc tools, DuckDB is faster to set up than Postgres and faster to query than SQLite for columnar workloads.

## 2. Prerequisites

- Completed [04-FileIO/04-Arrow](../../../04-FileIO/04-Arrow/README.md) — comfortable with progressive workshops.
- Familiar with SQL basics (SELECT, INSERT, GROUP BY).
- Understands `Result<T, E>`.

## 3. Concept: Connection and In-Memory Mode

DuckDB supports two connection modes:

- **In-memory** (`Connection::open_in_memory()`) — fast, ephemeral, perfect for tests.
- **File-backed** (`Connection::open("path/to/db.duckdb")`) — persistent, like SQLite.

For our tests, we use in-memory. Production code typically uses a file-backed DB or a `:memory:` with a teardown that explicitly exports results.

**In Python:**

```python
import duckdb
conn = duckdb.connect(":memory:")
```

**In Rust:**

```rust
use duckdb::Connection;
let conn = Connection::open_in_memory()?;
```

The `?` propagates any connection error (rare, but possible if the bundled lib has an issue).

## 4. Concept: DDL and Parameterized DML

DDL is the same SQL you'd write anywhere:

```rust
conn.execute(
    "CREATE TABLE products (id INTEGER PRIMARY KEY, name TEXT NOT NULL, region TEXT NOT NULL)",
    [],
)?;
```

DML with parameters uses `params![]` from the `duckdb` crate:

```rust
use duckdb::params;
conn.execute(
    "INSERT INTO products VALUES (?, ?, ?)",
    params![1, "Apple", "North"],
)?;
```

**This is SQL-injection-safe** — the values are bound as parameters, not interpolated into the string. Same principle as Python's `?` placeholders.

**In Python:**

```python
conn.execute("INSERT INTO products VALUES (?, ?, ?)", [1, "Apple", "North"])
```

The two are functionally identical. The Rust version's `?` chains errors; the Python version raises and must be caught.

## 5. Concept: Queries and Group-By

To get typed results, use `prepare` + `query` and map rows:

```rust
use duckdb::params;

let mut stmt = conn.prepare("SELECT id, name FROM products WHERE region = ?")?;
let rows = stmt.query_map(params![region], |row| {
    Ok((row.get::<_, i32>(0)?, row.get::<_, String>(1)?))
})?;
let mut result = Vec::new();
for row in rows {
    result.push(row?);
}
```

This pattern is verbose because Rust is statically typed — you must declare the types of each column. The upside: a column-type mismatch is a compile error, not a runtime crash.

**In Python**, this is one line:

```python
conn.execute("SELECT id, name FROM products WHERE region = ?", [region]).fetchall()
```

## 6. Concept: CSV Import with `read_csv_auto`

DuckDB's killer feature: it can read a CSV file **as if it were a SQL table**:

```sql
CREATE TABLE products AS SELECT * FROM read_csv_auto('data/products.csv');
```

The `read_csv_auto` function infers the schema from the header line and a sample of rows. It's a magic shortcut — no `CREATE TABLE` boilerplate, no `COPY` command, no schema validation.

**In Rust**, you wrap this in a function:

```rust
pub fn import_csv_from_file(conn: &Connection, table: &str, path: &str) -> Result<usize, duckdb::Error> {
    let sql = format!("CREATE TABLE {} AS SELECT * FROM read_csv_auto(?)", table);
    conn.execute(&sql, params![path])?;
    // Return the row count
    let count: i64 = conn.query_row(&format!("SELECT COUNT(*) FROM {}", table), [], |r| r.get(0))?;
    Ok(count as usize)
}
```

**In Python:**

```python
conn.execute("CREATE TABLE products AS SELECT * FROM read_csv_auto('data/products.csv')")
```

Same SQL, same result. The Rust version is more verbose because of the dynamic table name, which the user passes in.

## 7. Concept: Prepared Statements

Prepared statements are pre-compiled SQL that you can execute multiple times with different parameters:

```rust
let mut stmt = conn.prepare("SELECT COUNT(*) FROM products WHERE region = ?")?;
let n: i64 = stmt.query_row(params!["North"], |r| r.get(0))?;
```

The `?` is bound at execute time, not parse time. This is:
- **Faster** for repeated queries (no re-parsing)
- **Safer** (parameter binding, not string interpolation)
- **Required** for parameter values that come from user input

**In Python:**

```python
stmt = conn.prepare("SELECT COUNT(*) FROM products WHERE region = ?")
stmt.execute(["North"]).fetchone()
```

## 8. Putting It All Together

`lib.rs` is organized in six progressive steps:

1. **Step 1 (`step_01_connection`)** — open an in-memory connection.
2. **Step 2 (`step_02_table`)** — DDL + DML, count rows.
3. **Step 3 (`step_03_queries`)** — filter, group-by, return `Vec`.
4. **Step 4 (`step_04_csv_import`)** — `read_csv_auto` from disk.
5. **Step 5 (`step_05_prepared`)** — parameterized prepared statement.
6. **Step 6 (`step_06_ad_hoc_sql`)** — execute arbitrary SQL, return rows as strings.

`main.rs` ties it together: open a connection, create a table, insert rows, import a CSV, run aggregate queries.

## 9. Complete Code Reference

See [`workshop/src/lib.rs`](workshop/src/lib.rs) and [`workshop/src/main.rs`](workshop/src/main.rs). Sample CSV is at [`workshop/data/products.csv`](workshop/data/products.csv).

## 10. Summary

| Concept | Used In |
|---------|---------|
| `Connection::open_in_memory()` | `open_in_memory` |
| `conn.execute(sql, params)` | `create_products_table`, `insert_product` |
| `conn.query_row(sql, params, mapper)` | `count_products`, `prepared_count` |
| `conn.prepare` + `query_map` | `products_in_region`, `regions_with_count` |
| `read_csv_auto` | `import_csv_from_file` |
| Dynamic table name with `format!` | `import_csv_from_file` |
| `params!` macro | All parameterized queries |

## Further Reading

- [DuckDB Rust API docs](https://duckdb.org/docs/api/rust.html)
- [DuckDB SQL reference](https://duckdb.org/docs/sql/introduction)
- [DuckDB + Arrow zero-copy interop](https://duckdb.org/docs/api/rust.html#arrow)
- Mark Raasveldt, "DuckDB: Embedding an OLAP Engine in your Application" (CMU 2024)

## Exercises

1. **Easy**: Add `delete_product(conn: &Connection, id: i32) -> Result<()>` that runs `DELETE FROM products WHERE id = ?` and 1 test.
2. **Medium**: Add `top_n_products(conn, n: i64) -> Result<Vec<(i32, String)>>` that uses `ORDER BY id LIMIT n` and 1 test.
3. **Hard**: Add `join_products_with_sales(conn) -> Result<Vec<(String, i64)>>` that creates a second `sales(product_id, units)` table, joins on `id`, groups by `name`, and returns total units per product.

---

> **Compile-heavy workshop**: This project depends on the `duckdb` crate with the
> `bundled` feature, which compiles the full DuckDB C++ engine from source. The
> first `cargo test` may take **10-20 minutes** and requires a C++ toolchain
> (`build-essential` on Linux, MSVC on Windows). Subsequent runs are cached and
> fast.

**Goal**: Implement all functions in `src/lib.rs` to pass all 7 tests.

## Functions to Implement

### Step 1 — Connection

#### `open_in_memory`
- **Signature**: `pub fn open_in_memory() -> Result<Connection>`
- **Task**: `Connection::open_in_memory()`

### Step 2 — Table DDL/DML

#### `create_products_table`
- **Signature**: `pub fn create_products_table(conn: &Connection) -> Result<()>`
- **Task**: `CREATE TABLE products (id INTEGER PRIMARY KEY, name TEXT NOT NULL, region TEXT NOT NULL)`

#### `insert_product`
- **Signature**: `pub fn insert_product(conn: &Connection, id: i32, name: &str, region: &str) -> Result<()>`
- **Task**: `conn.execute("INSERT INTO products VALUES (?, ?, ?)", params![id, name, region])`

#### `count_products`
- **Signature**: `pub fn count_products(conn: &Connection) -> Result<i64>`
- **Task**: `conn.query_row("SELECT COUNT(*) FROM products", [], |r| r.get(0))`

### Step 3 — Queries

#### `products_in_region`
- **Signature**: `pub fn products_in_region(conn: &Connection, region: &str) -> Result<Vec<(i32, String)>>`
- **Task**: Prepare, query, map rows to `(id, name)` tuples.

#### `regions_with_count`
- **Signature**: `pub fn regions_with_count(conn: &Connection) -> Result<Vec<(String, i64)>>`
- **Task**: `SELECT region, COUNT(*) FROM products GROUP BY region` → `Vec<(String, i64)>`.

### Step 4 — CSV import

#### `import_csv_from_file`
- **Signature**: `pub fn import_csv_from_file(conn: &Connection, table: &str, path: &str) -> Result<usize>`
- **Task**: `CREATE TABLE <table> AS SELECT * FROM read_csv_auto(?)` then return row count.
- **Hint**: Use `params![path]` to bind the path safely.

### Step 5 — Prepared statements

#### `prepared_count`
- **Signature**: `pub fn prepared_count(conn: &Connection, region: &str) -> Result<i64>`
- **Task**: `SELECT COUNT(*) FROM products WHERE region = ?` with `params![region]`.

### Step 6 — Ad-hoc SQL

#### `run_sql`
- **Signature**: `pub fn run_sql(conn: &Connection, sql: &str) -> Result<Vec<Vec<String>>>`
- **Task**: Prepare a statement from `sql`, execute, map each row to a `Vec<String>` (using `to_string()` on each column).

## Test Modules

| Module | Tests | What It Tests |
|--------|-------|---------------|
| step_01_connection | 1 | Open in-memory connection |
| step_02_table | 1 | DDL + DML, COUNT(*) |
| step_03_queries | 2 | Filter by region, group-by count |
| step_04_csv_import | 1 | read_csv_auto + CREATE TABLE AS |
| step_05_prepared | 1 | Parameterized prepared statement |
| step_06_ad_hoc_sql | 1 | Raw SQL aggregation |

## How to Run Tests
```bash
cargo test
```
