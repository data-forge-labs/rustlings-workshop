# 📊 OpenTelemetry — Traces, Spans, and Correlation IDs for Data Pipelines

*Subtitle: structured `tracing` + `tracing-subscriber` JSON output + atomic pipeline metrics — the OTel data model in pure Rust.*

> **Test-driven approach**: This project includes a Cargo project with progressive
> unit tests. Each function in `src/lib.rs` starts as a `todo!()` stub. As you
> follow each section, replace `todo!()` with real code and run `cargo test` to
> watch the pass count grow. Your goal: **all 14 tests pass**.

---

## Why Observability for Data Pipelines?

**Python pain:** A Python ETL pipeline fails silently. You only know because
yesterday's `customer_count` is in the row count table — three hours late.
The traceback is gone. The Airflow log was rotated. You re-run the job and
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

## At a Glance

| # | Concept | Rust | Python | Why it matters |
|---|---------|------|--------|----------------|
| 1 | Structured events | `tracing::info!` | `logging.info` (msg=...) | Field-based, not string-concat |
| 2 | Spans | `tracing::info_span!` | `with tracer.start_as_current_span(...)` | Parent/child timing built in |
| 3 | JSON output | `tracing_subscriber::fmt().json()` | `python-json-logger` | One log line = one event |
| 4 | Correlation ids | `Uuid::new_v4()` | `uuid.uuid4()` | Tie together logs from one request |
| 5 | Span attributes | `HashMap<String, String>` | OTel `Attributes` | Key-value context on a span |
| 6 | Atomic metrics | `AtomicU64::fetch_add` | `threading.Lock` counter | Lock-free, multi-thread safe |
| 7 | Log level filter | `EnvFilter` | `LOG_LEVEL=INFO` | Same env var pattern, no code change |
| 8 | OTel data model | `SpanRecord` struct | `ReadableSpan` | The shape that exporters send |

---
