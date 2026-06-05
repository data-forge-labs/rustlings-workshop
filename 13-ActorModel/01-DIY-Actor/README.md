# 🎭 DIY Actor — Build an Actor from `mpsc` + `oneshot`

*Subtitle: 30 lines of Tokio = the whole actor model. No crate, no runtime, no magic.*

> **Test-driven approach**: This project includes a Cargo project with progressive
> async tests. Each function in `src/lib.rs` starts as a `todo!()` stub. As you
> follow each section, replace `todo!()` with real code and run `cargo test` to
> watch the pass count grow. Your goal: **all 6 tests pass**.

---

## Why Build Your Own Actor First?

**Python pain:** You reach for Celery, Dramatiq, or RQ to get task queues. The
trade-off: a worker process, a broker (Redis/RabbitMQ), serialization on every
message, and a deployment story. For in-process concurrency — "I want a serial
event loop with mutable state" — the overhead is enormous.

**Rust fix:** An actor in Tokio is **just an `mpsc` loop with mutable state**.
A `tokio::spawn` task owns the state, processes one message at a time, replies
to `oneshot` senders. No locks, no broker, no serialization (in-process).
This workshop builds one in 30 lines so you understand what crates like
`ractor` are doing under the hood.

```rust
let (tx, mut rx) = mpsc::channel(8);
tokio::spawn(async move {
    let mut state = 0i32;
    while let Some(msg) = rx.recv().await {
        match msg {
            CounterMsg::Increment(d) => state += d,
            CounterMsg::Get { reply } => { let _ = reply.send(state); }
            CounterMsg::Stop => break,
        }
    }
});
```

## At a Glance

| # | Concept | Rust | Python | Why it matters |
|---|---------|------|--------|----------------|
| 1 | Mailbox | `tokio::sync::mpsc::channel(n)` | `asyncio.Queue(maxsize=n)` | Bounded queue with backpressure |
| 2 | `oneshot` reply | `oneshot::channel()` | return value of a coroutine | Request/response within the actor model |
| 3 | Tell (fire-and-forget) | `tx.send(msg).await?` | `asyncio.create_task` | Cheap, no reply |
| 4 | Ask (request-response) | `(tx, rx) = oneshot::channel(); tx.send(Get { reply: tx2 })` | n/a | Sync-style call on async state |
| 5 | Single owner of state | the spawned task | n/a | No locks, no `Arc<Mutex<T>>` |
| 6 | Sequential processing | `while let Some(msg) = rx.recv().await` | n/a | Order guaranteed |
| 7 | Graceful shutdown | `Stop` message + `break` | n/a | Drain mailbox, then exit |
| 8 | Supervisor | `JoinHandle` | n/a | Restart on panic if needed |

---
