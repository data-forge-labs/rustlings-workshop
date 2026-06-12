# Workshop: Data Race

**Goal**: Implement all functions in `src/lib.rs` to pass all 17 tests.

## Functions to Implement

### `greet_thread`
- **Signature**: `pub fn greet_thread() -> String`
- **Task**: Spawn a thread that returns a greeting containing "Hello" or "Hi".
- **Tests**: test_greet_returns_string, test_greet_contains_greeting

### `sum_in_parallel`
- **Signature**: `pub fn sum_in_parallel(data: Vec<i32>) -> i32`
- **Task**: Sum all numbers in a vector using multiple threads.
- **Tests**: test_sum_positive_numbers, test_sum_empty, test_sum_large

### `move_closure_example`
- **Signature**: `pub fn move_closure_example(data: Vec<i32>) -> Vec<i32>`
- **Task**: Double each element using a `move` closure on a spawned thread.
- **Tests**: test_double_values, test_empty, test_single_element

### `shared_counter_arc_mutex`
- **Signature**: `pub fn shared_counter_arc_mutex(ops: usize) -> usize`
- **Task**: Increment an `Arc<Mutex<usize>>` counter `ops` times per thread.
- **Tests**: test_counter_ten, test_counter_zero, test_counter_hundred

### `shared_counter_rwlock`
- **Signature**: `pub fn shared_counter_rwlock(ops: usize) -> usize`
- **Task**: Increment an `Arc<RwLock<usize>>` counter `ops` times per thread.
- **Tests**: test_rwlock_counter_ten, test_rwlock_counter_zero, test_rwlock_counter_hundred

### `condvar_coordinate`
- **Signature**: `pub fn condvar_coordinate() -> String`
- **Task**: Coordinate threads with a Condvar and return a completion message.
- **Tests**: test_condvar_returns_string, test_condvar_contains_status, test_condvar_idempotent

## Test Modules

| Module | Tests | What It Tests |
|--------|-------|---------------|
| step_01_threads | 5 | Greeting thread and parallel sum |
| step_02_move_closures | 3 | Move closure with threads |
| step_03_arc_mutex | 3 | Arc+Mutex shared counter |
| step_04_rwlock | 3 | RwLock shared counter |
| step_05_condvar | 3 | Condition variable coordination |

## How to Run Tests
```bash
cargo test
```

## Exercises

* **Easy** – modify the existing function to handle an extra edge case.
* **Medium** – extend the project with a new helper function that re‑uses the core logic.

