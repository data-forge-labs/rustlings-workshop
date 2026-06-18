# 🎭 Ractor — Production-Grade Actor Framework

*Subtitle: the `ractor` crate wraps the DIY pattern with supervision, RPC, and clustering.*

> **Test-driven approach**: This project includes a Cargo project with progressive
> async tests. Each function in `src/lib.rs` starts as a `todo!()` stub. As you
> follow each section, replace `todo!()` with real code and run `cargo test` to
> watch the pass count grow. Your goal: **all 6 tests pass**.

---

## What Is Ractor?

Production-grade actor framework — supervision, RPC, and clustering built on the DIY pattern.

### Python equivalent

```python
# Python actor frameworks: pykka, actorio, thespian
from thespian.actors import Actor

class MyActor(Actor):
    def receiveMessage(self, message, sender):
        self.send(sender, "processed")
```

```rust
impl Actor for Counter {
    type Msg = CounterMsg;
    type State = CounterState;
    async fn handle(&self, _myself, msg, state) -> ... {
        match msg { Increment(d) => state.value += d, ... }
    }
}
let (actor, _h) = Counter.spawn(()).await?;
```

## At a Glance

| # | Concept | Rust | Python | Why it matters |
|---|---------|------|--------|----------------|
| 1 | `Actor` trait | `#[ractor::async_trait] impl Actor` | n/a | The framework contract |
| 2 | `pre_start` | `async fn pre_start(...)` | `__init__` | Initial state |
| 3 | `handle` | `async fn handle(msg, state)` | `__call__` | Per-message handler |
| 4 | `cast` (tell) | `actor.cast(Msg::X).unwrap()` | `asyncio.create_task` | Fire-and-forget |
| 5 | `call` (ask) | `ractor::call!(actor, Msg::Get)` | n/a | Typed RPC with reply |
| 6 | `CallResult` | `Success` / `Timeout` / `SenderError` | n/a | Discriminated result |
| 7 | `ActorRef` | `actor_ref.send(...)` | n/a | Cheap clone of a handle |
| 8 | Supervision | `ractor::Supervision` (opt-in) | n/a | Restart-on-panic |

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
cargo new --lib ractor_workshop
cd ractor_workshop
```

### 2. Add the dependencies

Open `Cargo.toml` and replace whatever is there with this:

```toml
[package]
name = "ractor_workshop"
version = "0.1.0"
edition = "2024"

[dependencies]
ractor = "0.13"
tokio = { version = "1", features = ["full"] }

```

### 3. Copy the test stubs as your starting point

This project follows a **test-driven** approach. Each function in `src/lib.rs` starts as a `todo!()` stub, and progressive tests guide your implementation.

```bash
cp "13-ActorModel/02-Ractor/workshop/src/lib.rs" src/lib.rs
cp "13-ActorModel/02-Ractor/workshop/src/main.rs" src/main.rs
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

## Exercises

* **Easy** – modify the existing function to handle an extra edge case.
* **Medium** – extend the project with a new helper function that re‑uses the core logic.

