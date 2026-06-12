# Section 9: Observability & Testing — Logs, Config, Tests, Property-Based, Mocking, Snapshots

*The production engineering practices that turn a working Rust binary into a service you can operate at 3am: structured logging, layered configuration, unit/integration testing, property-based fuzzing, trait mocking, and snapshot testing.*

---

## Why This Section?

### The Problem — Python's Production Practices Have Gaps

Every production data pipeline needs three things — and Python's solutions have gaps:

```
┌─────────────────────────────────────────────────────┐
│  1. Logging                                         │
│     Python: print() or logging module               │
│     Pain: No structured output, hard to parse       │
│     Solution: loguru, structlog (external)          │
├─────────────────────────────────────────────────────┤
│  2. Configuration                                   │
│     Python: configparser, env vars, argparse        │
│     Pain: No unified interface, env override messy  │
│     Solution: pydantic-settings (external)          │
├─────────────────────────────────────────────────────┤
│  3. Testing                                         │
│     Python: unittest, pytest                        │
│     Pain: No compile-time verification              │
│     Solution: mypy, pytest (runtime only)           │
└─────────────────────────────────────────────────────┘
```

**The common thread:** Python's solutions are either built-in but limited, or excellent but require external packages. Rust's ecosystem provides **batteries-included** approaches that integrate with the compiler.

### The Rust Advantage

```
┌─────────────────────────────────────────────────────┐
│  Rust built-in advantages:                           │
│                                                      │
│  Logging:    log facade + env_logger                 │
│              → Zero-cost if disabled at compile time │
│                                                      │
│  Config:     config crate + serde                    │
│              → Types are checked at compile time     │
│                                                      │
│  Testing:    #[test] + cargo test                    │
│              → Integrated, no extra tools needed     │
│                                                      │
│  Property:   proptest — generate 1000s of inputs     │
│              → Finds edge cases humans miss          │
│                                                      │
│  Mocking:    mockall — auto-mock any trait           │
│              → Test pipelines without I/O            │
│                                                      │
│  Snapshots:  insta — capture exact output            │
│              → Review diffs in PRs                   │
└─────────────────────────────────────────────────────┘
```

These aren't just "nice to haves" — they're **essential** for data pipelines that run unattended in production for weeks.

---

## Concepts at a Glance

### 1. The `log` Crate — Logging Facade

```rust
use log::{info, warn, error};

fn process_file(path: &str) {
    info!("Processing file: {}", path);
    if let Err(e) = do_work(path) {
        error!("Failed to process {}: {}", path, e);
    }
}
```

The `log` crate is a **facade** — your library depends on it, and the binary chooses the implementation (`env_logger`, `tracing-subscriber`, `slog`, etc.).

### 2. `env_logger` — Configure via Environment

```bash
# Control verbosity at runtime — no code changes
RUST_LOG=info cargo run
RUST_LOG=debug cargo run
RUST_LOG=my_crate=trace,other_crate=warn cargo run
```

### 3. `tracing` — Structured, Span-Based Logging

```rust
use tracing::{info, span, Level};
use tracing_subscriber;

fn main() {
    tracing_subscriber::fmt::init();
    let span = span!(Level::INFO, "pipeline", batch_size = 1000);
    let _guard = span.enter();
    info!("Processing batch");  // automatically tagged with batch_size=1000
    // ... span auto-closes when _guard drops
}
```

In Python: `structlog` with context variables.

### 4. The `config` Crate — Multi-Format Configuration

```rust
use config::{Config, File, Environment};

let settings = Config::builder()
    .add_source(File::with_name("config/default"))
    .add_source(File::with_name("config/production").required(false))
    .add_source(Environment::with_prefix("APP"))
    .build()?;

let db_url: String = settings.get("database.url")?;  // type-checked at runtime
```

Supports: TOML, YAML, JSON, INI, environment variables — and layers them so production overrides beat dev overrides beat defaults.

### 5. `#[test]` — Built-in Testing

```rust
#[cfg(test)]
mod tests {
    #[test]
    fn test_addition() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    #[should_panic(expected = "overflow")]
    fn test_overflow() {
        let x: u8 = 255;
        let _ = x + 1;  // panics in debug mode
    }

    #[test]
    fn test_with_result() -> Result<(), String> {
        let result = parse_csv("data.csv")?;
        assert!(result > 0);
        Ok(())
    }
}
```

### 6. Integration Tests

```
  my-crate/
  ├── src/
  │   ├── lib.rs    ← library code
  │   └── main.rs   ← entry point
  └── tests/
      └── integration_test.rs  ← can only use the public API
```

Integration tests in `tests/` see only what your users see — that's the point.

### 7. Property-Based Testing with `proptest`

```rust
use proptest::prelude::*;

proptest! {
    #[test]
    fn doesnt_crash(s in "\\PC*") {
        process_input(&s);  // proptest generates 1000s of random strings
    }
}
```

In Python: `@given(st.text())` from `hypothesis`.

### 8. Trait Mocking with `mockall`

```rust
use mockall::{automock, predicate::*};

#[automock]
trait DataSource {
    fn fetch(&self, key: &str) -> Result<String, Error>;
}

#[test]
fn test_transformer_with_mock_source() {
    let mut mock = MockDataSource::new();
    mock.expect_fetch()
        .with(eq("key1"))
        .returning(|_| Ok("value1".to_string()));
    let t = Transformer::new(mock);
    assert_eq!(t.run("key1").unwrap(), "processed:value1");
}
```

Test a `Transformer` against a fake `DataSource` — no I/O, deterministic, fast.

### 9. Snapshot Testing with `insta`

```rust
use insta::assert_debug_snapshot;

#[test]
fn output_shape() {
    let report = build_report(&data);
    assert_debug_snapshot!(report);
}
```

The first run creates a `.snap` file. Subsequent runs compare against it; if the output changed, you run `cargo insta review` and accept/reject each diff. Perfect for complex nested data structures.

---

## Prerequisites

- Completed [Section 3: Collections](../../../../../03-Collections/README.md) — comfortable with `Result` and `?`
- Familiar with `cargo test` and basic Rust

## Projects in This Section

| # | Project | Concepts | Format |
|---|---------|----------|--------|
| 01 | **Logging** — structured logging with multiple backends | `log` crate facade, `env_logger`, `tracing` spans, log levels, structured output | Workshop |
| 02 | **Configuration** — multi-format config parsing | `config` crate, TOML/JSON/YAML parsing, `serde` derive, env override merging | Workshop |
| 03 | **Testing** — comprehensive test patterns | `#[test]`, `#[should_panic]`, `Result<T,E>` in tests, property-based patterns, integration tests | Workshop |
| 04 | **Proptest** — property-based testing | `proptest` 1, strategies, random sampling, shrinking, invariants | Workshop |
| 05 | **Mockall** — mocking traits for testable pipelines | `mockall` 0.13, `#[automock]`, `&dyn Trait`, predicate matchers, error simulation | Workshop |
| 06 | **Insta** — snapshot testing | `insta` 1, inline snapshots, `cargo insta review`, struct Debug snapshots | Workshop |

## Learning Path

1. **01-Logging** — the simplest and most immediately useful; add `tracing` from day one of any new project
2. **02-Configuration** — config management is essential for CLI tools and services
3. **03-Testing** — the foundational patterns: unit tests, integration tests, `Result` in tests
4. **04-Proptest** — generate thousands of inputs, find the counter-example
5. **05-Mockall** — test a `Transformer` against a fake `DataSource`, no I/O
6. **06-Insta** — capture pretty-printed output once, review the diff on every PR

---

## How This Section Fits in the Course

- **Builds on**: Section 2 (ownership, `Result`, `?`), Section 3 (collections, iterators)
- **Sets up for**: Section 10 (production services that need structured logging + config + tests), Section 11 (Rust ↔ Python interop benefits from stable, tested Rust cores)

Without these practices, the services in [Section 10: Production Systems](../../../10-ProductionSystems/README.md) cannot be operated safely.

## Exercises

* **Easy** – modify the existing function to handle an extra edge case.
* **Medium** – extend the project with a new helper function that re‑uses the core logic.

