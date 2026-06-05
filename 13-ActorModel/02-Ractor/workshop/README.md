# Workshop: Ractor — Production-Grade Actor Framework

> **Test-driven approach**: This project includes a Cargo project with progressive
> async tests. Each function in `src/lib.rs` starts as a `todo!()` stub. As you
> follow each section, replace `todo!()` with real code and run `cargo test` to
> watch the pass count grow. Your goal: **all 6 tests pass**.

**Goal**: Implement all functions in `src/lib.rs` to pass all 6 tests.

## The Actor

```rust
pub struct Counter;

#[ractor::async_trait]
impl Actor for Counter {
    type Msg = CounterMsg;
    type State = CounterState;
    type Arguments = ();

    async fn pre_start(&self, _myself, _: ()) -> Result<Self::State, ...> { ... }
    async fn handle(&self, _myself, message, state: &mut State) -> Result<(), ...> { ... }
}
```

## Functions to Implement

### `pre_start`
- **Task**: `Ok(CounterState { value: 0 })`.

### `handle`
- **Task**: Match on `message`:
- `Increment(d)` → `state.value += d`
- `Decrement(d)` → `state.value -= d`
- `Reset` → `state.value = 0`
- `Ok(())` at the end.

### `spawn_counter`
- **Task**: `Counter.spawn(())`. Returns the `ActorRef` and `JoinHandle`.

### `cast_increment` / `cast_decrement`
- **Task**: `actor.cast(CounterMsg::Increment(delta))` (note: `cast` is sync in ractor; wrap in `Ok(())`).

Actually ractor's `cast` returns `Result<(), SendError>` directly. So `actor.cast(...)` works.

### `call_get_value`
- **Task**: Use `ractor::call!(actor, CounterMsg::Increment, 0)`. The reply type is `i32`. But `Increment` doesn't reply. Use the `rpc::CallResult` API:
- `ractor::call!(actor, ractor::msg::GetState)`. Actually, the simpler approach: use the built-in `GetState` message that ractor auto-generates, or define a special `Get` message.

For the test, the simplest: implement `call_get_value` as:
```rust
let (tx, rx) = oneshot::channel();
actor.cast(CounterMsg::Get { reply: tx }).map_err(|_| "send failed")?;
rx.await.map_err(|_| "actor dropped")
```

This requires adding `Get { reply: oneshot::Sender<i32> }` to the enum. The test does the same thing under the hood.

Or, use `ractor::call!` with the built-in `GetState` request. The reply type is `CounterState`, so we extract `.value`.

For simplicity, let me use the built-in approach:
```rust
let r: ractor::rpc::CallResult<CounterState> = ractor::call!(actor, ractor::msg::GetState);
match r {
    CallResult::Success(s) => Ok(s.value),
    _ => Err("rpc failed"),
}
```

This avoids modifying the enum.

### `stop_counter`
- **Task**: `actor.stop(None)`. Returns `()`.

## Test Modules

| Module | Tests | What It Tests |
|--------|-------|---------------|
| (top-level) | 6 | spawn at 0, inc/dec, 50 casts, reset, 1000 casts, rpc CallResult type |

## How to Run Tests
```bash
cargo test
```

## Production Notes
- Ractor 0.13 is the current version; see https://docs.rs/ractor.
- For Erlang-style supervisors and clustering, enable the `cluster` feature.
