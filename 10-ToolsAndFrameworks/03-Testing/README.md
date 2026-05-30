# Rust Testing — Python pytest Equivalent

> **Test-driven approach**: This project includes a Cargo project with progressive unit tests. Each function in `workshop/src/lib.rs` starts as a `todo!()` stub. As you follow each section, replace `todo!()` with real code and run `cd workshop && cargo test` to watch the pass count grow. Your goal: **all 18 tests pass**.

## Table of Contents

1. [Introduction](#1-introduction)
2. [Prerequisites](#2-prerequisites)
3. [`#[test]` — The Unit Test Attribute](#3-test--the-unit-test-attribute)
4. [Assertions — `assert_eq!`, `assert_ne!`, `assert!`](#4-assertions--assert_eq-assert_ne-assert)
5. [`#[should_panic]` — Testing Error Conditions](#5-should_panic--testing-error-conditions)
6. [`Result<T, E>` in Tests](#6-resultt-e-in-tests)
7. [Organizing Tests with `#[cfg(test)]`](#7-organizing-tests-with-cfgtest)
8. [Integration Tests — The `tests/` Directory](#8-integration-tests--the-tests-directory)
9. [Property-Based Testing Patterns](#9-property-based-testing-patterns)
10. [Putting It All Together](#10-putting-it-all-together)
11. [Complete Code Reference](#11-complete-code-reference)
12. [Summary](#12-summary)

## 1. Introduction

Testing is where Rust truly shines. The language bakes testing into the compiler — no need for pytest plugins, fixtures, or conftest.py. Your tests live alongside your code, run in parallel by default, and integrate with `cargo` natively.

**What you'll learn:**
- `#[test]` attribute — Rust's equivalent of `def test_`
- Assertion macros — `assert_eq!`, `assert_ne!`, `assert!`
- `#[should_panic]` — the Rust version of `pytest.raises`
- `Result<T, E>` return types in tests
- `#[cfg(test)]` module organization
- Integration tests in the `tests/` directory
- Property-based testing patterns (without external crates)

## 2. Prerequisites

- Functions, `Result<T, E>`, `Option<T>`
- **Projects**: [01-BasicCalculator](../../01-Foundations/01-Intro/README.md), [03-MasterMind](../../01-Foundations/03-MasterMind/README.md) (for pattern matching)

## 3. `#[test]` — The Unit Test Attribute

### Explanation

In Python, pytest discovers functions starting with `test_`:
```python
def test_addition():
    assert 2 + 3 == 5
```

In Rust, you annotate functions with `#[test]`:
```rust
#[test]
fn test_addition() {
    assert_eq!(add(2, 3), 5);
}
```

Run with:
```bash
cd workshop && cargo test
```

| Python (pytest) | Rust |
|----------------|------|
| `def test_foo():` | `#[test] fn test_foo()` |
| `pytest tests/` | `cd workshop && cargo test` |
| `pytest -k pattern` | `cd workshop && cargo test pattern` |
| `pytest -v` | `cd workshop && cargo test` (always verbose for failures) |
| `@pytest.mark.skip` | `#[ignore]` |

### Applying to Our Project

Every test in this project uses `#[test]`. For example, tests for `add` are in `mod step_01_basic_tests`.

## 4. Assertions — `assert_eq!`, `assert_ne!`, `assert!`

### Explanation

Python:
```python
assert result == 5
assert result != 0
assert is_ok
assert result is None
```

Rust:
```rust
assert_eq!(result, 5);       // result == 5
assert_ne!(result, 0);       // result != 0
assert!(is_ok);              // boolean true
assert!(result.is_none());   // boolean expression
```

Key difference: `assert_eq!` and `assert_ne!` print **both** values on failure — like pytest's `assert a == b` introspection.

```rust
// On failure, cargo shows:
// left: 4
// right: 5
assert_eq!(add(2, 2), 5);
```

### Applying to Our Project

Use `assert_eq!` for `add`, `divide`, and `fibonacci`. Use `assert!` / `assert!(!...)` for `validate_email`. Use `assert_eq!` / `assert!(result.is_err())` for `find_item` and `divide`.

## 5. `#[should_panic]` — Testing Error Conditions

### Explanation

Python's `pytest.raises`:
```python
import pytest
def test_out_of_bounds():
    with pytest.raises(IndexError):
        lst = [1, 2, 3]
        _ = lst[10]
```

Rust's `#[should_panic]`:
```rust
#[test]
#[should_panic(expected = "index out of bounds")]
fn test_panic_on_oob() {
    let v = vec![1, 2, 3];
    let _ = v[10];
}
```

| Python | Rust |
|--------|------|
| `with pytest.raises(ValueError):` | `#[should_panic(expected = "...")]` |
| `match` in exception info | `expected` string (substring match) |

**Note**: `#[should_panic]` catches any panic, not a specific type. The `expected` parameter filters by panic message substring. For Result-based errors, use `assert!(result.is_err())` instead.

### Applying to Our Project

The project already includes `test_panic_on_oob` as a demonstration. You'll also write a `#[should_panic]` test for `add()` overflow.

## 6. `Result<T, E>` in Tests

### Explanation

Rust allows tests to return `Result` — when they return `Err`, the test fails with the error:

```rust
#[test]
fn test_divide_result() -> Result<(), String> {
    let result = divide(10.0, 2.0)?;
    assert_eq!(result, 5.0);
    Ok(())
}
```

This avoids panics and is the Rust equivalent of `pytest.raises` for valid-code-that-might-fail scenarios. In Python:
```python
def test_divide():
    result = divide(10, 2)
    assert result is not None
    assert result == 5.0
```

### Applying to Our Project

The project already includes `test_divide_result` as a demonstration of this pattern.

## 7. Organizing Tests with `#[cfg(test)]`

### Explanation

In Python you put tests in a separate `tests/` directory. In Rust you have **two** options:

**Unit tests** — alongside the code, inside `#[cfg(test)]` modules:
```rust
// src/lib.rs
pub fn add(a: i32, b: i32) -> i32 { a + b }

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_add() {
        assert_eq!(add(2, 3), 5);
    }
}
```

`#[cfg(test)]` means this module is only compiled during `cd workshop && cargo test` — zero runtime overhead in production builds. This is Rust's version of Python's `if __name__ == "__main__"` test guard, but enforced at compile time.

**Integration tests** — in a separate `tests/` directory at the project root, each file is its own crate:
```
my-project/
├── Cargo.toml
├── src/lib.rs
└── tests/
    └── integration.rs
```

### Applying to Our Project

Our tests are organized in nested modules within `#[cfg(test)]`. Each step has its own `mod` for clear progression.

## 8. Integration Tests — The `tests/` Directory

### Explanation

Create `tests/my_test.rs`:
```rust
// tests/integration.rs
use rust_testing;

#[test]
fn test_add_integration() {
    assert_eq!(rust_testing::add(10, 20), 30);
}
```

Each file in `tests/` compiles as a separate binary. This is like having multiple Python test files that all import the library.

## 9. Property-Based Testing Patterns

### Explanation

Python's `hypothesis` library generates random inputs:
```python
from hypothesis import given, strategies as st

@given(st.integers())
def test_add_commutative(a, b):
    assert add(a, b) == add(b, a)
```

While this project doesn't use the `proptest` crate, you can still express **property-based tests** manually by testing boundary conditions:

```rust
#[test]
fn test_fibonacci_properties() {
    // fib(0) = 0
    assert_eq!(fibonacci(0), 0);
    // fib(1) = 1
    assert_eq!(fibonacci(1), 1);
    // fib(n) = fib(n-1) + fib(n-2)
    for n in 2..15 {
        assert_eq!(fibonacci(n), fibonacci(n-1) + fibonacci(n-2));
    }
}
```

This tests the **property** (the recurrence relation) rather than a single value — the same mindset as property-based testing.

### Applying to Our Project

The `fibonacci` and `validate_email` tests cover multiple representative inputs to demonstrate property-like coverage.

## 10. Putting It All Together

Open `workshop/src/lib.rs` and implement each function:

**`add`** — Simple addition: `a + b`.

**`divide`** — Return `Ok(a / b)` if `b != 0.0`, else `Err("division by zero".into())`.

**`find_item`** — Use `.iter().position(|x| x == target)`.

**`fibonacci`** — Implement with a loop: `f(0)=0, f(1)=1, f(n)=f(n-1)+f(n-2)`.

**`validate_email`** — Check that the string contains exactly one `@`, has content before and after it, and the domain contains a `.`.

**`test_types`** — Return mapping pairs like `("#[test]", "def test_")`, `("#[should_panic]", "pytest.raises")`, etc.

Run tests after each function:
```bash
cd workshop && cargo test
```

For the integration test, create `tests/integration_test.rs`:
```rust
use rust_testing;

#[test]
fn test_add_integration() {
    assert_eq!(rust_testing::add(100, 200), 300);
}

#[test]
fn test_fibonacci_integration() {
    assert_eq!(rust_testing::fibonacci(10), 55);
}
```

## 11. Complete Code Reference

```rust
pub fn add(a: i32, b: i32) -> i32 {
    a.checked_add(b).expect("overflow")
}

pub fn divide(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        Err("division by zero".into())
    } else {
        Ok(a / b)
    }
}

pub fn find_item<T: PartialEq>(slice: &[T], target: &T) -> Option<usize> {
    slice.iter().position(|x| x == target)
}

pub fn fibonacci(n: u32) -> u64 {
    match n {
        0 => 0,
        1 => 1,
        _ => {
            let mut a = 0;
            let mut b = 1;
            for _ in 2..=n {
                let next = a + b;
                a = b;
                b = next;
            }
            b
        }
    }
}

pub fn validate_email(email: &str) -> bool {
    let parts: Vec<&str> = email.split('@').collect();
    if parts.len() != 2 {
        return false;
    }
    let (local, domain) = (parts[0], parts[1]);
    if local.is_empty() || domain.is_empty() {
        return false;
    }
    domain.contains('.') && !domain.starts_with('.') && !domain.ends_with('.')
}

pub fn test_types() -> Vec<(&'static str, &'static str)> {
    vec![
        ("#[test]", "def test_ (pytest)"),
        ("#[should_panic]", "pytest.raises(...)"),
        ("assert_eq!", "assert =="),
        ("assert_ne!", "assert !="),
        ("Result<T,E> in test", "pytest fixture return"),
        ("#[ignore]", "@pytest.mark.skip"),
        ("cd workshop && cargo test", "pytest"),
    ]
}

#[cfg(test)]
mod tests {
    // ... (see src/lib.rs for full test module hierarchy)
}
```

## 12. Summary

| Concept | Where Used | Python Equivalent |
|---------|-----------|-------------------|
| `#[test]` | All test functions | `def test_` in pytest |
| `assert_eq!` / `assert!` | All assertions | `assert` statement |
| `#[should_panic]` | Overflow, OOB tests | `pytest.raises()` |
| `Result<T,E>` return | `test_divide_result` | Returning from test function |
| `#[cfg(test)]` | Test module wrapper | `if __name__ == "__main__"` |
| `tests/` directory | Integration test crate | `tests/` in pytest |
| Property patterns | Fibonacci properties | `@given` in hypothesis |
