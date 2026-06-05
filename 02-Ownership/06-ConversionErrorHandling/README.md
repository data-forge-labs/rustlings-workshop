# 🔁 Conversion & Error Handling — `unwrap`, `?`, `From`, and the Whole Family

*Subtitle: the 20+ methods on `Option<T>` and `Result<T, E>` that turn an "if let" tower into one expression.*

> **Test-driven approach**: This project includes a Cargo project with progressive
> unit tests. Each function in `src/lib.rs` starts as a `todo!()` stub. As you
> follow each section, replace `todo!()` with real code and run `cargo test` to
> watch the pass count grow. Your goal: **all 10 tests pass**.

---

## Why Error Handling Is a First-Class Concept in Rust

**Python pain:** A pipeline function returns `None` for "missing", `""` for
"empty", `-1` for "not found", raises `ValueError` for "bad input", and
sometimes `print("error")` and returns `None`. The caller has to know which
convention this specific function uses. The IDE can't help. Refactors are
scary because removing a `try/except` block might break callers in a
non-obvious way.

**Rust fix:** `Option<T>` (presence/absence) and `Result<T, E>` (success/failure
with reason) are *enums* — the compiler enforces that you check both
variants. The `From` trait lets `?` automatically convert one error type into
another. Combinators like `map`, `and_then`, `unwrap_or`, `ok_or` turn 10
lines of `if let` into one expression that the compiler can verify is total.

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
