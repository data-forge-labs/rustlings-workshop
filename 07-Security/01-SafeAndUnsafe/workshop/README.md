# Workshop: SafeAndUnsafe

**Goal**: Implement all 6 functions in `src/lib.rs` to pass all 13 tests.

## Functions to Implement

### `safe_add`
- **Signature**: `pub fn safe_add(a: i32, b: i32) -> i32`
- **Task**: Return the sum of `a` and `b` (safe Rust).
- **Tests**: test_safe_add_normal, test_safe_add_negative, test_safe_add_zero

### `unsafe_dereference`
- **Signature**: `pub unsafe fn unsafe_dereference(ptr: *const i32) -> i32`
- **Task**: Dereference the raw pointer and return its value, or 0 if null.
- **Tests**: test_unsafe_dereference_valid, test_unsafe_dereference_null

### `unsafe_write`
- **Signature**: `pub unsafe fn unsafe_write(ptr: *mut i32, val: i32)`
- **Task**: Write `val` to the memory location pointed to by `ptr`.
- **Tests**: test_unsafe_write

### `safe_split_sum`
- **Signature**: `pub fn safe_split_sum(slice: &mut [i32]) -> (i32, i32)`
- **Task**: Split the slice in half and return the sum of each half (0 for empty).
- **Tests**: test_safe_split_sum_even, test_safe_split_sum_odd, test_safe_split_sum_empty

### `safe_index`
- **Signature**: `pub fn safe_index(slice: &[i32], index: usize) -> Option<i32>`
- **Task**: Return `Some(value)` at index, or `None` if out of bounds.
- **Tests**: test_safe_index_valid, test_safe_index_out_of_bounds

### `safety_concepts`
- **Signature**: `pub fn safety_concepts() -> Vec<&'static str>`
- **Task**: Return a list of safety concepts covered (e.g., "unsafe", "raw pointer").
- **Tests**: test_safety_concepts_non_empty, test_safety_concepts_contains_keywords

## Test Modules

| Module | Tests | What It Tests |
|--------|-------|---------------|
| step_01_safe_functions | 5 | Safe addition and safe indexing |
| step_02_unsafe_functions | 3 | Raw pointer dereference and write |
| step_03_memory_safety | 3 | `split_at_mut` safe interior mutability |
| step_04_concepts | 2 | Listing covered safety concepts |

## How to Run Tests
```bash
cargo test
```
