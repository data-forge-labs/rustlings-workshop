# Workshop: Intro — Rust Syntax Primer

**Goal**: Implement all functions in `src/lib.rs` to pass all **21** tests.

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

## Test Modules

| Module | Tests | What It Tests |
|--------|-------|---------------|
| step_01_functions | 8 | Function syntax, return values, expressions |
| step_02_variables | 4 | Variables, slices, edge cases (empty) |
| step_03_control_flow | 5 | if/else as expression, branching |
| step_04_guess_game | 4 | Boolean return values, equality |

## How to Run Tests
```bash
cargo test
```
