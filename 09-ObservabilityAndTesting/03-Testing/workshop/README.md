# Workshop: Testing

**Goal**: Implement all 6 functions in `src/lib.rs` to pass all 21 tests.

## Functions to Implement

### `add`
- **Signature**: `pub fn add(a: i32, b: i32) -> i32`
- **Task**: Return the sum; panics on overflow (debug mode).
- **Tests**: test_add_normal, test_add_negative, test_add_zero, test_add_overflow

### `divide`
- **Signature**: `pub fn divide(a: f64, b: f64) -> Result<f64, String>`
- **Task**: Return `Ok(a / b)` or `Err` when dividing by zero.
- **Tests**: test_divide_normal, test_divide_by_zero, test_divide_roundtrip, test_divide_result

### `find_item`
- **Signature**: `pub fn find_item<T: PartialEq>(slice: &[T], target: &T) -> Option<usize>`
- **Task**: Return `Some(index)` of first match, or `None` if not found.
- **Tests**: test_find_item_found, test_find_item_not_found, test_find_item_empty_slice

### `fibonacci`
- **Signature**: `pub fn fibonacci(n: u32) -> u64`
- **Task**: Return the nth Fibonacci number (F0=0, F1=1).
- **Tests**: test_fibonacci_0, test_fibonacci_1, test_fibonacci_2, test_fibonacci_10

### `validate_email`
- **Signature**: `pub fn validate_email(email: &str) -> bool`
- **Task**: Return `true` if the email has a local part, `@`, and a domain with a dot.
- **Tests**: test_validate_email_valid, test_validate_email_invalid, test_validate_email_empty

### `test_types`
- **Signature**: `pub fn test_types() -> Vec<(&'static str, &'static str)>`
- **Task**: Return Rust test concepts mapped to Python equivalents (e.g., `("#[test]", "@pytest.mark.test")`).
- **Tests**: test_test_types_non_empty, test_test_types_maps_pytest

## Test Modules

| Module | Tests | What It Tests |
|--------|-------|---------------|
| step_01_basic_tests | 6 | `add` and `divide` with #[test] and #[should_panic] |
| step_02_option_result_tests | 4 | `find_item` (Option) and `divide` (Result) |
| step_03_property_tests | 7 | `fibonacci` (edge cases) and `validate_email` |
| step_04_test_types | 2 | Rust→Python test framework equivalents |
| (top-level) | 2 | Panic on OOB, Result-returning test |

## How to Run Tests
```bash
cargo test
```
