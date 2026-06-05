# Workshop: DIY Actor — Build an Actor from `mpsc` + `oneshot`

> **Test-driven approach**: This project includes a Cargo project with progressive
> async tests. Each function in `src/lib.rs` starts as a `todo!()` stub. As you
> follow each section, replace `todo!()` with real code and run `cargo test` to
> watch the pass count grow. Your goal: **all 6 tests pass**.

**Goal**: Implement all functions in `src/lib.rs` to pass all 6 tests.

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
