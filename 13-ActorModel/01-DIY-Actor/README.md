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

> **Test-driven approach**: This project includes a Cargo project with progressive
> async tests. Each function in `src/lib.rs` starts as a `todo!()` stub. As you
> follow each section, replace `todo!()` with real code and run `cargo test` to
> watch the pass count grow. Your goal: **all 6 tests pass**.

**Goal**: Implement all functions in `src/lib.rs` to pass all 6 tests.


---

## Setup: Create the Project from Scratch

This is a hands-on workshop. You will write the code yourself following the steps below.

### 1. Create the new Cargo project

```bash
cargo new --lib diy_actor_workshop
cd diy_actor_workshop
```

### 2. Add the dependencies

Open `Cargo.toml` and replace whatever is there with this:

```toml
[package]
name = "diy_actor_workshop"
version = "0.1.0"
edition = "2024"

[dependencies]
tokio = { version = "1", features = ["full"] }
tokio-util = "0.7"
futures = "0.3"

```

### 3. Copy the test stubs as your starting point

This project follows a **test-driven** approach. Each function in `src/lib.rs` starts as a `todo!()` stub, and progressive tests guide your implementation.

```bash
cp "13-ActorModel/01-DIY-Actor/workshop/src/lib.rs" src/lib.rs
cp "13-ActorModel/01-DIY-Actor/workshop/src/main.rs" src/main.rs
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

## The Actor

An actor has three properties:
1. **Single owner of state** — no locks, no `Arc<Mutex<T>>`.
2. **Processes messages sequentially** — incoming mailbox is an `mpsc` channel.
3. **Talks back via `oneshot` reply channels** for request/response.

```rust
pub enum CounterMsg {
    Increment(i32),
    Decrement(i32),
    Get { reply: oneshot::Sender<i32> },
    Stop,
}
```

## Functions to Implement

### `spawn_counter`
- **Signature**: `pub fn spawn_counter(buffer: usize) -> ActorHandle`
- **Task**: `let (tx, mut rx) = mpsc::channel(buffer); let join = tokio::spawn(async move { let mut state = CounterActor::new(); while let Some(msg) = rx.recv().await { match msg { CounterMsg::Increment(d) => state.value += d, CounterMsg::Decrement(d) => state.value -= d, CounterMsg::Get { reply } => { let _ = reply.send(state.value); } CounterMsg::Stop => break, } } }); ActorHandle { tx, join }`.

### `send_increment` / `send_decrement`
- **Task**: `handle.tx.send(CounterMsg::Increment(delta)).await.map_err(|_| "actor stopped")`.

### `ask_value`
- **Task**: `let (tx, rx) = oneshot::channel(); handle.tx.send(CounterMsg::Get { reply: tx }).await.map_err(|_| "actor stopped")?; rx.await.map_err(|_| "actor dropped reply")`.

### `stop_actor`
- **Task**: Send `Stop`, then `handle.join.await.map_err(|_| "join failed")`.

## Test Modules

| Module | Tests | What It Tests |
|--------|-------|---------------|
| (top-level) | 6 | spawn-at-zero, increment/decrement, 100 increments, stop, order, send-after-stop |

## How to Run Tests
```bash
cargo test
```
