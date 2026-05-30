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
12. [Appendix: Original Step-by-Step Tutorial](#12-appendix-original-step-by-step-tutorial)

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

The original step-by-step tutorial content has been merged into the [Appendix](#12-appendix-original-step-by-step-tutorial) below.

### Next Project

Proceed to [5-TicketV2](../02-Ownership/03-TicketV2/README.md) to learn about **enums** and **error handling with `Result`** — essential for production data pipelines.

---

## 12. Appendix: Original Step-by-Step Tutorial

### 00 Intro

In the previous chapter we covered the basics of Rust's type and ownership system.
It's time to dig deeper: we'll explore **traits**, Rust's take on interfaces.

Once you learn about traits, you'll start seeing their fingerprints all over the place.
In fact, you've already seen traits in action throughout the previous chapter, e.g. `.into()` invocations as well as operators like `==` and `+`.

On top of traits as a concept, we'll also cover some of the key traits that are defined in Rust's standard library:

- Operator traits (e.g. `Add`, `Sub`, `PartialEq`, etc.)
- `From` and `Into`, for infallible conversions
- `Clone` and `Copy`, for copying values
- `Deref` and deref coercion
- `Sized`, to mark types with a known size
- `Drop`, for custom cleanup logic

Since we'll be talking about conversions, we'll seize the opportunity to plug some of the "knowledge gaps" from the previous chapter—e.g. what is `"A title"`, exactly? Time to learn more about slices too!

### 01 Trait

Let's look again at our `Ticket` type:

```rust
pub struct Ticket {
    title: String,
    description: String,
    status: String,
}
```

All our tests, so far, have been making assertions using `Ticket`'s fields.

```rust
assert_eq!(ticket.title(), "A new title");
```

What if we wanted to compare two `Ticket` instances directly?

```rust
let ticket1 = Ticket::new(/* ... */);
let ticket2 = Ticket::new(/* ... */);
ticket1 == ticket2
```

The compiler will stop us:

```text
error[E0369]: binary operation `==` cannot be applied to type `Ticket`
  --> src/main.rs:18:13
   |
18 |     ticket1 == ticket2
   |     ------- ^^ ------- Ticket
   |     |
   |     Ticket
   |
note: an implementation of `PartialEq` might be missing for `Ticket`
```

`Ticket` is a new type. Out of the box, there is **no behavior attached to it**.
Rust doesn't magically infer how to compare two `Ticket` instances just because they contain `String`s.

The Rust compiler is nudging us in the right direction though: it's suggesting that we might be missing an implementation of `PartialEq`. `PartialEq` is a **trait**!

#### What are traits?

Traits are Rust's way of defining **interfaces**.
A trait defines a set of methods that a type must implement to satisfy the trait's contract.

##### Defining a trait

The syntax for a trait definition goes like this:

```rust
trait <TraitName> {
    fn <method_name>(<parameters>) -> <return_type>;
}
```

We might, for example, define a trait named `MaybeZero` that requires its implementors to define an `is_zero` method:

```rust
trait MaybeZero {
    fn is_zero(self) -> bool;
}
```

##### Implementing a trait

To implement a trait for a type we use the `impl` keyword, just like we do for regular[^inherent] methods, but the syntax is a bit different:

```rust
impl <TraitName> for <TypeName> {
    fn <method_name>(<parameters>) -> <return_type> {
        // Method body
    }
}
```

For example, to implement the `MaybeZero` trait for a custom number type, `WrappingU32`:

```rust
pub struct WrappingU32 {
    inner: u32,
}

impl MaybeZero for WrappingU32 {
    fn is_zero(self) -> bool {
        self.inner == 0
    }
}
```

##### Invoking a trait method

To invoke a trait method, we use the `.` operator, just like we do with regular methods:

```rust
let x = WrappingU32 { inner: 5 };
assert!(!x.is_zero());
```

To invoke a trait method, two things must be true:

- The type must implement the trait.
- The trait must be in scope.

To satisfy the latter, you may have to add a `use` statement for the trait:

```rust
use crate::MaybeZero;
```

This is not necessary if:

- The trait is defined in the same module where the invocation occurs.
- The trait is defined in the standard library's **prelude**.
  The prelude is a set of traits and types that are automatically imported into every Rust program.
  It's as if `use std::prelude::*;` was added at the beginning of every Rust module.

You can find the list of traits and types in the prelude in the
[Rust documentation](https://doc.rust-lang.org/std/prelude/index.html).

[^inherent]: A method defined directly on a type, without using a trait, is also known as an **inherent method**.

### 02 Orphan Rule

When a type is defined in another crate (e.g. `u32`, from Rust's standard library), you can't directly define new methods for it. If you try:

```rust
impl u32 {
    fn is_even(&self) -> bool {
        self % 2 == 0
    }
}
```

the compiler will complain:

```text
error[E0390]: cannot define inherent `impl` for primitive types
  |
1 | impl u32 {
  | ^^^^^^^^
  |
  = help: consider using an extension trait instead
```

#### Extension trait

An **extension trait** is a trait whose primary purpose is to attach new methods to foreign types, such as `u32`.
That's exactly the pattern you deployed in the previous exercise, by defining the `IsEven` trait and then implementing it for `i32` and `u32`. You are then free to call `is_even` on those types as long as `IsEven` is in scope.

```rust
// Bring the trait in scope
use my_library::IsEven;

fn main() {
    // Invoke its method on a type that implements it
    if 4.is_even() {
        // [...]
    }
}
```

#### One implementation

There are limitations to the trait implementations you can write.
The simplest and most straight-forward one: you can't implement the same trait twice, in a crate, for the same type.

For example:

```rust
trait IsEven {
    fn is_even(&self) -> bool;
}

impl IsEven for u32 {
    fn is_even(&self) -> bool {
        true
    }
}

impl IsEven for u32 {
    fn is_even(&self) -> bool {
        false
    }
}
```

The compiler will reject it:

```text
error[E0119]: conflicting implementations of trait `IsEven` for type `u32`
   |
5  | impl IsEven for u32 {
   | ------------------- first implementation here
...
11 | impl IsEven for u32 {
   | ^^^^^^^^^^^^^^^^^^^ conflicting implementation for `u32`
```

There can be no ambiguity as to what trait implementation should be used when `IsEven::is_even` is invoked on a `u32` value, therefore there can only be one.

#### Orphan rule

Things get more nuanced when multiple crates are involved.
In particular, at least one of the following must be true:

- The trait is defined in the current crate
- The implementor type is defined in the current crate

This is known as Rust's **orphan rule**. Its goal is to make the method resolution process unambiguous.

Imagine the following situation:

- Crate `A` defines the `IsEven` trait
- Crate `B` implements `IsEven` for `u32`
- Crate `C` provides a (different) implementation of the `IsEven` trait for `u32`
- Crate `D` depends on both `B` and `C` and calls `1.is_even()`

Which implementation should be used? The one defined in `B`? Or the one defined in `C`?
There's no good answer, therefore the orphan rule was defined to prevent this scenario. Thanks to the orphan rule, neither crate `B` nor crate `C` would compile.

#### Further reading

- There are some caveats and exceptions to the orphan rule as stated above.
  Check out [the reference](https://doc.rust-lang.org/reference/items/implementations.html#trait-implementation-coherence) if you want to get familiar with its nuances.

### 03 Operator Overloading

Now that we have a basic understanding of what traits are, let's circle back to **operator overloading**.
Operator overloading is the ability to define custom behavior for operators like `+`, `-`, `*`, `/`, `==`, `!=`, etc.

#### Operators are traits

In Rust, operators are traits.
For each operator, there is a corresponding trait that defines the behavior of that operator. By implementing that trait for your type, you **unlock** the usage of the corresponding operators.

For example, the [`PartialEq` trait](https://doc.rust-lang.org/std/cmp/trait.PartialEq.html) defines the behavior of the `==` and `!=` operators:

```rust
// The `PartialEq` trait definition, from Rust's standard library
// (It is *slightly* simplified, for now)
pub trait PartialEq {
    // Required method
    //
    // `Self` is a Rust keyword that stands for 
    // "the type that is implementing the trait"
    fn eq(&self, other: &Self) -> bool;

    // Provided method
    fn ne(&self, other: &Self) -> bool { ... }
}
```

When you write `x == y` the compiler will look for an implementation of the `PartialEq` trait for the types of `x` and `y` and replace `x == y` with `x.eq(y)`. It's syntactic sugar!

This is the correspondence for the main operators:

| Operator                 | Trait                                                                   |
| ------------------------ | ----------------------------------------------------------------------- |
| `+`                      | [`Add`](https://doc.rust-lang.org/std/ops/trait.Add.html)               |
| `-`                      | [`Sub`](https://doc.rust-lang.org/std/ops/trait.Sub.html)               |
| `*`                      | [`Mul`](https://doc.rust-lang.org/std/ops/trait.Mul.html)               |
| `/`                      | [`Div`](https://doc.rust-lang.org/std/ops/trait.Div.html)               |
| `%`                      | [`Rem`](https://doc.rust-lang.org/std/ops/trait.Rem.html)               |
| `==` and `!=`            | [`PartialEq`](https://doc.rust-lang.org/std/cmp/trait.PartialEq.html)   |
| `<`, `>`, `<=`, and `>=` | [`PartialOrd`](https://doc.rust-lang.org/std/cmp/trait.PartialOrd.html) |

Arithmetic operators live in the [`std::ops`](https://doc.rust-lang.org/std/ops/index.html) module, while comparison ones live in the [`std::cmp`](https://doc.rust-lang.org/std/cmp/index.html) module.

#### Default implementations

The comment on `PartialEq::ne` states that "`ne` is a provided method".
It means that `PartialEq` provides a **default implementation** for `ne` in the trait definition—the `{ ... }` elided block in the definition snippet.
If we expand the elided block, it looks like this:

```rust
pub trait PartialEq {
    fn eq(&self, other: &Self) -> bool;

    fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }
}
```

It's what you expect: `ne` is the negation of `eq`.
Since a default implementation is provided, you can skip implementing `ne` when you implement `PartialEq` for your type. It's enough to implement `eq`:

```rust
struct WrappingU8 {
    inner: u8,
}

impl PartialEq for WrappingU8 {
    fn eq(&self, other: &WrappingU8) -> bool {
        self.inner == other.inner
    }
    
    // No `ne` implementation here
}
```

You are not forced to use the default implementation though. You can choose to override it when you implement the trait:

```rust
struct MyType;

impl PartialEq for MyType {
    fn eq(&self, other: &MyType) -> bool {
        // Custom implementation
    }

    fn ne(&self, other: &MyType) -> bool {
        // Custom implementation
    }
}
```

### 04 Derive

Implementing `PartialEq` for `Ticket` was a bit tedious, wasn't it?
You had to manually compare each field of the struct.

#### Destructuring syntax

Furthermore, the implementation is brittle: if the struct definition changes (e.g. a new field is added), you have to remember to update the `PartialEq` implementation.

You can mitigate the risk by **destructuring** the struct into its fields:

```rust
impl PartialEq for Ticket {
    fn eq(&self, other: &Self) -> bool {
        let Ticket {
            title,
            description,
            status,
        } = self;
        // [...]
    }
}
```

If the definition of `Ticket` changes, the compiler will error out, complaining that your destructuring is no longer exhaustive.
You can also rename struct fields, to avoid variable shadowing:

```rust
impl PartialEq for Ticket {
    fn eq(&self, other: &Self) -> bool {
        let Ticket {
            title,
            description,
            status,
        } = self;
        let Ticket {
            title: other_title,
            description: other_description,
            status: other_status,
        } = other;
        // [...]
    }
}
```

Destructuring is a useful pattern to have in your toolkit, but there's an even more convenient way to do this: **derive macros**.

#### Macros

You've already encountered a few macros in past exercises:

- `assert_eq!` and `assert!`, in the test cases
- `println!`, to print to the console

Rust macros are **code generators**.
They generate new Rust code based on the input you provide, and that generated code is then compiled alongside the rest of your program. Some macros are built into Rust's standard library, but you can also write your own. We won't be creating our own macro in this course, but you can find some useful pointers in the ["Further reading" section](#further-reading).

##### Inspection

Some IDEs let you expand a macro to inspect the generated code. If that's not possible, you can use [`cargo-expand`](https://github.com/dtolnay/cargo-expand).

##### Derive macros

A **derive macro** is a particular flavour of Rust macro. It is specified as an **attribute** on top of a struct.

```rust
#[derive(PartialEq)]
struct Ticket {
    title: String,
    description: String,
    status: String
}
```

Derive macros are used to automate the implementation of common (and "obvious") traits for custom types. In the example above, the `PartialEq` trait is automatically implemented for `Ticket`. If you expand the macro, you'll see that the generated code is functionally equivalent to the one you wrote manually, although a bit more cumbersome to read:

```rust
#[automatically_derived]
impl ::core::cmp::PartialEq for Ticket {
    #[inline]
    fn eq(&self, other: &Ticket) -> bool {
        self.title == other.title 
            && self.description == other.description
            && self.status == other.status
    }
}
```

The compiler will nudge you to derive traits when possible.

#### Further reading

- [The little book of Rust macros](https://veykril.github.io/tlborm/)
- [Proc macro workshop](https://github.com/dtolnay/proc-macro-workshop)

### 05 Trait Bounds

We've seen two use cases for traits so far:

- Unlocking "built-in" behaviour (e.g. operator overloading)
- Adding new behaviour to existing types (i.e. extension traits)

There's a third use case: **generic programming**.

#### The problem

All our functions and methods, so far, have been working with **concrete types**.
Code that operates on concrete types is usually straightforward to write and understand. But it's also limited in its reusability.
Let's imagine, for example, that we want to write a function that returns `true` if an integer is even. Working with concrete types, we'd have to write a separate function for each integer type we want to support:

```rust
fn is_even_i32(n: i32) -> bool {
    n % 2 == 0
}

fn is_even_i64(n: i64) -> bool {
    n % 2 == 0
}

// Etc.
```

Alternatively, we could write a single extension trait and then different implementations for each integer type:

```rust
trait IsEven {
    fn is_even(&self) -> bool;
}

impl IsEven for i32 {
    fn is_even(&self) -> bool {
        self % 2 == 0
    }
}

impl IsEven for i64 {
    fn is_even(&self) -> bool {
        self % 2 == 0
    }
}

// Etc.
```

The duplication remains.

#### Generic programming

We can do better using **generics**.
Generics allow us to write code that works with a **type parameter** instead of a concrete type:

```rust
fn print_if_even<T>(n: T)
where
    T: IsEven + Debug
{
    if n.is_even() {
        println!("{n:?} is even");
    }
}
```

`print_if_even` is a **generic function**.
It isn't tied to a specific input type. Instead, it works with any type `T` that:

- Implements the `IsEven` trait.
- Implements the `Debug` trait.

This contract is expressed with a **trait bound**: `T: IsEven + Debug`.
The `+` symbol is used to require that `T` implements multiple traits. `T: IsEven + Debug` is equivalent to "where `T` implements `IsEven` **and** `Debug`".

#### Trait bounds

What purpose do trait bounds serve in `print_if_even`?
To find out, let's try to remove them:

```rust
fn print_if_even<T>(n: T) {
    if n.is_even() {
        println!("{n:?} is even");
    }
}
```

This code won't compile:

```text
error[E0599]: no method named `is_even` found for type parameter `T` 
              in the current scope
  --> src/lib.rs:2:10
   |
1 | fn print_if_even<T>(n: T) {
   |                  - method `is_even` not found 
   |                    for this type parameter
2 |     if n.is_even() {
   |          ^^^^^^^ method not found in `T`

error[E0277]: `T` doesn't implement `Debug`
  --> src/lib.rs:3:19
   |
3 |         println!("{n:?} is even");
   |                   ^^^^^ 
   |   `T` cannot be formatted using `{:?}` because 
   |         it doesn't implement `Debug`
   |
help: consider restricting type parameter `T`
   |
1 | fn print_if_even<T: std::fmt::Debug>(n: T) {
   |                   +++++++++++++++++
```

Without trait bounds, the compiler doesn't know what `T` **can do**.
It doesn't know that `T` has an `is_even` method, and it doesn't know how to format `T` for printing. From the compiler point of view, a bare `T` has no behaviour at all.
Trait bounds restrict the set of types that can be used by ensuring that the behaviour required by the function body is present.

#### Syntax: inlining trait bounds

All the examples above used a **`where` clause** to specify trait bounds:

```rust
fn print_if_even<T>(n: T)
where
    T: IsEven + Debug
//  ^^^^^^^^^^^^^^^^^
//  This is a `where` clause
{
    // [...]
}
```

If the trait bounds are simple, you can **inline** them directly next to the type parameter:

```rust
fn print_if_even<T: IsEven + Debug>(n: T) {
    //           ^^^^^^^^^^^^^^^^^
    //           This is an inline trait bound
    // [...]
}
```

#### Syntax: meaningful names

In the examples above, we used `T` as the type parameter name. This is a common convention when a function has only one type parameter.
Nothing stops you from using a more meaningful name, though:

```rust
fn print_if_even<Number: IsEven + Debug>(n: Number) {
    // [...]
}
```

It is actually **desirable** to use meaningful names when there are multiple type parameters at play or when the name `T` doesn't convey enough information about the type's role in the function. Maximize clarity and readability when naming type parameters, just as you would with variables or function parameters. Follow Rust's conventions, though: use [upper camel case for type parameter names](https://rust-lang.github.io/api-guidelines/naming.html#casing-conforms-to-rfc-430-c-case).

#### The function signature is king

You may wonder why we need trait bounds at all. Can't the compiler infer the required traits from the function's body?
It could, but it won't.
The rationale is the same as for [explicit type annotations on function parameters](../02_basic_calculator/02_variables.md#function-arguments-are-variables): each function signature is a contract between the caller and the callee, and the terms must be explicitly stated. This allows for better error messages, better documentation, less unintentional breakages across versions, and faster compilation times.

### 06 Str Slice

Throughout the previous chapters you've seen quite a few **string literals** being used in the code, like `"To-Do"` or `"A ticket description"`. They were always followed by a call to `.to_string()` or `.into()`. It's time to understand why!

#### String literals

You define a string literal by enclosing the raw text in double quotes:

```rust
let s = "Hello, world!";
```

The type of `s` is `&str`, a **reference to a string slice**.

#### Memory layout

`&str` and `String` are different types—they're not interchangeable.
Let's recall the memory layout of a `String` from our [previous exploration](../03_ticket_v1/09_heap.md). If we run:

```rust
let mut s = String::with_capacity(5);
s.push_str("Hello");
```

we'll get this scenario in memory:

```text
      +---------+--------+----------+
Stack | pointer | length | capacity | 
      |  |      |   5    |    5     |
      +--|------+--------+----------+
         |
         |
         v
       +---+---+---+---+---+
Heap:  | H | e | l | l | o |
       +---+---+---+---+---+
```

If you remember, we've [also examined](../03_ticket_v1/10_references_in_memory.md) how a `&String` is laid out in memory:

```text
     --------------------------------------
     |                                    |         
+----v----+--------+----------+      +----|----+
| pointer | length | capacity |      | pointer |
|    |    |   5    |    5     |      |         |
+----|----+--------+----------+      +---------+
     |        s                          &s 
     |       
     v       
   +---+---+---+---+---+
   | H | e | l | l | o |
   +---+---+---+---+---+
```

`&String` points to the memory location where the `String`'s metadata is stored. If we follow the pointer, we get to the heap-allocated data. In particular, we get to the first byte of the string, `H`.

What if we wanted a type that represents a **substring** of `s`? E.g. `ello` in `Hello`?

#### String slices

A `&str` is a **view** into a string, a **reference** to a sequence of UTF-8 bytes stored elsewhere. You can, for example, create a `&str` from a `String` like this:

```rust
let mut s = String::with_capacity(5);
s.push_str("Hello");
// Create a string slice reference from the `String`, 
// skipping the first byte.
let slice: &str = &s[1..];
```

In memory, it'd look like this:

```text
                    s                              slice
      +---------+--------+----------+      +---------+--------+
Stack | pointer | length | capacity |      | pointer | length |
      |    |    |   5    |    5     |      |    |    |   4    |
      +----|----+--------+----------+      +----|----+--------+
           |        s                           |  
           |                                    |
           v                                    | 
         +---+---+---+---+---+                  |
Heap:    | H | e | l | l | o |                  |
         +---+---+---+---+---+                  |
               ^                                |
               |                                |
               +--------------------------------+
```

`slice` stores two pieces of information on the stack:

- A pointer to the first byte of the slice.
- The length of the slice.

`slice` doesn't own the data, it just points to it: it's a **reference** to the `String`'s heap-allocated data. When `slice` is dropped, the heap-allocated data won't be deallocated, because it's still owned by `s`. That's why `slice` doesn't have a `capacity` field: it doesn't own the data, so it doesn't need to know how much space it was allocated for it; it only cares about the data it references.

#### `&str` vs `&String`

As a rule of thumb, use `&str` rather than `&String` whenever you need a reference to textual data. `&str` is more flexible and generally considered more idiomatic in Rust code.

If a method returns a `&String`, you're promising that there is heap-allocated UTF-8 text somewhere that **matches exactly** the one you're returning a reference to.
If a method returns a `&str`, instead, you have a lot more freedom: you're just saying that _somewhere_ there's a bunch of text data and that a subset of it matches what you need, therefore you're returning a reference to it.

### 07 Deref

In the previous exercise you didn't have to do much, did you?

Changing

```rust
impl Ticket {
    pub fn title(&self) -> &String {
        &self.title
    }
}
```

to

```rust
impl Ticket {
    pub fn title(&self) -> &str {
        &self.title
    }
}
```

was all you needed to do to get the code to compile and the tests to pass. Some alarm bells should be ringing in your head though.

#### It shouldn't work, but it does

Let's review the facts:

- `self.title` is a `String`
- `&self.title` is, therefore, a `&String`
- The output of the (modified) `title` method is `&str`

You would expect a compiler error, wouldn't you? `Expected &String, found &str` or something similar. Instead, it just works. **Why**?

#### `Deref` to the rescue

The `Deref` trait is the mechanism behind the language feature known as [**deref coercion**](https://doc.rust-lang.org/std/ops/trait.Deref.html#deref-coercion).
The trait is defined in the standard library, in the `std::ops` module:

```rust
// I've slightly simplified the definition for now.
// We'll see the full definition later on.
pub trait Deref {
    type Target;
    
    fn deref(&self) -> &Self::Target;
}
```

`type Target` is an **associated type**.
It's a placeholder for a concrete type that must be specified when the trait is implemented.

#### Deref coercion

By implementing `Deref<Target = U>` for a type `T` you're telling the compiler that `&T` and `&U` are somewhat interchangeable.
In particular, you get the following behavior:

- References to `T` are implicitly converted into references to `U` (i.e. `&T` becomes `&U`)
- You can call on `&T` all the methods defined on `U` that take `&self` as input.

There is one more thing around the dereference operator, `*`, but we don't need it yet (see `std`'s docs if you're curious).

#### `String` implements `Deref`

`String` implements `Deref` with `Target = str`:

```rust
impl Deref for String {
    type Target = str;
    
    fn deref(&self) -> &str {
        // [...]
    }
}
```

Thanks to this implementation and deref coercion, a `&String` is automatically converted into a `&str` when needed.

#### Don't abuse deref coercion

Deref coercion is a powerful feature, but it can lead to confusion.
Automatically converting types can make the code harder to read and understand. If a method with the same name is defined on both `T` and `U`, which one will be called?

We'll examine later in the course the "safest" use cases for deref coercion: smart pointers.

### 08 Sized

There's more to `&str` than meets the eye, even after having investigated deref coercion.
From our previous [discussion on memory layouts](../03_ticket_v1/10_references_in_memory.md), it would have been reasonable to expect `&str` to be represented as a single `usize` on the stack, a pointer. That's not the case though. `&str` stores some **metadata** next to the pointer: the length of the slice it points to. Going back to the example from [a previous section](06_str_slice.md):

```rust
let mut s = String::with_capacity(5);
s.push_str("Hello");
// Create a string slice reference from the `String`, 
// skipping the first byte.
let slice: &str = &s[1..];
```

In memory, we get:

```text
                    s                              slice
      +---------+--------+----------+      +---------+--------+
Stack | pointer | length | capacity |      | pointer | length |
      |    |    |   5    |    5     |      |    |    |   4    |
      +----|----+--------+----------+      +----|----+--------+
           |        s                           |  
           |                                    |
           v                                    | 
         +---+---+---+---+---+                  |
Heap:    | H | e | l | l | o |                  |
         +---+---+---+---+---+                  |
               ^                                |
               |                                |
               +--------------------------------+
```

What's going on?

#### Dynamically sized types

`str` is a **dynamically sized type** (DST).
A DST is a type whose size is not known at compile time. Whenever you have a reference to a DST, like `&str`, it has to include additional information about the data it points to. It is a **fat pointer**.
In the case of `&str`, it stores the length of the slice it points to. We'll see more examples of DSTs in the rest of the course.

#### The `Sized` trait

Rust's `std` library defines a trait called `Sized`.

```rust
pub trait Sized {
    // This is an empty trait, no methods to implement.
}
```

A type is `Sized` if its size is known at compile time. In other words, it's not a DST.

##### Marker traits

`Sized` is your first example of a **marker trait**.
A marker trait is a trait that doesn't require any methods to be implemented. It doesn't define any behavior. It only serves to **mark** a type as having certain properties. The mark is then leveraged by the compiler to enable certain behaviors or optimizations.

##### Auto traits

In particular, `Sized` is also an **auto trait**.
You don't need to implement it explicitly; the compiler implements it automatically for you based on the type's definition.

##### Examples

All the types we've seen so far are `Sized`: `u32`, `String`, `bool`, etc.

`str`, as we just saw, is not `Sized`.
`&str` is `Sized` though! We know its size at compile time: two `usize`s, one for the pointer and one for the length.

### 09 From

Let's go back to where our string journey started:

```rust
let ticket = Ticket::new(
    "A title".into(), 
    "A description".into(), 
    "To-Do".into()
);
```

We now know enough to start unpacking what `.into()` is doing here.

#### The problem

This is the signature of the `new` method:

```rust
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

We've also seen that string literals (such as `"A title"`) are of type `&str`.
We have a type mismatch here: a `String` is expected, but we have a `&str`. No magical coercion will come to save us this time; we need **to perform a conversion**.

#### `From` and `Into`

The Rust standard library defines two traits for **infallible conversions**: `From` and `Into`, in the `std::convert` module.

```rust
pub trait From<T>: Sized {
    fn from(value: T) -> Self;
}

pub trait Into<T>: Sized {
    fn into(self) -> T;
}
```

These trait definitions showcase a few concepts that we haven't seen before: **supertraits** and **implicit trait bounds**. Let's unpack those first.

##### Supertrait / Subtrait

The `From: Sized` syntax implies that `From` is a **subtrait** of `Sized`: any type that implements `From` must also implement `Sized`. Alternatively, you could say that `Sized` is a **supertrait** of `From`.

##### Implicit trait bounds

Every time you have a generic type parameter, the compiler implicitly assumes that it's `Sized`.

For example:

```rust
pub struct Foo<T> {
    inner: T,
}
```

is actually equivalent to:

```rust
pub struct Foo<T: Sized> {
    inner: T,
}
```

In the case of `From<T>`, the trait definition is equivalent to:

```rust
pub trait From<T: Sized>: Sized {
    fn from(value: T) -> Self;
}
```

In other words, _both_ `T` and the type implementing `From<T>` must be `Sized`, even though the former bound is implicit.

##### Negative trait bounds

You can opt out of the implicit `Sized` bound with a **negative trait bound**:

```rust
pub struct Foo<T: ?Sized> {
    //            ^^^^^^^
    //            This is a negative trait bound
    inner: T,
}
```

This syntax reads as "`T` may or may not be `Sized`", and it allows you to bind `T` to a DST (e.g. `Foo<str>`). It is a special case, though: negative trait bounds are exclusive to `Sized`, you can't use them with other traits.

#### `&str` to `String`

In [`std`'s documentation](https://doc.rust-lang.org/std/convert/trait.From.html#implementors) you can see which `std` types implement the `From` trait.
You'll find that `String` implements `From<&str> for String`. Thus, we can write:

```rust
let title = String::from("A title");
```

We've been primarily using `.into()`, though.
If you check out the [implementors of `Into`](https://doc.rust-lang.org/std/convert/trait.Into.html#implementors) you won't find `Into<String> for &str`. What's going on?

`From` and `Into` are **dual traits**.
In particular, `Into` is implemented for any type that implements `From` using a **blanket implementation**:

```rust
impl<T, U> Into<U> for T
where
    U: From<T>,
{
    fn into(self) -> U {
        U::from(self)
    }
}
```

If a type `U` implements `From<T>`, then `Into<U> for T` is automatically implemented. That's why we can write `let title = "A title".into();`.

#### `.into()`

Every time you see `.into()`, you're witnessing a conversion between types. What's the target type, though?

In most cases, the target type is either:

- Specified by the signature of a function/method (e.g. `Ticket::new` in our example above)
- Specified in the variable declaration with a type annotation (e.g. `let title: String = "A title".into();`)

`.into()` will work out of the box as long as the compiler can infer the target type from the context without ambiguity.

### 10 Assoc Vs Generic

Let's re-examine the definition for two of the traits we studied so far, `From` and `Deref`:

```rust
pub trait From<T> {
    fn from(value: T) -> Self;
}

pub trait Deref {
    type Target;
    
    fn deref(&self) -> &Self::Target;
}
```

They both feature type parameters.
In the case of `From`, it's a generic parameter, `T`.
In the case of `Deref`, it's an associated type, `Target`.

What's the difference? Why use one over the other?

#### At most one implementation

Due to how deref coercion works, there can only be one "target" type for a given type. E.g. `String` can only deref to `str`. It's about avoiding ambiguity: if you could implement `Deref` multiple times for a type, which `Target` type should the compiler choose when you call a `&self` method?

That's why `Deref` uses an associated type, `Target`.
An associated type is uniquely determined **by the trait implementation**. Since you can't implement `Deref` more than once, you'll only be able to specify one `Target` for a given type and there won't be any ambiguity.

#### Generic traits

On the other hand, you can implement `From` multiple times for a type, **as long as the input type `T` is different**. For example, you can implement `From` for `WrappingU32` using both `u32` and `u16` as input types:

```rust
impl From<u32> for WrappingU32 {
    fn from(value: u32) -> Self {
        WrappingU32 { inner: value }
    }
}

impl From<u16> for WrappingU32 {
    fn from(value: u16) -> Self {
        WrappingU32 { inner: value.into() }
    }
}
```

This works because `From<u16>` and `From<u32>` are considered **different traits**. There is no ambiguity: the compiler can determine which implementation to use based on type of the value being converted.

#### Case study: `Add`

As a closing example, consider the `Add` trait from the standard library:

```rust
pub trait Add<RHS = Self> {
    type Output;
    
    fn add(self, rhs: RHS) -> Self::Output;
}
```

It uses both mechanisms:

- it has a generic parameter, `RHS` (right-hand side), which defaults to `Self`
- it has an associated type, `Output`, the type of the result of the addition

##### `RHS`

`RHS` is a generic parameter to allow for different types to be added together. For example, you'll find these two implementations in the standard library:

```rust
impl Add<u32> for u32 {
    type Output = u32;
    
    fn add(self, rhs: u32) -> u32 {
      //                      ^^^
      // This could be written as `Self::Output` instead.
      // The compiler doesn't care, as long as the type you
      // specify here matches the type you assigned to `Output` 
      // right above.
      // [...]
    }
}

impl Add<&u32> for u32 {
    type Output = u32;
    
    fn add(self, rhs: &u32) -> u32 {
        // [...]
    }
}
```

This allows the following code to compile:

```rust
let x = 5u32 + &5u32 + 6u32;
```

because `u32` implements `Add<&u32>` _as well as_ `Add<u32>`.

##### `Output`

`Output` represents the type of the result of the addition.

Why do we need `Output` in the first place? Can't we just use `Self` as output, the type implementing `Add`? We could, but it would limit the flexibility of the trait. In the standard library, for example, you'll find this implementation:

```rust
impl Add<&u32> for &u32 {
    type Output = u32;

    fn add(self, rhs: &u32) -> u32 {
        // [...]
    }
}
```

The type they're implementing the trait for is `&u32`, but the result of the addition is `u32`. It would be impossible[^flexible] to provide this implementation if `add` had to return `Self`, i.e. `&u32` in this case. `Output` lets `std` decouple the implementor from the return type, thus supporting this case.

On the other hand, `Output` can't be a generic parameter. The output type of the operation **must** be uniquely determined once the types of the operands are known. That's why it's an associated type: for a given combination of implementor and generic parameters, there is only one `Output` type.

#### Conclusion

To recap:

- Use an **associated type** when the type must be uniquely determined for a given trait implementation.
- Use a **generic parameter** when you want to allow multiple implementations of the trait for the same type, with different input types.

[^flexible]: Flexibility is rarely free: the trait definition is more complex due to `Output`, and implementors have to reason about what they want to return. The trade-off is only justified if that flexibility is actually needed. Keep that in mind when designing your own traits.

### 11 Clone

In the previous chapter we introduced ownership and borrowing.
We stated, in particular, that:

- Every value in Rust has a single owner at any given time.
- When a function takes ownership of a value ("it consumes it"), the caller can't use that value anymore.

These restrictions can be somewhat limiting.
Sometimes we might have to call a function that takes ownership of a value, but we still need to use that value afterward.

```rust
fn consumer(s: String) { /* */ }

fn example() {
     let mut s = String::from("hello");
     consumer(s);
     s.push_str(", world!"); // error: value borrowed here after move
}
```

That's where `Clone` comes in.

#### `Clone`

`Clone` is a trait defined in Rust's standard library:

```rust
pub trait Clone {
    fn clone(&self) -> Self;
}
```

Its method, `clone`, takes a reference to `self` and returns a new **owned** instance of the same type.

#### In action

Going back to the example above, we can use `clone` to create a new `String` instance before calling `consumer`:

```rust
fn consumer(s: String) { /* */ }

fn example() {
     let mut s = String::from("hello");
     let t = s.clone();
     consumer(t);
     s.push_str(", world!"); // no error
}
```

Instead of giving ownership of `s` to `consumer`, we create a new `String` (by cloning `s`) and give that to `consumer` instead. `s` remains valid and usable after the call to `consumer`.

#### In memory

Let's look at what happened in memory in the example above. When `let mut s = String::from("hello");` is executed, the memory looks like this:

```text
                    s
      +---------+--------+----------+
Stack | pointer | length | capacity | 
      |  |      |   5    |    5     |
      +--|------+--------+----------+
         |
         |
         v
       +---+---+---+---+---+
Heap:  | H | e | l | l | o |
       +---+---+---+---+---+
```

When `let t = s.clone()` is executed, a whole new region is allocated on the heap to store a copy of the data:

```text
                    s                                    t
      +---------+--------+----------+      +---------+--------+----------+
Stack | pointer | length | capacity |      | pointer | length | capacity |
      |  |      |   5    |    5     |      |  |      |   5    |    5     |
      +--|------+--------+----------+      +--|------+--------+----------+
         |                                    |
         |                                    |
         v                                    v
       +---+---+---+---+---+                +---+---+---+---+---+
Heap:  | H | e | l | l | o |                | H | e | l | l | o |
       +---+---+---+---+---+                +---+---+---+---+---+
```

If you're coming from a language like Java, you can think of `clone` as a way to create a deep copy of an object.

#### Implementing `Clone`

To make a type `Clone`-able, we have to implement the `Clone` trait for it.
You almost always implement `Clone` by deriving it:

```rust
#[derive(Clone)]
struct MyType {
    // fields
}
```

The compiler implements `Clone` for `MyType` as you would expect: it clones each field of `MyType` individually and then constructs a new `MyType` instance using the cloned fields.
Remember that you can use `cargo expand` (or your IDE) to explore the code generated by `derive` macros.

### 12 Copy

Let's consider the same example as before, but with a slight twist: using `u32` rather than `String` as a type.

```rust
fn consumer(s: u32) { /* */ }

fn example() {
     let s: u32 = 5;
     consumer(s);
     let t = s + 1;
}
```

It'll compile without errors! What's going on here? What's the difference between `String` and `u32` that makes the latter work without `.clone()`?

#### `Copy`

`Copy` is another trait defined in Rust's standard library:

```rust
pub trait Copy: Clone { }
```

It is a marker trait, just like `Sized`.

If a type implements `Copy`, there's no need to call `.clone()` to create a new instance of the type: Rust does it **implicitly** for you. `u32` is an example of a type that implements `Copy`, which is why the example above compiles without errors: when `consumer(s)` is called, Rust creates a new `u32` instance by performing a **bitwise copy** of `s`, and then passes that new instance to `consumer`. It all happens behind the scenes, without you having to do anything.

#### What can be `Copy`?

`Copy` is not equivalent to "automatic cloning", although it implies it. Types must meet a few requirements in order to be allowed to implement `Copy`.

First of all, it must implement `Clone`, since `Copy` is a subtrait of `Clone`. This makes sense: if Rust can create a new instance of a type _implicitly_, it should also be able to create a new instance _explicitly_ by calling `.clone()`.

That's not all, though. A few more conditions must be met:

1. The type doesn't manage any _additional_ resources (e.g. heap memory, file handles, etc.) beyond the `std::mem::size_of` bytes that it occupies in memory.
2. The type is not a mutable reference (`&mut T`).

If both conditions are met, then Rust can safely create a new instance of the type by performing a **bitwise copy** of the original instance—this is often referred to as a `memcpy` operation, after the C standard library function that performs the bitwise copy.

##### Case study 1: `String`

`String` is a type that doesn't implement `Copy`. Why? Because it manages an additional resource: the heap-allocated memory buffer that stores the string's data.

Let's imagine that Rust allowed `String` to implement `Copy`. Then, when a new `String` instance is created by performing a bitwise copy of the original instance, both the original and the new instance would point to the same memory buffer:

```text
              s                                 copied_s
+---------+--------+----------+      +---------+--------+----------+
| pointer | length | capacity |      | pointer | length | capacity |
|  |      |   5    |    5     |      |  |      |   5    |    5     |
+--|------+--------+----------+      +--|------+--------+----------+
   |                                    |
   |                                    |
   v                                    |
 +---+---+---+---+---+                  |
 | H | e | l | l | o |                  |
 +---+---+---+---+---+                  |
   ^                                    |
   |                                    |
   +------------------------------------+
```

This is bad! Both `String` instances would try to free the memory buffer when they go out of scope, leading to a double-free error. You could also create two distinct `&mut String` references that point to the same memory buffer, violating Rust's borrowing rules.

##### Case study 2: `u32`

`u32` implements `Copy`. All integer types do, in fact. An integer is "just" the bytes that represent the number in memory. There's nothing more! If you copy those bytes, you get another perfectly valid integer instance. Nothing bad can happen, so Rust allows it.

##### Case study 3: `&mut u32`

When we introduced ownership and mutable borrows, we stated one rule quite clearly: there can only ever be _one_ mutable borrow of a value at any given time. That's why `&mut u32` doesn't implement `Copy`, even though `u32` does.

If `&mut u32` implemented `Copy`, you could create multiple mutable references to the same value and modify it in multiple places at the same time. That'd be a violation of Rust's borrowing rules! It follows that `&mut T` never implements `Copy`, no matter what `T` is.

#### Implementing `Copy`

In most cases, you don't need to manually implement `Copy`. You can just derive it, like this:

```rust
#[derive(Copy, Clone)]
struct MyStruct {
    field: u32,
}
```

### 13 Drop

When we introduced [destructors](../03_ticket_v1/11_destructor.md), we mentioned that the `drop` function:

1. reclaims the memory occupied by the type (i.e. `std::mem::size_of` bytes)
2. cleans up any additional resources that the value might be managing (e.g. the heap buffer of a `String`)

Step 2. is where the `Drop` trait comes in.

```rust
pub trait Drop {
    fn drop(&mut self);
}
```

The `Drop` trait is a mechanism for you to define _additional_ cleanup logic for your types, beyond what the compiler does for you automatically. Whatever you put in the `drop` method will be executed when the value goes out of scope.

#### `Drop` and `Copy`

When talking about the `Copy` trait, we said that a type can't implement `Copy` if it manages additional resources beyond the `std::mem::size_of` bytes that it occupies in memory.

You might wonder: how does the compiler know if a type manages additional resources? That's right: `Drop` trait implementations! If your type has an explicit `Drop` implementation, the compiler will assume that your type has additional resources attached to it and won't allow you to implement `Copy`.

```rust
// This is a unit struct, i.e. a struct with no fields.
#[derive(Clone, Copy)]
struct MyType;

impl Drop for MyType {
    fn drop(&mut self) {
       // We don't need to do anything here,
       // it's enough to have an "empty" Drop implementation
    }
}
```

The compiler will complain with this error message:

```text
error[E0184]: the trait `Copy` cannot be implemented for this type; 
              the type has a destructor
  --> src/lib.rs:2:17
   |
2 | #[derive(Clone, Copy)]
   |                 ^^^^ `Copy` not allowed on types with destructors
```

### 14 Outro

We've covered quite a few different traits in this chapter—and we've only scratched the surface! It may feel like you have a lot to remember, but don't worry: you'll bump into these traits so often when writing Rust code that they'll soon become second nature.

#### Closing thoughts

Traits are powerful, but don't overuse them.
A few guidelines to keep in mind:

- Don't make a function generic if it is always invoked with a single type. It introduces indirection in your codebase, making it harder to understand and maintain.
- Don't create a trait if you only have one implementation. It's a sign that the trait is not needed.
- Implement standard traits for your types (`Debug`, `PartialEq`, etc.) whenever it makes sense. It will make your types more idiomatic and easier to work with, unlocking a lot of functionality provided by the standard library and ecosystem crates.
- Implement traits from third-party crates if you need the functionality they unlock within their ecosystem.
- Beware of making code generic solely to use mocks in your tests. The maintainability cost of this approach can be high, and it's often better to use a different testing strategy. Check out the [testing masterclass](https://github.com/mainmatter/rust-advanced-testing-workshop) for details on high-fidelity testing.

#### Testing your knowledge

Before moving on, let's go through one last exercise to consolidate what we've learned. You'll have minimal guidance this time—just the exercise description and the tests to guide you.
