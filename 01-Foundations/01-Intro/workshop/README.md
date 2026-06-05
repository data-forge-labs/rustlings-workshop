# Workshop: Intro — Rust Syntax Primer

**Goal**: Implement all functions in `src/lib.rs` to pass all **26** tests.

This is the **gentle first project** in the course. The topics are intentionally limited to the basics — variables, functions, control flow, tuples, and fixed-size arrays — each mapped to its Python equivalent. Advanced topics (ownership, borrowing, slices, error handling, collections) are covered in subsequent projects.

## Functions to Implement

### `celsius_to_fahrenheit`
- **Signature**: `pub fn celsius_to_fahrenheit(c: f64) -> f64`
- **Task**: Convert Celsius to Fahrenheit using formula `c * 9.0 / 5.0 + 32.0`
- **Tests**: test_celsius_to_fahrenheit_freezing, test_celsius_to_fahrenheit_boiling, test_celsius_to_fahrenheit_negative
- **README §5**

### `fahrenheit_to_celsius`
- **Signature**: `pub fn fahrenheit_to_celsius(f: f64) -> f64`
- **Task**: Convert Fahrenheit to Celsius using formula `(f - 32.0) * 5.0 / 9.0`
- **Tests**: test_fahrenheit_to_celsius_freezing, test_fahrenheit_to_celsius_boiling
- **README §5**

### `classify_temp`
- **Signature**: `pub fn classify_temp(temp: i32) -> &'static str`
- **Task**: Return `"cold"` if temp < 10, `"mild"` if 10..30, `"hot"` if >= 30
- **Tests**: test_classify_cold, test_classify_threshold_cold, test_classify_mild, test_classify_threshold_hot, test_classify_hot
- **README §7**

### `count_positive`
- **Signature**: `pub fn count_positive(values: [i32; 5]) -> usize`
- **Task**: Count how many of the 5 values are strictly positive (> 0)
- **Tests**: test_count_positive_mixed, test_count_positive_all, test_count_positive_none
- **README §8**

### `sum_five`
- **Signature**: `pub fn sum_five(values: [i32; 5]) -> i32`
- **Task**: Sum all 5 values
- **Tests**: test_sum_five_basic, test_sum_five_with_negatives, test_sum_five_all_zeros
- **README §8 (exercise)**

### `categorize_row`
- **Signature**: `pub fn categorize_row(row: (u32, f64, bool)) -> &'static str`
- **Task**: Return `"ok"` if `is_valid && value > 0`, `"invalid"` if `!is_valid`, `"zero"` otherwise
- **Tests**: test_categorize_row_ok, test_categorize_row_zero, test_categorize_row_invalid, test_categorize_row_negative_value
- **README §9**

### `max_of_five`
- **Signature**: `pub fn max_of_five(values: [i32; 5]) -> i32`
- **Task**: Return the largest of the 5 values
- **Tests**: test_max_of_five_positive, test_max_of_five_negative, test_max_of_five_mixed
- **README §10**

### `hot_readings_summary`
- **Signature**: `pub fn hot_readings_summary(readings: [i32; 5]) -> (usize, &'static str)`
- **Task**: Count how many readings are `>= 30`; return `(count, label)` where label is `"few"` (0), `"some"` (1..=2), or `"many"` (3+)
- **Tests**: test_hot_readings_few, test_hot_readings_some, test_hot_readings_many
- **README §11**

## Test Modules

| Module | Tests | What It Tests |
|--------|-------|---------------|
| step_01_functions | 5 | Function syntax, return values, type annotations |
| step_02_if_else | 5 | if/else as expression, branching |
| step_03_loops | 6 | `for` over arrays, `if` inside loops, sums |
| step_04_tuples | 4 | Tuple destructuring, tuple parameter |
| step_05_arrays | 3 | Fixed-size arrays, indexing, ranges |
| step_06_combined | 3 | Combining arrays, loops, if, and tuples |

## How to Run Tests

```bash
cargo test
```

## What's NOT in This Workshop

These topics are covered in later projects — don't worry about them yet:

- `&[T]` slices and borrowing → [02-Ownership](../../02-Ownership/README.md)
- `String` vs `&str`, `std::io`, `read_line`, `.expect()`, external crates (`rand`), custom `enum` → [02-GuessGame](../02-GuessGame/README.md)
- `panic!`, integer overflow, `as` casting, `#[should_panic]` → [03-BasicCalculator](../03-BasicCalculator/README.md)
- `Option<T>`, `Vec<T>`, `HashMap`, structs and `impl`, exhaustive `match` → [04-MasterMind](../04-MasterMind/README.md)
- Ownership deep dive, `Result<T, E>`, the `?` operator → [02-Ownership](../../02-Ownership/README.md)
- Full collection tour (`Vec`, `HashMap`, `HashSet`, etc.) → [03-Collections](../../03-Collections/README.md)
