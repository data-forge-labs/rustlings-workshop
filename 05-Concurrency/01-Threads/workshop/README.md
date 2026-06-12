# Workshop: Threads

**Goal**: Implement all functions in `src/lib.rs` to pass all 9 tests.

## Functions to Implement

### `spawn_and_join`
- **Signature**: `pub fn spawn_and_join() -> String`
- **Task**: Spawn a thread that returns a greeting string, then join it.
- **Tests**: test_spawn_and_join

### `sum_in_parallel`
- **Signature**: `pub fn sum_in_parallel(data: Vec<i32>) -> i32`
- **Task**: Split a vector across two threads and sum all elements.
- **Tests**: test_sum_in_parallel, test_sum_in_parallel_empty, test_sum_in_parallel_single

### `channel_send_receive`
- **Signature**: `pub fn channel_send_receive() -> Vec<String>`
- **Task**: Send messages through an mpsc channel and collect them.
- **Tests**: test_channel_send_receive

### `shared_counter`
- **Signature**: `pub fn shared_counter(ops: usize) -> usize`
- **Task**: Increment a shared `Arc<Mutex<usize>>` counter by `ops` across threads.
- **Tests**: test_shared_counter, test_shared_counter_zero

### `rwlock_counter`
- **Signature**: `pub fn rwlock_counter(ops: usize) -> usize`
- **Task**: Increment a shared `Arc<RwLock<usize>>` counter by `ops` across threads.
- **Tests**: test_rwlock_counter

### `scoped_worker`
- **Signature**: `pub fn scoped_worker(data: Vec<i32>) -> Vec<i32>`
- **Task**: Run a closure on a scoped thread that processes the data.
- **Tests**: test_scoped_worker

## Test Modules

| Module | Tests | What It Tests |
|--------|-------|---------------|
| step_01_threads | 4 | Thread spawn/join and parallel sum |
| step_02_scoped_threads | 1 | Scoped thread worker |
| step_03_channels | 1 | mpsc channel message passing |
| step_04_locks | 3 | Mutex and RwLock shared counters |

## How to Run Tests
```bash
cargo test
```

## Exercises

* **Easy** – modify the existing function to handle an extra edge case.
* **Medium** – extend the project with a new helper function that re‑uses the core logic.

