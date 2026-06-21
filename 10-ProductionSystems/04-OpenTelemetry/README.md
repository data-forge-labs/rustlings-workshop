# 📊 OpenTelemetry — Traces, Spans, and Correlation IDs for Data Pipelines

*Subtitle: structured `tracing` + `tracing-subscriber` JSON output + atomic pipeline metrics — the OTel data model in pure Rust.*

> **Test-driven approach**: This project includes a Cargo project with progressive
> unit tests. Each function in `src/lib.rs` starts as a `todo!()` stub. As you
> follow each section, replace `todo!()` with real code and run `cargo test` to
> watch the pass count grow. Your goal: **all 14 tests pass**.

---

## What Is This Project?

Structured observability with `tracing` — traces, spans, and correlation IDs for data pipelines.

### Python equivalent

```python
import logging

logging.basicConfig(level=logging.INFO)
logging.info("Processing batch %s", batch_id)
# No spans, no correlation IDs, no structured JSON output
``` You re-run the job and
it works. You're scared of the next failure.

**Rust fix:** `tracing` gives you structured events with spans — the same
data model as OpenTelemetry — but you don't need an OTel collector to start
using it. A single `tracing_subscriber::fmt().json().init()` in `main.rs`
turns every `tracing::info!` into a JSON line that any log aggregator
(Loki, Datadog, ELK) can index. Add a correlation id per request/row, and
the "where did this row go" question becomes a single grep.

```rust
let span = tracing::info_span!("ingest", rows = rows.len());
let _enter = span.enter();
tracing::info!(correlation_id = %cid, "ingested");
```

### Topics covered

| # | Concept | Why it matters |
|---|---------|----------------|
| 1 | Structured events | `tracing::info!` — field-based, not string-concat |
| 2 | Spans | Parent/child timing built in |
| 3 | JSON output | One log line = one event |
| 4 | Correlation IDs | Tie together logs from one request |
| 5 | Atomic metrics | Lock-free, multi-thread safe |
| 6 | `EnvFilter` | Same env var pattern, no code change |

---


---

## Setup: Create the Project from Scratch

This is a hands-on workshop. You will write the code yourself following the steps below.

### 1. Create the new Cargo project

```bash
cargo new --lib open_telemetry_workshop
cd open_telemetry_workshop
```

### 2. Add the dependencies

Open `Cargo.toml` and replace whatever is there with this:

```toml
[package]
name = "open_telemetry_workshop"
version = "0.1.0"
edition = "2024"

[dependencies]
tracing = "0.1"
tracing-subscriber = { version = "0.3", features = ["env-filter", "json", "fmt"] }
uuid = { version = "1", features = ["v4"] }
serde = { version = "1", features = ["derive"] }
serde_json = "1"
chrono = { version = "0.4", features = ["serde"] }
thiserror = "1"

```

### 3. Copy the test stubs as your starting point

This project follows a **test-driven** approach. Each function in `src/lib.rs` starts as a `todo!()` stub, and progressive tests guide your implementation.

```bash
cp "10-ProductionSystems/04-OpenTelemetry/workshop/src/lib.rs" src/lib.rs
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

## Functions to Implement

### Step 1 — Correlation IDs

#### `new_correlation_id`
- **Signature**: `pub fn new_correlation_id() -> String`
- **Task**: `Uuid::new_v4().to_string()`. Each call returns a unique id.

### Step 2 — Log line

#### `format_log_line`
- **Signature**: `pub fn format_log_line(level: &str, target: &str, message: &str, correlation_id: Option<&str>) -> LogLine`
- **Task**: Build a `LogLine` with `timestamp: Utc::now()`, the three string fields, and `correlation_id.map(String::from)`.

### Step 3 — Span record

#### `build_span`
- **Signature**: `pub fn build_span(name: &str, correlation_id: &str, attributes: HashMap<String, String>, duration: Duration) -> SpanRecord`
- **Task**: Build a `SpanRecord` with `name`, `correlation_id`, `start: Utc::now() - duration`, `duration_ms: span_duration_ms(duration)`, `attributes`.

### Step 4 — Closure helper

#### `with_correlation`
- **Signature**: `pub fn with_correlation<F: FnOnce(&str) -> R, R>(correlation_id: &str, f: F) -> R`
- **Task**: `f(correlation_id)`. (In production, the real wrapper would open a `tracing::span!` here and run `f` inside it; this workshop keeps the test pure.)

### Step 5 — Log level

#### `parse_log_level`
- **Signature**: `pub fn parse_log_level(s: &str) -> Result<u8, String>`
- **Task**: Match on uppercase: `"TRACE"→10`, `"DEBUG"→20`, `"INFO"→30`, `"WARN"→40`, `"ERROR"→50`. Otherwise `Err(format!("unknown level: {s}"))`.

### Step 6 — Duration helper

#### `span_duration_ms`
- **Signature**: `pub fn span_duration_ms(d: Duration) -> u64`
- **Task**: `d.as_millis() as u64`.

### Step 7 — Merge

#### `merge_attributes`
- **Signature**: `pub fn merge_attributes(a: HashMap<String, String>, b: HashMap<String, String>) -> HashMap<String, String>`
- **Task**: Build a new map starting with `a`, then extend with `b` (b wins on conflict).

### Step 8 — OTel attribute tuple

#### `otel_attribute`
- **Signature**: `pub fn otel_attribute(key: &str, value: &str) -> (String, String)`
- **Task**: `(key.to_string(), value.to_string())`. The factory lets OTel-typed attribute keys be discovered in one place.

### Step 9 — Atomic metrics

#### `PipelineMetrics::record_success` / `record_failure` / `record_span` / `snapshot`
- **Task**: Each `record_*` does `self.<field>.fetch_add(1, Ordering::Relaxed)`. `snapshot` returns `(processed.load(...), failed.load(...), spans_emitted.load(...))`.

## Test Modules

| Module | Tests | What It Tests |
|--------|-------|---------------|
| step_01_correlation | 2 | uuid parsing + uniqueness |
| step_02_log_line | 2 | basic + with correlation |
| step_03_span | 1 | build span fields + attrs |
| step_04_with_correlation | 2 | passes id + returns value |
| step_05_log_level | 2 | ok + unknown |
| step_06_duration | 1 | ms + sec conversion |
| step_07_merge | 2 | disjoint + overlap (b wins) |
| step_08_otel_attr | 1 | tuple passthrough |
| step_09_metrics | 4 | success / failure / span / mixed |

## How to Run Tests
```bash
cargo test
```

## Exercises

* **Easy** – modify the existing function to handle an extra edge case.
* **Medium** – extend the project with a new helper function that re‑uses the core logic.

