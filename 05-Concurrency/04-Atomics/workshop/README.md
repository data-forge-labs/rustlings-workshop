# Workshop: Atomics

**Goal**: Review the implemented functions in `src/lib.rs` and pass all 12 tests.

## Functions to Study

### `atomic_counter`
- **Signature**: `pub fn atomic_counter(ops: usize) -> usize`
- **Task**: Spawn 4 threads each doing `ops` `fetch_add(1, Relaxed)` on an `AtomicUsize`. Already implemented.
- **Tests**: test_atomic_counter_normal, test_atomic_counter_zero_ops

### `atomic_flag_toggle`
- **Signature**: `pub fn atomic_flag_toggle() -> bool`
- **Task**: Toggle an `AtomicBool` from `false` to `true` in a spawned thread. Already implemented.
- **Tests**: test_atomic_flag_toggles_true, test_atomic_flag_toggle_repeated

### `relaxed_ordering_demo`
- **Signature**: `pub fn relaxed_ordering_demo() -> (usize, usize)`
- **Task**: Demonstrate `Relaxed` ordering with two atomics. Already implemented.
- **Tests**: test_relaxed_ordering_both_equal, test_relaxed_ordering_non_zero

### `acquire_release_demo`
- **Signature**: `pub fn acquire_release_demo() -> (usize, usize)`
- **Task**: Demonstrate `Acquire`/`Release` ordering for data visibility. Already implemented.
- **Tests**: test_acquire_release_data_visible, test_acquire_release_flag_set

### `fetch_add_demo`
- **Signature**: `pub fn fetch_add_demo(values: Vec<usize>) -> usize`
- **Task**: Sum values across threads using `fetch_add`. Already implemented.
- **Tests**: test_fetch_add_sum, test_fetch_add_empty, test_fetch_add_large, test_fetch_add_single_element

## Test Modules

| Module | Tests | What It Tests |
|--------|-------|---------------|
| step_01_atomic_types | 4 | AtomicUsize counter and AtomicBool toggle |
| step_02_memory_ordering | 4 | Relaxed and Acquire/Release ordering |
| step_03_atomic_operations | 4 | Fetch-add parallel sum |

## How to Run Tests
```bash
cargo test
```
