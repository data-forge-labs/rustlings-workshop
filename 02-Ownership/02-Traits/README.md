# Rust for Python Data Engineers — Traits: Rust's Interfaces

*Learn how traits work in Rust — the equivalent of Python's protocols/ABCs/interfaces — and how they enable polymorphism without inheritance.*

> **Test-driven approach**: This project includes a Cargo project with progressive unit tests. Each function in `workshop/src/lib.rs` starts as a `todo!()` stub. As you follow each section, replace `todo!()` with real code and run `cd workshop && cargo test` to watch the pass count grow. Your goal: **all 14 tests pass**.

---

## Why This Project?

### The Problem

In Python, you define shared behavior using protocols, ABCs, or duck typing:

```python
from typing import Protocol

class Summarizable(Protocol):
    def summarize(self) -> str: ...

def print_summary(item: Summarizable):
    print(item.summarize())

class Ticket:
    def summarize(self):
        return f"[{self.status}] {self.title}"

# This works, but nothing forces Ticket to implement summarize
# If you mis-spell "summarize" as "sumarize", you get a runtime error
```

Python's protocols are **structural** — any class with matching methods satisfies the protocol. This is flexible but error-prone: there's no explicit declaration that `Ticket` implements `Summarizable`. A typo in a method name only fails at runtime. There's also no way to add behavior to types you don't control (e.g., making `int` printable with custom formatting) without modifying the class.

For data engineers: you want to define operations like "can be serialized to CSV" or "can be summed" as reusable interfaces. Python gives you ABCs and protocols, but they're unenforced conventions — the compiler never checks.

```
Python traits are "duck typed":
  if it quacks like a Summarizable, it is one
  → runtime check, no compile-time guarantee
  → no way to implement Summarizable for existing types
```

### The Rust Solution

Rust uses **traits** — explicit, named sets of methods that types can implement:

```rust
trait Summarizable {
    fn summarize(&self) -> String;
}

impl Summarizable for Ticket {
    fn summarize(&self) -> String {
        format!("[{}] {}", self.status, self.title)
    }
}

fn print_summary(item: &impl Summarizable) {
    println!("{}", item.summarize());
}
```

Traits are **nominal** — `Ticket` explicitly declares `impl Summarizable for Ticket`. The compiler ensures the implementation matches the trait definition. You can implement external traits for your types (and vice versa, with the orphan rule as the only restriction). Traits enable polymorphism without inheritance — composition over inheritance.

---

## What You'll Learn

| # | Concept | Rust Type / Module | Python Equivalent | Purpose |
|---|---------|--------------------|------------------|---------|
| 1 | Trait Definition | `trait` | `Protocol` / `ABC` | Define shared behavior interfaces |
| 2 | Trait Implementation | `impl Trait for Type` | Subclass / register | Add behavior to any type |
| 3 | Orphan Rule | Coherence | Multiple inheritance | Prevent conflicting implementations |
| 4 | Trait Bounds | `T: Trait` / `where` | Type hints | Constrain generic functions |
| 5 | Derive Macros | `#[derive(...)]` | `@dataclass` | Auto-implement common traits |
| 6 | Display | `std::fmt::Display` | `__str__` | Human-readable output `{}` |
| 7 | Debug | `std::fmt::Debug` | `__repr__` | Debug output `{:?}` |
| 8 | From / Into | `From<T>`, `Into<T>` | `__init__` from type | Type conversions |
| 9 | Clone | `Clone` | `copy.deepcopy` | Explicit deep copy |
| 10 | Copy | `Copy` | None (implicit refs) | Implicit bitwise copy for stack types |
| 11 | Drop | `Drop` | `__del__` | Deterministic cleanup on scope exit |
| 12 | Operator Overloading | `Add`, `Sub`, etc. | `__add__`, `__sub__` | Custom operators on types |
| 13 | PartialEq / Eq | `PartialEq`, `Eq` | `__eq__` | Equality and total equality |

## Concepts at a Glance

### 1. Trait Definition
A trait is a set of method signatures, like Python's `Protocol`. `trait Summary { fn summarize(&self) -> String; }` vs Python `class Summarizable(Protocol): def summarize(self) -> str: ...`. Rust traits are nominal (explicit opt-in), not structural (duck-typed).

### 2. Trait Implementation
`impl Summary for Ticket { ... }` explicitly declares that `Ticket` implements `Summary`. Python would accept any class with a `summarize` method. Rust requires the explicit `impl` block.

### 3. Orphan Rule
You can implement a trait for a type only if you own the trait or the type. Python has no equivalent — you can monkey-patch anything. The orphan rule prevents conflicting implementations across crates.

### 4. Trait Bounds
`fn process<T: Summary>(item: &T)` constrains generic types. Python's type hints (`item: Summarizable`) are advisory; Rust's trait bounds are compiler-enforced.

### 5. Derive Macros
`#[derive(Debug, Clone, PartialEq)]` auto-generates trait implementations. Like Python's `@dataclass` auto-generates `__init__`, `__repr__`, `__eq__`. Derive is the standard way to add common traits.

### 6. Display
`impl Display for Ticket` provides `"{}"` formatting. Python equivalent: `__str__`. Must be implemented manually (no derive). Used for user-facing output.

### 7. Debug
`impl Debug for Ticket` provides `"{:?}"` formatting. Python equivalent: `__repr__`. Can be derived with `#[derive(Debug)]`. Used for developer-facing output.

### 8. From / Into
`impl From<&str> for Ticket` enables `Ticket::from("title")` and `let t: Ticket = "title".into()`. Python equivalent: `__init__` accepting different types.

### 9. Clone
`x.clone()` creates an explicit deep copy. Python: `copy.deepcopy(x)`. Rust makes copying explicit — no accidental duplication of large datasets.

### 10. Copy
Types that implement `Copy` (integers, bools) are implicitly duplicated on assignment. Python has no equivalent — everything is a reference. In Rust, `let y = x` for a `Copy` type keeps `x` valid.

### 11. Drop
`Drop::drop()` runs automatically at end of scope. Python's `__del__` is GC-driven and non-deterministic. Rust guarantees cleanup when the owner exits scope.

### 12. Operator Overloading
Rust overloads operators via traits: `impl Add for Point { type Output = Point; ... }`. Python: `__add__`. Both allow custom `+`, `-`, `*`, etc.

### 13. PartialEq / Eq
`#[derive(PartialEq, Eq)]` enables `==` comparison. Python: `__eq__`. `Eq` requires `PartialEq` and guarantees total equivalence (no NaN-like values).

---

## Table of Contents

1. [Project Overview](#1-project-overview)
2. [Concept: What Is a Trait?](#2-concept-what-is-a-trait)
3. [Concept: Implementing a Trait](#3-concept-implementing-a-trait)
4. [Concept: Trait Bounds](#4-concept-trait-bounds)
5. [Concept: `derive` Macros](#5-concept-derive-macros)
6. [Concept: `From` and `Into`](#6-concept-from-and-into)
7. [Concept: `Clone` and `Copy`](#7-concept-clone-and-copy)
8. [Concept: `Drop` — Cleanup](#8-concept-drop--cleanup)
9. [Concept: Operator Overloading](#9-concept-operator-overloading)
10. [Putting It All Together](#10-putting-it-all-together)
11. [Summary](#11-summary)

---

## 1. Project Overview

We'll extend our Ticket system with traits that make it behave more like a Python object:

| Trait | Python Equivalent | Purpose |
|---|---|---|
| `Display` | `__str__` | Human-readable output |
| `Debug` | `__repr__` | Debug output |
| `PartialEq` | `__eq__` | Equality comparison |
| `Clone` | `copy.deepcopy` | Explicit duplication |
| `From` / `Into` | `__init__` (type conversion) | Convert between types |
| `Drop` | `__del__` / context manager | Resource cleanup |

---

## 2. Concept: What Is a Trait?

### Python Protocols vs Rust Traits

```python
# Python — Protocol (structural typing)
from typing import Protocol

class Summarizable(Protocol):
    def summarize(self) -> str:
        ...

def print_summary(item: Summarizable):
    print(item.summarize())
```

```rust
// Rust — Trait (nominal typing)
trait Summarizable {
    fn summarize(&self) -> String;
}

fn print_summary(item: &impl Summarizable) {
    println!("{}", item.summarize());
}
```

### Defining a Trait

```rust
trait Summary {
    fn summarize(&self) -> String;

    // Methods CAN have default implementations
    fn summarize_short(&self) -> String {
        let s = self.summarize();
        if s.len() > 50 {
            format!("{}...", &s[..47])
        } else {
            s
        }
    }
}
```

| Python | Rust |
|---|---|
| `class MyProtocol(Protocol):` | `trait MyTrait { }` |
| `def method(self) -> str` | `fn method(&self) -> String` |
| Method with default impl | Allowed (same syntax) |
| Duck typing (structural) | Nominal typing (must explicitly implement) |

---

## 3. Concept: Implementing a Trait

```rust
struct Ticket {
    title: String,
    description: String,
}

impl Summary for Ticket {
    fn summarize(&self) -> String {
        format!("Ticket: {}", self.title)
    }
}

fn main() {
    let t = Ticket {
        title: String::from("Bug fix"),
        description: String::from("Fix the login bug"),
    };
    println!("{}", t.summarize());          // "Ticket: Bug fix"
    println!("{}", t.summarize_short());    // Uses default impl
}
```

### Orphan Rule

> **You can implement a trait for a type only if EITHER the trait OR the type is defined in your crate.**

```rust
// ✅ OK: Your trait + your type
impl MyTrait for MyStruct {}

// ✅ OK: Standard trait + your type  
impl Display for MyStruct {}

// ✅ OK: Your trait + standard type
trait MyTrait {}
impl MyTrait for String {}

// ❌ ERROR: Standard trait + standard type (orphan rule)
impl Display for String {}  // Not in your crate!
```

### Why? To prevent conflicting implementations across crates.

---

## 4. Concept: Trait Bounds

### Generic Functions with Trait Bounds

```python
# Python — duck typing
def summarize_all(items):
    return [item.summarize() for item in items]
```

```rust
// Rust — trait bounds
fn summarize_all(items: &[impl Summary]) -> Vec<String> {
    items.iter().map(|item| item.summarize()).collect()
}

// Alternative syntax (same thing):
fn summarize_all<T: Summary>(items: &[T]) -> Vec<String> {
    items.iter().map(|item| item.summarize()).collect()
}
```

### Multiple Trait Bounds

```rust
fn debug_and_summarize(item: &(impl Summary + std::fmt::Debug)) {
    println!("{:?}", item);
    println!("{}", item.summarize());
}

// Alternative:
fn debug_and_summarize<T: Summary + std::fmt::Debug>(item: &T) { ... }
```

### `where` Clauses (for readability)

```rust
fn complex_function<T, U>(a: &T, b: &U) -> String
where
    T: Summary + std::fmt::Debug,
    U: Summary + Clone,
{
    format!("{:?} — {}", a, b.summarize())
}
```

---

## 5. Concept: `derive` Macros

The most common traits can be **auto-implemented** with `#[derive(...)]`:

```rust
#[derive(Debug, Clone, PartialEq, Eq)]
struct Ticket {
    title: String,
    description: String,
    status: String,
}

// Compiler generates:
// - impl Debug for Ticket  (format with {:?})
// - impl Clone for Ticket (explicit .clone())
// - impl PartialEq for Ticket (== comparison)
// - impl Eq for Ticket (total equality)
```

| Derive | Python | Usage |
|---|---|---|
| `Debug` | `__repr__` | `println!("{:?}", x)` |
| `Clone` | `copy.deepcopy` | `x.clone()` |
| `Copy` | Implicit copy for simple types | `let y = x;` (x still valid) |
| `PartialEq` | `__eq__` | `x == y` |
| `Eq` | N/A (Python dicts require hash) | Total equality |
| `Hash` | `__hash__` | Use as HashMap key |
| `Default` | `__init__()` with defaults | `Ticket::default()` |

### Without Derive (Manual Implementation)

```rust
impl std::fmt::Debug for Ticket {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Ticket")
            .field("title", &self.title)
            .field("status", &self.status)
            .finish()
    }
}
```

> Use `#[derive]` unless you need custom behavior. It's the Rust equivalent of Python's `@dataclass`.

---

## 6. Concept: `From` and `Into`

### Type Conversion

```python
# Python — implicit conversion in many cases
x = 5       # int
y = float(x)  # 5.0
z = str(x)    # "5"
```

```rust
// Rust — explicit conversion via From/Into
let x: i32 = 5;
let y: f64 = x.into();    // From<i32> for f64
let z: String = x.to_string(); // Display trait
```

### Implementing `From`

```rust
struct Status(String);

// Create Status from &str
impl From<&str> for Status {
    fn from(s: &str) -> Status {
        Status(s.to_string())
    }
}

let status = Status::from("Open");
// Or using .into():
let status: Status = "Open".into();
```

### Practical Data Engineering Example

```rust
#[derive(Debug)]
struct DataRow {
    id: u64,
    value: f64,
    label: String,
}

// Convert a CSV record into a DataRow
impl From<&csv::StringRecord> for DataRow {
    fn from(record: &csv::StringRecord) -> DataRow {
        DataRow {
            id: record[0].parse().unwrap_or(0),
            value: record[1].parse().unwrap_or(0.0),
            label: record[2].to_string(),
        }
    }
}
```

---

## 7. Concept: `Clone` and `Copy`

### `Clone` — Explicit Duplication

```rust
// Clone makes .clone() available
#[derive(Clone)]
struct Ticket {
    title: String,
}

fn main() {
    let t1 = Ticket { title: String::from("Bug") };
    let t2 = t1.clone();  // Deep copy
    // t1 is still valid!
}
```

### `Copy` — Implicit Duplication (Stack-Only)

```rust
// Copy: bits are copied (no heap data)
#[derive(Copy, Clone)]
struct Point {
    x: f64,
    y: f64,
}

let p1 = Point { x: 1.0, y: 2.0 };
let p2 = p1;        // Implicit copy — p1 still valid!
```

### What Can Be `Copy`?

| Can be `Copy` | Cannot be `Copy` |
|---|---|
| All integers (`u32`, `i64`, etc.) | `String` (heap-allocated) |
| `f32`, `f64` | `Vec<T>` (heap-allocated) |
| `bool` | `&T` references (usually) |
| Structs of only `Copy` types | Structs with `String` fields |
| `char` | Any type implementing `Drop` |

### Python Comparison

```python
# Python — everything is reference semantics
a = [1, 2, 3]
b = a           # b is a reference, not a copy
b.append(4)     # a is also changed!
```

```rust
// Rust — be explicit about copying
let a = vec![1, 2, 3];
let b = a.clone();  // Explicit deep copy — a still valid
// let b = a;       // Move — a becomes invalid
```

---

## 8. Concept: `Drop` — Cleanup

```rust
struct DatabaseConnection {
    url: String,
}

impl Drop for DatabaseConnection {
    fn drop(&mut self) {
        println!("Closing connection to {}", self.url);
        // Release resources here
    }
}

fn main() {
    let conn = DatabaseConnection {
        url: String::from("postgres://localhost:5432/mydb"),
    };
    // ... use conn ...
    // conn.drop() is called automatically here
}
```

### Python vs Rust Cleanup

```python
# Python — __del__ is unreliable (may never be called)
class Connection:
    def __del__(self):
        self.close()  # Not guaranteed!
```

```rust
// Rust — Drop is deterministic (always called at scope end)
impl Drop for Connection {
    fn drop(&mut self) {
        self.close();  // Guaranteed!
    }
}
```

### For Data Engineers

```rust
struct LargeDataset {
    data: Vec<Vec<f64>>,
    path: String,
}

impl Drop for LargeDataset {
    fn drop(&mut self) {
        println!("Freeing {} MB dataset", 
            self.data.len() * std::mem::size_of::<Vec<f64>>() / 1_048_576);
        self.data.clear();  // Free memory NOW, not when GC decides
    }
}
```

---

## 9. Concept: Operator Overloading

Rust lets you overload operators by implementing the corresponding trait:

```rust
use std::ops::Add;

#[derive(Debug, Clone)]
struct DataPoint {
    x: f64,
    y: f64,
    value: f64,
}

// Add two data points (e.g., aggregating readings)
impl Add for DataPoint {
    type Output = DataPoint;

    fn add(self, other: DataPoint) -> DataPoint {
        DataPoint {
            x: self.x,  // Keep first point's position
            y: self.y,
            value: self.value + other.value,  // Sum values
        }
    }
}

fn main() {
    let a = DataPoint { x: 1.0, y: 2.0, value: 10.0 };
    let b = DataPoint { x: 3.0, y: 4.0, value: 20.0 };
    let sum = a + b;  // Uses our Add impl
    println!("{:?}", sum);  // value = 30.0
}
```

### Common Operator Traits

| Operator | Trait | Python |
|---|---|---|
| `a + b` | `Add` | `__add__` |
| `a - b` | `Sub` | `__sub__` |
| `a * b` | `Mul` | `__mul__` |
| `a / b` | `Div` | `__truediv__` |
| `-a` | `Neg` | `__neg__` |
| `a == b` | `PartialEq` | `__eq__` |
| `a < b` | `PartialOrd` | `__lt__` |
| `a[i]` | `Index` | `__getitem__` |

---

## 10. Putting It All Together

```rust
use std::fmt;

/// A ticket with derived traits for common operations
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Ticket {
    pub title: String,
    pub description: String,
    pub status: String,
}

// Manual Display for user-friendly output
impl fmt::Display for Ticket {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}] {}", self.status, self.title)
    }
}

// Default status via From
impl From<&str> for Ticket {
    fn from(title: &str) -> Ticket {
        Ticket {
            title: title.to_string(),
            description: String::new(),
            status: "Open".to_string(),
        }
    }
}

// Generic printer using trait bounds
fn print_summary<T: fmt::Display + fmt::Debug>(item: &T) {
    println!("Display: {}", item);
    println!("Debug: {:?}", item);
}

fn main() {
    let ticket = Ticket {
        title: String::from("Fix login bug"),
        description: String::from("SSO login broken"),
        status: String::from("Open"),
    };

    // Display trait
    println!("{}", ticket);  // [Open] Fix login bug

    // Debug trait
    println!("{:?}", ticket);

    // Clone trait
    let backup = ticket.clone();

    // PartialEq trait
    println!("Self-equal: {}", ticket == backup);  // true

    // From trait
    let quick: Ticket = "Quick bug".into();
    println!("{}", quick);

    // Generic function with trait bounds
    print_summary(&ticket);
}
```

---

## 11. Summary

| Trait | Python | Purpose | Auto-derivable |
|---|---|---|---|
| `Debug` | `__repr__` | Debug output `{:?}` | ✅ |
| `Display` | `__str__` | User output `{}` | ❌ (manual) |
| `Clone` | `copy.deepcopy` | Explicit deep copy | ✅ |
| `Copy` | Implicit for primitives | Bitwise copy | ✅ |
| `PartialEq` | `__eq__` | `==` comparison | ✅ |
| `Eq` | N/A | Total equality | ✅ |
| `Hash` | `__hash__` | HashMap key | ✅ |
| `Default` | Default constructor | `Type::default()` | ✅ |
| `From<T>` | `__init__` from T | `Type::from(x)` | ❌ |
| `Into<T>` | Implicit conversion | `x.into()` | ❌ (auto if From) |
| `Drop` | `__del__` | Cleanup on scope exit | ❌ |
| `Add`, `Sub`, ... | `__add__`, etc. | Operator overloading | ❌ |

### Key Takeaways for Data Engineers

1. **Use `#[derive(Debug, Clone, PartialEq)]`** on your data structs as a matter of habit
2. **`Clone` is explicit** — no accidental copying of large datasets
3. **`Drop` provides deterministic cleanup** — no GC pauses
4. **Traits enable polymorphism** without inheritance (composition over inheritance)
5. **`From`/`Into`** make type conversion in data pipelines clean and explicit

### Further Reading

The following lesson files in this folder provide deeper dives into each concept:

| File | Topics |
|------|--------|
| [00_intro.md](./00_intro.md) | Project introduction |
| [01_trait.md](./01_trait.md) | Trait definition, implementation, associated types |
| [02_orphan_rule.md](./02_orphan_rule.md) | Orphan rule and coherence |
| [03_operator_overloading.md](./03_operator_overloading.md) | Operator overloading with traits |
| [04_derive.md](./04_derive.md) | `#[derive]` macros, auto-implementing traits |
| [05_trait_bounds.md](./05_trait_bounds.md) | Generic bounds, `where` clauses |
| [06_str_slice.md](./06_str_slice.md) | `&str` vs `String`, string slice internals |
| [07_deref.md](./07_deref.md) | `Deref` trait, smart pointer dereferencing |
| [08_sized.md](./08_sized.md) | `Sized` trait, `?Sized` |
| [09_from.md](./09_from.md) | `From` and `Into` traits |
| [10_assoc_vs_generic.md](./10_assoc_vs_generic.md) | Associated types vs generic type parameters |
| [11_clone.md](./11_clone.md) | `Clone` trait, explicit duplication |
| [12_copy.md](./12_copy.md) | `Copy` trait, implicit bitwise copy |
| [13_drop.md](./13_drop.md) | `Drop` trait, deterministic cleanup |
| [14_outro.md](./14_outro.md) | Section wrap-up |

### Next Project

Proceed to [5-TicketV2](../02-Ownership/03-TicketV2/README.md) to learn about **enums** and **error handling with `Result`** — essential for production data pipelines.
