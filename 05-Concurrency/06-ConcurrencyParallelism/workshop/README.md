# Workshop: Concurrency vs Parallelism

**Goal**: Review the implemented functions in `src/lib.rs` and pass all 20 tests.

## Functions to Study

### `is_send` / `is_sync`
- **Signature**: `pub fn is_send<T: Send>(_: &T) -> bool` / `pub fn is_sync<T: Sync>(_: &T) -> bool`
- **Task**: Generic trait-bound functions that always return `true` (compile-time demonstrations). Already implemented.
- **Tests**: test_is_send_with_i32, test_is_send_with_arc, test_is_sync_with_i32, test_is_sync_with_arc, test_is_sync_with_string

### `rwlock_read_heavy`
- **Signature**: `pub fn rwlock_read_heavy(readers: usize, ops_per_reader: usize) -> usize`
- **Task**: Spawn `readers` threads that each acquire a read lock `ops_per_reader` times. Already implemented.
- **Tests**: test_rwlock_read_heavy_normal, test_rwlock_read_heavy_zero_readers, test_rwlock_read_heavy_zero_ops

### `rwlock_write_once`
- **Signature**: `pub fn rwlock_write_once(ops: usize) -> usize`
- **Task**: Spawn `ops` threads that each acquire a write lock and increment. Already implemented.
- **Tests**: test_rwlock_write_once_normal, test_rwlock_write_once_zero_ops, test_rwlock_write_once_single

### `parallel_sum`
- **Signature**: `pub fn parallel_sum(data: Vec<i32>) -> i32`
- **Task**: Sum a vector in parallel using thread::scope and chunks. Already implemented.
- **Tests**: test_parallel_sum_normal, test_parallel_sum_empty, test_parallel_sum_single_element, test_parallel_sum_large

### `parallel_map`
- **Signature**: `pub fn parallel_map(data: Vec<i32>, mapper: fn(i32) -> i32) -> Vec<i32>`
- **Task**: Map a function over a vector in parallel using thread::scope. Already implemented.
- **Tests**: test_parallel_map_double, test_parallel_map_empty, test_parallel_map_identity, test_parallel_map_negate

## Test Modules

| Module | Tests | What It Tests |
|--------|-------|---------------|
| step_01_send_sync | 6 | Send/Sync trait bounds |
| step_02_rwlock | 6 | RwLock read-heavy and write-once patterns |
| step_03_parallelism | 8 | Parallel sum and map with thread::scope |

## How to Run Tests
```bash
cargo test
```

## Exercises

* **Easy** – modify the existing function to handle an extra edge case.
* **Medium** – extend the project with a new helper function that re‑uses the core logic.

