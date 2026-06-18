# 🌀 Async Patterns — Real-World Tokio

*Subtitle: `select!`, `Semaphore`, `Notify`, `JoinSet`, `CancellationToken` — the patterns every production async Rust program uses.*

> **Test-driven approach**: This project includes a Cargo project with progressive
> async tests. Each function in `src/lib.rs` starts as a `todo!()` stub. As you
> follow each section, replace `todo!()` with real code and run `cargo test` to
> watch the pass count grow. Your goal: **all 12 tests pass**.

---

## What Are Async Patterns?

Production-ready Tokio patterns — `select!`, `Semaphore`, `Notify`, `JoinSet`, `CancellationToken`.

### Python equivalent

```python
import asyncio

async def fetch(url):
    async with aiohttp.ClientSession() as session:
        return await session.get(url)

# Ad-hoc: gather, timeout, cancel, semaphore — all manual
results = await asyncio.gather(*[fetch(u) for u in urls])
```
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

> **Test-driven approach**: This project includes a Cargo project with progressive
> async tests. Each function in `src/lib.rs` starts as a `todo!()` stub. As you
> follow each section, replace `todo!()` with real code and run `cargo test` to
> watch the pass count grow. Your goal: **all 12 tests pass**.

**Goal**: Implement all functions in `src/lib.rs` to pass all 12 tests.


---

## Setup: Create the Project from Scratch

This is a hands-on workshop. You will write the code yourself following the steps below.

### 1. Create the new Cargo project

```bash
cargo new --lib async_patterns_workshop
cd async_patterns_workshop
```

### 2. Add the dependencies

Open `Cargo.toml` and replace whatever is there with this:

```toml
[package]
name = "async_patterns_workshop"
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
cp "05-Concurrency/13-AsyncPatterns/workshop/src/lib.rs" src/lib.rs
cp "05-Concurrency/13-AsyncPatterns/workshop/src/main.rs" src/main.rs
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

## Functions to Implement

### Step 1 — `tokio::select!` and `timeout`

#### `race_two`
- **Signature**: `pub async fn race_two(fast: Duration, slow: Duration) -> &'static str`
- **Task**: `tokio::select! { _ = tokio::time::sleep(fast) => "fast", _ = tokio::time::sleep(slow) => "slow" }`.

#### `with_timeout`
- **Signature**: `pub async fn with_timeout<F: Future>(fut: F, limit: Duration) -> Result<F::Output, Duration>`
- **Task**: `match timeout(limit, fut).await { Ok(v) => Ok(v), Err(_) => Err(limit) }`.

### Step 2 — `Semaphore`

#### `acquire_permits`
- **Signature**: `pub async fn acquire_permits(sem: &Semaphore, n: u32) -> Vec<OwnedSemaphorePermit>`
- **Task**: Build a vec of `n` permits via `sem.clone().acquire_owned().await.unwrap()`.

#### `run_with_concurrency_limit`
- **Signature**: `pub async fn run_with_concurrency_limit<F, T>(items, limit, op) -> Vec<T>`
- **Task**: Use `futures::stream::iter(items).map(...).buffer_unordered(limit).collect().await`. (No `Semaphore` needed — `buffer_unordered` provides the cap.)

### Step 3 — `Notify`

#### `notify_one`
- **Task**: `notify.notify_one()`.

#### `wait_for`
- **Task**: `notify.notified().await`.

### Step 4 — `JoinSet`

#### `joinset_spawn_all`
- **Task**: For each `i` in `0..count`, call `set.spawn(f(i))`, then drain with `while let Some(r) = set.join_next().await { out.push(r.unwrap()) }`. Use `Arc<AtomicUsize>` for ordering or just iterate `0..count` and push to a `Vec` directly.

#### `joinset_first_error`
- **Task**: Spawn all; race to the first `Err` and abort the rest. Use `tokio::select!` with `join_next` and cancel the set on first error.

### Step 5 — Bounded `mpsc`

#### `bounded_send_n`
- **Task**: For `i in 1..=n`, `tx.send(i).await?`.

#### `bounded_drain`
- **Task**: `let mut out = Vec::with_capacity(n); while out.len() < n { if let Some(v) = rx.recv().await { out.push(v) } else { break } } out`.

### Step 6 — `CancellationToken`

#### `is_cancelled`
- **Task**: `token.is_cancelled()`.

#### `cancel_after`
- **Task**: `tokio::time::sleep(delay).await; token.cancel()`.

## Test Modules

| Module | Tests | What It Tests |
|--------|-------|---------------|
| step_01_select | 3 | race winner, timeout ok, timeout expires |
| step_02_semaphore | 2 | acquire + concurrency limit ≤ 4 |
| step_03_notify | 1 | one waiter wakes, one sleeps |
| step_04_joinset | 2 | spawn-all + first-error |
| step_05_bounded | 2 | send n + drain n |
| step_06_cancel | 2 | is_cancelled + cancel_after propagates |

## How to Run Tests
```bash
cargo test
```

## Exercises

* **Easy** – modify the existing function to handle an extra edge case.
* **Medium** – extend the project with a new helper function that re‑uses the core logic.

