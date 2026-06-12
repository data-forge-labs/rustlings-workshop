# Workshop: RustJupyterNotebook

**Goal**: Study the provided implementations and implement `list_interactive_crates` and `rust_notebook_use_cases` to pass all 21 tests.

## Functions to Implement

### `Matrix::new`
- **Signature**: `pub fn new(values: Vec<T>, row_size: usize) -> Self`
- **Task**: Create a matrix; panics if row_size == 0 or values length not divisible by row_size. _(already implemented)_

### `Matrix::row`, `Matrix::get`, `Matrix::to_html`
- **Task**: Access rows, elements, and render as HTML table. _(already implemented)_

### `SimpleDataFrame::new`, `SimpleDataFrame::to_html`
- **Task**: Columnar data structure with HTML rendering. _(already implemented)_

### `range_f64`
- **Signature**: `pub fn range_f64(start: f64, end: f64, step: f64) -> Vec<f64>`
- **Task**: Generate a range like Python's `range()` for floats. _(already implemented)_

### `list_interactive_crates`
- **Signature**: `pub fn list_interactive_crates() -> Vec<&'static str>`
- **Task**: Return crates useful with evcxr_jupyter (e.g., "serde", "rayon", "plotters").
- **Tests**: test_list_interactive_crates

### `rust_notebook_use_cases`
- **Signature**: `pub fn rust_notebook_use_cases() -> Vec<&'static str>`
- **Task**: Return use cases for Rust Jupyter notebooks (e.g., "Educational tool").
- **Tests**: test_rust_notebook_use_cases

## Test Modules

| Module | Tests | What It Tests |
|--------|-------|---------------|
| step_01_matrix | 10 | Matrix creation, access, bounds checking |
| step_02_dataframe | 3 | SimpleDataFrame creation and validation |
| step_03_html_display | 6 | HTML table rendering and range_f64 |
| step_04_concepts | 2 | Knowledge of Jupyter ecosystem crates |

## How to Run Tests
```bash
cargo test
```

## Exercises

* **Easy** – modify the existing function to handle an extra edge case.
* **Medium** – extend the project with a new helper function that re‑uses the core logic.

