# 🔗 ETL Pipeline — Source → Transform → Sink as Actors

*Subtitle: three Tokio tasks, two bounded channels, atomic metrics — an entire data pipeline.*

> **Test-driven approach**: This project includes a Cargo project with progressive
> async tests. Each function in `src/lib.rs` starts as a `todo!()` stub. As you
> follow each section, replace `todo!()` with real code and run `cargo test` to
> watch the pass count grow. Your goal: **all 5 tests pass**.

---

## Why Pipeline Stages as Independent Tasks?

**Python pain:** A pandas ETL script reads CSV, transforms in a loop, writes
to Postgres. Memory blows up at 50M rows because the loop holds every row in
memory. Adding backpressure requires manual chunking. Adding parallelism
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
