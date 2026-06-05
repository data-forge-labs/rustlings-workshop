# Workshop: Insta — Snapshot Testing

> **Test-driven approach**: This project includes a Cargo project with progressive
> snapshot tests using **inline** snapshots. Each function in `src/lib.rs` starts
> as a `todo!()` stub. As you follow each section, replace `todo!()` with real
> code and run `cargo test`. The expected output is right in the test as
> `insta::assert_snapshot!(value, @"expected")`. Your goal: **all 8 tests pass**.

**Goal**: Implement all functions in `src/lib.rs` to match the inline snapshots.

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
