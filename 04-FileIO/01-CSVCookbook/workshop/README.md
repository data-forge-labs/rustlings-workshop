# Workshop: CSV Cookbook

**Goal**: Implement all functions in `src/lib.rs` to pass all 7 tests.

## Functions to Implement

### `parse_csv_line`
- **Signature**: `pub fn parse_csv_line(line: &str) -> Vec<String>`
- **Task**: Split a CSV line into fields separated by commas, returning each as a `String`.
- **Tests**: test_parse_simple_line, test_parse_empty_line, test_parse_single_field

### `format_csv_line`
- **Signature**: `pub fn format_csv_line(fields: &[&str]) -> String`
- **Task**: Join a slice of string fields into a single comma-separated CSV line.
- **Tests**: test_format_csv_line, test_format_single_field

### `count_lines`
- **Signature**: `pub fn count_lines(csv_content: &str) -> usize`
- **Task**: Count the number of lines in a CSV content string.
- **Tests**: test_count_lines

### `column_values`
- **Signature**: `pub fn column_values(csv_content: &str, col_index: usize) -> Vec<String>`
- **Task**: Extract all values from a given column index across all rows.
- **Tests**: test_column_values

## Test Modules

| Module | Tests | What It Tests |
|--------|-------|---------------|
| step_01_parse | 3 | CSV line parsing edge cases |
| step_02_format | 2 | CSV line formatting |
| step_03_content | 2 | Line counting and column extraction |

## How to Run Tests
```bash
cargo test
```

## Exercises

* **Easy** – modify the existing function to handle an extra edge case.
* **Medium** – extend the project with a new helper function that re‑uses the core logic.

