# 📸 Insta — Snapshot Testing

*Subtitle: stop hand-writing expected strings. Capture the output once; review changes on every PR.*

> **Test-driven approach**: This project includes a Cargo project with progressive
> snapshot tests using **inline** snapshots. Each function in `src/lib.rs` starts
> as a `todo!()` stub. As you follow each section, replace `todo!()` with real
> code and run `cargo test`. The expected output is right in the test as
> `insta::assert_snapshot!(value, @"expected")`. Your goal: **all 8 tests pass**.

---

## What Is Snapshot Testing?

Capture expected output once, review changes on every PR with `cargo insta review`.

### Python equivalent

```python
def test_format_output():
    result = pretty_print(df)
    expected = "id | name\n---+-----\n1  | a"
    assert result == expected  # breaks on minor formatting changes
``` Three months later, you
update 12 such strings and miss one. CI fails. You re-discover the change.

**Rust fix:** `insta` records the output as a snapshot. The first run writes
`.snap.new`; you run `cargo insta review` to accept or reject. Every future
run compares against the accepted snapshot. The diff is shown in the test
output — you can see exactly what changed, line by line, character by
character. For inline snapshots, the expected string lives in the test file
itself, version-controlled and visible in PR reviews.

```rust
#[test]
fn test_format_currency() {
    insta::assert_snapshot!(format_currency_cents(12_345), @"$123.45");
}
```

### Topics covered

| # | Concept | Why it matters |
|---|---------|----------------|
| 1 | Inline snapshots | Expected value lives in the test file |
| 2 | External snapshots | `.snap` files in `src/snapshots/` |
| 3 | Review workflow | `cargo insta review` — diff and accept interactively |
| 4 | `assert_debug_snapshot!` | Show struct state |
| 5 | Glob feature | One config for the whole test suite |
| 6 | CI integration | Block merges when snapshots differ |

---

> **Test-driven approach**: This project includes a Cargo project with progressive
> snapshot tests using **inline** snapshots. Each function in `src/lib.rs` starts
> as a `todo!()` stub. As you follow each section, replace `todo!()` with real
> code and run `cargo test`. The expected output is right in the test as
> `insta::assert_snapshot!(value, @"expected")`. Your goal: **all 8 tests pass**.

**Goal**: Implement all functions in `src/lib.rs` to match the inline snapshots.


---

## Setup: Create the Project from Scratch

This is a hands-on workshop. You will write the code yourself following the steps below.

### 1. Create the new Cargo project

```bash
cargo new --lib insta_workshop
cd insta_workshop
```

### 2. Add the dependencies

Open `Cargo.toml` and replace whatever is there with this:

```toml
[package]
name = "insta_workshop"
version = "0.1.0"
edition = "2024"

[dependencies]
insta = { version = "1", features = ["glob"] }
chrono = "0.4"

```

### 3. Copy the test stubs as your starting point

This project follows a **test-driven** approach. Each function in `src/lib.rs` starts as a `todo!()` stub, and progressive tests guide your implementation.

```bash
cp "09-ObservabilityAndTesting/06-Insta/workshop/src/lib.rs" src/lib.rs
cp "09-ObservabilityAndTesting/06-Insta/workshop/src/main.rs" src/main.rs
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

### `format_bytes`
- **Signature**: `pub fn format_bytes(n: u64) -> String`
- **Task**: Use 1024-based (binary) units: `0..1024` → `"<n> B"`, else compute KiB/MiB/GiB with 2 decimals (`"1.00 KiB"`).

### `format_duration`
- **Signature**: `pub fn format_duration(d: Duration) -> String`
- **Task**: `< 1s` → `"<ms>ms"`, `< 60s` → `"<s_with_2_decimals>s"`, else `"<m>m<ss>s"`.

### `format_sql_select`
- **Signature**: `pub fn format_sql_select(table: &str, columns: &[&str]) -> String`
- **Task**: `format!("SELECT {} FROM {}", columns.join(", "), table)`.

### `format_log_line`
- **Signature**: `pub fn format_log_line(level: &str, target: &str, message: &str) -> String`
- **Task**: `format!("[{}] {}: {}", level, target, message)`.

### `format_path`
- **Signature**: `pub fn format_path(parts: &[&str]) -> String`
- **Task**: Join with `/`. Treat empty first segment as absolute leading `/`.

### `format_table`
- **Signature**: `pub fn format_table(headers: &[&str], rows: &[Vec<String>]) -> String`
- **Task**: Build `"h1 | h2 | h3\n---+----+---\nv1 | v2 | v3"`. Compute column widths from headers + rows, pad to left, join with `" | "`.

### `format_currency_cents`
- **Signature**: `pub fn format_currency_cents(cents: u64) -> String`
- **Task**: `format!("${}.{:02}", cents / 100, cents % 100)`.

### `format_error_chain`
- **Signature**: `pub fn format_error_chain(top: &str, sources: &[&str]) -> String`
- **Task**: If `sources` is empty → `top.to_string()`. Else fold: start with `top`, then for each `s` in `sources`, append `: caused by: {s}`.

## Test Modules

| Module | Tests | What It Tests |
|--------|-------|---------------|
| test_format_bytes | 1 | 5 inline snapshots (0 B → 1.00 GiB) |
| test_format_duration | 1 | 4 inline snapshots (0ms → 1m30s) |
| test_format_sql_select | 1 | column list + `*` |
| test_format_log_line | 1 | INFO + ERROR lines |
| test_format_path | 1 | relative + absolute |
| test_format_table | 1 | 2-row table with separators |
| test_format_currency | 1 | 4 inline snapshots |
| test_format_error_chain | 1 | empty chain + 2-level chain |

## How to Run Tests
```bash
cargo test
```

To update snapshots after intentional changes:
```bash
cargo install cargo-insta
cargo insta review
```

## Exercises

* **Easy** – modify the existing function to handle an extra edge case.
* **Medium** – extend the project with a new helper function that re‑uses the core logic.

