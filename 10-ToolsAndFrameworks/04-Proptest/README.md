# 🧪 Proptest — Property-Based Testing

*Subtitle: stop hand-writing 50 test cases. Describe the invariant and let proptest find the counter-example.*

> **Test-driven approach**: This project includes a Cargo project with progressive
> property-based tests. Each function in `src/lib.rs` starts as a `todo!()` stub.
> As you follow each section, replace `todo!()` with real code and run `cargo test`
> to watch the pass count grow. Proptest runs **32 random inputs per property**.
> Your goal: **all 8 properties pass** (256+ random inputs total).

---

## Why Property Tests for Data Pipelines?

**Python pain:** A parser reads a million rows. You wrote 5 unit tests with
hand-picked inputs. The 6th test on production data finds a buffer overflow
on negative numbers in column 3. You add the 6th test. The 7th test on
production finds a UTF-8 BOM issue. The list never ends.

**Rust fix:** A property test says "for **all** valid inputs, this invariant
holds." Proptest then *generates* thousands of inputs, **shrinking** any
counter-example to its smallest form before reporting. The test for
`count_above` becomes a single line: *the result equals the hand-written
reference*. Proptest tries negative numbers, empty vecs, all-equal inputs,
boundary values, and shrinks a failure down to `vec![-1]` with `threshold = 0`.

```rust
proptest! {
    #[test]
    fn prop_count_matches_filter(
        v in vec(-100i32..100, 0..50),
        t in -50i32..50
    ) {
        let actual = count_above(&v, t);
        let expected = v.iter().filter(|&&x| x > t).count();
        prop_assert_eq!(actual, expected);
    }
}
```

## At a Glance

| # | Concept | Rust | Python | Why it matters |
|---|---------|------|--------|----------------|
| 1 | Property-based testing | `proptest::proptest!` | `hypothesis.given(...)` | Generate inputs, not hand-pick them |
| 2 | Strategies | `proptest::collection::vec(...)` | `hypothesis.strategies.integers()` | The "any input" type |
| 3 | Random sampling | `ProptestConfig::with_cases(N)` | `@settings(max_examples=N)` | Control coverage vs speed |
| 4 | Shrinking | automatic | automatic | Failure becomes smallest reproducer |
| 5 | Invariants | `prop_assert!`, `prop_assert_eq!` | `assert ...` inside `@given` | The property the test must hold |
| 6 | Idempotence check | `sort(sort(x)) == sort(x)` | n/a | Classic property |
| 7 | Reference comparison | `count_above(x) == filter(x).count()` | n/a | Test the function via a known correct version |
| 8 | Bound check | `result >= 0.0` (epsilon) | n/a | Numerical stability across inputs |

---

> **Test-driven approach**: This project includes a Cargo project with progressive
> property-based tests. Each function in `src/lib.rs` starts as a `todo!()` stub.
> As you follow each section, replace `todo!()` with real code and run `cargo test`
> to watch the pass count grow. Proptest runs **32 random inputs per property**.
> Your goal: **all 8 properties pass** (256+ random inputs total).

**Goal**: Implement all functions in `src/lib.rs` to pass all 8 properties.


---

## Setup: Create the Project from Scratch

This is a hands-on workshop. You will write the code yourself following the steps below.

### 1. Create the new Cargo project

```bash
cargo new --lib proptest_workshop
cd proptest_workshop
```

### 2. Add the dependencies

Open `Cargo.toml` and replace whatever is there with this:

```toml
[package]
name = "proptest_workshop"
version = "0.1.0"
edition = "2024"

[dependencies]
proptest = "1"

```

### 3. Copy the test stubs as your starting point

This project follows a **test-driven** approach. Each function in `src/lib.rs` starts as a `todo!()` stub, and progressive tests guide your implementation.

```bash
cp "10-ToolsAndFrameworks/04-Proptest/workshop/src/lib.rs" src/lib.rs
cp "10-ToolsAndFrameworks/04-Proptest/workshop/src/main.rs" src/main.rs
```

### 4. Run the tests to see them fail (this is expected!)

```bash
cargo test
```

You should see all tests fail with the message "not yet implemented". That's the starting point — you are about to make them pass.

### 5. Follow the step-by-step sections below

Each section below corresponds to a step module in the test file. Implement the function(s) described, then run:

```bash
cargo test step_XX_name
```

to watch the pass count grow. The test module names match the section headings.

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
