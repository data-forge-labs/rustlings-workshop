# Workshop: ETL Pipeline — Source → Transform → Sink as Actors

> **Test-driven approach**: This project includes a Cargo project with progressive
> async tests. Each function in `src/lib.rs` starts as a `todo!()` stub. As you
> follow each section, replace `todo!()` with real code and run `cargo test` to
> watch the pass count grow. Your goal: **all 5 tests pass**.

**Goal**: Implement all functions in `src/lib.rs` to pass all 5 tests.

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
