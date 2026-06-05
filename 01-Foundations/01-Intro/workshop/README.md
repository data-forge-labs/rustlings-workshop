# Workshop: Intro — Rust Syntax Primer

**Goal**: Implement all functions in `src/lib.rs` to pass all **31** tests.

## Functions to Implement

### `celsius_to_fahrenheit`
- **Signature**: `pub fn celsius_to_fahrenheit(c: f64) -> f64`
- **Task**: Convert Celsius to Fahrenheit using formula `c * 9.0 / 5.0 + 32.0`
- **Tests**: test_celsius_to_fahrenheit_freezing, test_celsius_to_fahrenheit_boiling, test_celsius_to_fahrenheit_negative

### `fahrenheit_to_celsius`
- **Signature**: `pub fn fahrenheit_to_celsius(f: f64) -> f64`
- **Task**: Convert Fahrenheit to Celsius using formula `(f - 32.0) * 5.0 / 9.0`
- **Tests**: test_fahrenheit_to_celsius_freezing, test_fahrenheit_to_celsius_boiling

### `double`
- **Signature**: `pub fn double(x: i32) -> i32`
- **Task**: Return `x * 2`
- **Tests**: test_double_positive, test_double_negative, test_double_zero

### `mean`
- **Signature**: `pub fn mean(values: &[f64]) -> f64`
- **Task**: Return the arithmetic mean; return `f64::NAN` for empty slices
- **Tests**: test_mean_basic, test_mean_single, test_mean_zero_values, test_mean_empty_returns_nan

### `classify_temp`
- **Signature**: `pub fn classify_temp(temp: i32) -> &'static str`
- **Task**: Return `"cold"` if temp < 10, `"mild"` if 10..30, `"hot"` if >= 30
- **Tests**: test_classify_hot, test_classify_threshold_hot, test_classify_mild, test_classify_cold, test_classify_threshold_cold

### `play_guess_game`
- **Signature**: `pub fn play_guess_game(secret: u32, guess: u32) -> bool`
- **Task**: Return `true` if guess matches secret
- **Tests**: test_correct_guess_wins, test_incorrect_guess_loses, test_guess_zero, test_guess_boundary

### `categorize_row`
- **Signature**: `pub fn categorize_row(row: (u32, f64, bool)) -> &'static str`
- **Task**: Return `"ok"` if `is_valid && value > 0`, `"invalid"` if `!is_valid`, `"zero"` otherwise
- **Tests**: test_categorize_row_ok, test_categorize_row_zero, test_categorize_row_invalid, test_categorize_row_negative_value

### `summarize`
- **Signature**: `pub fn summarize(values: &[f64]) -> (f64, f64, usize)`
- **Task**: Return `(min, max, count)` of a slice of values
- **Tests**: test_summarize_basic, test_summarize_single

### `second_half`
- **Signature**: `pub fn second_half(data: &[f64]) -> &[f64]`
- **Task**: Return the slice from `len/2` to end
- **Tests**: test_second_half_odd_length, test_second_half_even_length, test_second_half_single_element, test_second_half_empty

## Test Modules

| Module | Tests | What It Tests |
|--------|-------|---------------|
| step_01_functions | 8 | Function syntax, return values, expressions |
| step_02_variables | 4 | Variables, slices, edge cases (empty) |
| step_03_control_flow | 5 | if/else as expression, branching |
| step_04_guess_game | 4 | Boolean return values, equality |
| step_05_tuples | 6 | Tuple destructuring, tuples as return values |
| step_06_slices | 4 | Slice ranges, `&[T]` return values, empty handling |

## How to Run Tests
```bash
cargo test
```
