# Workshop: CLI Salad

**Goal**: Implement all functions in `src/lib.rs` to pass all 10 tests.

## Functions to Implement

### `list_fruits`
- **Signature**: `pub fn list_fruits() -> Vec<String>`
- **Task**: Return the hard-coded list of 10 fruit names. Already implemented.
- **Tests**: test_contains_expected_fruits, test_list_length, test_consistent_order

### `create_fruit_salad`
- **Signature**: `pub fn create_fruit_salad(num_fruits: usize) -> Vec<String>`
- **Task**: Shuffle the fruit list and return `num_fruits` randomly selected fruits (max 10).
- **Tests**: test_returns_correct_count, test_returns_subset_of_fruits, test_handles_zero, test_handles_overflow

### `fruit_salad_cli`
- **Signature**: `pub fn fruit_salad_cli(args: Vec<String>) -> Result<String, String>`
- **Task**: Parse CLI args like `["program", "5"]`, return formatted salad or error.
- **Tests**: test_cli_valid_number, test_cli_invalid_input, test_cli_missing_arg

## Test Modules

| Module | Tests | What It Tests |
|--------|-------|---------------|
| step_01_fruit_list | 3 | Static fruit list content and order |
| step_02_fruit_salad | 4 | Random salad creation edge cases |
| step_03_cli | 3 | CLI argument parsing and error handling |

## How to Run Tests
```bash
cargo test
```
