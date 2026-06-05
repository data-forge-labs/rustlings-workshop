# Workshop: Conversion & Error Handling — `unwrap`, `?`, `From`, and the Whole Family

> **Test-driven approach**: This project includes a Cargo project with progressive
> unit tests. Each function in `src/lib.rs` starts as a `todo!()` stub. As you
> follow each section, replace `todo!()` with real code and run `cargo test` to
> watch the pass count grow. Your goal: **all 10 tests pass**.

**Goal**: Implement all functions in `src/lib.rs` to pass all 10 tests.

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
