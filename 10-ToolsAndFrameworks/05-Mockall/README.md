# 🎭 Mockall — Mocking Traits for Testable Pipelines

*Subtitle: test a `Transformer` against a fake `DataSource` — no disk, no network, no test database.*

> **Test-driven approach**: This project includes a Cargo project with progressive
> unit tests that exercise a `MockDataSource`. Each function in `src/lib.rs` starts
> as a `todo!()` stub. As you follow each section, replace `todo!()` with real code
> and run `cargo test` to watch the pass count grow. Your goal: **all 11 tests pass**.

---

## Why Mock an External Dependency?

**Python pain:** To test your `EtlPipeline`, you spin up a Postgres container
in `docker-compose`, seed it with fixture data, run the pipeline, assert, tear
down. CI takes 15 minutes. The pipeline test occasionally flakes when the
container is slow to start. A colleague's laptop doesn't have Docker.

**Rust fix:** Define a `DataSource` trait. The production code uses
`PostgresDataSource` (or `S3DataSource`, or `KafkaSource`). The test code
uses `MockDataSource` from `#[automock]` — set return values per-method,
count invocations, simulate errors, all in 5 lines. CI runs in seconds.

```rust
let mut mock = MockDataSource::new();
mock.expect_fetch()
    .with(eq("SELECT 1"))
    .returning(|_| Ok(vec!["42".into()]));
assert_eq!(run_etl(&mock, "SELECT 1").unwrap(), 1);
```

## At a Glance

| # | Concept | Rust | Python | Why it matters |
|---|---------|------|--------|----------------|
| 1 | `#[automock]` | `mockall` proc macro | `unittest.mock.MagicMock` | Generate a mock from a trait |
| 2 | `&dyn Trait` | dynamic dispatch | `Protocol` with `runtime_checkable` | Pass the mock where a real source is expected |
| 3 | Expectations | `mock.expect_xxx().returning(...)` | `mock.method.return_value = ...` | One fluent builder per call |
| 4 | Argument matchers | `predicate::eq(s)` | `mock.method.assert_called_with(s)` | Constrain call args |
| 5 | `return_const` | `return_const(true)` | `side_effect = lambda: True` | Constant returns |
| 6 | Error simulation | `returning(\|_\| Err(...))` | `side_effect = MyError` | Test error paths without breaking prod |
| 7 | Multiple calls | chain `.times(2).returning(...)` | `mock.method.call_count` | Count invocations |
| 8 | `with(...).returning(...)` | mockall fluent | `mock.method.return_value` per-arg | Different return for each call |

---

> **Test-driven approach**: This project includes a Cargo project with progressive
> unit tests that exercise a `MockDataSource`. Each function in `src/lib.rs` starts
> as a `todo!()` stub. As you follow each section, replace `todo!()` with real code
> and run `cargo test` to watch the pass count grow. Your goal: **all 11 tests pass**.

**Goal**: Implement all functions in `src/lib.rs` to pass all 11 tests.


---

## Setup: Create the Project from Scratch

This is a hands-on workshop. You will write the code yourself following the steps below.

### 1. Create the new Cargo project

```bash
cargo new --lib mockall_workshop
cd mockall_workshop
```

### 2. Add the dependencies

Open `Cargo.toml` and replace whatever is there with this:

```toml
[package]
name = "mockall_workshop"
version = "0.1.0"
edition = "2024"

[dependencies]
mockall = "0.13"

```

### 3. Copy the test stubs as your starting point

This project follows a **test-driven** approach. Each function in `src/lib.rs` starts as a `todo!()` stub, and progressive tests guide your implementation.

```bash
cp "10-ToolsAndFrameworks/05-Mockall/workshop/src/lib.rs" src/lib.rs
cp "10-ToolsAndFrameworks/05-Mockall/workshop/src/main.rs" src/main.rs
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
