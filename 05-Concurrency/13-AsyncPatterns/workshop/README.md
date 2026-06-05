# Workshop: Async Patterns — Real-World Tokio

> **Test-driven approach**: This project includes a Cargo project with progressive
> async tests. Each function in `src/lib.rs` starts as a `todo!()` stub. As you
> follow each section, replace `todo!()` with real code and run `cargo test` to
> watch the pass count grow. Your goal: **all 12 tests pass**.

**Goal**: Implement all functions in `src/lib.rs` to pass all 12 tests.

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
