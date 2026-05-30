# Workshop: Parquet

**Goal**: Implement all functions in `src/lib.rs` to pass all 4 tests.

## Functions to Implement

### `filter_by_threshold`
- **Signature**: `pub fn filter_by_threshold(records: &[Record], threshold: f64) -> Vec<&Record>`
- **Task**: Return references to records whose `value` field is >= `threshold`.
- **Tests**: test_filter_by_threshold

### `total_value`
- **Signature**: `pub fn total_value(records: &[Record]) -> f64`
- **Task**: Compute the sum of `value * count` for all records.
- **Tests**: test_total_value, test_total_value_empty

### `record_summary`
- **Signature**: `pub fn record_summary(record: &Record) -> String`
- **Task**: Return a formatted string containing the record's name and value.
- **Tests**: test_record_summary

## Structs

### `Record`
- Fields: `name: String`, `value: f64`, `count: u32`

## Test Modules

| Module | Tests | What It Tests |
|--------|-------|---------------|
| step_01_records | 4 | Total value, filtering, and record summary |

## How to Run Tests
```bash
cargo test
```
