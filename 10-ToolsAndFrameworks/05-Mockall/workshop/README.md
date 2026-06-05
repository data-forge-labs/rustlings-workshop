# Workshop: Mockall — Mocking Traits for Testable Pipelines

> **Test-driven approach**: This project includes a Cargo project with progressive
> unit tests that exercise a `MockDataSource`. Each function in `src/lib.rs` starts
> as a `todo!()` stub. As you follow each section, replace `todo!()` with real code
> and run `cargo test` to watch the pass count grow. Your goal: **all 11 tests pass**.

**Goal**: Implement all functions in `src/lib.rs` to pass all 11 tests.

## The Trait

```rust
#[automock]
pub trait DataSource {
    fn fetch(&self, query: &str) -> Result<Vec<String>, String>;
    fn health(&self) -> bool;
    fn schema(&self) -> Vec<String>;
    fn record_count(&self, table: &str) -> Result<u64, String>;
}
```

`#[automock]` generates a `MockDataSource` that lets you set return values
and expectations for each method. Tests then create a mock and pass it to
your function as `&dyn DataSource`.

## Functions to Implement

### `run_etl`
- **Signature**: `pub fn run_etl(source: &dyn DataSource, query: &str) -> Result<usize, String>`
- **Task**: Call `source.fetch(query)`, return `Ok(rows.len())` or propagate the error.

### `count_records`
- **Signature**: `pub fn count_records(source: &dyn DataSource, query: &str) -> Result<u64, String>`
- **Task**: Same as `run_etl` but typed as `u64` (`rows.len() as u64`).

### `is_healthy`
- **Signature**: `pub fn is_healthy(source: &dyn DataSource) -> bool`
- **Task**: `source.health()`.

### `get_schema`
- **Signature**: `pub fn get_schema(source: &dyn DataSource) -> Vec<String>`
- **Task**: `source.schema()`.

### `filter_rows`
- **Signature**: `pub fn filter_rows(source: &dyn DataSource, query: &str, prefix: &str) -> Result<Vec<String>, String>`
- **Task**: `source.fetch(query)`. Return `Ok(rows.into_iter().filter(|r| r.starts_with(prefix)).collect())`.

### `batch_etl`
- **Signature**: `pub fn batch_etl(source: &dyn DataSource, queries: &[&str]) -> Result<Vec<Vec<String>>, String>`
- **Task**: For each `q` in `queries`, call `source.fetch(q)` and collect. If any fails, propagate.

### `validate_pipeline`
- **Signature**: `pub fn validate_pipeline(source: &dyn DataSource) -> Result<&'static str, String>`
- **Task**: If `!source.health()` → `Err("source unhealthy")`. Else call `source.fetch("SELECT 1")`; if `Ok(rows)` and `!rows.is_empty()` → `Ok("ok")`, otherwise `Err("validation failed")`.

### `total_rows`
- **Signature**: `pub fn total_rows(source: &dyn DataSource, tables: &[&str]) -> Result<u64, String>`
- **Task**: For each `t` in `tables`, call `source.record_count(t)`, sum, return.

## Test Modules

| Module | Tests | What It Tests |
|--------|-------|---------------|
| test_run_etl_* | 3 | data, empty, error |
| test_count_records | 1 | typed count |
| test_is_healthy | 1 | two mocks: true and false |
| test_get_schema | 1 | schema passthrough |
| test_filter_rows | 1 | prefix filter |
| test_batch_etl | 1 | two-query batch |
| test_validate_pipeline_* | 3 | ok, unhealthy, empty |
| test_total_rows | 1 | sum across tables |

## How to Run Tests
```bash
cargo test
```
