# 🎭 Ractor — Production-Grade Actor Framework

*Subtitle: the `ractor` crate wraps the DIY pattern with supervision, RPC, and clustering.*

> **Test-driven approach**: This project includes a Cargo project with progressive
> async tests. Each function in `src/lib.rs` starts as a `todo!()` stub. As you
> follow each section, replace `todo!()` with real code and run `cargo test` to
> watch the pass count grow. Your goal: **all 6 tests pass**.

---

## Why a Crate When You Can DIY?

**Python pain:** You built the DIY actor in project 01-DIY-Actor. It works.
But you discover you need: link monitoring (reconnect on peer death), remote
actor references, `GenServer`-style calls, supervisor restart strategies.
You re-implement them. Every project re-implements them, slightly differently,
all subtly broken.

**Rust fix:** `ractor` provides the production patterns. It implements the
Erlang `gen_server` model: a typed `Actor` trait with `pre_start`, `handle`,
`post_stop`. `cast` for fire-and-forget, `call` for typed request/response
with `CallResult`. Supervision and clustering are opt-in features. The DIY
project shows you what ractor is doing under the hood.

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
