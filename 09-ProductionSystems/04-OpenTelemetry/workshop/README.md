# Workshop: OpenTelemetry — Traces, Spans, and Correlation IDs

> **Test-driven approach**: This project includes a Cargo project with progressive
> unit tests. Each function in `src/lib.rs` starts as a `todo!()` stub. As you
> follow each section, replace `todo!()` with real code and run `cargo test` to
> watch the pass count grow. Your goal: **all 14 tests pass**.

**Goal**: Implement all functions in `src/lib.rs` to pass all 14 tests.

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
