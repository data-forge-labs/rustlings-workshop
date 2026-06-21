# Project 58: Interactive Rust with evcxr Jupyter kernel — plotting and arrays

> **Test-driven approach**: This project includes a Cargo project with progressive
> unit tests. Each function in `workshop/src/lib.rs` starts as a `todo!()` stub. As you
> follow each section, replace `todo!()` with real code and run `cd workshop && cargo test` to
> watch the pass count grow. Your goal: **all 21 tests pass**.

## What Is This Project?

Rust in Jupyter via `evcxr` — interactive Rust cells with rich HTML output.

### Python equivalent

```python
# Jupyter: Shift+Enter, see DataFrame as HTML, iterate fast
import pandas as pd
df = pd.DataFrame({"city": ["Lisbon", "Porto"], "pop": [504718, 249633]})
df  # rendered as HTML table in Jupyter
```

```rust
:dep rust_jupyter_notebook = { path = "." }
use rust_jupyter_notebook::*;
let df = SimpleDataFrame::new(
    vec!["city".to_string(), "pop".to_string()],
    vec![
        vec!["Lisbon".to_string(), "504718".to_string()],
        vec!["Porto".to_string(), "249633".to_string()],
    ],
);
":html " + &df.to_html()    // rich HTML table in the cell
```

This project builds the Rust-side toolkit — `Matrix<T>`, `SimpleDataFrame`, `range_f64` — so you get Rust's performance with Jupyter's iterative workflow.

---

### Topics covered

| # | Concept | Why it matters |
|---|---------|----------------|
| 1 | Generic struct `Matrix<T>` | Type-safe 2D matrix |
| 2 | Generic trait bounds | Restrict generic params at compile time |
| 3 | Safe indexing with `Option` | Bounds-checked element access |
| 4 | HTML rendering | Rich table display in Jupyter |
| 5 | `SimpleDataFrame` | Columnar data, row-oriented storage |
| 6 | `evcxr` kernel | Interactive Rust in notebook cells |

---

---

## Table of Contents

1. [Introduction](#1-introduction)
2. [Prerequisites](#2-prerequisites)
3. [Concept: Generic Matrix Struct](#3-concept-generic-matrix-struct)
4. [Concept: HTML Display for Jupyter](#4-concept-html-display-for-jupyter)
5. [Concept: SimpleDataFrame](#5-concept-simpledataframe)
6. [Concept: range_f64 — Rust's range for floats](#6-concept-range_f64--rusts-range-for-floats)
7. [Putting It All Together](#7-putting-it-all-together)
8. [Complete Code Reference](#8-complete-code-reference)
9. [Summary](#9-summary)

## 1. Introduction

Jupyter notebooks are the standard interactive environment for Python data
work. With the `evcxr` Jupyter kernel, you can run Rust code in the same
notebook interface — combining Rust's performance with Jupyter's
prose+code+output workflow.

This workshop builds the Rust-side foundation for interactive data exploration
in Jupyter: a generic `Matrix<T>`, a `SimpleDataFrame` with HTML rendering, and
a `range_f64` helper — all displayable as rich HTML tables directly in notebook
cells.

**Python -> Rust**: In Python, `pandas.DataFrame` has a built-in `_repr_html_()`
method that Jupyter auto-renders as a styled table. In Rust with `evcxr`, you
can achieve the same by calling `.to_html()` and using `evcxr`'s `:html`
directive. No framework — you build the HTML string yourself.

## 2. Prerequisites

- [02-Ownership/01-TicketV1](../../../../02-Ownership/01-TicketV1/README.md) --
  structs, ownership, generics
- [02-Ownership/03-TicketV2](../../../../02-Ownership/03-TicketV2/README.md) --
  Result, Option, pattern matching
- [03-Collections/12-RustIterators](../../../../03-Collections/12-RustIterators/README.md) --
  iterators
- Installed: Rust toolchain, `cargo`
- Optional: `evcxr_jupyter` kernel (`cargo install evcxr_jupyter`)

## 3. Concept: Generic Matrix Struct

### Explanation

In Python, a 2D array is usually a NumPy `ndarray` or a list of lists:

```python
import numpy as np
m = np.array([[1, 2], [3, 4]])
m.shape  # (2, 2)
```

In Rust, we define a generic `Matrix<T>` that stores values in a flat `Vec<T>`
and interprets them as a row-major grid:

```rust
pub struct Matrix<T> {
    pub values: Vec<T>,
    pub row_size: usize,
}
```

```
Memory layout (4 elements, row_size=2):

 values: [1, 2, 3, 4]
          └─row 0─┘  └─row 1─┘

 Logical view:
   ┌───┬───┐
   │ 1 │ 2 │  ← row 0  (indices 0..2)
   ├───┼───┤
   │ 3 │ 4 │  ← row 1  (indices 2..4)
   └───┴───┘
```

Row access is O(1) — a slice into the flat Vec:

```rust
pub fn row(&self, index: usize) -> &[T] {
    let start = index * self.row_size;
    &self.values[start..start + self.row_size]
}
```

**Python comparison**: NumPy's `ndarray` is a C-optimised n-dimensional array.
Rust's `Matrix<T>` is a simpler 2D wrapper around a flat `Vec`. Both store data
contiguously in memory. The key difference: NumPy arrays are typed by element
(dtype), while Rust's `Matrix<T>` is generic over any type `T`.

### Applying to Our Project

The `Matrix::new()` constructor validates inputs:

- `row_size` must be > 0
- `values.len()` must be divisible by `row_size`

```rust
let m = Matrix::new(vec![1, 2, 3, 4], 2);
assert_eq!(m.get(0, 0), Some(&1));
assert_eq!(m.get(1, 1), Some(&4));
```

## 4. Concept: HTML Display for Jupyter

### Explanation

In a Python Jupyter notebook, evaluating a `pandas.DataFrame` at the end of a
cell automatically renders an HTML table:

```python
import pandas as pd
df = pd.DataFrame({"name": ["Alice"], "age": [30]})
df  # rendered as styled HTML table
```

In Rust with `evcxr_jupyter`, you can output HTML by using the `:html` directive
or by calling `evcxr::display()`. Our `Matrix::to_html()` method generates a
raw HTML `<table>` string:

```rust
pub fn to_html(&self) -> String {
    let mut html = String::from("<table>\n");
    for r in 0..self.num_rows() {
        html.push_str("  <tr>\n");
        for c in 0..self.num_cols() {
            if let Some(val) = self.get(r, c) {
                html.push_str(&format!("    <td>{:?}</td>\n", val));
            }
        }
        html.push_str("  </tr>\n");
    }
    html.push_str("</table>");
    html
}
```

```html
<!-- Generated HTML for Matrix<f64>:
<table>
  <tr>
    <td>1.0</td>
    <td>2.0</td>
  </tr>
  <tr>
    <td>3.0</td>
    <td>4.0</td>
  </tr>
</table>
-->

In Jupyter with evcxr (in a notebook cell):
:dep plotters
// ... code ...
let m = Matrix::new(vec![1.0, 2.0, 3.0, 4.0], 2);
":html " + &m.to_html()
```

**Python comparison**: Python DataFrames auto-display in notebooks via
`_repr_html_()`. Rust requires an explicit `.to_html()` call and an `:html`
directive — more manual but gives full control over the output format.

### Applying to Our Project

Both `Matrix<T>` and `SimpleDataFrame` implement `to_html()`. You can test the
HTML output directly:

```rust
let m = Matrix::new(vec![1, 2, 3, 4], 2);
let html = m.to_html();
assert!(html.contains("<table>"));
assert!(html.contains("<td>2</td>"));
```

## 5. Concept: SimpleDataFrame

### Explanation

In Python:
```python
import pandas as pd
df = pd.DataFrame({"name": ["Alice", "Bob"], "age": [30, 25]})
```

In Rust, we build a simpler version: column names stored separately from row
data (`Vec<Vec<String>>`):

```rust
pub struct SimpleDataFrame {
    pub columns: Vec<String>,
    pub data: Vec<Vec<String>>,
}
```

```
┌──────────┬─────┐
│ name     │ age │  ← columns: Vec<String>
├──────────┼─────┤
│ "Alice"  │ 30  │  ← data: Vec<Vec<String>>
│ "Bob"    │ 25  │
└──────────┴─────┘
```

The constructor validates that every row has the same number of columns as the
header:

```rust
let df = SimpleDataFrame::new(
    vec!["name".to_string(), "age".to_string()],
    vec![
        vec!["Alice".to_string(), "30".to_string()],
        vec!["Bob".to_string(), "25".to_string()],
    ],
);
```

Its `to_html()` generates a table with `<th>` header cells:

```rust
for col in &self.columns {
    html.push_str(&format!("    <th>{}</th>\n", col));
}
for row in &self.data {
    html.push_str("  <tr>\n");
    for val in row {
        html.push_str(&format!("    <td>{}</td>\n", val));
    }
}
```

**Python comparison**: pandas DataFrame stores column-oriented data internally
(columnar arrays). Our `SimpleDataFrame` stores row-oriented data
(`Vec<Vec<String>>`), matching how CSV data is typically parsed. All values are
strings — no type inference. This is deliberate: it maps to how raw CSV or
JSON data arrives before type conversion.

### Applying to Our Project

Use `SimpleDataFrame` to display tabular data in a Jupyter notebook. The
`to_html()` method produces a rendered table that `evcxr` can display via the
`:html` directive.

## 6. Concept: range_f64 — Rust's range for floats

### Explanation

In Python:
```python
list(range(0, 5))         # [0, 1, 2, 3, 4]
import numpy as np
np.arange(0.0, 3.0, 0.5)  # [0.0, 0.5, 1.0, 2.0, 2.5]
```

Rust's built-in range syntax `0..5` only works for integers. For floating-point
ranges, we implement our own:

```rust
pub fn range_f64(start: f64, end: f64, step: f64) -> Vec<f64> {
    if step == 0.0 {
        panic!("step must be non-zero");
    }
    let mut result = Vec::new();
    let mut current = start;
    if step > 0.0 {
        while current < end {
            result.push(current);
            current += step;
        }
    } else {
        while current > end {
            result.push(current);
            current += step;
        }
    }
    result
}
```

```
range_f64(0.0, 3.0, 1.0)  →  [0.0, 1.0, 2.0]
range_f64(3.0, 0.0, -1.0) →  [3.0, 2.0, 1.0]
range_f64(0.0, 0.0, 1.0)  →  []              (empty)
```

**Python comparison**: `np.arange()` uses a C loop internally and handles edge
cases like floating-point rounding. Our Rust version is a straightforward
while-loop — no dependencies required. It panics on `step == 0` (like
Python's `ZeroDivisionError`).

### Applying to Our Project

`range_f64` is useful for generating x-axis values for plots in Jupyter
notebooks (e.g., feeding into `plotters` for charting).

## 7. Putting It All Together

Open `workshop/src/lib.rs`. You'll find these items with `todo!()`:

1. **`Matrix<T>` struct** — generic 2D matrix with `new()`, `num_rows()`,
   `num_cols()`, `row()`, `get()`, and `to_html()`
2. **`SimpleDataFrame` struct** — columnar data with `new()`, `num_rows()`,
   `num_cols()`, and `to_html()`
3. **`range_f64(start, end, step)`** — floating-point range generator
4. **`list_interactive_crates()`** — returns crate names useful in evcxr
5. **`rust_notebook_use_cases()`** — returns use-case descriptions

Implement them one at a time. After each step, run:

```bash
cd workshop && cargo test
```

Tests are grouped into four modules: `step_01_matrix`, `step_02_dataframe`,
`step_03_html_display`, and `step_04_concepts`.

Finally, run:

```bash
cd workshop && cargo run
```

You'll see the HTML output printed as text (raw HTML strings) plus a list
of recommended crates and use cases.

To see the real interactive experience, install `evcxr_jupyter`:

```bash
cargo install evcxr_jupyter
jupyter notebook
```

Then in a notebook cell:

```
:dep rust_jupyter_notebook = { path = "." }
use rust_jupyter_notebook::*;
let df = SimpleDataFrame::new(
    vec!["city".to_string(), "pop".to_string()],
    vec![
        vec!["Lisbon".to_string(), "504718".to_string()],
        vec!["Porto".to_string(), "249633".to_string()],
    ],
);
":html " + &df.to_html()
```

## 8. Complete Code Reference

```rust
/// A generic matrix that can display as HTML in Jupyter
pub struct Matrix<T> {
    pub values: Vec<T>,
    pub row_size: usize,
}

impl<T: std::fmt::Debug> Matrix<T> {
    pub fn new(values: Vec<T>, row_size: usize) -> Self {
        if row_size == 0 {
            panic!("row_size must be greater than 0");
        }
        if values.len() % row_size != 0 {
            panic!("values length must be divisible by row_size");
        }
        Matrix { values, row_size }
    }

    pub fn num_rows(&self) -> usize {
        self.values.len() / self.row_size
    }

    pub fn num_cols(&self) -> usize {
        self.row_size
    }

    pub fn row(&self, index: usize) -> &[T] {
        if index >= self.num_rows() {
            panic!("row index out of bounds");
        }
        let start = index * self.row_size;
        &self.values[start..start + self.row_size]
    }

    pub fn get(&self, row: usize, col: usize) -> Option<&T> {
        if row >= self.num_rows() || col >= self.num_cols() {
            return None;
        }
        Some(&self.values[row * self.row_size + col])
    }

    pub fn to_html(&self) -> String {
        let mut html = String::from("<table>\n");
        for r in 0..self.num_rows() {
            html.push_str("  <tr>\n");
            for c in 0..self.num_cols() {
                if let Some(val) = self.get(r, c) {
                    html.push_str(&format!("    <td>{:?}</td>\n", val));
                }
            }
            html.push_str("  </tr>\n");
        }
        html.push_str("</table>");
        html
    }
}

pub struct SimpleDataFrame {
    pub columns: Vec<String>,
    pub data: Vec<Vec<String>>,
}

impl SimpleDataFrame {
    pub fn new(columns: Vec<String>, data: Vec<Vec<String>>) -> Self {
        let num_cols = columns.len();
        for (i, row) in data.iter().enumerate() {
            if row.len() != num_cols {
                panic!(
                    "row {} has {} columns, expected {}",
                    i, row.len(), num_cols
                );
            }
        }
        SimpleDataFrame { columns, data }
    }

    pub fn num_rows(&self) -> usize {
        self.data.len()
    }

    pub fn num_cols(&self) -> usize {
        self.columns.len()
    }

    pub fn to_html(&self) -> String {
        let mut html = String::from("<table>\n  <tr>\n");
        for col in &self.columns {
            html.push_str(&format!("    <th>{}</th>\n", col));
        }
        html.push_str("  </tr>\n");
        for row in &self.data {
            html.push_str("  <tr>\n");
            for val in row {
                html.push_str(&format!("    <td>{}</td>\n", val));
            }
            html.push_str("  </tr>\n");
        }
        html.push_str("</table>");
        html
    }
}

pub fn range_f64(start: f64, end: f64, step: f64) -> Vec<f64> {
    if step == 0.0 {
        panic!("step must be non-zero");
    }
    let mut result = Vec::new();
    let mut current = start;
    if step > 0.0 {
        while current < end {
            result.push(current);
            current += step;
        }
    } else {
        while current > end {
            result.push(current);
            current += step;
        }
    }
    result
}

pub fn list_interactive_crates() -> Vec<&'static str> {
    vec![
        "plotters",
        "itertools",
        "serde",
        "polars",
        "regex",
        "rayon",
        "ndarray",
    ]
}

pub fn rust_notebook_use_cases() -> Vec<&'static str> {
    vec![
        "Educational tool for teaching Rust",
        "Data processing and analysis",
        "Algorithm development and prototyping",
        "Interactive documentation",
        "Research and experimentation",
    ]
}
```

## 9. Summary

| Concept | Python Equivalent | Rust Implementation |
|---|---|---|
| Generic 2D matrix | `numpy.ndarray` | `Matrix<T>` with flat `Vec<T>` |
| HTML table rendering | `df._repr_html_()` | `to_html()` on Matrix and SimpleDataFrame |
| Columnar data | `pandas.DataFrame` | `SimpleDataFrame` with `columns` + `data` |
| Float range | `numpy.arange()` / `range()` | `range_f64(start, end, step)` |
| Interactive kernel | `ipykernel` | `evcxr_jupyter` |
| Recommended crates | numpy, pandas, matplotlib | serde, rayon, plotters, ndarray, polars |

**Exercises:**

1. (Easy) Add a `to_csv()` method to `SimpleDataFrame` that returns a
   comma-separated string (header + rows).
2. (Medium) Implement a `transpose()` method on `Matrix<T>` that returns a new
   Matrix with rows and columns swapped.
3. (Hard) Write an evcxr notebook cell that loads a CSV file using the `csv`
   crate, populates a `SimpleDataFrame`, and displays it as HTML.

## Exercises

* **Easy** – modify the existing function to handle an extra edge case.
* **Medium** – extend the project with a new helper function that re‑uses the core logic.

