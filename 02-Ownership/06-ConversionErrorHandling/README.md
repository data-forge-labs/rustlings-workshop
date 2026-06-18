# 🔁 Conversion & Error Handling — `unwrap`, `?`, `From`, and the Whole Family

*Subtitle: the 20+ methods on `Option<T>` and `Result<T, E>` that turn an "if let" tower into one expression.*

> **Test-driven approach**: This project includes a Cargo project with progressive
> unit tests. Each function in `src/lib.rs` starts as a `todo!()` stub. As you
> follow each section, replace `todo!()` with real code and run `cargo test` to
> watch the pass count grow. Your goal: **all 10 tests pass**.

---

## What Is This Reference?

A comprehensive reference for `Option<T>` and `Result<T, E>` combinators — the 20+ methods that turn `if let` towers into single expressions.

### Python equivalent

```python
# Python — inconsistent error conventions
def find_user(id):
    if id == 0:
        return None  # "missing"
    if id < 0:
        return ""    # "empty"
    return {"id": id}  # "found"
# Caller must guess which convention this function uses

# Rust: Option<T> and Result<T, E> are enums — the compiler
# forces you to handle both variants. Combinators like .map(),
# .and_then(), .unwrap_or() chain operations cleanly.
```

```rust
fn read_age(s: &str) -> Result<u32, AppError> {
    let n: u32 = s.trim().parse().map_err(AppError::from)?;
    if n > 150 { return Err(AppError::Validation("too old".into())); }
    Ok(n)
}
```

## At a Glance

| # | Concept | Rust | Python | Why it matters |
|---|---------|------|--------|----------------|
| 1 | `Option::unwrap_or(default)` | `opt.unwrap_or(0)` | `x or 0` | Default value when None |
| 2 | `Option::unwrap_or_default()` | `opt.unwrap_or_default()` | n/a | Uses `T::default()` |
| 3 | `Option::map_or(f, \|v\| ...)` | `opt.map_or(0, \|v\| v * 2)` | `v * 2 if v else 0` | Branch in one line |
| 4 | `Option::ok_or(err)` | `opt.ok_or(AppError::Missing)` | n/a | Lift `Option` to `Result` |
| 5 | `Result::map_err(f)` | `r.map_err(AppError::from)` | `try/except` + rewrap | Convert error types |
| 6 | `Result::and_then(f)` | `r.and_then(\|v\| process(v))` | chained `if r: return r` | Monadic bind |
| 7 | `?` operator | `let v = fallible()?;` | `try/except` | Auto-propagate errors |
| 8 | `From<E1> for E2` | `impl From<io::Error> for AppError` | n/a | `?` does the conversion for free |
| 9 | `thiserror` | `#[derive(Error)]` + `#[from]` | `class AppError(Exception)` | Less boilerplate |
| 10 | `Option::filter` | `opt.filter(\|v\| *v > 0)` | `if v > 0 else None` | Conditional None |
| 11 | `Option::or_else` | `a.or_else(\|\| b())` | `a if a else b()` | Lazy fallback |
| 12 | `Option::transpose` | `opt.transpose()` | n/a | Flip `Option<Result>` ↔ `Result<Option>` |

---

> **Test-driven approach**: This project includes a Cargo project with progressive
> unit tests. Each function in `src/lib.rs` starts as a `todo!()` stub. As you
> follow each section, replace `todo!()` with real code and run `cargo test` to
> watch the pass count grow. Your goal: **all 10 tests pass**.

**Goal**: Implement all functions in `src/lib.rs` to pass all 10 tests.


---

## Setup: Create the Project from Scratch

This is a hands-on workshop. You will write the code yourself following the steps below.

### 1. Create the new Cargo project

```bash
cargo new --lib conversion_error_handling_workshop
cd conversion_error_handling_workshop
```

### 2. Add the dependencies

Open `Cargo.toml` and replace whatever is there with this:

```toml
[package]
name = "conversion_error_handling_workshop"
version = "0.1.0"
edition = "2024"

[dependencies]
thiserror = "1"

```

### 3. Copy the test stubs as your starting point

This project follows a **test-driven** approach. Each function in `src/lib.rs` starts as a `todo!()` stub, and progressive tests guide your implementation.

```bash
cp "02-Ownership/06-ConversionErrorHandling/workshop/src/lib.rs" src/lib.rs
cp "02-Ownership/06-ConversionErrorHandling/workshop/src/main.rs" src/main.rs
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

### Step 1 — `Option` methods

#### `unwrap_or_default_when_none`
- **Signature**: `pub fn unwrap_or_default_when_none(opt: Option<i32>) -> i32`
- **Task**: `opt.unwrap_or_default()` — uses `i32::default()` (which is `0`).

#### `map_or_default`
- **Signature**: `pub fn map_or_default(opt: Option<i32>, fallback: i32) -> i32`
- **Task**: `opt.map_or(fallback, |v| v)`.

#### `ok_or_convert`
- **Signature**: `pub fn ok_or_convert(opt: Option<String>) -> Result<String, AppError>`
- **Task**: `opt.ok_or(AppError::Missing("value"))`.

### Step 2 — `Result` methods

#### `map_err_convert`
- **Signature**: `pub fn map_err_convert(s: &str) -> Result<i32, AppError>`
- **Task**: `s.parse::<i32>().map_err(AppError::from)`.

#### `and_then_chain`
- **Signature**: `pub fn and_then_chain(s: &str) -> Result<i32, AppError>`
- **Task**: `let n: i32 = s.parse().map_err(AppError::from)?; Ok(n * 2)`.

### Step 3 — `From` for error conversion

#### `read_and_parse`
- **Signature**: `pub fn read_and_parse(line: &str) -> Result<i32, AppError>`
- **Task**: `let n: i32 = line.trim().parse().map_err(AppError::from)?; Ok(n)`. The `?` works because `AppError: From<ParseIntError>` is auto-derived by `#[from]`.

### Step 4 — `?` operator pipelines

#### `multi_step_pipeline`
- **Signature**: `pub fn multi_step_pipeline(s: &str) -> Result<i32, AppError>`
- **Task**: Read s, parse, multiply by 3, return. Use `?` to propagate.

#### `first_present`
- **Signature**: `pub fn first_present<'a>(opts: &'a [Option<&'a str>]) -> Result<&'a str, AppError>`
- **Task**: `opts.iter().filter_map(|o| *o).next().ok_or(AppError::Missing("first_present"))`.

## Test Modules

| Module | Tests | What It Tests |
|--------|-------|---------------|
| step_01_option_methods | 3 | unwrap_or_default, map_or_default, ok_or_convert |
| step_02_result_methods | 2 | map_err_convert + and_then_chain |
| step_03_from_conversion | 2 | read_and_parse ok + err |
| step_04_question_mark | 3 | multi_step ok/err + first_present |

## How to Run Tests
```bash
cargo test
```

## Exercises

* **Easy** – modify the existing function to handle an extra edge case.
* **Medium** – extend the project with a new helper function that re‑uses the core logic.

