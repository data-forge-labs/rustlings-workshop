# Rust for Python Data Engineers — TicketV2: Enums & Error Handling

*Learn Rust's enum system and the `Result` type — the foundation of robust error handling in data pipelines.*

> **Test-driven approach**: This project includes a Cargo project with progressive unit tests. Each function in `workshop/src/lib.rs` starts as a `todo!()` stub. As you follow each section, replace `todo!()` with real code and run `cd workshop && cargo test` to watch the pass count grow. Your goal: **all 17 tests pass**.

---

## Why This Project?

### The Problem

In Python, error handling is easy to get wrong. Using a string for status invites typos and runtime crashes. Exceptions propagate implicitly — you can forget to handle them:

```python
def parse_ticket(data):
    status = data["status"]  # KeyError if missing
    if status == "Opeen":    # Typo — no error until runtime
        ...
    return {"status": status}

# Did parse_ticket succeed? Did it raise? No way to tell from the type!
result = parse_ticket(row)
# result could be a dict, or the function could have crashed
```

Python's `Optional` and exceptions are runtime-based. You don't know if a function returns `None` or raises until you run it. For data pipelines processing millions of records, a single unhandled `KeyError` or `ValueError` can crash the entire job.

Python's `Enum` class helps, but `match`/`case` (Python 3.10+) is not exhaustive — the compiler won't warn you if you forget a variant.

```
Python error handling:
  Function may: return value, return None, raise anything
  Caller must: read docs, write try/except, guess which errors
  No type-level guarantee → runtime surprises
```

### The Rust Solution

Rust makes error handling explicit and exhaustive. `Result<T, E>` encodes success or failure in the return type. `match` checks all variants at compile time. `Option<T>` forces you to handle `None`:

```rust
#[derive(Debug)]
enum Status { Open, InProgress, Resolved, Closed }

fn parse_status(s: &str) -> Result<Status, String> {
    match s {
        "Open" => Ok(Status::Open),
        "In Progress" => Ok(Status::InProgress),
        "Resolved" => Ok(Status::Resolved),
        "Closed" => Ok(Status::Closed),
        other => Err(format!("Invalid status: {other}")),
    }
}

let result = parse_status("Opeen");
match result {
    Ok(status) => println!("Got: {status:?}"),
    Err(e) => eprintln!("Error: {e}"),  // Compiler forces this match!
}
```

Every fallible function returns `Result`. Every nullable value returns `Option`. The compiler ensures you handle both cases — zero surprises at runtime.

---

## What You'll Learn

| # | Concept | Rust Type / Module | Python Equivalent | Purpose |
|---|---------|--------------------|------------------|---------|
| 1 | Enums | `enum` | `Enum` / constants | Type-safe named variants |
| 2 | match | `match` | `match` / `if/elif` | Exhaustive pattern matching |
| 3 | Enums with Data | Variant fields | Dataclass variants | Variants holding values |
| 4 | if let | `if let` | `isinstance` check | Match a single pattern concisely |
| 5 | Option | `Option<T>` | `Optional[T]` / `None` | Handle missing or optional data |
| 6 | Result | `Result<T, E>` | `try` / `except` | Recoverable errors with type info |
| 7 | Custom Error Types | Error `enum` | Custom `Exception` | Rich, structured error information |
| 8 | ? Operator | `?` | `raise` (implicit) | Propagate errors concisely |
| 9 | thiserror | `thiserror` crate | Custom exceptions | Ergonomic derive macro for error types |
| 10 | anyhow | `anyhow` crate | `raise Exception` | Simple, context-rich app-level errors |

## Concepts at a Glance

### 1. Enums
Named variants like `enum Status { Open, InProgress, Resolved, Closed }`. Python: `class Status(Enum): OPEN = 1`. Rust's enums are type-safe — `Status::Open` is a distinct type, not an integer.

### 2. match
Exhaustive pattern matching — the compiler checks ALL variants are handled. Python 3.10+ `match`/`case` is not exhaustive. Rust's `match` forces you to cover every case or use `_`.

### 3. Enums with Data
Variants can hold values: `enum Source { Csv(String), Db { host: String, port: u16 } }`. Python: dataclass subclasses or `Union` types. Rust stores the tag + data in a compact layout.

### 4. if let
Shorthand for matching one variant: `if let Status::Open = s { ... }`. Python: `if isinstance(s, StatusOpen)`. Concise when you only care about one case.

### 5. Option
`Option<T>` is `Some(T)` or `None`. Python: `Optional[T]` or `None`. Rust forces you to handle `None` via `match`, `unwrap_or`, or `?` — no more `AttributeError: 'NoneType'`.

### 6. Result
`Result<T, E>` is `Ok(T)` or `Err(E)`. Python: function either returns or raises. Rust makes the error path explicit in the return type — callers must handle both possibilities.

### 7. Custom Error Types
Rich error enums with fields: `enum TicketError { EmptyTitle, TitleTooLong { max: usize, actual: usize } }`. Python: custom `Exception` subclasses. Rust's error types are more structured and matchable.

### 8. ? Operator
`let x = func()?;` returns early on error, unwrapping on success. Python: `raise` propagates implicitly. Rust's `?` makes error propagation visible in the code.

### 9. thiserror
`#[derive(Error)]` from the `thiserror` crate auto-implements `Display` and `Error` for custom error enums. Python: defining `class MyError(Exception): pass` is similarly ergonomic.

### 10. anyhow
`anyhow::Result<T>` provides simple error handling with `with_context` for app-level code. Python: raising `Exception("message")`. `anyhow` is for applications; `thiserror` is for libraries.

---

## Table of Contents

1. [Project Overview](#1-project-overview)
2. [Concept: Enums — Better Than Booleans](#2-concept-enums--better-than-booleans)
3. [Concept: `match` — Exhaustive Pattern Matching](#3-concept-match--exhaustive-pattern-matching)
4. [Concept: Enums with Data](#4-concept-enums-with-data)
5. [Concept: `if let` — When You Only Care About One Variant](#5-concept-if-let--when-you-only-care-about-one-variant)
6. [Concept: `Option<T>` — Handling Missing Data](#6-concept-optiont--handling-missing-data)
7. [Concept: `Result<T, E>` — Recoverable Errors](#7-concept-resultt-e--recoverable-errors)
8. [Concept: Custom Error Types](#8-concept-custom-error-types)
9. [Concept: `?` Operator — Error Propagation](#9-concept--operator--error-propagation)
10. [Concept: `thiserror` and `anyhow` Crates](#10-concept-thiserror-and-anyhow-crates)
11. [Putting It All Together](#11-putting-it-all-together)
12. [Summary](#12-summary)
13. [Appendix: Original Step-by-Step Tutorial](#13-appendix-original-step-by-step-tutorial)

---

## 1. Project Overview

We'll enhance our ticket system with:

- A proper `Status` enum (not a `String`)
- Error handling with `Result` instead of `panic!`
- Custom error types for validation failures
- The `?` operator for clean error propagation

### What You'll Learn

| Concept | Python Equivalent | Why It Matters |
|---|---|---|
| `enum` | `Enum` class / constants | Type-safe status, states |
| `match` | `match` / `if/elif` | Exhaustive pattern matching |
| `Option<T>` | `None` / `Optional` | Missing data handling |
| `Result<T, E>` | `try/except` | Recoverable errors |
| `?` operator | `try` / exception propagation | Clean error flow |
| `thiserror` | Custom exceptions | Ergonomic error types |

---

## 2. Concept: Enums — Better Than Booleans

### The Problem with Booleans

```python
# Python — boolean for status
ticket = {"status": True}  # True = open? True = closed?
# What does this mean?
```

### Rust Enums

```rust
enum Status {
    Open,
    InProgress,
    Resolved,
    Closed,
}
```

### Memory Layout

```
enum Status { Open, InProgress, Resolved, Closed }

Each variant is a DISCRIMINANT (integer tag):
┌────────┐
│ tag: 0 │  ← Open
├────────┤
│ tag: 1 │  ← InProgress
├────────┤
│ tag: 2 │  ← Resolved
├────────┤
│ tag: 3 │  ← Closed
└────────┘

Total size: 1 byte (or more, depending on alignment)
```

### Why Enums Over Strings?

```python
# Python — runtime errors
status = "Opeen"  # Typo, no error until runtime
if status == "Open":
    process()
```

```rust
// Rust — compile-time guarantee
let status = Status::Opeen;  // ❌ Compiler: "no variant named `Opeen`"
let status = Status::Open;   // ✅

match status {
    Status::Open => process(),  // Compiler checks ALL variants covered
    // Forgot InProgress, Resolved, Closed? ❌ Compiler error!
}
```

### Python vs Rust Enums

```python
# Python 3.4+ Enum
from enum import Enum

class Status(Enum):
    OPEN = 1
    IN_PROGRESS = 2
    RESOLVED = 3
    CLOSED = 4

# Still not exhaustively checked
if status == Status.OPEN:  # No warning if you forget others
    ...
```

```rust
// Rust — match is EXHAUSTIVE
match status {
    Status::Open => println!("Open"),
    Status::InProgress => println!("Working"),
    Status::Resolved => println!("Done"),
    Status::Closed => println!("Closed"),
    // No default needed — all variants covered
}
```

---

## 3. Concept: `match` — Exhaustive Pattern Matching

### Basic `match`

```rust
let status = Status::Resolved;

match status {
    Status::Open => println!("Ticket is open"),
    Status::InProgress => println!("Working on it"),
    Status::Resolved => println!("Done!"),
    Status::Closed => println!("Closed"),
}
```

### `match` Returns a Value (Expression)

```rust
let msg = match status {
    Status::Open => "Ticket is open",
    Status::InProgress => "Working on it",
    Status::Resolved => "Done!",
    Status::Closed => "Closed",
};
println!("{msg}");
```

### Catch-All with `_` (Wildcard)

```rust
let msg = match status {
    Status::Open => "Ticket is open",
    _ => "Not open",  // Matches everything else
};
```

> Use `_` sparingly — it bypasses exhaustiveness checking. Prefer listing all variants.

---

## 4. Concept: Enums with Data

### Variants Can Hold Values

```rust
enum TicketAction {
    Create { title: String, description: String },
    UpdateStatus(Status),
    AddComment(String),
    Close,
}
```

### Memory Layout

```
enum TicketAction {
    Create { title: String, desc: String },
    UpdateStatus(Status),
    AddComment(String),
    Close,
}

┌──────────────────────────────┐
│ tag: 0 │ String │ String     │ ← Create
├──────────────────────────────┤
│ tag: 1 │ Status │ (unused)   │ ← UpdateStatus
├──────────────────────────────┤
│ tag: 2 │ String │ (unused)   │ ← AddComment
├──────────────────────────────┤
│ tag: 3 │ (unused) │ (unused) │ ← Close
└──────────────────────────────┘
Size = tag (1 byte) + largest variant's data + padding
```

### Matching on Enums with Data

```rust
match action {
    TicketAction::Create { title, description } => {
        println!("Creating: {}", title);
    }
    TicketAction::UpdateStatus(new_status) => {
        println!("Updating status to {:?}", new_status);
    }
    TicketAction::AddComment(text) => {
        println!("Adding comment: {}", text);
    }
    TicketAction::Close => {
        println!("Closing ticket");
    }
}
```

### Data Engineering Example

```rust
#[derive(Debug)]
enum DataSource {
    CsvFile(String),           // Path
    Database { host: String, port: u16, db: String },
    ApiEndpoint(String),       // URL
    InlineData(Vec<Vec<f64>>), // Direct data
}

fn connect(source: DataSource) {
    match source {
        DataSource::CsvFile(path) => {
            println!("Reading CSV: {}", path);
        }
        DataSource::Database { host, port, db } => {
            println!("Connecting to {}:{} on {}", host, port, db);
        }
        DataSource::ApiEndpoint(url) => {
            println!("Fetching: {}", url);
        }
        DataSource::InlineData(data) => {
            println!("Processing {} inline rows", data.len());
        }
    }
}
```

---

## 5. Concept: `if let` — When You Only Care About One Variant

```rust
// Instead of:
match status {
    Status::Open => println!("Opening ticket"),
    _ => {},  // Ungainly empty arm
}

// Use:
if let Status::Open = status {
    println!("Opening ticket");
}
```

### With Data

```rust
let action = TicketAction::AddComment(String::from("Looks good"));

if let TicketAction::AddComment(text) = &action {
    println!("Comment: {}", text);  // Only runs for AddComment
}
```

### `while let` — Loop Until Pattern Doesn't Match

```rust
let mut stack = vec![1, 2, 3];

while let Some(top) = stack.pop() {
    println!("Popped: {}", top);
}
// Prints: 3, 2, 1
```

---

## 6. Concept: `Option<T>` — Handling Missing Data

### No More `None` Crashes

```python
# Python — AttributeError at 3 AM
result = get_user(id)  # Returns None
print(result.name)     # 💥 AttributeError: 'NoneType' object has no attribute 'name'
```

```rust
// Rust — compiler forces you to handle None
fn find_user(id: u64) -> Option<User> {
    if id == 0 {
        None
    } else {
        Some(User { id, name: String::from("Alice") })
    }
}

let user = find_user(42);
match user {
    Some(u) => println!("Found: {}", u.name),
    None => println!("User not found"),  // Compiler forces this!
}
// Cannot access user.name without handling None
```

### `Option<T>` Methods

```rust
let x: Option<i32> = Some(5);

// Unwrap with fallback
let val = x.unwrap_or(0);        // 5
let val = x.unwrap_or_else(|| calculate_default());

// Transform
let doubled = x.map(|n| n * 2);  // Some(10)

// Chain
let result = x.and_then(|n| if n > 0 { Some(n * 2) } else { None });

// Filter
let positive = x.filter(|&n| n > 0);  // Some(5) if x is Some(5)

// Default if None
let y: Option<i32> = None;
let val = y.unwrap_or(0);  // 0
```

### Python vs Option

| Python | Rust Option |
|---|---|
| `value = func() or default` | `func().unwrap_or(default)` |
| `if value is not None:` | `if let Some(v) = value` |
| `value = func()` (might be None) | Returns `Option<T>` |
| `value.field` (crash if None) | `match value { Some(v) => v.field, None => ... }` |

---

## 7. Concept: `Result<T, E>` — Recoverable Errors

### `panic!` vs `Result`

So far we've used `panic!` for errors. But panics crash the program. For recoverable errors, use `Result`:

```rust
enum Result<T, E> {
    Ok(T),   // Success
    Err(E),  // Failure with error info
}
```

### From `panic!` to `Result`

```rust
// BAD: panics on error
fn parse_int(s: &str) -> i32 {
    match s.parse::<i32>() {
        Ok(n) => n,
        Err(_) => panic!("Invalid number: {}", s),
    }
}

// GOOD: returns Result — caller decides what to do
fn parse_int(s: &str) -> Result<i32, String> {
    match s.parse::<i32>() {
        Ok(n) => Ok(n),
        Err(_) => Err(format!("Invalid number: {}", s)),
    }
}

// Caller can:
match parse_int("42") {
    Ok(n) => println!("Got: {}", n),
    Err(e) => eprintln!("Error: {}", e),
}
```

### Using `Result` in the Ticket System

```rust
pub fn new(title: String, description: String, status: String) -> Result<Ticket, String> {
    if title.is_empty() {
        return Err(String::from("Title cannot be empty"));
    }
    if title.len() > 50 {
        return Err(String::from("Title too long (max 50 chars)"));
    }
    // ... more validation ...

    Ok(Ticket { title, description, status })
}
```

### Python vs Rust Error Handling

```python
# Python — try/except
def parse_csv_line(line: str) -> list[str]:
    try:
        return line.split(",")
    except Exception as e:
        raise ValueError(f"Failed to parse: {line}") from e
```

```rust
// Rust — Result
fn parse_csv_line(line: &str) -> Result<Vec<&str>, String> {
    if line.is_empty() {
        return Err("Empty line".to_string());
    }
    Ok(line.split(',').collect())
}
```

---

## 8. Concept: Custom Error Types

### Why Custom Errors?

```rust
// BAD: String errors lose information
fn validate_ticket(title: &str) -> Result<(), String> {
    // What kind of error? No way to distinguish!
}

// GOOD: Custom error type
#[derive(Debug)]
enum TicketError {
    EmptyTitle,
    TitleTooLong(usize),  // Contains the actual length
    InvalidStatus(String),
    EmptyDescription,
    DescriptionTooLong(usize),
}
```

### Implementing `Display` and `Error`

```rust
use std::fmt;

#[derive(Debug)]
enum TicketError {
    EmptyTitle,
    TitleTooLong { max: usize, actual: usize },
    InvalidStatus(String),
}

impl fmt::Display for TicketError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TicketError::EmptyTitle => write!(f, "Title cannot be empty"),
            TicketError::TitleTooLong { max, actual } => {
                write!(f, "Title too long: max {max}, got {actual}")
            }
            TicketError::InvalidStatus(s) => {
                write!(f, "Invalid status: '{}'", s)
            }
        }
    }
}

// Required for the `?` operator to work
impl std::error::Error for TicketError {}
```

### Python vs Rust Custom Errors

```python
# Python
class TicketError(Exception):
    pass

class EmptyTitleError(TicketError):
    pass

class TitleTooLongError(TicketError):
    def __init__(self, max_len, actual_len):
        self.max_len = max_len
        self.actual_len = actual_len
```

```rust
// Rust
#[derive(Debug)]
enum TicketError {
    EmptyTitle,
    TitleTooLong { max: usize, actual: usize },
}
```

---

## 9. Concept: `?` Operator — Error Propagation

### The Problem: Nested Match Hell

```rust
fn process_file(path: &str) -> Result<String, String> {
    let content = match std::fs::read_to_string(path) {
        Ok(c) => c,
        Err(e) => return Err(format!("Read error: {}", e)),
    };
    let parsed = match parse_csv(&content) {
        Ok(p) => p,
        Err(e) => return Err(format!("Parse error: {}", e)),
    };
    let result = match analyze(&parsed) {
        Ok(r) => r,
        Err(e) => return Err(format!("Analysis error: {}", e)),
    };
    Ok(result)
}
```

### The Solution: `?` Operator

```rust
fn process_file(path: &str) -> Result<String, String> {
    let content = std::fs::read_to_string(path)
        .map_err(|e| format!("Read error: {}", e))?;

    let parsed = parse_csv(&content)
        .map_err(|e| format!("Parse error: {}", e))?;

    let result = analyze(&parsed)
        .map_err(|e| format!("Analysis error: {}", e))?;

    Ok(result)
}
```

### How `?` Works

```
let x = func()?;
// is equivalent to:
let x = match func() {
    Ok(val) => val,      // Unwrap the Ok value
    Err(e) => return Err(e.into()),  // Convert error and return early
};
```

### Flow Diagram

```
func() returns Result<T, E>
         │
         ▼
    ┌─────────┐
    │  Ok(t)   │──→ x = t, continue
    └─────────┘
         │
    ┌─────────┐
    │ Err(e)  │──→ return Err(e.into()) from current function
    └─────────┘
```

### For Python Data Engineers

```python
# Python — exceptions propagate automatically
def load_and_process(path):
    data = pd.read_csv(path)     # FileNotFoundError propagates
    cleaned = data.dropna()      # Any error propagates
    result = cleaned.mean()      # Any error propagates
    return result
```

```rust
// Rust — ? propagates errors explicitly
fn load_and_process(path: &str) -> Result<DataFrame, Box<dyn std::error::Error>> {
    let data = DataFrame::from_csv(path)?;  // Error propagates
    let cleaned = data.dropna()?;
    let result = cleaned.mean()?;
    Ok(result)
}
```

### Using `Box<dyn Error>` for Simplicity

```rust
use std::error::Error;

fn flexible_func() -> Result<Value, Box<dyn Error>> {
    let x = std::fs::read_to_string("file.txt")?;  // io::Error → Box<dyn Error>
    let y = "42".parse::<i32>()?;                  // ParseIntError → Box<dyn Error>
    Ok(Value::from(y))
}
```

---

## 10. Concept: `thiserror` and `anyhow` Crates

### `thiserror` — Define Custom Errors Easily

```toml
# Cargo.toml
[dependencies]
thiserror = "1"
```

```rust
use thiserror::Error;

#[derive(Error, Debug)]
pub enum TicketError {
    #[error("Title cannot be empty")]
    EmptyTitle,

    #[error("Title too long: max {max}, got {actual}")]
    TitleTooLong { max: usize, actual: usize },

    #[error("Invalid status: '{0}'")]
    InvalidStatus(String),
}

// That's it! Display and Error are auto-implemented
```

### `anyhow` — Simple Error Handling in Apps

```toml
[dependencies]
anyhow = "1"
```

```rust
use anyhow::{anyhow, Result, Context};

fn load_file(path: &str) -> Result<String> {
    let content = std::fs::read_to_string(path)
        .with_context(|| format!("Failed to read {}", path))?;

    if content.is_empty() {
        return Err(anyhow!("File is empty: {}", path));
    }

    Ok(content)
}

fn main() -> Result<()> {
    let data = load_file("data.csv")?;
    println!("Loaded {} bytes", data.len());
    Ok(())
}
```

| Crate | Use Case |
|---|---|
| `thiserror` | **Libraries** — define rich custom error types |
| `anyhow` | **Applications** — simple error handling with context |
| `Box<dyn Error>` | Quick prototyping |
| Custom `enum` | Full control, no dependencies |

---

## 11. Putting It All Together

### Complete Ticket System with Result-Based Error Handling

```toml
# Cargo.toml
[package]
name = "ticket-v2"
version = "0.1.0"
edition = "2021"

[dependencies]
thiserror = "1"
```

```rust
use std::fmt;
use thiserror::Error;

// === Custom Error Type ===
#[derive(Error, Debug)]
pub enum TicketError {
    #[error("Title cannot be empty")]
    EmptyTitle,

    #[error("Title too long (max {max}, got {actual})")]
    TitleTooLong { max: usize, actual: usize },

    #[error("Description cannot be empty")]
    EmptyDescription,

    #[error("Description too long (max {max}, got {actual})")]
    DescriptionTooLong { max: usize, actual: usize },

    #[error("Invalid status: '{0}'")]
    InvalidStatus(String),
}

// === Status Enum ===
#[derive(Debug, Clone, PartialEq)]
pub enum Status {
    Open,
    InProgress,
    Resolved,
    Closed,
}

impl Status {
    pub fn from_str(s: &str) -> Result<Status, TicketError> {
        match s {
            "Open" => Ok(Status::Open),
            "In Progress" => Ok(Status::InProgress),
            "Resolved" => Ok(Status::Resolved),
            "Closed" => Ok(Status::Closed),
            other => Err(TicketError::InvalidStatus(other.to_string())),
        }
    }
}

impl fmt::Display for Status {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Status::Open => write!(f, "Open"),
            Status::InProgress => write!(f, "In Progress"),
            Status::Resolved => write!(f, "Resolved"),
            Status::Closed => write!(f, "Closed"),
        }
    }
}

// === Ticket Struct ===
#[derive(Debug, Clone)]
pub struct Ticket {
    title: String,
    description: String,
    status: Status,
}

impl Ticket {
    pub fn new(title: String, description: String, status: Status) -> Result<Ticket, TicketError> {
        // Validate title
        if title.is_empty() {
            return Err(TicketError::EmptyTitle);
        }
        if title.len() > 50 {
            return Err(TicketError::TitleTooLong { max: 50, actual: title.len() });
        }

        // Validate description
        if description.is_empty() {
            return Err(TicketError::EmptyDescription);
        }
        if description.len() > 500 {
            return Err(TicketError::DescriptionTooLong { max: 500, actual: description.len() });
        }

        Ok(Ticket { title, description, status })
    }

    pub fn title(&self) -> &str { &self.title }
    pub fn description(&self) -> &str { &self.description }
    pub fn status(&self) -> &Status { &self.status }

    pub fn set_status(&mut self, status: Status) {
        self.status = status;
    }
}

impl fmt::Display for Ticket {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}] {}", self.status, self.title)
    }
}

// === Main ===
fn main() -> Result<(), TicketError> {
    // Creating with Result — error must be handled!
    let ticket = Ticket::new(
        String::from("Fix login bug"),
        String::from("SSO login is broken"),
        Status::from_str("Open")?,
    )?;  // ? propagates errors

    println!("Created: {ticket}");
    println!("Debug: {ticket:#?}");

    // Error case — won't panic!
    match Ticket::new(
        String::from(""),
        String::from("Desc"),
        Status::Open,
    ) {
        Ok(_) => println!("Created?"),
        Err(e) => println!("Error: {e}"),  // "Error: Title cannot be empty"
    }

    Ok(())
}
```

---

## 12. Summary

### Concept Reference

| Concept | Description | Python Equivalent |
|---|---|---|
| `enum` | Type with multiple named variants | `Enum` / constants |
| `match` | Exhaustive pattern matching | `match` / `if/elif` |
| Enum with data | Variants that hold values | Dataclass variants |
| `if let` | Match single pattern | `isinstance` check |
| `Option<T>` | Value or `None` | `Optional[T]` / `None` |
| `Result<T, E>` | Value or error | Exception / `try` |
| `?` operator | Propagate error to caller | `raise` (implicit) |
| `thiserror` | Derive custom errors | Custom exception class |
| `anyhow` | Simple app-level errors | `raise Exception` |

### Error Handling Strategy

```
┌──────────────────────────────────────────┐
│                Operation                  │
│     ╱              ╲                     │
│    ╱                ╲                    │
│   ✅ Success         ❌ Failure          │
│   │                  │                   │
│   ▼                  ▼                   │
│  Use value        Recoverable?           │
│                    ╱       ╲             │
│                   ╱         ╲            │
│                Yes            No         │
│                │               │         │
│                ▼               ▼         │
│           Result<T,E>       panic!       │
│           (handle it)     (stop program) │
└──────────────────────────────────────────┘
```

### Further Reading

The following lesson files in this folder provide deeper dives into each concept:

| File | Topics |
|------|--------|
| [00_intro.md](./00_intro.md) | Project introduction |
| [01_enum.md](./01_enum.md) | `enum` definition, variants, discriminant |
| [02_match.md](./02_match.md) | Exhaustive `match`, patterns, wildcard `_` |
| [03_variants_with_data.md](./03_variants_with_data.md) | Enums with tuple and struct variants |
| [04_if_let.md](./04_if_let.md) | `if let` and `while let` patterns |
| [05_nullability.md](./05_nullability.md) | `Option<T>`, `unwrap`, `map`, `and_then` |
| [06_fallibility.md](./06_fallibility.md) | `Result<T, E>`, recoverable errors |
| [07_unwrap.md](./07_unwrap.md) | `unwrap`, `expect`, `unwrap_or` |
| [08_error_enums.md](./08_error_enums.md) | Custom error enums |
| [09_error_trait.md](./09_error_trait.md) | `std::error::Error` trait |
| [10_packages.md](./10_packages.md) | Cargo packages, workspace structure |
| [11_dependencies.md](./11_dependencies.md) | Adding and managing dependencies |
| [12_thiserror.md](./12_thiserror.md) | `thiserror` derive macro |
| [13_try_from.md](./13_try_from.md) | `TryFrom` / `TryInto` for fallible conversion |
| [14_source.md](./14_source.md) | Error source chains |
| [15_outro.md](./15_outro.md) | Section wrap-up |

### Next Project

Proceed to [6-TicketManagement](../03-Collections/01-TicketManagement/README.md) for **collections, iterators, and HashMap** — essential tools for data pipelines.

## Appendix: Original Step-by-Step Tutorial

### Appendix 1: Introduction

# Modelling A Ticket, pt. 2

The `Ticket` struct we worked on in the previous chapters is a good start,
but it still screams "I'm a beginner Rustacean!".

We'll use this chapter to refine our Rust domain modelling skills.
We'll need to introduce a few more concepts along the way:

- `enum`s, one of Rust's most powerful features for data modeling
- The `Option` type, to model nullable values
- The `Result` type, to model recoverable errors
- The `Debug` and `Display` traits, for printing
- The `Error` trait, to mark error types
- The `TryFrom` and `TryInto` traits, for fallible conversions
- Rust's package system, explaining what's a library, what's a binary, how to use third-party crates

### Appendix 2: Enumerations

# Enumerations

Based on the validation logic you wrote [in a previous chapter](../03_ticket_v1/02_validation.md),
there are only a few valid statuses for a ticket: `To-Do`, `InProgress` and `Done`.\
This is not obvious if we look at the `status` field in the `Ticket` struct or at the type of the `status`
parameter in the `new` method:

```rust
#[derive(Debug, PartialEq)]
pub struct Ticket {
    title: String,
    description: String,
    status: String,
}

impl Ticket {
    pub fn new(
        title: String, 
        description: String, 
        status: String
    ) -> Self {
        // [...]
    }
}
```

In both cases we're using `String` to represent the `status` field.
`String` is a very general type—it doesn't immediately convey the information that the `status` field
has a limited set of possible values. Even worse, the caller of `Ticket::new` will only find out **at runtime**
if the status they provided is valid or not.

We can do better than that with **enumerations**.

## `enum`

An enumeration is a type that can have a fixed set of values, called **variants**.\
In Rust, you define an enumeration using the `enum` keyword:

```rust
enum Status {
    ToDo,
    InProgress,
    Done,
}
```

`enum`, just like `struct`, defines **a new Rust type**.

### Appendix 3: `match`

# `match`

You may be wondering—what can you actually **do** with an enum?\
The most common operation is to **match** on it.

```rust
enum Status {
    ToDo,
    InProgress,
    Done
}

impl Status {
    fn is_done(&self) -> bool {
        match self {
            Status::Done => true,
            // The `|` operator lets you match multiple patterns.
            // It reads as "either `Status::ToDo` or `Status::InProgress`".
            Status::InProgress | Status::ToDo => false
        }
    }
}
```

A `match` statement that lets you compare a Rust value against a series of **patterns**.\
You can think of it as a type-level `if`. If `status` is a `Done` variant, execute the first block;
if it's a `InProgress` or `ToDo` variant, execute the second block.

## Exhaustiveness

There's one key detail here: `match` is **exhaustive**. You must handle all enum variants.\
If you forget to handle a variant, Rust will stop you **at compile-time** with an error.

E.g. if we forget to handle the `ToDo` variant:

```rust
match self {
    Status::Done => true,
    Status::InProgress => false,
}
```

the compiler will complain:

```text
error[E0004]: non-exhaustive patterns: `ToDo` not covered
 --> src/main.rs:5:9
  |
5 |     match status {
  |     ^^^^^^^^^^^^ pattern `ToDo` not covered
```

This is a big deal!\
Codebases evolve over time—you might add a new status down the line, e.g. `Blocked`. The Rust compiler
will emit an error for every single `match` statement that's missing logic for the new variant.
That's why Rust developers often sing the praises of "compiler-driven refactoring"—the compiler tells you
what to do next, you just have to fix what it reports.

## Catch-all

If you don't care about one or more variants, you can use the `_` pattern as a catch-all:

```rust
match status {
    Status::Done => true,
    _ => false
}
```

The `_` pattern matches anything that wasn't matched by the previous patterns.

<div class="warning">
By using this catch-all pattern, you _won't_ get the benefits of compiler-driven refactoring.\
If you add a new enum variant, the compiler _won't_ tell you that you're not handling it.

If you're keen on correctness, avoid using catch-alls. Leverage the compiler to re-examine all matching sites and determine how new enum variants should be handled.

</div>

### Appendix 4: Variants Can Hold Data

# Variants can hold data

```rust
enum Status {
    ToDo,
    InProgress,
    Done,
}
```

Our `Status` enum is what's usually called a **C-style enum**.\
Each variant is a simple label, a bit like a named constant. You can find this kind of enum in many programming
languages, like C, C++, Java, C#, Python, etc.

Rust enums can go further though. We can **attach data to each variant**.

## Variants

Let's say that we want to store the name of the person who's currently working on a ticket.\
We would only have this information if the ticket is in progress. It wouldn't be there for a to-do ticket or
a done ticket.
We can model this by attaching a `String` field to the `InProgress` variant:

```rust
enum Status {
    ToDo,
    InProgress {
        assigned_to: String,
    },
    Done,
}
```

`InProgress` is now a **struct-like variant**.\
The syntax mirrors, in fact, the one we used to define a struct—it's just "inlined" inside the enum, as a variant.

## Accessing variant data

If we try to access `assigned_to` on a `Status` instance,

```rust
let status: Status = /* */;

// This won't compile
println!("Assigned to: {}", status.assigned_to);
```

the compiler will stop us:

```text
error[E0609]: no field `assigned_to` on type `Status`
 --> src/main.rs:5:40
  |
5 |     println!("Assigned to: {}", status.assigned_to);
  |                                        ^^^^^^^^^^^ unknown field
```

`assigned_to` is **variant-specific**, it's not available on all `Status` instances.\
To access `assigned_to`, we need to use **pattern matching**:

```rust
match status {
    Status::InProgress { assigned_to } => {
        println!("Assigned to: {}", assigned_to);
    },
    Status::ToDo | Status::Done => {
        println!("ToDo or Done");
    }
}
```

## Bindings

In the match pattern `Status::InProgress { assigned_to }`, `assigned_to` is a **binding**.\
We're **destructuring** the `Status::InProgress` variant and binding the `assigned_to` field to
a new variable, also named `assigned_to`.\
If we wanted, we could bind the field to a different variable name:

```rust
match status {
    Status::InProgress { assigned_to: person } => {
        println!("Assigned to: {}", person);
    },
    Status::ToDo | Status::Done => {
        println!("ToDo or Done");
    }
}
```

### Appendix 5: Concise Branching

# Concise branching

Your solution to the previous exercise probably looks like this:

```rust
impl Ticket {
    pub fn assigned_to(&self) -> &str {
        match &self.status {
            Status::InProgress { assigned_to } => assigned_to,
            Status::Done | Status::ToDo => {
                panic!(
                    "Only `In-Progress` tickets can be \
                    assigned to someone"
                )
            }
        }
    }
}
```

You only care about the `Status::InProgress` variant.
Do you really need to match on all the other variants?

New constructs to the rescue!

## `if let`

The `if let` construct allows you to match on a single variant of an enum,
without having to handle all the other variants.

Here's how you can use `if let` to simplify the `assigned_to` method:

```rust
impl Ticket {
    pub fn assigned_to(&self) -> &str {
        if let Status::InProgress { assigned_to } = &self.status {
            assigned_to
        } else {
            panic!(
                "Only `In-Progress` tickets can be assigned to someone"
            );
        }
    }
}
```

## `let/else`

If the `else` branch is meant to return early (a panic counts as returning early!),
you can use the `let/else` construct:

```rust
impl Ticket {
    pub fn assigned_to(&self) -> &str {
        let Status::InProgress { assigned_to } = &self.status else {
            panic!(
                "Only `In-Progress` tickets can be assigned to someone"
            );
        };
        assigned_to
    }
}
```

It allows you to assign the destructured variable without incurring
any "right drift", i.e. the variable is assigned at the same indentation level
as the code that precedes it.

## Style

Both `if let` and `let/else` are idiomatic Rust constructs.\
Use them as you see fit to improve the readability of your code,
but don't overdo it: `match` is always there when you need it.

### Appendix 6: Nullability

# Nullability

Our implementation of the `assigned` method is fairly blunt: panicking for to-do and done tickets is far from ideal.\
We can do better using **Rust's `Option` type**.

## `Option`

`Option` is a Rust type that represents **nullable values**.\
It is an enum, defined in Rust's standard library:

```rust
enum Option<T> {
    Some(T),
    None,
}
```

`Option` encodes the idea that a value might be present (`Some(T)`) or absent (`None`).\
It also forces you to **explicitly handle both cases**. You'll get a compiler error if you are working with
a nullable value and you forget to handle the `None` case.\
This is a significant improvement over "implicit" nullability in other languages, where you can forget to check
for `null` and thus trigger a runtime error.

## `Option`'s definition

`Option`'s definition uses a Rust construct that you haven't seen before: **tuple-like variants**.

### Tuple-like variants

`Option` has two variants: `Some(T)` and `None`.\
`Some` is a **tuple-like variant**: it's a variant that holds **unnamed fields**.

Tuple-like variants are often used when there is a single field to store, especially when we're looking at a
"wrapper" type like `Option`.

### Tuple-like structs

They're not specific to enums—you can define tuple-like structs too:

```rust
struct Point(i32, i32);
```

You can then access the two fields of a `Point` instance using their positional index:

```rust
let point = Point(3, 4);
let x = point.0;
let y = point.1;
```

### Tuples

It's weird to say that something is tuple-like when we haven't seen tuples yet!\
Tuples are another example of a primitive Rust type.
They group together a fixed number of values with (potentially different) types:

```rust
// Two values, same type
let first: (i32, i32) = (3, 4);
// Three values, different types
let second: (i32, u32, u8) = (-42, 3, 8);
```

The syntax is simple: you list the types of the values between parentheses, separated by commas.
You can access the fields of a tuple using the dot notation and the field index:

```rust
assert_eq!(second.0, -42);
assert_eq!(second.1, 3);
assert_eq!(second.2, 8);
```

Tuples are a convenient way of grouping values together when you can't be bothered to define a dedicated struct type.

### Appendix 7: Fallibility

# Fallibility

Let's revisit the `Ticket::new` function from the previous exercise:

```rust
impl Ticket {
    pub fn new(
        title: String, 
        description: String, 
        status: Status
    ) -> Ticket {
        if title.is_empty() {
            panic!("Title cannot be empty");
        }
        if title.len() > 50 {
            panic!("Title cannot be longer than 50 bytes");
        }
        if description.is_empty() {
            panic!("Description cannot be empty");
        }
        if description.len() > 500 {
            panic!("Description cannot be longer than 500 bytes");
        }

        Ticket {
            title,
            description,
            status,
        }
    }
}
```

As soon as one of the checks fails, the function panics.
This is not ideal, as it doesn't give the caller a chance to **handle the error**.

It's time to introduce the `Result` type, Rust's primary mechanism for error handling.

## The `Result` type

The `Result` type is an enum defined in the standard library:

```rust
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

It has two variants:

- `Ok(T)`: represents a successful operation. It holds `T`, the output of the operation.
- `Err(E)`: represents a failed operation. It holds `E`, the error that occurred.

Both `Ok` and `Err` are generic, allowing you to specify your own types for the success and error cases.

## No exceptions

Recoverable errors in Rust are **represented as values**.\
They're just an instance of a type, being passed around and manipulated like any other value.
This is a significant difference from other languages, such as Python or C#, where **exceptions** are used to signal errors.

Exceptions create a separate control flow path that can be hard to reason about.\
You don't know, just by looking at a function's signature, if it can throw an exception or not.
You don't know, just by looking at a function's signature, **which** exception types it can throw.\
You must either read the function's documentation or look at its implementation to find out.

Exception handling logic has very poor locality: the code that throws the exception is far removed from the code
that catches it, and there's no direct link between the two.

## Fallibility is encoded in the type system

Rust, with `Result`, forces you to **encode fallibility in the function's signature**.\
If a function can fail (and you want the caller to have a shot at handling the error), it must return a `Result`.

```rust
// Just by looking at the signature, you know that this function 
// can fail. You can also inspect `ParseIntError` to see what 
// kind of failures to expect.
fn parse_int(s: &str) -> Result<i32, ParseIntError> {
    // ...
}
```

That's the big advantage of `Result`: it makes fallibility explicit.

Keep in mind, though, that panics exist. They aren't tracked by the type system, just like exceptions in other languages.
But they're meant for **unrecoverable errors** and should be used sparingly.

### Appendix 8: Unwrapping

# Unwrapping

`Ticket::new` now returns a `Result` instead of panicking on invalid inputs.\
What does this mean for the caller?

## Failures can't be (implicitly) ignored

Unlike exceptions, Rust's `Result` forces you to **handle errors at the call site**.\
If you call a function that returns a `Result`, Rust won't allow you to implicitly ignore the error case.

```rust
fn parse_int(s: &str) -> Result<i32, ParseIntError> {
    // ...
}

// This won't compile: we're not handling the error case.
// We must either use `match` or one of the combinators provided by 
// `Result` to "unwrap" the success value or handle the error.
let number = parse_int("42") + 2;
```

## You got a `Result`. Now what?

When you call a function that returns a `Result`, you have two key options:

- Panic if the operation failed.
  This is done using either the `unwrap` or `expect` methods.
  ```rust
  // Panics if `parse_int` returns an `Err`.
  let number = parse_int("42").unwrap();
  // `expect` lets you specify a custom panic message.
  let number = parse_int("42").expect("Failed to parse integer");
  ```
- Destructure the `Result` using a `match` expression to deal with the error case explicitly.
  ```rust
  match parse_int("42") {
      Ok(number) => println!("Parsed number: {}", number),
      Err(err) => eprintln!("Error: {}", err),
  }
  ```

### Appendix 9: Error Enums

# Error enums

Your solution to the previous exercise may have felt awkward: matching on strings is not ideal!\
A colleague might rework the error messages returned by `Ticket::new` (e.g. to improve readability) and,
all of a sudden, your calling code would break.

You already know the machinery required to fix this: enums!

## Reacting to errors

When you want to allow the caller to behave differently based on the specific error that occurred, you can
use an enum to represent the different error cases:

```rust
// An error enum to represent the different error cases
// that may occur when parsing a `u32` from a string.
enum U32ParseError {
    NotANumber,
    TooLarge,
    Negative,
}
```

Using an error enum, you're encoding the different error cases in the type system—they become part of the
signature of the fallible function.\
This simplifies error handling for the caller, as they can use a `match` expression to react to the different
error cases:

```rust
match s.parse_u32() {
    Ok(n) => n,
    Err(U32ParseError::Negative) => 0,
    Err(U32ParseError::TooLarge) => u32::MAX,
    Err(U32ParseError::NotANumber) => {
        panic!("Not a number: {}", s);
    }
}
```

### Appendix 10: Error Trait

# Error trait

## Error reporting

In the previous exercise you had to destructure the `TitleError` variant to extract the error message and
pass it to the `panic!` macro.\
This is a (rudimentary) example of **error reporting**: transforming an error type into a representation that can be
shown to a user, a service operator, or a developer.

It's not practical for each Rust developer to come up with their own error reporting strategy: it'd be a waste of time
and it wouldn't compose well across projects.
That's why Rust provides the `std::error::Error` trait.

## The `Error` trait

There are no constraints on the type of the `Err` variant in a `Result`, but it's a good practice to use a type
that implements the `Error` trait.
`Error` is the cornerstone of Rust's error handling story:

```rust
// Slightly simplified definition of the `Error` trait
pub trait Error: Debug + Display {}
```

You might recall the `:` syntax from [the `From` trait](../04_traits/09_from.md#supertrait--subtrait)—it's used to specify **supertraits**.
For `Error`, there are two supertraits: `Debug` and `Display`. If a type wants to implement `Error`, it must also
implement `Debug` and `Display`.

## `Display` and `Debug`

We've already encountered the `Debug` trait in [a previous exercise](../04_traits/04_derive.md)—it's the trait used by
`assert_eq!` to display the values of the variables it's comparing when the assertion fails.

From a "mechanical" perspective, `Display` and `Debug` are identical—they encode how a type should be converted
into a string-like representation:

```rust
// `Debug`
pub trait Debug {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error>;
}

// `Display`
pub trait Display {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error>;
}
```

The difference is in their _purpose_: `Display` returns a representation that's meant for "end-users",
while `Debug` provides a low-level representation that's more suitable to developers and service operators.\
That's why `Debug` can be automatically implemented using the `#[derive(Debug)]` attribute, while `Display`
**requires** a manual implementation.

### Appendix 11: Libraries and Binaries

# Libraries and binaries

It took a bit of code to implement the `Error` trait for `TicketNewError`, didn't it?\
A manual `Display` implementation, plus an `Error` impl block.

We can remove some of the boilerplate by using [`thiserror`](https://docs.rs/thiserror/latest/thiserror/),
a Rust crate that provides a **procedural macro** to simplify the creation of custom error types.\
But we're getting ahead of ourselves: `thiserror` is a third-party crate, it'd be our first dependency!

Let's take a step back to talk about Rust's packaging system before we dive into dependencies.

## What is a package?

A Rust package is defined by the `[package]` section in a `Cargo.toml` file, also known as its **manifest**.
Within `[package]` you can set the package's metadata, such as its name and version.

Go check the `Cargo.toml` file in the directory of this section's exercise!

## What is a crate?

Inside a package, you can have one or more **crates**, also known as **targets**.\
The two most common crate types are **binary crates** and **library crates**.

### Binaries

A binary is a program that can be compiled to an **executable file**.\
It must include a function named `main`—the program's entry point. `main` is invoked when the program is executed.

### Libraries

Libraries, on the other hand, are not executable on their own. You can't _run_ a library,
but you can _import its code_ from another package that depends on it.\
A library groups together code (i.e. functions, types, etc.) that can be leveraged by other packages as a **dependency**.

All the exercises you've solved so far have been structured as libraries, with a test suite attached to them.

### Conventions

There are some conventions around Rust packages that you need to keep in mind:

- The package's source code is usually located in the `src` directory.
- If there's a `src/lib.rs` file, `cargo` will infer that the package contains a library crate.
- If there's a `src/main.rs` file, `cargo` will infer that the package contains a binary crate.

You can override these defaults by explicitly declaring your targets in the `Cargo.toml` file—see
[`cargo`'s documentation](https://doc.rust-lang.org/cargo/reference/cargo-targets.html#cargo-targets) for more details.

Keep in mind that while a package can contain multiple crates, it can only contain one library crate.

### Appendix 12: Dependencies

# Dependencies

A package can depend on other packages by listing them in the `[dependencies]` section of its `Cargo.toml` file.\
The most common way to specify a dependency is by providing its name and version:

```toml
[dependencies]
thiserror = "1"
```

This will add `thiserror` as a dependency to your package, with a **minimum** version of `1.0.0`.
`thiserror` will be pulled from [crates.io](https://crates.io), Rust's official package registry.
When you run `cargo build`, `cargo` will go through a few stages:

- Dependency resolution
- Downloading the dependencies
- Compiling your project (your own code and the dependencies)

Dependency resolution is skipped if your project has a `Cargo.lock` file and your manifest files are unchanged.
A lockfile is automatically generated by `cargo` after a successful round of dependency resolution: it contains
the exact versions of all dependencies used in your project, and is used to ensure that the same versions are
consistently used across different builds (e.g. in CI). If you're working on a project with multiple developers,
you should commit the `Cargo.lock` file to your version control system.

You can use `cargo update` to update the `Cargo.lock` file with the latest (compatible) versions of all your dependencies.

### Path dependencies

You can also specify a dependency using a **path**. This is useful when you're working on multiple local packages.

```toml
[dependencies]
my-library = { path = "../my-library" }
```

The path is relative to the `Cargo.toml` file of the package that's declaring the dependency.

### Other sources

Check out the [Cargo documentation](https://doc.rust-lang.org/cargo/reference/specifying-dependencies.html) for more
details on where you can get dependencies from and how to specify them in your `Cargo.toml` file.

## Dev dependencies

You can also specify dependencies that are only needed for development—i.e. they only get pulled in when you're
running `cargo test`.\
They go in the `[dev-dependencies]` section of your `Cargo.toml` file:

```toml
[dev-dependencies]
static_assertions = "1.1.0"
```

We've been using a few of these throughout the book to shorten our tests.

### Appendix 13: `thiserror`

# `thiserror`

That was a bit of detour, wasn't it? But a necessary one!\
Let's get back on track now: custom error types and `thiserror`.

## Custom error types

We've seen how to implement the `Error` trait "manually" for a custom error type.\
Imagine that you have to do this for most error types in your codebase. That's a lot of boilerplate, isn't it?

We can remove some of the boilerplate by using [`thiserror`](https://docs.rs/thiserror/latest/thiserror/),
a Rust crate that provides a **procedural macro** to simplify the creation of custom error types.

```rust
#[derive(thiserror::Error, Debug)]
enum TicketNewError {
    #[error("{0}")]
    TitleError(String),
    #[error("{0}")]
    DescriptionError(String),
}
```

## You can write your own macros

All the `derive` macros we've seen so far were provided by the Rust standard library.\
`thiserror::Error` is the first example of a **third-party** `derive` macro.

`derive` macros are a subset of **procedural macros**, a way to generate Rust code at compile time.
We won't get into the details of how to write a procedural macro in this course, but it's important
to know that you can write your own!\
A topic to approach in a more advanced Rust course.

## Custom syntax

Each procedural macro can define its own syntax, which is usually explained in the crate's documentation.
In the case of `thiserror`, we have:

- `#[derive(thiserror::Error)]`: this is the syntax to derive the `Error` trait for a custom error type, helped by `thiserror`.
- `#[error("{0}")]`: this is the syntax to define a `Display` implementation for each variant of the custom error type.
  `{0}` is replaced by the zero-th field of the variant (`String`, in this case) when the error is displayed.

### Appendix 14: `TryFrom` and `TryInto`

# `TryFrom` and `TryInto`

In the previous chapter we looked at the [`From` and `Into` traits](../04_traits/09_from.md),
Rust's idiomatic interfaces for **infallible** type conversions.\
But what if the conversion is not guaranteed to succeed?

We now know enough about errors to discuss the **fallible** counterparts of `From` and `Into`:
`TryFrom` and `TryInto`.

## `TryFrom` and `TryInto`

Both `TryFrom` and `TryInto` are defined in the `std::convert` module, just like `From` and `Into`.

```rust
pub trait TryFrom<T>: Sized {
    type Error;
    fn try_from(value: T) -> Result<Self, Self::Error>;
}

pub trait TryInto<T>: Sized {
    type Error;
    fn try_into(self) -> Result<T, Self::Error>;
}
```

The main difference between `From`/`Into` and `TryFrom`/`TryInto` is that the latter return a `Result` type.\
This allows the conversion to fail, returning an error instead of panicking.

## `Self::Error`

Both `TryFrom` and `TryInto` have an associated `Error` type.
This allows each implementation to specify its own error type, ideally the most appropriate for the conversion
being attempted.

`Self::Error` is a way to refer to the `Error` associated type defined in the trait itself.

## Duality

Just like `From` and `Into`, `TryFrom` and `TryInto` are dual traits.\
If you implement `TryFrom` for a type, you get `TryInto` for free.

### Appendix 15: `Error::source`

# `Error::source`

There's one more thing we need to talk about to complete our coverage of the `Error` trait: the `source` method.

```rust
// Full definition this time!
pub trait Error: Debug + Display {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}
```

The `source` method is a way to access the **error cause**, if any.\
Errors are often chained, meaning that one error is the cause of another: you have a high-level error (e.g.
cannot connect to the database) that is caused by a lower-level error (e.g. can't resolve the database hostname).
The `source` method allows you to "walk" the full chain of errors, often used when capturing error context in logs.

## Implementing `source`

The `Error` trait provides a default implementation that always returns `None` (i.e. no underlying cause). That's why
you didn't have to care about `source` in the previous exercises.\
You can override this default implementation to provide a cause for your error type.

```rust
use std::error::Error;

#[derive(Debug)]
struct DatabaseError {
    source: std::io::Error
}

impl std::fmt::Display for DatabaseError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Failed to connect to the database")
    }
}

impl std::error::Error for DatabaseError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(&self.source)
    }
}
```

In this example, `DatabaseError` wraps an `std::io::Error` as its source.
We then override the `source` method to return this source when called.

## `&(dyn Error + 'static)`

What's this `&(dyn Error + 'static)` type?\
Let's unpack it:

- `dyn Error` is a **trait object**. It's a way to refer to any type that implements the `Error` trait.
- `'static` is a special **lifetime specifier**.
  `'static` implies that the reference is valid for "as long as we need it", i.e. the entire program execution.

Combined: `&(dyn Error + 'static)` is a reference to a trait object that implements the `Error` trait
and is valid for the entire program execution.

Don't worry too much about either of these concepts for now. We'll cover them in more detail in future chapters.

## Implementing `source` using `thiserror`

`thiserror` provides three ways to automatically implement `source` for your error types:

- A field named `source` will automatically be used as the source of the error.
  ```rust
  use thiserror::Error;

  #[derive(Error, Debug)]
  pub enum MyError {
      #[error("Failed to connect to the database")]
      DatabaseError {
          source: std::io::Error
      }
  }
  ```
- A field annotated with the `#[source]` attribute will automatically be used as the source of the error.
  ```rust
  use thiserror::Error;

  #[derive(Error, Debug)]
  pub enum MyError {
      #[error("Failed to connect to the database")]
      DatabaseError {
          #[source]
          inner: std::io::Error
      }
  }
  ```
- A field annotated with the `#[from]` attribute will automatically be used as the source of the error **and**
  `thiserror` will automatically generate a `From` implementation to convert the annotated type into your error type.
  ```rust
  use thiserror::Error;

  #[derive(Error, Debug)]
  pub enum MyError {
      #[error("Failed to connect to the database")]
      DatabaseError {
          #[from]
          inner: std::io::Error
      }
  }
  ```

## The `?` operator

The `?` operator is a shorthand for propagating errors.\
When used in a function that returns a `Result`, it will return early with an error if the `Result` is `Err`.

For example:

```rust
use std::fs::File;

fn read_file() -> Result<String, std::io::Error> {
    let mut file = File::open("file.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}
```

is equivalent to:

```rust
use std::fs::File;

fn read_file() -> Result<String, std::io::Error> {
    let mut file = match File::open("file.txt") {
        Ok(file) => file,
        Err(e) => {
            return Err(e);
        }
    };
    let mut contents = String::new();
    match file.read_to_string(&mut contents) {
        Ok(_) => (),
        Err(e) => {
            return Err(e);
        }
    }
    Ok(contents)
}
```

You can use the `?` operator to shorten your error handling code significantly.\
In particular, the `?` operator will automatically convert the error type of the fallible operation into the error type
of the function, if a conversion is possible (i.e. if there is a suitable `From` implementation)

### Appendix 16: Wrapping Up

# Wrapping up

When it comes to domain modelling, the devil is in the details.\
Rust offers a wide range of tools to help you represent the constraints of your domain directly in the type system,
but it takes some practice to get it right and write code that looks idiomatic.

Let's close the chapter with one final refinement of our `Ticket` model.\
We'll introduce a new type for each of the fields in `Ticket` to encapsulate the respective constraints.\
Every time someone accesses a `Ticket` field, they'll get back a value that's guaranteed to be valid—i.e. a
`TicketTitle` instead of a `String`. They won't have to worry about the title being empty elsewhere in the code:
as long as they have a `TicketTitle`, they know it's valid **by construction**.

This is just an example of how you can use Rust's type system to make your code safer and more expressive.

## Further reading

- [Parse, don't validate](https://lexi-lambda.github.io/blog/2019/11/05/parse-don-t-validate/)
- [Using types to guarantee domain invariants](https://www.lpalmieri.com/posts/2020-12-11-zero-to-production-6-domain-modelling/)
