# 📸 Insta — Snapshot Testing

*Subtitle: stop hand-writing expected strings. Capture the output once; review changes on every PR.*

> **Test-driven approach**: This project includes a Cargo project with progressive
> snapshot tests using **inline** snapshots. Each function in `src/lib.rs` starts
> as a `todo!()` stub. As you follow each section, replace `todo!()` with real
> code and run `cargo test`. The expected output is right in the test as
> `insta::assert_snapshot!(value, @"expected")`. Your goal: **all 8 tests pass**.

---

## Why Snapshot Tests for Data Pipeline Output?

**Python pain:** You write `assert pretty_print(df) == "id | name\n---+-----\n1  | a"`.
You re-run the pipeline after a Polars upgrade, the output changes from
`"1  | a"` to `"1 | a"` (one less space — pandas 2.0). You check the diff
manually, decide it's fine, update the string. Three months later, you
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

## At a Glance

| # | Concept | Rust | Python | Why it matters |
|---|---------|------|--------|----------------|
| 1 | Inline snapshots | `assert_snapshot!(v, @"x")` | n/a (no clean equivalent) | Expected value lives in the test file |
| 2 | External snapshots | `.snap` files in `src/snapshots/` | `pytest --snapshot` | Filesystem-managed snapshots |
| 3 | Review workflow | `cargo insta review` | `pytest --snapshot-update` | Diff and accept interactively |
| 4 | `assert_snapshot!` | redacted by default | n/a | Strings, Debug, JSON, YAML |
| 5 | `assert_debug_snapshot!` | uses `{:?}` | n/a | Show struct state |
| 6 | `assert_yaml_snapshot!` | uses serde_yaml | n/a | Pretty-print serde types |
| 7 | Glob feature | redactions across all snapshots | n/a | One config for the whole test suite |
| 8 | CI integration | `cargo insta test --review` | `--update-snapshots` | Block merges when snapshots differ |

---
