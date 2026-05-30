# Workshop: Data Races & Race Conditions

**Goal**: Implement all functions in `src/lib.rs` to pass all 12 tests.

## Functions to Implement

### `cell_counter`
- **Signature**: `pub fn cell_counter(ops: usize) -> usize`
- **Task**: Increment a `Cell<usize>` `ops` times and return the final value.
- **Tests**: test_cell_counter_basic, test_cell_counter_zero, test_cell_counter_large

### `cell_string`
- **Signature**: `pub fn cell_string(initial: &str, append: &str) -> String`
- **Task**: Use a `Cell<String>` equivalent pattern to concatenate two strings.
- **Tests**: test_cell_string_basic, test_cell_string_empty_initial, test_cell_string_empty_append

### `refcell_demo`
- **Signature**: `pub fn refcell_demo(values: Vec<i32>) -> Vec<i32>`
- **Task**: Double each value using `RefCell` interior mutability.
- **Tests**: test_refcell_demo_basic, test_refcell_demo_empty, test_refcell_demo_single

### `refcell_borrow_error`
- **Signature**: `pub fn refcell_borrow_error() -> Result<String, String>`
- **Task**: Violate `RefCell` borrow rules and return a descriptive error.
- **Tests**: test_refcell_borrow_error_violation

### `simulate_race_condition`
- **Signature**: `pub fn simulate_race_condition() -> usize`
- **Task**: Spawn 8 threads each incrementing a shared counter 1000 times WITHOUT synchronization, causing lost updates.
- **Tests**: test_simulate_race_condition_lost_updates, test_simulate_race_condition_non_zero

## Test Modules

| Module | Tests | What It Tests |
|--------|-------|---------------|
| step_01_cell | 6 | Cell counter and Cell string operations |
| step_02_refcell | 4 | RefCell mutable borrowing and borrow error |
| step_03_race_conditions | 2 | Unsynchronized race condition demonstration |

## How to Run Tests
```bash
cargo test
```
