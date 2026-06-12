# Workshop: Rayon Challenge

**Goal**: Implement all functions in `src/lib.rs` to pass all 11 tests.

## Functions to Implement

### `parallel_sum`
- **Signature**: `pub fn parallel_sum(data: Vec<i32>) -> i32`
- **Task**: Sum all elements using `rayon::par_iter()`.
- **Tests**: test_parallel_sum_basic, test_parallel_sum_empty

### `parallel_increment`
- **Signature**: `pub fn parallel_increment(data: Vec<i32>) -> Vec<i32>`
- **Task**: Increment each element by 1 using `par_iter().map()`.
- **Tests**: test_parallel_increment_basic, test_parallel_increment_empty

### `parallel_filter`
- **Signature**: `pub fn parallel_filter(data: Vec<i32>, threshold: i32) -> Vec<i32>`
- **Task**: Return elements greater than `threshold` using `par_iter().filter()`.
- **Tests**: test_parallel_filter_some, test_parallel_filter_all, test_parallel_filter_none

### `cpu_count`
- **Signature**: `pub fn cpu_count() -> usize`
- **Task**: Return the number of available CPU cores.
- **Tests**: test_cpu_count_positive

### `parallel_frequency`
- **Signature**: `pub fn parallel_frequency<'a>(text: Vec<&'a str>) -> HashMap<&'a str, usize>`
- **Task**: Count word frequencies in parallel using `par_iter().fold()`.
- **Tests**: test_parallel_frequency_basic, test_parallel_frequency_empty

### `compute_speedup`
- **Signature**: `pub fn compute_speedup(data_size: usize) -> f64`
- **Task**: Return a theoretical speedup that increases with data size (>= 1.0).
- **Tests**: test_compute_speedup_increases_with_data_size, test_compute_speedup_minimum

## Test Modules

| Module | Tests | What It Tests |
|--------|-------|---------------|
| step_01_par_iter | 4 | par_iter sum and map-increment |
| step_02_par_filter | 3 | par_iter filter with threshold |
| step_03_parallel_workload | 4 | CPU count, frequency, speedup |

## How to Run Tests
```bash
cargo test
```

## Exercises

* **Easy** – modify the existing function to handle an extra edge case.
* **Medium** – extend the project with a new helper function that re‑uses the core logic.

