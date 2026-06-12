# Workshop: Graph Visualize

**Goal**: Study the implemented `src/lib.rs` and pass all 15 tests.

## Functions to Study

### `generate_sample_data`
- **Signature**: `pub fn generate_sample_data() -> Vec<f64>`
- **Task**: Return a hard-coded vector of 9 sample f64 values. Already implemented.

### `ascii_bar_chart`
- **Signature**: `pub fn ascii_bar_chart(data: &[f64], labels: &[&str]) -> Vec<String>`
- **Task**: Format data as an ASCII bar chart with labels and values. Already implemented.

### `data_summary`
- **Signature**: `pub fn data_summary(data: &[f64]) -> (f64, f64, f64)`
- **Task**: Compute min, max, and mean of a data slice (returns NaN for empty). Already implemented.

### `normalize_data`
- **Signature**: `pub fn normalize_data(data: &[f64]) -> Vec<f64>`
- **Task**: Normalize data to 0..100 range using min-max scaling. Already implemented.

### `create_series`
- **Signature**: `pub fn create_series<'a>(names: &[&'a str], values: &[f64]) -> Vec<(&'a str, f64)>`
- **Task**: Zip names and values into a vector of pairs (panics on length mismatch). Already implemented.

## Test Modules

| Module | Tests | What It Tests |
|--------|-------|---------------|
| step_01_data_basics | 5 | Sample data generation and summary statistics |
| step_02_visualization | 6 | ASCII bar chart and normalization |
| step_03_series | 4 | Labeled data series creation |

## How to Run Tests
```bash
cargo test
```

## Exercises

* **Easy** – modify the existing function to handle an extra edge case.
* **Medium** – extend the project with a new helper function that re‑uses the core logic.

