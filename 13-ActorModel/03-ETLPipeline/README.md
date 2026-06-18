# 🔗 ETL Pipeline — Source → Transform → Sink as Actors

*Subtitle: three Tokio tasks, two bounded channels, atomic metrics — an entire data pipeline.*

> **Test-driven approach**: This project includes a Cargo project with progressive
> async tests. Each function in `src/lib.rs` starts as a `todo!()` stub. As you
> follow each section, replace `todo!()` with real code and run `cargo test` to
> watch the pass count grow. Your goal: **all 5 tests pass**.

---

## What Is This Project?

ETL pipeline with Source → Transform → Sink as independent Tokio tasks with bounded channels.

### Python equivalent

```python
import pandas as pd

# Monolithic: reads everything into memory
df = pd.read_csv("input.csv")
df["value"] = df["value"] * 2
df.to_parquet("output.parquet")
``` Adding backpressure requires manual chunking. Adding parallelism
requires Dask or a thread pool. The pipeline is one function — you can't
inspect it mid-run, you can't scale one stage, you can't restart the
transform without re-reading the source.

**Rust fix:** Each stage runs in its own Tokio task with an `mpsc` channel
between them. Channels are bounded → **backpressure is automatic** (the
source blocks when the transform blocks, when the sink blocks). The source
can be swapped for an HTTP poller, the sink for a Kafka producer, the
transform for a Rayon-parallel batch processor — without rewriting the
others. The `PipelineMetrics` struct shows you exactly which stage is
slow.

```rust
let (src_tx, src_rx) = mpsc::channel(4);
let (trans_tx, trans_rx) = mpsc::channel(4);
let m = Arc::new(PipelineMetrics::new());
tokio::spawn(run_source(rows, src_tx, ...));
tokio::spawn(run_transform(src_rx, trans_tx, ..., filter));
tokio::spawn(run_sink(trans_rx, ...));
```

## At a Glance

| # | Concept | Rust | Python | Why it matters |
|---|---------|------|--------|----------------|
| 1 | Source task | `tokio::spawn` over `Vec<Row>` | generator | Reads from any source |
| 2 | Bounded channel | `mpsc::channel(n)` | `asyncio.Queue(maxsize=n)` | Backpressure by construction |
| 3 | Filter | `fn predicate(&Row) -> bool` | `df[df[col] > x]` | Pluggable transformation |
| 4 | Atomic metrics | `AtomicUsize::fetch_add` | `threading.Lock` counter | Lock-free, multi-task safe |
| 5 | Drop sender to close | `drop(tx)` | n/a | Signals end of stream |
| 6 | Sink task | `while let Some(row) = rx.recv()` | async for-loop | Collects or writes anywhere |
| 7 | Backpressure | channel fills → task blocks | manual chunking | Self-throttling |
| 8 | Failure isolation | per-task panic | n/a | One stage down ≠ all down |

---

> **Test-driven approach**: This project includes a Cargo project with progressive
> async tests. Each function in `src/lib.rs` starts as a `todo!()` stub. As you
> follow each section, replace `todo!()` with real code and run `cargo test` to
> watch the pass count grow. Your goal: **all 5 tests pass**.

**Goal**: Implement all functions in `src/lib.rs` to pass all 5 tests.


---

## Setup: Create the Project from Scratch

This is a hands-on workshop. You will write the code yourself following the steps below.

### 1. Create the new Cargo project

```bash
cargo new --lib etl_pipeline_workshop
cd etl_pipeline_workshop
```

### 2. Add the dependencies

Open `Cargo.toml` and replace whatever is there with this:

```toml
[package]
name = "etl_pipeline_workshop"
version = "0.1.0"
edition = "2024"

[dependencies]
tokio = { version = "1", features = ["full"] }
tokio-util = "0.7"
futures = "0.3"
async-trait = "0.1"

```

### 3. Copy the test stubs as your starting point

This project follows a **test-driven** approach. Each function in `src/lib.rs` starts as a `todo!()` stub, and progressive tests guide your implementation.

```bash
cp "13-ActorModel/03-ETLPipeline/workshop/src/lib.rs" src/lib.rs
cp "13-ActorModel/03-ETLPipeline/workshop/src/main.rs" src/main.rs
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

## The Pipeline

Three stages, each running in its own task, connected by bounded `mpsc` channels:

```
[Source] → channel → [Transform (filter)] → channel → [Sink]
```

Backpressure flows naturally: if the sink is slow, the channel between
transform and sink fills, the transform task blocks on `tx.send`, the
source task blocks on its own `tx.send`. The pipeline runs at the rate of
the slowest stage.

## Functions to Implement

### `run_source`
- **Task**: For each `row` in `rows`, `tx.send(row.clone()).await?` (or `ok_or` pattern); bump `metrics.source_emitted` after each success. (Errors are counted in `source_errors`; in this workshop we don't return errors.)

### `run_transform`
- **Task**: `while let Some(row) = rx.recv().await { if predicate(&row) { tx.send(row).await?; metrics.transform_passed += 1 } else { metrics.transform_dropped += 1 } }`.

### `run_sink`
- **Task**: `let mut out = vec![]; while let Some(row) = rx.recv().await { out.push(row); metrics.sink_written += 1 } out`.

### Filter helpers
- **`filter_positive`**: `r.value > 0.0`.
- **`filter_large`**: `r.value > 100.0`.

## Test Modules

| Module | Tests | What It Tests |
|--------|-------|---------------|
| (top-level) | 5 | source emits all, transform filters, sink collects, end-to-end pipeline, empty pipeline |

## How to Run Tests
```bash
cargo test
```

## Why This Pattern Beats a For-Loop

- **Backpressure**: if the sink is slow, the upstream channels fill, the
  upstream task blocks on `tx.send`. The whole pipeline throttles itself.
- **Failure isolation**: if the transform task panics, the source and sink
  keep running until they hit a closed channel. A supervisor can restart
  the transform without taking the pipeline down.
- **Metrics per stage**: `PipelineMetrics` shows you where the bottleneck is
  (low pass rate at transform, low write rate at sink, low emit rate at
  source — each is a different kind of slowness).
- **Horizontal scale**: each stage can be replaced with N parallel tasks
  (just clone the channel sender) to add throughput.

## Exercises

* **Easy** – modify the existing function to handle an extra edge case.
* **Medium** – extend the project with a new helper function that re‑uses the core logic.

