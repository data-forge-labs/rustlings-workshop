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
