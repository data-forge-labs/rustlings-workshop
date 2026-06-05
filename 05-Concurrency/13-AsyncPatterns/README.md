# 🌀 Async Patterns — Real-World Tokio

*Subtitle: `select!`, `Semaphore`, `Notify`, `JoinSet`, `CancellationToken` — the patterns every production async Rust program uses.*

> **Test-driven approach**: This project includes a Cargo project with progressive
> async tests. Each function in `src/lib.rs` starts as a `todo!()` stub. As you
> follow each section, replace `todo!()` with real code and run `cargo test` to
> watch the pass count grow. Your goal: **all 12 tests pass**.

---

## Why Tokio Patterns Beat Ad-Hoc Async

**Python pain:** `asyncio.gather` returns when all are done. `asyncio.wait_for`
times out. But you need to *cancel the rest when one fails* (`.gather` with
`return_exceptions=False`). You need to *limit concurrency* (use
`asyncio.Semaphore` and acquire per task). You need *graceful shutdown* (set
a flag and check it in every coroutine). The same five lines appear in every
codebase, slightly differently, all subtly broken.

**Rust fix:** Tokio gives you battle-tested primitives. `tokio::select!`
is the asynchronous equivalent of a `match` over futures — whichever
completes first wins, the rest are cancelled. `tokio::sync::Semaphore` is
`asyncio.Semaphore` but lock-free and cloneable. `tokio_util::sync::CancellationToken`
is the standard shutdown signal: parent cancels, all `child.cancelled().await`
futures wake up simultaneously.

```rust
tokio::select! {
    r = work() => process(r),
    _ = token.cancelled() => cleanup(),
    _ = tokio::time::sleep(Duration::from_secs(30)) => retry(),
}
```

## At a Glance

| # | Concept | Rust | Python | Why it matters |
|---|---------|------|--------|----------------|
| 1 | `tokio::select!` | `select! { a => .., b => .. }` | `asyncio.wait({a, b}, return_when=FIRST_COMPLETED)` | Race N futures, cancel the rest |
| 2 | `timeout` | `tokio::time::timeout(d, fut)` | `asyncio.wait_for(coro, d)` | Bound any future by time |
| 3 | `Semaphore` | `Semaphore::new(n)`, `acquire_owned()` | `asyncio.Semaphore(n)` | Limit concurrency |
| 4 | `Notify` | `notify_one()` / `notify_waiters()` | `asyncio.Event` | One-shot or broadcast event |
| 5 | `JoinSet` | `JoinSet::spawn(f)` | `asyncio.TaskGroup` (3.11+) | Fan out + collect, abort on error |
| 6 | `mpsc::channel(n)` | bounded → backpressure | `asyncio.Queue(maxsize=n)` | Apply backpressure to producers |
| 7 | `CancellationToken` | `tokio_util::sync::CancellationToken` | custom `asyncio.Event` per shutdown | Tree-cancellable shutdown |
| 8 | `buffer_unordered(n)` | `StreamExt::buffer_unordered` | `asyncio.as_completed` + semaphore | Run up to N at once, preserve stream order |

---
