# Workshop: ExploringPandas

**Goal**: Work with the provided `FruitRecord` struct and implement 6 functions (2 already partially complete) to pass all 17 tests.

## Functions to Implement

### `read_fruits`
- **Signature**: `pub fn read_fruits(bytes: &[u8]) -> Result<Vec<FruitRecord>, String>`
- **Task**: Parse CSV bytes into `FruitRecord` values (like `pd.read_csv`).
- **Tests**: read_basic_csv, read_empty_body, read_empty_input _(already implemented)_

### `mean_price_per_fruit`
- **Signature**: `pub fn mean_price_per_fruit(records: &[FruitRecord]) -> Vec<(String, f64)>`
- **Task**: Group by fruit name and compute mean price (like `df.groupby("fruit")["price"].mean()`).
- **Tests**: mean_price_single_fruit, mean_price_multiple_fruits, mean_price_empty _(already implemented)_

### `mean_price_per_year`
- **Signature**: `pub fn mean_price_per_year(records: &[FruitRecord]) -> Vec<(u32, f64)>`
- **Task**: Group by year and compute mean price, sorted by year.
- **Tests**: mean_price_per_year_multiple, mean_price_per_year_single _(already implemented)_

### `filter_by_price`
- **Signature**: `pub fn filter_by_price(records: &[FruitRecord], threshold: f64) -> Vec<FruitRecord>`
- **Task**: Return records where price > threshold (like `df[df["price"] > threshold]`).
- **Tests**: some_match, none_match, all_match, empty_records _(already implemented)_

### `write_fruits`
- **Signature**: `pub fn write_fruits(records: &[FruitRecord]) -> Result<String, String>`
- **Task**: Serialize records to CSV string (like `df.to_csv`).
- **Tests**: write_and_roundtrip, write_empty _(already implemented)_

### `summary_stats`
- **Signature**: `pub fn summary_stats(records: &[FruitRecord]) -> (f64, f64, f64, usize)`
- **Task**: Return (min, max, mean, count) of prices (like `df.describe()`).
- **Tests**: normal_stats, single_record, empty_records _(already implemented)_

## Test Modules

| Module | Tests | What It Tests |
|--------|-------|---------------|
| step_01_csv_io | 5 | CSV reading and writing |
| step_02_groupby | 5 | Group-by mean (fruit and year) |
| step_03_filtering | 4 | Filter records by price threshold |
| step_04_statistics | 3 | Summary statistics (min, max, mean, count) |

## How to Run Tests
```bash
cargo test
```

## Exercises

* **Easy** – modify the existing function to handle an extra edge case.
* **Medium** – extend the project with a new helper function that re‑uses the core logic.

