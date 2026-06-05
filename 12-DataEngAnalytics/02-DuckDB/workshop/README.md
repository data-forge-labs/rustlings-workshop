# Workshop: DuckDB

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
