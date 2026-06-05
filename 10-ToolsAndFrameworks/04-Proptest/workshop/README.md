# Workshop: Proptest — Property-Based Testing

> **Test-driven approach**: This project includes a Cargo project with progressive
> property-based tests. Each function in `src/lib.rs` starts as a `todo!()` stub.
> As you follow each section, replace `todo!()` with real code and run `cargo test`
> to watch the pass count grow. Proptest runs **32 random inputs per property**.
> Your goal: **all 8 properties pass** (256+ random inputs total).

**Goal**: Implement all functions in `src/lib.rs` to pass all 8 properties.

## Functions to Implement

### Step 1 — Sorted check
- **`is_sorted_ascending(v: &[i32]) -> bool`**: `v.windows(2).all(|w| w[0] <= w[1])`.

### Step 2 — Sort
- **`sort_ascending(v: Vec<i32>) -> Vec<i32>`**: `let mut s = v; s.sort(); s`.

### Step 3 — Reverse
- **`reverse_vec(v: Vec<i32>) -> Vec<i32>`**: `v.into_iter().rev().collect()`.

### Step 4 — Count
- **`count_above(v: &[i32], threshold: i32) -> usize`**: `v.iter().filter(|&&x| x > threshold).count()`.

### Step 5 — Sum
- **`sum_vec(v: Vec<i32>) -> i32`**: `v.iter().sum()`.

### Step 6 — Normalize
- **`normalize_floats(v: Vec<f64>) -> Vec<f64>`**: Find min and max; if equal, return zeros. Else map each to `(v - min) / (max - min)`.

### Step 7 — Min / max
- **`min_max(v: &[i32]) -> Option<(i32, i32)>`**: `v.iter().min().zip(v.iter().max())` mapped to a tuple of `i32` (`.copied()`).

### Step 8 — Dedup
- **`dedup_sorted(v: Vec<i32>) -> Vec<i32>`**: `let mut s = v; s.sort(); s.dedup(); s`.

## Properties Tested

| # | Property | Why it matters |
|---|----------|----------------|
| 1 | `sort(sort(x)) == sort(x)` | Sort is idempotent |
| 2 | `is_sorted_ascending(sort(x))` | Output is sorted |
| 3 | `reverse(reverse(x)) == x` | Reverse is its own inverse |
| 4 | `count_above(x, t) == filter(|&x > t|).count()` | Matches a hand-written reference |
| 5 | `sum(x) == sum(reverse(x))` | Sum is order-independent |
| 6 | All values in `[0, 1]` after `normalize` | Bounds check (with 1e-9 epsilon) |
| 7 | `min ≤ max` for any non-empty input | Min/max sanity |
| 8 | `dedup_sorted(x).len() ≤ x.len()` | Dedup never grows |

## How to Run Tests
```bash
cargo test
```

Each property runs 32 random inputs by default; to expand coverage:
```bash
PROPTEST_CASES=10000 cargo test
```
