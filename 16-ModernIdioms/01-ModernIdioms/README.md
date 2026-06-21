# 🦀 Modern Rust Idioms — Python to Rust Workshop

> **Test-driven approach**: This project includes a Cargo project with progressive
> unit tests. Each function in `src/lib.rs` starts as a `todo!()` stub. As you
> follow each section, replace `todo!()` with real code and run `cargo test` to
> watch the pass count grow. Your goal: **all 14 tests pass**.

---

## What Is This Workshop?

Modern Rust idioms from versions 1.80-1.96 that make your code cleaner, safer, and more efficient.

### Python equivalent

```python
# Python: global config with lazy loading
import threading
_config = None
_config_lock = threading.Lock()

def get_config():
    global _config
    if _config is None:
        with _config_lock:
            if _config is None:
                _config = load_config()
    return _config

# Python: sliding windows
def sliding_sum(data, window_size):
    return [sum(data[i:i+window_size]) for i in range(len(data) - window_size + 1)]

# Python: complex conditional matching
def process_message(msg_id, content):
    if msg_id is not None and content is not None:
        return f"Message {msg_id}: {content}"
    elif msg_id is None:
        return "Missing message ID"
    else:
        return "Missing message content"
```

In this project you'll learn to build this in Rust — and along the way
you'll discover **LazyLock**, **array_windows**, **if let chains**, **cfg_select!**, and **assert_matches!**.

### Topics covered

| # | Concept | Why it matters |
|---|---------|----------------|
| 1 | `LazyLock` / `LazyCell` | Thread-safe lazy initialization without third-party crates |
| 2 | `array_windows` | Sliding window operations on slices at compile-time size |
| 3 | `if let` chains | Complex conditional pattern matching in one expression |
| 4 | `cfg_select!` | Platform-specific code without repetition |
| 5 | `assert_matches!` | Better test assertions for pattern matching |

---

## Table of Contents

1. [Introduction](#1-introduction)
2. [Prerequisites](#2-prerequisites)
3. [Concept: LazyLock for Global State](#3-concept-lazylock-for-global-state)
4. [Concept: array_windows for Sliding Windows](#4-concept-array_windows-for-sliding-windows)
5. [Concept: if let Chains for Complex Matching](#5-concept-if-let-chains-for-complex-matching)
6. [Concept: cfg_select! for Conditional Compilation](#6-concept-cfg_select-for-conditional-compilation)
7. [Concept: assert_matches! for Better Tests](#7-concept-assert_matches-for-better-tests)
8. [Putting It All Together](#8-putting-it-all-together)
9. [Exercises](#9-exercises)
10. [Summary](#10-summary)

## 1. Introduction

Rust evolves rapidly, and versions 1.80 through 1.96 introduced several features that make everyday code cleaner and more expressive. This workshop covers five modern idioms that are particularly useful for data engineering:

1. **LazyLock** — Initialize expensive resources (config, connection pools) once, thread-safely
2. **array_windows** — Process sliding windows over data with zero-cost abstractions
3. **if let chains** — Write complex conditional logic without deeply nested `if let` blocks
4. **cfg_select!** — Select platform-specific implementations without repeating boilerplate
5. **assert_matches!** — Write clearer test assertions for pattern matching

These features replace older patterns that required third-party crates or verbose workarounds.

## 2. Prerequisites

- **Rust 1.96+** — Some features require recent stable versions
- Familiarity with:
  - Ownership and borrowing (Section 02)
  - Pattern matching with `match` and `if let` (Section 01)
  - Traits and generics (Section 02)
  - Basic testing with `#[test]` (Section 09)

## 3. Concept: LazyLock for Global State

### Explanation

In Python, you might use a module-level variable with a lock for lazy initialization:

```python
import threading

_config = None
_lock = threading.Lock()

def get_config():
    global _config
    if _config is None:
        with _lock:
            if _config is None:
                _config = load_config()
    return _config
```

Before Rust 1.80, you needed the `lazy_static!` or `once_cell` crates for thread-safe lazy initialization. Now `LazyLock` is in the standard library.

### Example

```rust
use std::sync::LazyLock;
use std::collections::HashMap;

struct Config {
    max_retries: u32,
    feature_flags: HashMap<String, bool>,
}

static CONFIG: LazyLock<Config> = LazyLock::new(|| {
    Config {
        max_retries: 3,
        feature_flags: HashMap::from([
            ("enable_cache".to_string(), true),
        ]),
    }
});

fn main() {
    // First access initializes the config
    println!("Max retries: {}", CONFIG.max_retries);
    
    // Subsequent accesses reuse the same instance
    println!("Cache enabled: {}", CONFIG.feature_flags["enable_cache"]);
}
```

### Applying to Our Project

In `lib.rs`, the `get_config()` function uses `LazyLock` to provide thread-safe access to global configuration:

```rust
static CONFIG: LazyLock<Config> = LazyLock::new(|| {
    Config {
        max_retries: 3,
        timeout_ms: 5000,
        feature_flags: HashMap::from([
            ("enable_cache".to_string(), true),
            ("debug_mode".to_string(), false),
        ]),
    }
});

pub fn get_config() -> &'static Config {
    &CONFIG
}
```

**Your task**: The `get_config()` function is already implemented. Review it and understand how `LazyLock` works.

## 4. Concept: array_windows for Sliding Windows

### Explanation

In Python, sliding windows are common in data processing:

```python
def sliding_sum(data, window_size):
    return [sum(data[i:i+window_size]) for i in range(len(data) - window_size + 1)]

# Or using itertools
from itertools import islice
def sliding_window(iterable, n):
    it = iter(iterable)
    win = list(islice(it, n))
    if len(win) == n:
        yield tuple(win)
    for item in it:
        win = win[1:] + [item]
        yield tuple(win)
```

Rust 1.94 stabilized `array_windows`, which gives you fixed-size array references with zero runtime overhead.

### Example

```rust
fn main() {
    let data = vec![1, 2, 3, 4, 5, 6, 7, 8];
    
    // Sliding windows of size 3
    for window in data.array_windows::<3>() {
        println!("Window: {:?}", window);
        // window is &[i32; 3]
    }
    
    // Compute sliding sums
    let sums: Vec<i32> = data.array_windows::<3>()
        .map(|w| w.iter().sum())
        .collect();
    println!("Sums: {:?}", sums); // [6, 9, 12, 15, 18, 21]
}
```

### Applying to Our Project

Implement `sliding_sum` using `array_windows`:

```rust
pub fn sliding_sum(data: &[i32], window_size: usize) -> Vec<i32> {
    // array_windows requires a const generic, so we handle common sizes
    // For a general solution, we'll use a different approach
    data.windows(window_size)
        .map(|w| w.iter().sum())
        .collect()
}
```

**Your task**: Replace the `todo!()` in `sliding_sum` with a working implementation using `array_windows` for known window sizes.

## 5. Concept: if let Chains for Complex Matching

### Explanation

In Python, complex conditional logic uses multiple `if` statements:

```python
def process_message(msg_id, content):
    if msg_id is not None and content is not None:
        return f"Message {msg_id}: {content}"
    elif msg_id is None:
        return "Missing message ID"
    else:
        return "Missing message content"
```

Before Rust 2024 Edition, you needed nested `if let` blocks:

```rust
// Old way - nested and verbose
if let Some(id) = msg_id {
    if let Some(content) = content {
        return format!("Message {}: {}", id, content);
    } else {
        return "Missing message content".to_string();
    }
} else {
    return "Missing message ID".to_string();
}
```

Rust 1.88 introduced `if let` chains (in Rust 2024 Edition), allowing you to combine multiple `if let` conditions:

```rust
// New way - flat and readable
if let Some(id) = msg_id
    && let Some(content) = content
{
    return format!("Message {}: {}", id, content);
} else if msg_id.is_none() {
    return "Missing message ID".to_string();
} else {
    return "Missing message content".to_string();
}
```

### Example

```rust
fn process_pair(first: Option<i32>, second: Option<String>) -> String {
    if let Some(f) = first
        && let Some(s) = second
        && f > 0
    {
        format!("Pair({}, {})", f, s)
    } else {
        "Invalid pair".to_string()
    }
}
```

### Applying to Our Project

Implement `process_message` using if let chains:

```rust
pub fn process_message(id: Option<i32>, content: Option<&str>) -> String {
    if let Some(id) = id
        && let Some(content) = content
    {
        format!("Message {}: {}", id, content)
    } else if id.is_none() {
        "Missing message ID".to_string()
    } else {
        "Missing message content".to_string()
    }
}
```

**Your task**: Replace the `todo!()` in `process_message` with an implementation using if let chains.

## 6. Concept: cfg_select! for Conditional Compilation

### Explanation

In Python, platform-specific code uses `sys.platform`:

```python
import sys

def get_platform_info():
    if sys.platform == "linux":
        return "Linux"
    elif sys.platform == "darwin":
        return "macOS"
    elif sys.platform == "win32":
        return "Windows"
    else:
        return "Unknown"
```

In Rust, you'd use multiple `#[cfg]` attributes:

```rust
// Old way - repetitive
#[cfg(target_os = "linux")]
fn platform_info() -> &'static str { "Linux" }

#[cfg(target_os = "macos")]
fn platform_info() -> &'static str { "macOS" }

#[cfg(target_os = "windows")]
fn platform_info() -> &'static str { "Windows" }

#[cfg(not(any(target_os = "linux", target_os = "macos", target_os = "windows")))]
fn platform_info() -> &'static str { "Unknown" }
```

Rust 1.95 introduced `cfg_select!`, which is cleaner:

```rust
fn platform_info() -> &'static str {
    cfg_select! {
        target_os = "linux" => "Linux",
        target_os = "macos" => "macOS",
        target_os = "windows" => "Windows",
        _ => "Unknown",
    }
}
```

### Example

```rust
fn get_path_separator() -> &'static str {
    cfg_select! {
        windows => "\\",
        _ => "/",
    }
}

fn main() {
    let path = format!("home{}user", get_path_separator());
    println!("Path: {}", path);
}
```

### Applying to Our Project

Implement `get_platform_info` using `cfg_select!`:

```rust
pub fn get_platform_info() -> &'static str {
    cfg_select! {
        target_os = "linux" => "Linux",
        target_os = "macos" => "macOS",
        target_os = "windows" => "Windows",
        _ => "Unknown",
    }
}
```

**Your task**: Replace the `todo!()` in `get_platform_info` with an implementation using `cfg_select!`.

## 7. Concept: assert_matches! for Better Tests

### Explanation

In Python, you might use multiple assertions to check pattern matching:

```python
def test_parse_response():
    result = parse_response("OK:42")
    assert isinstance(result, Success)
    assert result.code == 42
    assert result.message == "OK"
```

In Rust, you'd use `assert!(matches!(...))`:

```rust
#[test]
fn test_parse_response() {
    let result = parse_response("OK:42");
    assert!(matches!(result, ParsedResponse::Success { code: 42, .. }));
}
```

Rust 1.96 introduced `assert_matches!`, which provides better error messages:

```rust
use core::assert_matches::assert_matches;

#[test]
fn test_parse_response() {
    let result = parse_response("OK:42");
    assert_matches!(result, ParsedResponse::Success { code: 42, message: _ });
}
```

### Example

```rust
#[derive(Debug, PartialEq)]
enum Shape {
    Circle(f64),
    Rectangle(f64, f64),
    Triangle(f64, f64, f64),
}

fn classify_shape(shape: &Shape) -> &'static str {
    match shape {
        Shape::Circle(r) if *r > 10.0 => "large circle",
        Shape::Circle(_) => "small circle",
        Shape::Rectangle(w, h) if w == h => "square",
        Shape::Rectangle(_, _) => "rectangle",
        Shape::Triangle(_, _, _) => "triangle",
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use core::assert_matches::assert_matches;

    #[test]
    fn test_classify_large_circle() {
        let shape = Shape::Circle(15.0);
        assert_matches!(classify_shape(&shape), "large circle");
    }

    #[test]
    fn test_classify_square() {
        let shape = Shape::Rectangle(5.0, 5.0);
        assert_matches!(classify_shape(&shape), "square");
    }
}
```

### Applying to Our Project

The tests in `lib.rs` use `matches!` macro. Once you implement `parse_response`, you can enhance tests with `assert_matches!`:

```rust
#[test]
fn test_parse_response_success() {
    let result = parse_response("OK:42");
    assert_matches!(result, ParsedResponse::Success { code: 42, .. });
}
```

**Your task**: Implement `parse_response` and update the tests to use `assert_matches!` for clearer assertions.

## 8. Putting It All Together

Now let's implement all the functions. Replace each `todo!()` with the implementation:

```rust
// In lib.rs

pub fn sliding_sum(data: &[i32], window_size: usize) -> Vec<i32> {
    data.windows(window_size)
        .map(|w| w.iter().sum())
        .collect()
}

pub fn process_message(id: Option<i32>, content: Option<&str>) -> String {
    if let Some(id) = id
        && let Some(content) = content
    {
        format!("Message {}: {}", id, content)
    } else if id.is_none() {
        "Missing message ID".to_string()
    } else {
        "Missing message content".to_string()
    }
}

pub fn get_platform_info() -> &'static str {
    cfg_select! {
        target_os = "linux" => "Linux",
        target_os = "macos" => "macOS",
        target_os = "windows" => "Windows",
        _ => "Unknown",
    }
}

pub fn parse_response(input: &str) -> ParsedResponse {
    if let Some(code_str) = input.strip_prefix("OK:") {
        if let Ok(code) = code_str.parse::<u32>() {
            ParsedResponse::Success {
                code,
                message: "OK".to_string(),
            }
        } else {
            ParsedResponse::Error("Invalid code".to_string())
        }
    } else if let Some(msg) = input.strip_prefix("ERROR:") {
        ParsedResponse::Error(msg.to_string())
    } else {
        ParsedResponse::Error("Invalid format".to_string())
    }
}
```

Run `cargo test` to verify all 14 tests pass.

## 9. Exercises

### Easy

1. **Extend Config**: Add a `log_level` field to the `Config` struct and initialize it in `LazyLock`.

2. **Window patterns**: Write a function `find_increasing_windows(data: &[i32], window_size: usize) -> Vec<(usize, i32)>` that returns indices and sums of strictly increasing windows.

### Medium

3. **Platform-specific path**: Implement `normalize_path(path: &str) -> String` that uses `cfg_select!` to convert between Windows and Unix path separators.

4. **Complex parsing**: Extend `parse_response` to handle a third format: `"WARN:code:message"`.

### Hard

5. **Generic sliding window**: Implement a generic `SlidingWindow<T>` struct that works with any type implementing `Clone` and provides an iterator over windows.

## 10. Summary

| Concept | What You Learned | Where It's Used |
|---------|------------------|-----------------|
| `LazyLock` | Thread-safe lazy initialization without third-party crates | Global config, connection pools |
| `array_windows` | Sliding window operations with compile-time size | Data stream processing |
| `if let` chains | Complex conditional pattern matching in one expression | ETL logic, parsing |
| `cfg_select!` | Platform-specific code without repetition | Cross-platform tools |
| `assert_matches!` | Better test assertions for pattern matching | Testing pattern matching |

These modern idioms make Rust code more expressive and maintainable. Use them to replace older patterns that required workarounds or third-party dependencies.

---

**Further Reading**:
- [LazyLock documentation](https://doc.rust-lang.org/std/sync/struct.LazyLock.html)
- [array_windows RFC](https://rust-lang.github.io/rfcs/3635-cfg-select.html)
- [if let chains tracking issue](https://github.com/rust-lang/rust/issues/53667)
- [cfg_select! stabilization](https://blog.rust-lang.org/2026/04/16/Rust-1.95.0.html)
- [assert_matches! stabilization](https://blog.rust-lang.org/2026/05/28/Rust-1.96.0.html)