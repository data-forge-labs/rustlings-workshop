# Workshop: BasicCalculator — Integers, Overflow, and Slice Ops

**Goal**: Implement all functions in `src/lib.rs` to pass all **34** tests.

## Functions to Implement

### `add`, `subtract`, `multiply`
- **Signatures**: `pub fn add/subtract/multiply(a: i32, b: i32) -> i32`
- **Task**: Basic arithmetic operations
- **Tests**: test_add, test_add_negative, test_add_zero, test_subtract, test_subtract_negative_result, test_multiply, test_multiply_zero, test_multiply_negative

### `divide`
- **Signature**: `pub fn divide(a: i32, b: i32) -> i32`
- **Task**: Integer division; **panic** with `"Cannot divide by zero"` if `b == 0`
- **Tests**: test_divide, test_divide_truncates, test_divide_negative, test_divide_by_zero

### `factorial_safe`
- **Signature**: `pub fn factorial_safe(n: u32) -> u32`
- **Task**: Compute `n!` using saturating multiplication; clamp to `u32::MAX` on overflow
- **Tests**: test_factorial_safe_0, test_factorial_safe_1, test_factorial_safe_5, test_factorial_safe_10, test_factorial_safe_overflow_clamps

### `factorial_wrapping`
- **Signature**: `pub fn factorial_wrapping(n: u32) -> u32`
- **Task**: Compute `n!` using wrapping multiplication; wrap on overflow
- **Tests**: test_factorial_wrapping_5, test_factorial_wrapping_wraps

### `average`
- **Signature**: `pub fn average(a: u32, b: u32) -> u32`
- **Task**: Return `(a + b) / 2` using saturating addition to avoid overflow
- **Tests**: test_average_basic, test_average_rounds_down, test_average_overflow_safe

### `total_rows`
- **Signature**: `pub fn total_rows(counts: &[u64]) -> u64`
- **Task**: Sum a slice of `u64` using saturating addition
- **Tests**: test_total_rows_empty, test_total_rows_basic, test_total_rows_saturating

### `min_temp`, `max_temp`, `avg_temp`
- **Signatures**: `pub fn min/max_temp(temps: &[i32]) -> i32`, `pub fn avg_temp(temps: &[i32]) -> f64`
- **Task**: Return min, max, and mean of a temperature slice
- **Tests**: test_min_temp, test_min_temp_negative, test_max_temp, test_max_temp_negative, test_avg_temp, test_avg_temp_single

### `would_overflow`
- **Signature**: `pub fn would_overflow(a: u32, b: u32) -> bool`
- **Task**: Return `true` if `a + b` would overflow `u32`
- **Tests**: test_no_overflow, test_overflow_at_max, test_overflow_exact_boundary, test_overflow_large_values

## Test Modules

| Module | Tests | What It Tests |
|--------|-------|---------------|
| step_01_arithmetic | 8 | Integer types, add/subtract/multiply |
| step_02_division | 4 | Integer division, panic on zero |
| step_03_factorial | 6 | Loops, saturating vs wrapping arithmetic |
| step_04_saturating_utils | 6 | Saturating add/sum, overflow safety |
| step_05_temp_stats | 6 | Slice iteration, min/max/mean |
| step_06_overflow_detector | 4 | Overflow detection logic |

## How to Run Tests
```bash
cargo test
```

## Exercises

* **Easy** – modify the existing function to handle an extra edge case.
* **Medium** – extend the project with a new helper function that re‑uses the core logic.

