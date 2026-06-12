# Rust Logging — Python `loguru` Equivalent

> **Test-driven approach**: This project includes a Cargo project with progressive unit tests. Each function in `workshop/src/lib.rs` starts as a `todo!()` stub. As you follow each section, replace `todo!()` with real code and run `cd workshop && cargo test` to watch the pass count grow. Your goal: **all 8 tests pass**.

## Why Use the `log` Facade?

**Python pain:** Python has many loggers (`loguru`, `structlog`, stdlib `logging`) and the API varies. There's no compile-time check that you've imported the right one.

**Rust fix:** The `log` crate is a **facade** — it defines the API (`info!`, `warn!`, `error!`) but doesn't write anything. You plug in a backend (`env_logger`, `tracing-subscriber`, ...). The same code works with any backend:

```rust
use log::{info, warn, error};

fn main() {
    env_logger::init();              // choose backend
    info!("Server started on port {}", 8080);
}
```

## At a Glance

| # | Concept | Rust | Python | Why it matters |
|---|---------|------|--------|----------------|
| 1 | Facade | `log` crate | `logging` / `loguru` | Universal API; backend is pluggable |
| 2 | Terminal backend | `env_logger` | `loguru.add(sys.stderr)` | Simple stdout/stderr output |
| 3 | Structured backend | `tracing` | `structlog` | Span-based, async-aware logging |
| 4 | Log macros | `info!`, `warn!`, `error!` | `logger.info(...)` | Log at a level in one macro call |
| 5 | Level filtering | `RUST_LOG=info` | `LOG_LEVEL=INFO` env var | Configure verbosity at runtime |
| 6 | `log_enabled!` | `log_enabled!(Level::Info)` | `logger.level` | Skip expensive log call work |

---

## Table of Contents

1. [Introduction](#1-introduction)
2. [Prerequisites](#2-prerequisites)
3. [The `log` Crate — A Universal Facade](#3-the-log-crate--a-universal-facade)
4. [Log Levels and Filtering](#4-log-levels-and-filtering)
5. [`env_logger` — Simple Terminal Logging](#5-env_logger--simple-terminal-logging)
6. [`tracing` — Structured and Async Logging](#6-tracing--structured-and-async-logging)
7. [Putting It All Together](#7-putting-it-all-together)
8. [Complete Code Reference](#8-complete-code-reference)
9. [Summary](#9-summary)

## 1. Introduction

Logging is the data engineer's stethoscope. In Python you reach for **loguru** or **structlog**; in Rust the story is more modular — a **facade crate** (`log`) plus a **backend** (`env_logger`, `tracing-subscriber`, etc.).

**What you'll learn:**
- The `log` crate as a universal logging facade
- `env_logger` for simple terminal output
- `tracing` for structured, span-based logging
- Log levels, formatting, and filtering — all compared to Python's loguru

## 2. Prerequisites

- Basic Rust syntax (functions, strings, Vec)
- **Project**: [01-BasicCalculator](../../../../01-Foundations/01-Intro/README.md)
- Python knowledge: you have used loguru or the stdlib `logging` module

## 3. The `log` Crate — A Universal Facade

### Explanation

In Python, you do:
```python
from loguru import logger
logger.info("Server started on port {}", port)
```

In Rust, the `log` crate is a **facade** — it defines the API (macros like `info!`, `warn!`) but doesn't write anything itself. You plug in a **backend** like `env_logger`.

```rust
use log::{info, warn, error};

fn main() {
    env_logger::init();
    info!("Server started on port {}", 8080);
}
```

| Concept | Python (loguru) | Rust (log crate) |
|---------|----------------|------------------|
| Logger object | `logger.add(sink, level=...)` | Backend choice (env_logger, etc.) |
| Log a message | `logger.info("msg")` | `info!("msg")` |
| String interpolation | `logger.info("{}", val)` | `info!("{}", val)` |
| Level check | `logger.level` | `log_enabled!(Level::Info)` |

### Example

```rust
use log::{info, warn, error, debug, trace};

// Just macros — no output until a backend is registered
let _ = info!("this does nothing yet");
```

### Applying to Our Project

The function `demo_log_levels` will initialize `env_logger`, log at all five levels, and return the captured output.

## 4. Log Levels and Filtering

### Explanation

Rust's log levels match Python's exactly:

| Rust | Python (logging/loguru) | Use Case |
|------|------------------------|----------|
| `trace!` | `logger.trace()` | Finest-grained debugging |
| `debug!` | `logger.debug()` | Development details |
| `info!` | `logger.info()` | Normal operations |
| `warn!` | `logger.warning()` | Recoverable issues |
| `error!` | `logger.error()` | Failures |

Filtering works via environment variable with `env_logger`:
```bash
RUST_LOG=info cd workshop && cargo run     # Show info and above
RUST_LOG=debug,my_crate=trace cd workshop && cargo run  # Per-module filtering
```

Compared to Python's `loguru.remove()` + `loguru.add(sink, level=...)`.

### Example

```rust
use std::env;
env::set_var("RUST_LOG", "info");
env_logger::init();

// Only info/warn/error produce output
trace!("you won't see me");
info!("you will see me");
```

## 5. `env_logger` — Simple Terminal Logging

### Explanation

`env_logger` is the simplest Rust logging backend — the equivalent of `loguru.add(sys.stderr, format="...")`.

```rust
use env_logger;
use log::info;

fn main() {
    env_logger::builder()
        .format(|buf, record| {
            writeln!(buf, "[{}] {} - {}", record.level(), record.target(), record.args())
        })
        .init();

    info!("hello world");
}
```

Output: `[INFO] my_crate - hello world`

Unlike loguru, env_logger does **not** support structured (key=value) logging out of the box — that's where `tracing` comes in.

### Applying to Our Project

`log_structured_data` will format a log message as structured output (key=value pairs) and return the formatted string.

## 6. `tracing` — Structured and Async Logging

### Explanation

`tracing` is Rust's equivalent of Python's `structlog` — it supports **structured fields** and **spans** (execution context).

In Python with structlog:
```python
import structlog
log = structlog.get_logger()
log.info("request", method="GET", path="/api")
```

In Rust with tracing:
```rust
use tracing::{info, span, Level};
use tracing_subscriber;

tracing_subscriber::fmt::init();

let span = span!(Level::INFO, "request", method = "GET", path = "/api");
let _guard = span.enter();
info!("handling request");
```

Spans are like context managers: they track duration and parent-child relationships — perfect for tracing requests through a pipeline.

| Concept | Python | Rust (tracing) |
|---------|--------|----------------|
| Structured event | `log.info("evt", key=val)` | `info!(key = val, "evt")` |
| Span/context | `with loguru.catch():` | `let _g = span.enter();` |
| Async tracing | `loguru.patch()` | Native support via `tracing-futures` |

### Applying to Our Project

`tracing_demo` creates nested spans at increasing depths and returns their names in order, demonstrating how spans compose.

## 7. Putting It All Together

Open `workshop/src/lib.rs` and replace each `todo!()`:

**`demo_log_levels`** — Use a custom `Log` implementation that collects messages into a `Vec<String>`, log at all five levels, and return the captured strings.

**`log_structured_data`** — Format a structured log entry as `key=value` string.

**`tracing_demo`** — Create nested tracing spans using a recursive helper, capturing span names.

**`logging_overhead`** — Time a loop with and without logging calls.

**`loguru_equivalents`** — Return mapping pairs like `("log::info!", "logger.info()")`.

Run tests after each function:
```bash
cd workshop && cargo test
```

## 8. Complete Code Reference

```rust
use log::{info, warn, error, debug, trace, Level, LevelFilter, Log, Metadata, Record};
use std::sync::Mutex;

pub fn demo_log_levels() -> Vec<String> {
    struct CaptureLog(Mutex<Vec<String>>);
    impl Log for CaptureLog {
        fn enabled(&self, _: &Metadata) -> bool { true }
        fn log(&self, record: &Record) {
            let msg = format!("{} - {}", record.level(), record.args());
            self.0.lock().unwrap().push(msg);
        }
        fn flush(&self) {}
    }
    let collector = CaptureLog(Mutex::new(Vec::new()));
    log::set_boxed_logger(Box::new(collector)).unwrap();
    log::set_max_level(LevelFilter::Trace);
    trace!("trace message");
    debug!("debug message");
    info!("info message");
    warn!("warn message");
    error!("error message");
    // Return the logged messages
    todo!()
}

pub fn log_structured_data(key: &str, value: &str) -> String {
    format!("{}={}", key, value)
}

pub fn tracing_demo(depth: usize) -> Vec<String> {
    // Initialize tracing subscriber for testing
    let _ = tracing_subscriber::fmt()
        .with_test_writer()
        .try_init();
    let mut spans = Vec::new();
    for i in 0..depth {
        let span = tracing::span!(tracing::Level::INFO, "depth_{}", i);
        let _guard = span.enter();
        spans.push(format!("depth_{}", i));
    }
    spans
}

pub fn logging_overhead(iterations: usize) -> (u64, u64) {
    let _ = env_logger::builder().is_test(true).try_init();
    use std::time::Instant;
    let start = Instant::now();
    for i in 0..iterations {
        let _ = i + 1;
    }
    let no_log = start.elapsed().as_nanos() as u64;
    let start = Instant::now();
    for i in 0..iterations {
        info!("iteration {}", i);
    }
    let with_log = start.elapsed().as_nanos() as u64;
    (with_log, no_log)
}

pub fn loguru_equivalents() -> Vec<(&'static str, &'static str)> {
    vec![
        ("log::info!", "logger.info()"),
        ("log::warn!", "logger.warning()"),
        ("log::error!", "logger.error()"),
        ("log::debug!", "logger.debug()"),
        ("log::trace!", "logger.trace()"),
        ("env_logger::init()", "logger.add(sys.stderr)"),
        ("tracing span", "loguru.contextualize()"),
    ]
}
```

## 9. Summary

| Concept | Where Used | Python Equivalent |
|---------|-----------|-------------------|
| `log` crate facade | All logging macros | `loguru.logger` |
| `env_logger` | Terminal output | `logger.add(sys.stderr)` |
| `tracing` spans | Nested context tracking | `loguru.contextualize()` |
| `RUST_LOG` env var | Level filtering | `loguru.remove()/add()` |
| Structured logging | Key-value format | `loguru.bind()`, structlog |

## Exercises

* **Easy** – modify the existing function to handle an extra edge case.
* **Medium** – extend the project with a new helper function that re‑uses the core logic.

