# Workshop: Futures

**Goal**: Implement all functions in `src/lib.rs` to pass all 5 tests.

## Functions to Implement

### `async_hello`
- **Signature**: `pub async fn async_hello() -> String`
- **Task**: Return a greeting string from an async function.
- **Tests**: test_async_hello

### `process_chain`
- **Signature**: `pub async fn process_chain() -> String`
- **Task**: Chain two async operations sequentially using `.await`.
- **Tests**: test_process_chain

### `run_concurrent`
- **Signature**: `pub async fn run_concurrent() -> Vec<String>`
- **Task**: Spawn two concurrent tokio tasks and collect their results.
- **Tests**: test_run_concurrent

### `block_on_hello`
- **Signature**: `pub fn block_on_hello() -> String`
- **Task**: Block on an async function using a tokio runtime.
- **Tests**: test_block_on_hello

### `delayed_greeting`
- **Signature**: `pub async fn delayed_greeting(seconds: u64) -> String`
- **Task**: Simulate a delay using `tokio::time::sleep` then return a greeting.
- **Tests**: test_delayed_greeting

## Test Modules

| Module | Tests | What It Tests |
|--------|-------|---------------|
| step_01_async_fn | 2 | Basic async functions and chaining |
| step_02_spawn | 1 | Concurrent tokio::spawn tasks |
| step_03_runtime | 1 | Blocking on async code |
| step_04_delay | 1 | Async delay/sleep |

## How to Run Tests
```bash
cargo test
```
