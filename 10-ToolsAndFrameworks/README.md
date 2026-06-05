# Section 10: Tools & Frameworks

*Logging, configuration management, and testing frameworks — the tools you need for production Rust applications.*

---

## Why This Section?

### The Problem — Production Python Pain Points

Every production data pipeline needs these three things — and Python's solutions have gaps:

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
└─────────────────────────────────────────────────────┘
```

These aren't just "nice to haves" — they're **essential** for data pipelines that run unattended in production for weeks.

---

## What You'll Learn

| # | Concept | Rust Module / Crate | Python Equivalent | Purpose |
|---|---------|--------------------|------------------|---------|
| 1 | Logging facade | `log` crate | `logging` module | Library-agnostic logging API |
| 2 | Environment logger | `env_logger` | `logging.basicConfig` | Configure via RUST_LOG env var |
| 3 | Structured tracing | `tracing` crate | `structlog` | Span-based, structured logging |
| 4 | Log levels | `error!`, `warn!`, `info!`, `debug!`, `trace!` | Same | Granular verbosity control |
| 5 | Config management | `config` crate | `configparser` + `pyyaml` | Multi-format, layered config |
| 6 | TOML parsing | `toml` crate | `tomllib` / `toml` | Structured config files |
| 7 | Environment overrides | env var merging | `os.environ` | Deploy-specific overrides |
| 8 | Unit testing | `#[test]`, `assert_eq!` | `pytest`, `unittest` | Built-in test framework |
| 9 | Expected failures | `#[should_panic]` | `pytest.raises` | Test that code panics correctly |
| 10 | Result in tests | `-> Result<()>` | N/A | Use `?` inside tests |
| 11 | Test organization | `#[cfg(test)]` | `test_*.py` | Conditional compilation of tests |
| 12 | Integration tests | `tests/` directory | `tests/` | End-to-end testing |
| 13 | Property-based testing | `proptest` crate | `hypothesis` | Generate random test cases |

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

### 2. `env_logger` — Configuration via Environment

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
    info!("Processing batch");
    // ... spans auto-close when _guard drops
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

let db_url: String = settings.get("database.url")?;
// Type-checked at runtime, errors are clear
```

Supports: TOML, YAML, JSON, INI, environment variables.

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
  src/
  ├── lib.rs    ← library code
  └── main.rs   ← entry point
  tests/
  └── integration_test.rs  ← external testing
```

Integration tests in `tests/` directory can only use the **public API** — just like your users.

### 7. Property-Based Testing with `proptest`

```rust
use proptest::prelude::*;

proptest! {
    #[test]
    fn doesnt_crash(s in "\\PC*") {
        process_input(&s);  // test with random strings
    }
}
```

In Python: `@given(st.text())` from `hypothesis`.

---

## Prerequisites

- Completed [Section 3: Collections](../03-Collections/README.md)
- Familiar with `cargo test` and basic Rust

## Projects in This Section

| # | Project | Concepts | Format |
|---|---------|----------|--------|
| 1 | **Logging** — structured logging with multiple backends | `log` crate facade, `env_logger`, `tracing` spans, log levels, structured output | Project |
| 2 | **Configuration** — multi-format config parsing | `config` crate, TOML/JSON/YAML parsing, `serde` derive, env override merging | Project |
| 3 | **Testing** — comprehensive test patterns | `#[test]`, `#[should_panic]`, `Result<T,E>` in tests, property-based patterns, integration tests | Project |
| 4 | **Proptest** — property-based testing | `proptest` 1, strategies, random sampling, shrinking, invariants | Workshop |
| 5 | **Mockall** — mocking traits for testable pipelines | `mockall` 0.13, `#[automock]`, `&dyn Trait`, predicate matchers, error simulation | Workshop |
| 6 | **Insta** — snapshot testing | `insta` 1, inline snapshots, `cargo insta review`, struct Debug snapshots | Workshop |

## Learning Path

1. Start with **01-Logging** — it's the simplest and most immediately useful
2. Move to **02-Configuration** — config management is essential for CLI tools and services
3. Finish with **03-Testing** — build confidence in the testing patterns used across the entire course
4. **04-Proptest** — generate thousands of inputs, find the counter-example
5. **05-Mockall** — test a `Transformer` against a fake `DataSource`, no I/O
6. **06-Insta** — capture pretty-printed output once, review the diff on every PR
