# Workshop: Custom CLI Fruit Salad

**Goal**: Implement all functions in `src/lib.rs` to pass all 10 tests.

## Functions to Implement

### `create_fruit_salad`
- **Signature**: `pub fn create_fruit_salad(mut fruits: Vec<String>) -> Vec<String>`
- **Task**: Shuffle the fruits vector in-place and return it.
- **Tests**: test_create_fruit_salad_returns_correct_count, test_create_fruit_salad_contains_all_fruits, test_create_fruit_salad_empty

### `csv_to_vec`
- **Signature**: `pub fn csv_to_vec(csv: &str) -> Vec<String>`
- **Task**: Split a comma-separated string into trimmed tokens. Already implemented.
- **Tests**: test_csv_to_vec_basic, test_csv_to_vec_empty, test_csv_to_vec_single_item, test_csv_to_vec_whitespace

### `display_fruit_salad`
- **Signature**: `pub fn display_fruit_salad(fruits: &[String]) -> String`
- **Task**: Format fruits as a multi-line display string. Already implemented.
- **Tests**: test_display_fruit_salad_multiple, test_display_fruit_salad_single, test_display_fruit_salad_empty

## Test Modules

| Module | Tests | What It Tests |
|--------|-------|---------------|
| step_01_csv_parsing | 4 | CSV string parsing with trimming |
| step_02_fruit_salad | 3 | Shuffle-based salad creation |
| step_03_display | 3 | Display formatting |

## How to Run Tests
```bash
cargo test
```
