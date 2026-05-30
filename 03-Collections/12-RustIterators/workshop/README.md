# Workshop: RustIterators — Lazy Functional Iteration

**Goal**: Implement all functions in `src/lib.rs` to pass all **12** tests.

## Functions to Implement

### `sum_with_fold`
- **Signature**: `pub fn sum_with_fold(numbers: &[i32]) -> i32`
- **Task**: Sum elements using `iter().fold()`
- **Tests**: step_01_fold (2 tests)

### `keep_even`
- **Signature**: `pub fn keep_even(numbers: &[i32]) -> Vec<i32>`
- **Task**: Filter to keep only even numbers
- **Tests**: step_02_filter (2 tests)

### `double_all`
- **Signature**: `pub fn double_all(numbers: &[i32]) -> Vec<i32>`
- **Task**: Map each element to `x * 2`
- **Tests**: step_03_map (2 tests)

### `take_first_n`
- **Signature**: `pub fn take_first_n<T: Clone>(items: &[T], n: usize) -> Vec<T>`
- **Task**: Take first `n` elements (or all if n > len)
- **Tests**: step_04_take_skip (2 tests)

### `skip_first_n`
- **Signature**: `pub fn skip_first_n<T: Clone>(items: &[T], n: usize) -> Vec<T>`
- **Task**: Skip first `n` elements
- **Tests**: step_04_take_skip (2 tests)

### `reverse_slice`
- **Signature**: `pub fn reverse_slice<T: Clone>(items: &[T]) -> Vec<T>`
- **Task**: Reverse elements using `.rev()`
- **Tests**: step_05_rev (1 test)

### `flatten`
- **Signature**: `pub fn flatten<T: Clone>(nested: Vec<Vec<T>>) -> Vec<T>`
- **Task**: Flatten a nested Vec using `.flatten()`
- **Tests**: step_06_flatten (1 test)

## Test Modules

| Module | Tests | What It Tests |
|--------|-------|---------------|
| step_01_fold | 2 | fold accumulator |
| step_02_filter | 2 | filter combinator |
| step_03_map | 2 | map combinator |
| step_04_take_skip | 4 | take, skip adapters |
| step_05_rev | 1 | rev adapter |
| step_06_flatten | 1 | flatten on nested iterators |

## How to Run Tests
```bash
cargo test
```
