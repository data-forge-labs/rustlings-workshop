# Workshop: PyO3 Bindings — Calling Rust from Python

> **Test-driven approach**: This project includes a Cargo project with progressive
> unit tests. Each function in `src/lib.rs` starts as a `todo!()` stub. As you
> follow each section, replace `todo!()` with real code and run `cargo test` to
> watch the pass count grow. Your goal: **all 12 tests pass**.

> **Python build note**: To build the Python module, install `maturin`
> (`pip install maturin`) and run `maturin develop --release --features python`
> from this directory. The `cargo test` path runs without Python installed.

**Goal**: Implement all functions in `src/lib.rs` to pass all 12 tests.

## Functions to Implement

### Step 1 — Basic vectorized operations

#### `double_values`
- **Signature**: `pub fn double_values(values: &[f64]) -> Vec<f64>`
- **Task**: `values.iter().map(|x| x * 2.0).collect()`

#### `sum_values`
- **Signature**: `pub fn sum_values(values: &[f64]) -> f64`
- **Task**: `values.iter().sum()`

### Step 2 — Transforms

#### `normalize`
- **Signature**: `pub fn normalize(values: &[f64]) -> Vec<f64>`
- **Task**: Find min and max; map each value to `(v - min) / (max - min)`. If `max == min`, return zeros.

### Step 3 — Windowed

#### `moving_average`
- **Signature**: `pub fn moving_average(values: &[f64], window: usize) -> Vec<f64>`
- **Task**: For each index `i` in `0..=(values.len() - window)`, average `values[i..i+window]`. Returns `values.len() - window + 1` elements.

### Step 4 — Counting

#### `count_above_threshold`
- **Signature**: `pub fn count_above_threshold(values: &[f64], threshold: f64) -> usize`
- **Task**: `values.iter().filter(|&&v| v > threshold).count()`

### Step 5 — Misc

#### `hello_from_rust`
- **Signature**: `pub fn hello_from_rust(name: &str) -> String`
- **Task**: `format!("Hello, {} from Rust!", name)`

#### `reverse_in_place`
- **Signature**: `pub fn reverse_in_place(values: &mut [f64])`
- **Task**: `values.reverse()`

#### `parse_csv_line`
- **Signature**: `pub fn parse_csv_line(line: &str) -> Vec<f64>`
- **Task**: `line.split(',').filter_map(|s| s.trim().parse().ok()).collect()`

## Test Modules

| Module | Tests | What It Tests |
|--------|-------|---------------|
| step_01_basic | 4 | double_values, sum_values |
| step_02_transforms | 2 | normalize to [0,1] + constant input |
| step_03_windowed | 2 | moving_average with windows 2 and 3 |
| step_04_counting | 2 | count_above_threshold with hit and miss |
| step_05_misc | 3 | hello + reverse + parse_csv_line |

## Building the Python Module

```bash
pip install maturin numpy
cd 08-Interop/03-PyO3Bindings/workshop
maturin develop --release --features python
python -c "import pyo3_bindings_workshop; print(pyo3_bindings_workshop.hello_from_rust('Alice'))"
```

## How to Run Tests (Rust only, no Python needed)
```bash
cargo test
```
