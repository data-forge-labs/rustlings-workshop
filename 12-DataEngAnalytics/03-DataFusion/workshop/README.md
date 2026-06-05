# Workshop: DataFusion

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
