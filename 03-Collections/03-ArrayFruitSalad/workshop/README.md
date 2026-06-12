# Workshop: ArrayFruitSalad — Fixed-Size Arrays [T; N]

**Goal**: Implement all functions in `src/lib.rs` to pass all **6** tests.

## Functions to Implement

### `take_first`
- **Signature**: `pub fn take_first<T: Copy>(arr: &[T; 5], n: usize) -> Vec<T>`
- **Task**: Return a Vec of the first `n` elements (or fewer if n > 5)
- **Tests**: test_take_first_three, test_take_first_zero

### `sum_array`
- **Signature**: `pub fn sum_array(arr: &[i32; 5]) -> i32`
- **Task**: Sum all 5 elements
- **Tests**: test_sum_array, test_sum_array_negatives

### `max_array`
- **Signature**: `pub fn max_array(arr: &[i32; 5]) -> i32`
- **Task**: Find the maximum element
- **Tests**: test_max_array, test_max_array_negative

### `reverse_array`
- **Signature**: `pub fn reverse_array(arr: &[i32; 5]) -> [i32; 5]`
- **Task**: Reverse the array into a new fixed-size array
- **Tests**: test_reverse_array

## Test Modules

| Module | Tests | What It Tests |
|--------|-------|---------------|
| step_01_array_ops | 6 | Array indexing, copying, sum, max, reverse |

## How to Run Tests
```bash
cargo test
```

## Exercises

* **Easy** – modify the existing function to handle an extra edge case.
* **Medium** – extend the project with a new helper function that re‑uses the core logic.

