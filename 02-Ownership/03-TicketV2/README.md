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
