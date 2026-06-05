# Project 22: ASCII Bar Charts -- Data Visualization in the Terminal

> **Test-driven approach**: This project includes a Cargo project with progressive
> unit tests. Each function in `workshop/src/lib.rs` starts as a `todo!()` stub. As you
> follow each section, replace `todo!()` with real code and run `cd workshop && cargo test` to
> watch the pass count grow. Your goal: **all 16 tests pass**.

## Why Render ASCII Bar Charts by Hand?

**Python pain:** On a remote box you can't open a GUI, and even terminal libraries like `plotext` add 50-100ms of import time per script. Installing `matplotlib` pulls in hundreds of dependencies — overkill for a bar chart.

**Rust fix:** Render ASCII bar charts with string repetition — no dependencies beyond `std`, instant startup, works over SSH:

```rust
pub fn ascii_bar_chart(data: &[f64], labels: &[&str]) -> Vec<String> {
    let max = data.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
    data.iter().zip(labels).map(|(&val, &label)| {
        let bar_len = ((val / max) * 40.0).round() as usize;
        format!("{:<10} | {} ({})", label, "█".repeat(bar_len), val)
    }).collect()
}
```

## At a Glance

| # | Concept | Rust | Python | Why it matters |
|---|---------|------|--------|----------------|
| 1 | Summary statistics | `fold(f64::INFINITY, f64::min)` | `min()`, `max()`, `sum()/len` | Min, max, mean of a data slice |
| 2 | Min-max normalization | `(v - min) / (max - min) * 100` | Same formula | Rescale data to 0-100 range |
| 3 | String repetition | `"█".repeat(n)` | `"█" * n` | Build ASCII bars of variable length |
| 4 | Labeled data series | `zip` into `Vec<(&str, f64)>` | `list(zip(names, values))` | Pair names with numeric values |
| 5 | Format alignment | `format!("{:<10}", label)` | `f"{label:<10}"` | Left-align labels in fixed-width column |

---

---

## Table of Contents

1. [Introduction](#1-introduction)
2. [Prerequisites](#2-prerequisites)
3. [Concept: Data Summary Statistics](#3-concept-data-summary-statistics)
4. [Concept: Min-Max Normalization](#4-concept-min-max-normalization)
5. [Concept: ASCII Bar Chart Rendering](#5-concept-ascii-bar-chart-rendering)
6. [Concept: Labeled Data Series](#6-concept-labeled-data-series)
7. [Putting It All Together](#7-putting-it-all-together)
8. [Summary](#8-summary)

## 1. Introduction

Terminal-based data visualization is essential for data engineers who work on remote servers without GUI access. In this project, you will build text-based bar charts that render directly in the terminal.

You will implement:
- Statistical summaries (min, max, mean)
- Min-max normalization to 0-100 scale
- ASCII bar chart rendering with block characters
- Labeled data series creation

In Python, you would use `matplotlib` or `plotext` for terminal plotting. In Rust, you build the rendering logic manually using iterators and string formatting.

## 2. Prerequisites

- `Vec<f64>` and slice operations
- Iterators and `zip`
- Basic formatting with `format!`

## 3. Concept: Data Summary Statistics

### Explanation

Computing min, max, and mean over a slice of f64 values. In Python:

```python
data = [3.0, 7.0, 2.0, 9.0, 5.0]
min_val = min(data)
max_val = max(data)
mean = sum(data) / len(data)
```

In Rust, you fold over iterators:

```rust
pub fn data_summary(data: &[f64]) -> (f64, f64, f64)
```

- `min`: `.fold(f64::INFINITY, f64::min)`
- `max`: `.fold(f64::NEG_INFINITY, f64::max)`
- `mean`: `.sum::<f64>() / data.len() as f64`

For empty data, return `(f64::NAN, f64::NAN, f64::NAN)`.

### Applying to Our Project

`generate_sample_data` returns a hardcoded `Vec<f64>` for testing. `data_summary` computes statistics on any data slice.

## 4. Concept: Min-Max Normalization

### Explanation

Normalization rescales data to a fixed range. Min-max normalization to 0-100:

```
normalized(v) = ((v - min) / (max - min)) * 100
```

In Python:

```python
def normalize(data):
    if not data:
        return []
    mn, mx = min(data), max(data)
    if mx == mn:
        return [50.0] * len(data)
    return [(v - mn) / (mx - mn) * 100 for v in data]
```

In Rust:

```rust
pub fn normalize_data(data: &[f64]) -> Vec<f64>
```

Key edge cases:
- Empty data returns empty `Vec`
- Constant data (max == min) returns all 50.0

### Applying to Our Project

Normalization ensures data fits the 40-character bar width regardless of scale.

## 5. Concept: ASCII Bar Chart Rendering

### Explanation

An ASCII bar chart displays values as horizontal bars of block characters. In Python with plotext:

```python
import plotext as plt
plt.bar(["A", "B", "C"], [5, 10, 2])
plt.show()
```

In Rust, you build the chart with string repetition:

```rust
pub fn ascii_bar_chart(data: &[f64], labels: &[&str]) -> Vec<String>
```

For each `(label, value)` pair:
1. Compute bar length: `((value / max) * bar_width).round() as usize`
2. Repeat the block character `"█"` that many times
3. Format: `"{label:<10} | {bar} ({value})"`

If max is 0, return just the label column with no bars. If data is empty, return an empty Vec.

### Applying to Our Project

The function produces output like:

```
A          | ████████████████████████ (5)
B          | ████████████████████████████████████████████████████ (10)
C          | ██████████ (2)
```

## 6. Concept: Labeled Data Series

### Explanation

A data series pairs names with numeric values. In Python:

```python
names = ["Alice", "Bob", "Carol"]
values = [3.0, 7.0, 2.0]
series = list(zip(names, values))
```

In Rust:

```rust
pub fn create_series<'a>(names: &[&'a str], values: &[f64]) -> Vec<(&'a str, f64)>
```

This uses `zip` to combine two slices into a `Vec` of tuples. If lengths differ, the function panics with a clear message.

### Applying to Our Project

`create_series` feeds data into `ascii_bar_chart`, supplying both labels and values.

## 7. Putting It All Together

Open `workshop/src/lib.rs` and implement:

1. **`generate_sample_data`** -- return `vec![3.0, 7.0, 2.0, 9.0, 5.0, 6.0, 8.0, 1.0, 4.0]`
2. **`data_summary`** -- fold for min/max, sum/length for mean; handle empty with NaN
3. **`normalize_data`** -- compute min/max, apply formula; handle constant data
4. **`ascii_bar_chart`** -- compute max, build bars with `"█".repeat()`, format each line
5. **`create_series`** -- `zip(names, values)` into `Vec`, assert equal lengths

Run `cd workshop && cargo test` after each step. Groups: `step_01_data_basics` (5 tests), `step_02_visualization` (6 tests), `step_03_series` (5 tests).

## 8. Summary

| Concept | Rust Equivalent | Python Equivalent | Used In |
|---|---|---|---|
| Summary statistics | `fold(f64::INFINITY, f64::min)`, `.sum() / len` | `min()`, `max()`, `sum()/len` | `data_summary` |
| Min-max normalization | `(v - min) / (max - min) * 100` | `(v - mn) / (mx - mn) * 100` | `normalize_data` |
| String repetition | `"█".repeat(n)` | `"█" * n` | `ascii_bar_chart` |
| Labeled tuples | `zip` into `Vec<(&str, f64)>` | `zip()` into list of tuples | `create_series` |
| Format alignment | `format!("{:<10}", label)` | `f"{label:<10}"` | `ascii_bar_chart` |
