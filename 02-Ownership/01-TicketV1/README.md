# Rust for Python Data Engineers — TicketV1: Structs & Ownership

*The most important workshop in this course. Master Rust's ownership system — the concept that makes Rust unique — by building a ticket tracking system.*

> **Test-driven approach**: This project includes a Cargo project with progressive unit tests. Each function in `workshop/src/lib.rs` starts as a `todo!()` stub. As you follow each section, replace `todo!()` with real code and run `cd workshop && cargo test` to watch the pass count grow. Your goal: **all 20 tests pass**.

---

## Why Model Tickets with Structs?

**Python pain:** A function that takes a `list` of `dict`s can mutate the caller's list silently — there is no way to know who "owns" the data, and the GC never tells you. A 10,000-line ETL pipeline can lose data integrity to one accidental `.append()`.

**Rust fix:** Every value has **exactly one owner**. Pass by **move** (transfer ownership) or **borrow** (`&T` for read-only, `&mut T` for exclusive write). The compiler enforces these rules — no GC, no aliasing, no silent mutation. The same code as a `Ticket` struct is type-checked and validated at compile time:

```rust
fn add_outlier(data: Vec<i32>) -> Vec<i32> {
    let mut d = data;   // moved in
    d.push(999999);     // OK, we own it
    d                   // moved out
}
// records is now invalid — the compiler says so
```

## At a Glance

| # | Concept | Rust | Python | Why it matters |
|---|---------|------|--------|----------------|
| 1 | Structs | `struct Ticket { title: String }` | `class` / `@dataclass` | Custom data types with explicit field layout |
| 2 | Methods | `impl Ticket { fn title(&self) -> &String }` | methods inside `class` | Behavior attached to data type, with explicit `&self` |
| 3 | Modules & Visibility | `pub`, `mod` | `_` naming convention | Encapsulation is compiler-enforced, not convention |
| 4 | Ownership | one owner per value | GC at runtime | Memory safety *without* a garbage collector |
| 5 | Move Semantics | `let b = a` invalidates `a` | `b = a` creates alias | Prevents accidental aliasing of mutable data |
| 6 | References & Borrowing | `&T` (read) / `&mut T` (write) | pass-by-reference (any) | "Many readers OR one writer" enforced at compile time |
| 7 | Stack vs Heap | primitives on stack, `String`/`Vec` on heap | all objects on heap | Stack data is faster; explicit layout helps pipelines |
| 8 | Validation | `panic!` in `Ticket::new` | `raise ValueError` | "Make invalid states unrepresentable" — never accept a bad ticket |
| 9 | Encapsulation | private fields + `pub fn new` | `_` convention | Construction must go through the validator |
| 10 | Drop Trait | `Drop::drop()` runs at end of scope | `__del__` (non-deterministic) | Cleanup happens *immediately*, not whenever GC decides |
| 11 | String Type | `String` (heap, growable, ptr+len+cap) | `str` (immutable) | Mutable growable text, but with explicit capacity |
| 12 | Scopes | `{}` blocks trigger `Drop` | scope exit ≠ cleanup | Memory freed the moment a value is no longer needed |
| 13 | Three `self` Forms | `self` / `&self` / `&mut self` | `self` only | Signature *shows* whether a method consumes, reads, or writes |

---

## Table of Contents

1. [Project Overview](#1-project-overview)
2. [Prerequisites](#2-prerequisites)
3. [Running the Python Version](#3-running-the-python-version)
4. [Concept: Structs — Rust's Custom Types](#4-concept-structs--rusts-custom-types)
5. [Concept: Methods with `impl`](#5-concept-methods-with-impl)
6. [Concept: Validation — Enforcing Invariants](#6-concept-validation--enforcing-invariants)
7. [Concept: Modules and Visibility](#7-concept-modules-and-visibility)
8. [Concept: Encapsulation](#8-concept-encapsulation)
9. [Concept: Ownership — The Key to Rust](#9-concept-ownership--the-key-to-rust)
10. [Concept: Stack vs Heap — Where Data Lives](#10-concept-stack-vs-heap--where-data-lives)
11. [Concept: References and Borrowing](#11-concept-references-and-borrowing)
12. [Concept: Destructors and Drop](#12-concept-destructors-and-drop)
13. [Putting It All Together — The Complete Ticket System](#13-putting-it-all-together--the-complete-ticket-system)
14. [Exercises](#14-exercises)
15. [Summary](#15-summary)
16. [Appendix: Original Step-by-Step Tutorial](#appendix-original-step-by-step-tutorial)

---

## 1. Project Overview

We'll build a **ticket tracking system** (like Jira or Trello) that:

- Creates tickets with title, description, and status
- Validates ticket data (title not empty, status is valid)
- Controls access to fields via getters and setters
- Enforces rules at compile time using Rust's type system

### What You'll Learn

| Rust Concept | Python Equivalent | Why It Matters |
|---|---|---|
| `struct` | `class` (dataclass) | Model data in pipelines |
| `impl` | Methods inside class | Attach behavior to data |
| `pub` / `mod` | Public/private convention | Organize large codebases |
| **Ownership** | Nothing — Python uses GC | **Memory safety without GC** |
| **Borrowing** | Nothing — Python uses GC | **Zero-cost abstractions** |
| Stack vs Heap | `id()` / `is` | Understand performance |
| `Drop` | `__del__` / context manager | Resource cleanup |
| References `&` | Pass-by-reference semantics | Efficient data access |

---

## 2. Prerequisites

- Completed [Basic Calculator](../01-Foundations/03-BasicCalculator/README.md)
- Understand integers, `if/else`, loops
- Familiar with `cd workshop && cargo run`, `cd workshop && cargo test`

---

## 3. Running the Python Version

The Python version (`project.py`) shows what we're building:

```bash
cd rustlings-workshop/3-TicketV1
python project.py
```

---

## 4. Concept: Structs — Rust's Custom Types

### From Python Classes to Rust Structs

```python
# Python — class with __init__
class Ticket:
    def __init__(self, title, description, status):
        self.title = title
        self.description = description
        self.status = status
```

```rust
// Rust — struct definition
struct Ticket {
    title: String,
    description: String,
    status: String,
}
```

### Creating an Instance

```python
# Python — just call the class
ticket = Ticket("Bug", "Fix this bug", "Open")
```

```rust
// Rust — use struct literal syntax
let ticket = Ticket {
    title: String::from("Bug"),
    description: String::from("Fix this bug"),
    status: String::from("Open"),
};
```

### Accessing Fields

```python
# Python
print(ticket.title)
ticket.title = "New title"
```

```rust
// Rust
println!("{}", ticket.title);
ticket.title = String::from("New title");  // Only if mutable!
```

### Memory Layout of a Struct

```
struct Ticket {
    title: String,       // 24 bytes (ptr + len + cap)
    description: String, // 24 bytes
    status: String,      // 24 bytes
}
// Total: 72 bytes on stack

Stack:
┌──────────────────────────────┐
│ Ticket instance (72 bytes)   │
│ ┌──────────────────────────┐ │
│ │ title:                   │ │
│ │   [ptr] ───→ "Bug" (heap)│ │
│ │   [len:3] [cap:3]        │ │
│ ├──────────────────────────┤ │
│ │ description:             │ │
│ │   [ptr] ───→ "Fix" (heap)│ │
│ │   [len:3] [cap:3]        │ │
│ ├──────────────────────────┤ │
│ │ status:                  │ │
│ │   [ptr] ───→ "Open" (h.) │ │
│ │   [len:4] [cap:4]        │ │
│ └──────────────────────────┘ │
└──────────────────────────────┘
```

### Exercise: Define Your First Struct

```rust
// Step 1: Define the struct
struct Ticket {
    title: String,
    description: String,
    status: String,
}

// Step 2: Create an instance
fn main() {
    let ticket = Ticket {
        title: String::from("Build login page"),
        description: String::from("Create the login page UI"),
        status: String::from("In Progress"),
    };

    // Step 3: Access fields
    println!("Title: {}", ticket.title);
    println!("Status: {}", ticket.status);
}
```

---

## 5. Concept: Methods with `impl`

### Python vs Rust Methods

```python
# Python
class Ticket:
    def __init__(self, title, desc, status):
        self.title = title
        self.desc = desc
        self.status = status

    def is_open(self):      # 'self' is implicit
        return self.status == "Open"
```

```rust
// Rust
struct Ticket {
    title: String,
    description: String,
    status: String,
}

impl Ticket {
    // Method — takes &self (borrow, don't take ownership)
    fn is_open(&self) -> bool {
        self.status == "Open"
    }

    // Static method — no self (like @classmethod without cls)
    fn default() -> Ticket {
        Ticket {
            title: String::from("Default"),
            description: String::from(""),
            status: String::from("Open"),
        }
    }
}
```

### Calling Methods

```rust
let ticket = Ticket::default();          // Static method call
let open = ticket.is_open();             // Method call
let open2 = Ticket::is_open(&ticket);    // Function call syntax (same thing)
```

### The Three Forms of `self`

| Parameter | Owns? | Can Modify? | Use Case |
|---|---|---|---|
| `self` | Takes ownership (moves) | Yes | Consuming the value |
| `&self` | Borrows (shared ref) | No | Reading only |
| `&mut self` | Borrows (mutable ref) | Yes | Mutating |

```rust
impl Ticket {
    // Read-only access
    fn title(&self) -> &String {
        &self.title
    }

    // Mutable access
    fn set_title(&mut self, new_title: String) {
        self.title = new_title;
    }

    // Consumes the ticket
    fn into_summary(self) -> String {
        format!("{}: {}", self.title, self.status)
        // `self` is gone after this call
    }
}
```

---

## 6. Concept: Validation — Enforcing Invariants

### The Problem

```rust
let ticket = Ticket {
    title: String::from(""),        // Empty title — bad!
    description: String::from(""),
    status: String::from("NOPE"),   // Invalid status — bad!
};
```

### Solution: Constructor with Validation

```rust
impl Ticket {
    fn new(title: String, description: String, status: String) -> Ticket {
        // Validate title
        if title.is_empty() {
            panic!("Title cannot be empty");
        }
        if title.len() > 50 {
            panic!("Title too long (max 50 chars)");
        }

        // Validate description
        if description.is_empty() {
            panic!("Description cannot be empty");
        }
        if description.len() > 500 {
            panic!("Description too long (max 500 chars)");
        }

        // Validate status
        let valid_statuses = ["Open", "In Progress", "Resolved", "Closed"];
        if !valid_statuses.contains(&status.as_str()) {
            panic!("Invalid status: {}", status);
        }

        Ticket { title, description, status }
    }
}
```

### Python Comparison

```python
# Python — validation in __init__
class Ticket:
    VALID_STATUSES = ["Open", "In Progress", "Resolved", "Closed"]

    def __init__(self, title, description, status):
        if not title:
            raise ValueError("Title cannot be empty")
        if not description:
            raise ValueError("Description cannot be empty")
        if status not in self.VALID_STATUSES:
            raise ValueError(f"Invalid status: {status}")
        self.title = title
        self.description = description
        self.status = status
```

| Python | Rust |
|---|---|
| `raise ValueError` | `panic!("message")` |
| `if not title:` | `if title.is_empty()` |
| `status not in list` | `!list.contains(&status)` |

---

## 7. Concept: Modules and Visibility

### What is a Module?

A module groups related code under a namespace:

```rust
// src/lib.rs
mod ticket {
    pub struct Ticket {
        pub title: String,
        description: String,  // private!
        status: String,       // private!
    }
}
```

### Module File Structure

```
src/
  lib.rs           ← Root module (crate root)
  ticket.rs        ← module `ticket`
  └── ...          ← or ticket/mod.rs with submodules
```

### Visibility Rules

| Keyword | Accessible from |
|---|---|
| `(none)` | Private — only within current module |
| `pub` | Public — accessible everywhere |
| `pub(crate)` | Public within the crate |
| `pub(super)` | Public within parent module |

```rust
mod ticket {
    pub struct Ticket {
        pub title: String,       // Anyone can read/write
        pub(crate) description: String, // Only within this crate
        status: String,          // Private even within crate!
    }

    pub fn create_ticket(title: String) -> Ticket {
        // This function is public — anyone can call it
        Ticket {
            title,
            description: String::new(),
            status: String::from("Open"),
        }
    }
}
```

### Python vs Rust Visibility

```python
# Python — naming convention
class Ticket:
    def __init__(self):
        self.title = "Bug"       # Public
        self._status = "Open"    # "Private" (convention)
        self.__secret = "x"      # Name mangled (still accessible)
```

```rust
// Rust — compiler-enforced
struct Ticket {
    pub title: String,           // Public
    status: String,              // Private — can't access from outside
}
```

> **Key difference:** Python's privacy is by convention. Rust's is enforced by the compiler.

---

## 8. Concept: Encapsulation

### The Problem

Even with `Ticket::new` doing validation, nothing stops someone from:

```rust
let ticket = Ticket {
    title: String::from(""),    // Bypasses validation!
    description: String::from(""),
    status: String::from("Bad"),
};
```

### The Solution

Make fields private so they can **only** be set through the constructor:

```rust
// src/ticket.rs
pub struct Ticket {
    title: String,       // Private — not `pub`
    description: String, // Private
    status: String,      // Private
}

impl Ticket {
    pub fn new(title: String, description: String, status: String) -> Ticket {
        // Validation logic...
        Ticket { title, description, status }
    }
}
```

Now users **must** use `Ticket::new()`:

```rust
// This won't compile:
let t = Ticket { title: "...", description: "...", status: "..." };
// ❌ ERROR: field `title` of struct `Ticket` is private

// This works:
let t = Ticket::new("...".into(), "...".into(), "...".into());
```

### Why Encapsulation Matters for Data Engineers

In data pipelines, you want to ensure data integrity:

```rust
// Encapsulated data row — can only be created with valid data
pub struct DataRow {
    id: u64,
    timestamp: i64,
    value: f64,
}

impl DataRow {
    pub fn new(id: u64, timestamp: i64, value: f64) -> Result<DataRow, String> {
        if timestamp < 0 {
            return Err("Negative timestamp".into());
        }
        if value.is_nan() || value.is_infinite() {
            return Err("Invalid value".into());
        }
        Ok(DataRow { id, timestamp, value })
    }
}
```

---

## 9. Concept: Ownership — The Key to Rust

### What Is Ownership?

**Every value in Rust has exactly one owner.** When the owner goes out of scope, the value is dropped (memory freed).

### The Three Rules

1. **Each value has exactly one owner**
2. **When the owner goes out of scope, the value is dropped**
3. **There can be many readers OR one writer at a time**

### Ownership Transfer (Move)

```python
# Python — multiple references to same object
a = [1, 2, 3]
b = a            # b and a both point to the SAME list
a.append(4)      # b is also affected!
print(b)         # [1, 2, 3, 4]
```

```rust
// Rust — ownership MOVES
let a = vec![1, 2, 3];
let b = a;           // Ownership moves from a to b
// println!("{:?}", a); // ❌ ERROR: a is no longer valid!
println!("{:?}", b);    // ✅ b owns the data now
```

### Visual: Move in Memory

```
Before move:
a ──→ [1, 2, 3] (heap)

After `let b = a;`:
a ──→ (invalid — compiler prevents access)
b ──→ [1, 2, 3] (heap)
```

### Why Does This Matter?

No garbage collector needed! When `b` goes out of scope, the memory is freed instantly — no GC pause.

```python
# Python — GC needs to figure out when to clean up
def process():
    data = load_large_dataset()  # Millions of rows
    result = transform(data)
    return result
    # `data` might be freed now, or later — GC decides
```

```rust
// Rust — deterministic cleanup
fn process() -> Vec<f64> {
    let data = load_large_dataset();  // Millions of rows
    let result = transform(data);    // Ownership moves to transform
    // `data` is gone! transform now owns it
    result  // Ownership moves to caller
}  // result is dropped here (unless caller takes ownership)
```

### The Ticket Problem

```rust
impl Ticket {
    // BAD: takes ownership of self!
    fn title(self) -> String {
        self.title
    }
}

fn main() {
    let ticket = Ticket::new("Bug".into(), "Fix it".into(), "Open".into());
    let title = ticket.title();   // ticket is CONSUMED here!
    // println!("{}", ticket.status()); // ❌ ERROR: ticket was moved!
}
```

### Solution: Borrow Instead of Move

```rust
impl Ticket {
    // GOOD: borrows self (doesn't take ownership)
    fn title(&self) -> &String {
        &self.title
    }
}

fn main() {
    let ticket = Ticket::new("Bug".into(), "Fix it".into(), "Open".into());
    let title = ticket.title();   // ticket is BORROWED, not moved
    println!("{}", ticket.status()); // ✅ Still works!
}
```

---

## 10. Concept: Stack vs Heap — Where Data Lives

### The Stack

- **LIFO** (Last In, First Out)
- Super fast allocation/deallocation
- Size must be known at compile time
- Used for: local variables, function arguments

```
Stack layout when calling nested functions:

    main() calls greet() calls format()

    ┌──────────────────────┐
    │ format's frame       │
    ├──────────────────────┤
    │ greet's frame        │
    │   name: ptr → "Alice"│
    ├──────────────────────┤
    │ main's frame         │
    │   greeting: ptr      │
    └──────────────────────┘
    ↑ stack pointer
```

### The Heap

- Dynamic memory — slower to allocate
- Size can change at runtime
- Used for: `String`, `Vec<T>`, boxed values
- Must be freed when no longer needed

```
Heap layout for a String "Hello":

Stack (3 × 8 bytes = 24 bytes):   Heap:
┌──────────────────────────┐      ┌──────────────────┐
│ String metadata          │      │ 'H' │ 'e' │ 'l'  │
│ [ptr] ───────────────────────→  │ 'l' │ 'o' │      │
│ [len: 5]                  │      └──────────────────┘
│ [cap: 5]                  │
└──────────────────────────┘
```

### Python vs Rust: Memory Management

```python
# Python — everything on heap, GC manages
def process():
    data = [1, 2, 3]  # List object on heap, refcounted
    # ... Python tracks references, frees when refcount=0
```

```rust
// Rust — stack by default, explicit heap
fn process() {
    let x = 42;                     // i32 — stack allocated
    let s = String::from("hello");  // String — metadata on stack, chars on heap
    let v = vec![1, 2, 3];          // Vec — metadata on stack, elements on heap
    // When function returns, x, s, v all cleaned up (no GC needed)
}
```

### `size_of` — How Big Is Each Type?

```rust
println!("i32:    {} bytes", std::mem::size_of::<i32>());      // 4
println!("f64:    {} bytes", std::mem::size_of::<f64>());      // 8
println!("bool:   {} bytes", std::mem::size_of::<bool>());     // 1
println!("String: {} bytes", std::mem::size_of::<String>());   // 24 (on stack)
println!("&String:{} bytes", std::mem::size_of::<&String>());  // 8 (pointer size)
println!("Vec<i32>:{} bytes", std::mem::size_of::<Vec<i32>>());// 24
```

> A `String` is only 24 bytes on the stack — it's the **pointer to the heap** data, plus length and capacity.

---

## 11. Concept: References and Borrowing

### What Is a Reference?

A reference (`&T`) is a **non-owning pointer**. It lets you access data without taking ownership.

```rust
let s = String::from("hello");
let r = &s;           // r BORROWS s — doesn't own it
println!("{}", r);    // "hello"
println!("{}", s);    // ✅ Still valid! s still owns the data
```

### Visual: Reference in Memory

```
let s = String::from("Hey");
let r = &s;

Stack:
┌──────────────────────────────────────┐
│ s (String, 24 bytes):                │
│   ptr:  ──────────────────┐          │
│   len: 3                  │          │
│   cap: 5                  │          │
├──────────────────────────────────────┤
│ r (&String, 8 bytes):     │          │
│   ptr: ───────────────────┘          │
└──────────────────────────────────────┘
          │
          v
Heap:
┌──────────────────────────────┐
│ 'H' │ 'e' │ 'y' │   │   │   │
└──────────────────────────────┘
```

### Mutable References (`&mut`)

```rust
let mut s = String::from("hello");
let r = &mut s;       // Mutable borrow
r.push_str(", world"); // Modify through the reference
println!("{}", r);    // "hello, world"
```

### The Borrowing Rules (MOST IMPORTANT)

> **At any given time, you can have EITHER:**
> - **One mutable reference, OR**
> - **Any number of immutable references**

```rust
let mut s = String::from("hello");

let r1 = &s;     // ✅ One immutable ref
let r2 = &s;     // ✅ Another immutable ref — fine!
println!("{}, {}", r1, r2);
// r1 and r2 are no longer used here

let r3 = &mut s;  // ✅ Mutable ref — fine since r1, r2 are done
r3.push_str(" world");
```

```rust
let mut s = String::from("hello");

let r1 = &s;      // ✅ Immutable borrow
let r2 = &mut s;  // ❌ ERROR: can't borrow as mutable while immutable borrow exists!

println!("{}", r1); // r1 is still live here
```

### Why These Rules?

**Data races** — two threads reading and writing the same memory unpredictably — are eliminated at compile time.

```python
# Python — possible data race (GIL helps but doesn't eliminate)
def update(list, index, value):
    list[index] = value  # What if another thread is reading?

# This runs fine until it doesn't
```

```rust
// Rust — data race caught at compile time
let mut data = vec![1, 2, 3];
let r1 = &data;
let r2 = &mut data;  // ❌ Compiler: "You already have an immutable borrow!"
// If this compiled, r1 could see data change under it
```

### The Three Forms of `self` Revisited

```rust
impl Ticket {
    // 1. Immutable borrow: read-only, can have many
    fn title(&self) -> &String { &self.title }

    // 2. Mutable borrow: read+write, exclusive
    fn set_title(&mut self, new: String) { self.title = new; }

    // 3. Move: consumes self, can't use afterward
    fn consume(self) -> String { self.title }
}
```

| Method signature | Can call on `ticket: Ticket` | Can call on `&ticket` | Can call on `&mut ticket` |
|---|---|---|---|
| `fn title(&self)` | ✅ | ✅ | ✅ |
| `fn set_title(&mut self)` | ✅ (but can't use after) | ❌ | ✅ |
| `fn consume(self)` | ✅ (but can't use after) | ❌ | ❌ |

---

## 12. Concept: Destructors and Drop

> **Recap**: The `Drop` trait and RAII cleanup were taught in depth in [04-OBRM §4 — The Drop Trait](../04-OBRM/README.md#4-concept-the-drop-trait--automatic-cleanup). Read that first for the full conceptual coverage (4 worked examples, ASCII lifecycle diagram, data-engineering patterns).

The only point that matters for *this* project is: **Rust calls `drop` on every value when it goes out of scope, in reverse order of declaration.** This is what makes the `Ticket` type's resource cleanup automatic — we never had to write a `close()` call.

```rust
// What the compiler does behind the scenes for any `let`:
fn main() {
    let x = String::from("hello");
    let y = String::from("world");
    drop(y);  // inserted at end of scope
    drop(x);  // in reverse order of declaration
}
```

The [04-OBRM workshop](../04-OBRM/README.md) covers the `impl Drop for YourType { fn drop(&mut self) { ... } }` syntax, the resource-lifecycle diagram, and the data-engineering cleanup patterns. The two `impl Drop` examples in this file's `Putting It All Together` (the `File` and `Dataset` types) come directly from that teaching.

---

## 13. Putting It All Together — The Complete Ticket System

### Project Structure

```
3-TicketV1/
  src/
    lib.rs       ← Public API, re-exports
    ticket.rs    ← Ticket struct and implementation
  Cargo.toml
```

### `src/ticket.rs`

```rust
/// Represents a ticket in the tracking system.
pub struct Ticket {
    title: String,
    description: String,
    status: String,
}

impl Ticket {
    /// Create a new Ticket with validation.
    pub fn new(title: String, description: String, status: String) -> Ticket {
        // Validate title
        if title.is_empty() {
            panic!("Title cannot be empty");
        }
        if title.len() > 50 {
            panic!("Title too long (max 50 chars)");
        }
        if title.contains('\n') {
            panic!("Title cannot contain newlines");
        }

        // Validate description
        if description.is_empty() {
            panic!("Description cannot be empty");
        }
        if description.len() > 500 {
            panic!("Description too long (max 500 chars)");
        }

        // Validate status
        let valid = ["Open", "In Progress", "Resolved", "Closed"];
        if !valid.contains(&status.as_str()) {
            panic!("Invalid status: '{}'", status);
        }

        Ticket { title, description, status }
    }

    // Getters — borrow, don't own
    pub fn title(&self) -> &String { &self.title }
    pub fn description(&self) -> &String { &self.description }
    pub fn status(&self) -> &String { &self.status }

    // Setter — mutable borrow
    pub fn set_title(&mut self, title: String) {
        if title.is_empty() {
            panic!("Title cannot be empty");
        }
        if title.len() > 50 {
            panic!("Title too long (max 50 chars)");
        }
        self.title = title;
    }

    pub fn set_description(&mut self, description: String) {
        if description.is_empty() {
            panic!("Description cannot be empty");
        }
        if description.len() > 500 {
            panic!("Description too long (max 500 chars)");
        }
        self.description = description;
    }

    pub fn set_status(&mut self, status: String) {
        let valid = ["Open", "In Progress", "Resolved", "Closed"];
        if !valid.contains(&status.as_str()) {
            panic!("Invalid status: '{}'", status);
        }
        self.status = status;
    }
}
```

### `workshop/src/lib.rs`

```rust
mod ticket;

pub use ticket::Ticket;
```

### `workshop/src/main.rs`

```rust
use ticket_system::Ticket;

fn main() {
    // Create a validated ticket
    let mut ticket = Ticket::new(
        String::from("Fix login bug"),
        String::from("Users cannot log in with SSO"),
        String::from("Open"),
    );

    // Read through getters
    println!("Title: {}", ticket.title());
    println!("Status: {}", ticket.status());

    // Update through setters
    ticket.set_status(String::from("In Progress"));
    println!("New status: {}", ticket.status());

    // Borrow checker in action
    let title = ticket.title();       // Immutable borrow
    let status = ticket.status();     // Another immutable borrow — OK
    println!("{title} — {status}");
    // Immutable borrows end here

    ticket.set_status(String::from("Resolved"));  // Mutable borrow — OK now
}
```

---

## 14. Exercises

### Exercise 1: Fix the Move Error

```rust
struct Ticket {
    title: String,
}

impl Ticket {
    // Fix this method so it doesn't consume the ticket
    fn title(self) -> String {
        self.title
    }
}

fn main() {
    let t = Ticket { title: String::from("Bug") };
    let title = t.title();
    println!("{:?}", t.title());  // ❌ ERROR: t was moved
}
```

<details>
<summary>Solution</summary>

```rust
fn title(&self) -> &String {
    &self.title
}
```
</details>

### Exercise 2: Add a Priority Field

Add a `priority` field to `Ticket`:

1. Add `priority: String` to the struct
2. Validate it in `new()` — must be "Low", "Medium", or "High"
3. Add getter and setter
4. Ensure encapsulation (field is private)

<details>
<summary>Solution</summary>

```rust
pub struct Ticket {
    title: String,
    description: String,
    status: String,
    priority: String,  // NEW
}

impl Ticket {
    pub fn new(title: String, description: String, status: String, priority: String) -> Ticket {
        // ... existing validation ...
        let valid_priorities = ["Low", "Medium", "High"];
        if !valid_priorities.contains(&priority.as_str()) {
            panic!("Invalid priority: '{}'", priority);
        }
        Ticket { title, description, status, priority }
    }

    pub fn priority(&self) -> &String { &self.priority }

    pub fn set_priority(&mut self, priority: String) {
        let valid = ["Low", "Medium", "High"];
        if !valid.contains(&priority.as_str()) {
            panic!("Invalid priority: '{}'", priority);
        }
        self.priority = priority;
    }
}
```
</details>

### Exercise 3: Ownership Chain

Trace the ownership in this code — what's valid and what's not?

```rust
fn consume_string(s: String) -> usize {
    s.len()
    // s is dropped here
}

fn main() {
    let a = String::from("hello");
    let b = a;                     // What happens to a?
    let len = consume_string(b);   // What happens to b?
    // Can we use a here? Can we use b here?
    println!("Length: {}", len);
}
```

<details>
<summary>Answer</summary>

```rust
let a = String::from("hello");
let b = a;                     // a's ownership MOVES to b. a is dead.
let len = consume_string(b);   // b's ownership MOVES to consume_string. b is dead.
// Can't use a or b — both were moved
println!("Length: {}", len);   // ✅ len is valid (copy)
```
</details>

---

## 15. Summary

### Concept Reference

| Concept | Description | Python Equivalent |
|---|---|---|
| `struct` | Define a custom type | `class` / `dataclass` |
| `impl` | Attach methods to a struct | Methods inside class |
| `pub` / private | Visibility control | Naming convention (`_`) |
| **Ownership** | Each value has exactly one owner | Nothing (GC) |
| **Move** | Transfer ownership | `b = a` (both reference same object) |
| **Borrow** (`&`) | Temporary, non-owning access | Pass-by-reference |
| **Mutable borrow** (`&mut`) | Exclusive, writable access | N/A (everything can be mutated) |
| **Scope** | Region where a variable is alive | Variable lifetime |
| **Drop** | Cleanup when owner goes out of scope | `__del__` (unreliable) |
| **Stack** | Fast, fixed-size memory | Local variables |
| **Heap** | Dynamic, slower memory | All objects |

### The Ownership Rules (Memorize These)

1. **Each value has exactly one owner**
2. **Either: one mutable reference, OR many immutable references**
3. **References must always be valid** (no dangling pointers)

### Further Reading

The [Appendix](#appendix-original-step-by-step-tutorial) at the end of this document contains the original step-by-step lesson files merged into this README.

### Next Project

Proceed to [4-Traits](../02-Ownership/02-Traits/README.md) to learn about **traits** — Rust's version of interfaces and protocols.

---

## Appendix: Original Step-by-Step Tutorial

### 00_intro

# Modelling A Ticket

The first chapter should have given you a good grasp over some of Rust's primitive types, operators and
basic control flow constructs.\
In this chapter we'll go one step further and cover what makes Rust truly unique: **ownership**.\
Ownership is what enables Rust to be both memory-safe and performant, with no garbage collector.

As our running example, we'll use a (JIRA-like) ticket, the kind you'd use to track bugs, features, or tasks in
a software project.\
We'll take a stab at modeling it in Rust. It'll be the first iteration—it won't be perfect nor very idiomatic
by the end of the chapter. It'll be enough of a challenge though!\
To move forward you'll have to pick up several new Rust concepts, such as:

- `struct`s, one of Rust's ways to define custom types
- Ownership, references and borrowing
- Memory management: stack, heap, pointers, data layout, destructors
- Modules and visibility
- Strings

### 01_struct

# Structs

We need to keep track of three pieces of information for each ticket:

- A title
- A description
- A status

We can start by using a [`String`](https://doc.rust-lang.org/std/string/struct.String.html)
to represent them. `String` is the type defined in Rust's standard library to represent
[UTF-8 encoded](https://en.wikipedia.org/wiki/UTF-8) text.

But how do we **combine** these three pieces of information into a single entity?

## Defining a `struct`

A `struct` defines a **new Rust type**.

```rust
struct Ticket {
    title: String,
    description: String,
    status: String
}
```

A struct is quite similar to what you would call a class or an object in other programming languages.

## Defining fields

The new type is built by combining other types as **fields**.\
Each field must have a name and a type, separated by a colon, `:`. If there are multiple fields, they are separated by a comma, `,`.

Fields don't have to be of the same type, as you can see in the `Configuration` struct below:

```rust
struct Configuration {
   version: u32,
   active: bool
}
```

## Instantiation

You can create an instance of a struct by specifying the values for each field:

```rust
// Syntax: <StructName> { <field_name>: <value>, ... }
let ticket = Ticket {
    title: "Build a ticket system".into(),
    description: "A Kanban board".into(),
    status: "Open".into()
};
```

## Accessing fields

You can access the fields of a struct using the `.` operator:

```rust
// Field access
let x = ticket.description;
```

## Methods

We can attach behaviour to our structs by defining **methods**.\
Using the `Ticket` struct as an example:

```rust
impl Ticket {
    fn is_open(self) -> bool {
        self.status == "Open"
    }
}

// Syntax:
// impl <StructName> {
//    fn <method_name>(<parameters>) -> <return_type> {
//        // Method body
//    }
// }
```

Methods are pretty similar to functions, with two key differences:

1. methods must be defined inside an **`impl` block**
2. methods may use `self` as their first parameter.
   `self` is a keyword and represents the instance of the struct the method is being called on.

### `self`

If a method takes `self` as its first parameter, it can be called using the **method call syntax**:

```rust
// Method call syntax: <instance>.<method_name>(<parameters>)
let is_open = ticket.is_open();
```

This is the same calling syntax you used to perform saturating arithmetic operations on `u32` values
in [the previous chapter](../02_basic_calculator/09_saturating.md).

### Static methods

If a method doesn't take `self` as its first parameter, it's a **static method**.

```rust
struct Configuration {
    version: u32,
    active: bool
}

impl Configuration {
    // `default` is a static method on `Configuration`
    fn default() -> Configuration {
        Configuration { version: 0, active: false }
    }
}
```

The only way to call a static method is by using the **function call syntax**:

```rust
// Function call syntax: <StructName>::<method_name>(<parameters>)
let default_config = Configuration::default();
```

### Equivalence

You can use the function call syntax even for methods that take `self` as their first parameter:

```rust
// Function call syntax:
//   <StructName>::<method_name>(<instance>, <parameters>)
let is_open = Ticket::is_open(ticket);
```

The function call syntax makes it quite clear that `ticket` is being used as `self`, the first parameter of the method,
but it's definitely more verbose. Prefer the method call syntax when possible.

### 02_validation

# Validation

Let's go back to our ticket definition:

```rust
struct Ticket {
    title: String,
    description: String,
    status: String,
}
```

We are using "raw" types for the fields of our `Ticket` struct.
This means that users can create a ticket with an empty title, a suuuuuuuper long description or
a nonsensical status (e.g. "Funny").\
We can do better than that!

## Further reading

- Check out [`String`'s documentation](https://doc.rust-lang.org/std/string/struct.String.html)
  for a thorough overview of the methods it provides. You'll need it for the exercise!

### 03_modules

# Modules

The `new` method you've just defined is trying to enforce some **constraints** on the field values for `Ticket`.
But are those invariants really enforced? What prevents a developer from creating a `Ticket`
without going through `Ticket::new`?

To get proper **encapsulation** you need to become familiar with two new concepts: **visibility** and **modules**.
Let's start with modules.

## What is a module?

In Rust a **module** is a way to group related code together, under a common namespace (i.e. the module's name).\
You've already seen modules in action: the unit tests that verify the correctness of your code are defined in a
different module, named `tests`.

```rust
#[cfg(test)]
mod tests {
    // [...]
}
```

## Inline modules

The `tests` module above is an example of an **inline module**: the module declaration (`mod tests`) and the module
contents (the stuff inside `{ ... }`) are next to each other.

## Module tree

Modules can be nested, forming a **tree** structure.\
The root of the tree is the **crate** itself, which is the top-level module that contains all the other modules.
For a library crate, the root module is usually `src/lib.rs` (unless its location has been customized).
The root module is also known as the **crate root**.

The crate root can have submodules, which in turn can have their own submodules, and so on.

## External modules and the filesystem

Inline modules are useful for small pieces of code, but as your project grows you'll want to split your code into
multiple files. In the parent module, you declare the existence of a submodule using the `mod` keyword.

```rust
mod dog;
```

`cargo`, Rust's build tool, is then in charge of finding the file that contains
the module implementation.\
If your module is declared in the root of your crate (e.g. `src/lib.rs` or `src/main.rs`),
`cargo` expects the file to be named either:

- `src/<module_name>.rs`
- `src/<module_name>/mod.rs`

If your module is a submodule of another module, the file should be named:

- `[..]/<parent_module>/<module_name>.rs`
- `[..]/<parent_module>/<module_name>/mod.rs`

E.g. `src/animals/dog.rs` or `src/animals/dog/mod.rs` if `dog` is a submodule of `animals`.

Your IDE might help you create these files automatically when you declare a new module using the `mod` keyword.

## Item paths and `use` statements

You can access items defined in the same module without any special syntax. You just use their name.

```rust
struct Ticket {
    // [...]
}

// No need to qualify `Ticket` in any way here
// because we're in the same module
fn mark_ticket_as_done(ticket: Ticket) {
    // [...]
}
```

That's not the case if you want to access an entity from a different module.\
You have to use a **path** pointing to the entity you want to access.

You can compose the path in various ways:

- starting from the root of the current crate, e.g. `crate::module_1::MyStruct`
- starting from the parent module, e.g. `super::my_function`
- starting from the current module, e.g. `sub_module_1::MyStruct`

Both `crate` and `super` are **keywords**.\
`crate` refers to the root of the current crate, while `super` refers to the parent of the current module.

Having to write the full path every time you want to refer to a type can be cumbersome.
To make your life easier, you can introduce a `use` statement to bring the entity into scope.

```rust
// Bring `MyStruct` into scope
use crate::module_1::module_2::MyStruct;

// Now you can refer to `MyStruct` directly
fn a_function(s: MyStruct) {
     // [...]
}
```

### Star imports

You can also import all the items from a module with a single `use` statement.

```rust
use crate::module_1::module_2::*;
```

This is known as a **star import**.\
It is generally discouraged because it can pollute the current namespace, making it hard to understand
where each name comes from and potentially introducing name conflicts.\
Nonetheless, it can be useful in some cases, like when writing unit tests. You might have noticed
that most of our test modules start with a `use super::*;` statement to bring all the items from the parent module
(the one being tested) into scope.

## Visualizing the module tree

If you're struggling to picture the module tree of your project, you can try using
[`cargo-modules`](https://crates.io/crates/cargo-modules) to visualize it!

Refer to their documentation for installation instructions and usage examples.

### 04_visibility

# Visibility

When you start breaking down your code into multiple modules, you need to start thinking about **visibility**.
Visibility determines which regions of your code (or other people's code) can access a given entity,
be it a struct, a function, a field, etc.

## Private by default

By default, everything in Rust is **private**.\
A private entity can only be accessed:

1. within the same module where it's defined, or
2. by one of its submodules

We've used this extensively in the previous exercises:

- `create_todo_ticket` worked (once you added a `use` statement) because `helpers` is a submodule of the crate root,
  where `Ticket` is defined. Therefore, `create_todo_ticket` can access `Ticket` without any issues even
  though `Ticket` is private.
- All our unit tests are defined in a submodule of the code they're testing, so they can access everything without
  restrictions.

## Visibility modifiers

You can modify the default visibility of an entity using a **visibility modifier**.\
Some common visibility modifiers are:

- `pub`: makes the entity **public**, i.e. accessible from outside the module where it's defined, potentially from
  other crates.
- `pub(crate)`: makes the entity public within the same **crate**, but not outside of it.
- `pub(super)`: makes the entity public within the parent module.
- `pub(in path::to::module)`: makes the entity public within the specified module.

You can use these modifiers on modules, structs, functions, fields, etc.
For example:

```rust
pub struct Configuration {
    pub(crate) version: u32,
    active: bool,
}
```

`Configuration` is public, but you can only access the `version` field from within the same crate.
The `active` field, instead, is private and can only be accessed from within the same module or one of its submodules.

### 05_encapsulation

# Encapsulation

Now that we have a basic understanding of modules and visibility, let's circle back to **encapsulation**.\
Encapsulation is the practice of hiding the internal representation of an object. It is most commonly
used to enforce some **invariants** on the object's state.

Going back to our `Ticket` struct:

```rust
struct Ticket {
    title: String,
    description: String,
    status: String,
}
```

If all fields are made public, there is no encapsulation.\
You must assume that the fields can be modified at any time, set to any value that's allowed by
their type. You can't rule out that a ticket might have an empty title or a status
that doesn't make sense.

To enforce stricter rules, we must keep the fields private[^newtype].
We can then provide public methods to interact with a `Ticket` instance.
Those public methods will have the responsibility of upholding our invariants (e.g. a title must not be empty).

If at least one field is private it is no longer possible to create a `Ticket` instance directly using the struct
instantiation syntax:

```rust
// This won't work!
let ticket = Ticket {
    title: "Build a ticket system".into(),
    description: "A Kanban board".into(),
    status: "Open".into()
};
```

You've seen this in action in the previous exercise on visibility.\
We now need to provide one or more public **constructors**—i.e. static methods or functions that can be used
from outside the module to create a new instance of the struct.\
Luckily enough we already have one: `Ticket::new`, as implemented in [a previous exercise](02_validation.md).

## Accessor methods

In summary:

- All `Ticket` fields are private
- We provide a public constructor, `Ticket::new`, that enforces our validation rules on creation

That's a good start, but it's not enough: apart from creating a `Ticket`, we also need to interact with it.
But how can we access the fields if they're private?

We need to provide **accessor methods**.\
Accessor methods are public methods that allow you to read the value of a private field (or fields) of a struct.

Rust doesn't have a built-in way to generate accessor methods for you, like some other languages do.
You have to write them yourself—they're just regular methods.

[^newtype]: Or refine their type, a technique we'll explore [later on](../05_ticket_v2/15_outro.md).

### 06_ownership

# Ownership

If you solved the previous exercise using what this course has taught you so far,
your accessor methods probably look like this:

```rust
impl Ticket {
    pub fn title(self) -> String {
        self.title
    }

    pub fn description(self) -> String {
        self.description
    }

    pub fn status(self) -> String {
        self.status
    }
}
```

Those methods compile and are enough to get tests to pass, but in a real-world scenario they won't get you very far.
Consider this snippet:

```rust
if ticket.status() == "To-Do" {
    // We haven't covered the `println!` macro yet,
    // but for now it's enough to know that it prints 
    // a (templated) message to the console
    println!("Your next task is: {}", ticket.title());
}
```

If you try to compile it, you'll get an error:

```text
error[E0382]: use of moved value: `ticket`
  --> src/main.rs:30:43
   |
25 |     let ticket = Ticket::new(/* */);
   |         ------ move occurs because `ticket` has type `Ticket`, 
   |                which does not implement the `Copy` trait
26 |     if ticket.status() == "To-Do" {
   |               -------- `ticket` moved due to this method call
...
30 |         println!("Your next task is: {}", ticket.title());
   |                                           ^^^^^^ 
   |                                value used here after move
   |
note: `Ticket::status` takes ownership of the receiver `self`, 
      which moves `ticket`
  --> src/main.rs:12:23
   |
12 |         pub fn status(self) -> String {
   |                       ^^^^
```

Congrats, this is your first borrow-checker error!

## The perks of Rust's ownership system

Rust's ownership system is designed to ensure that:

- Data is never mutated while it's being read
- Data is never read while it's being mutated
- Data is never accessed after it has been destroyed

These constraints are enforced by the **borrow checker**, a subsystem of the Rust compiler,
often the subject of jokes and memes in the Rust community.

Ownership is a key concept in Rust, and it's what makes the language unique.
Ownership enables Rust to provide **memory safety without compromising performance**.
All these things are true at the same time for Rust:

1. There is no runtime garbage collector
2. As a developer, you rarely have to manage memory directly
3. You can't cause dangling pointers, double frees, and other memory-related bugs

Languages like Python, JavaScript, and Java give you 2. and 3., but not 1.\
Language like C or C++ give you 1., but neither 2. nor 3.

Depending on your background, 3. might sound a bit arcane: what is a "dangling pointer"?
What is a "double free"? Why are they dangerous?\
Don't worry: we'll cover these concepts in more details during the rest of the course.

For now, though, let's focus on learning how to work within Rust's ownership system.

## The owner

In Rust, each value has an **owner**, statically determined at compile-time.
There is only one owner for each value at any given time.

## Move semantics

Ownership can be transferred.

If you own a value, for example, you can transfer ownership to another variable:

```rust
let a = "hello, world".to_string(); // <- `a` is the owner of the String
let b = a;  // <- `b` is now the owner of the String
```

Rust's ownership system is baked into the type system: each function has to declare in its signature
_how_ it wants to interact with its arguments.

So far, all our methods and functions have **consumed** their arguments: they've taken ownership of them.
For example:

```rust
impl Ticket {
    pub fn description(self) -> String {
        self.description
    }
}
```

`Ticket::description` takes ownership of the `Ticket` instance it's called on.\
This is known as **move semantics**: ownership of the value (`self`) is **moved** from the caller to
the callee, and the caller can't use it anymore.

That's exactly the language used by the compiler in the error message we saw earlier:

```text
error[E0382]: use of moved value: `ticket`
  --> src/main.rs:30:43
   |
25 |     let ticket = Ticket::new(/* */);
   |         ------ move occurs because `ticket` has type `Ticket`, 
   |                which does not implement the `Copy` trait
26 |     if ticket.status() == "To-Do" {
   |               -------- `ticket` moved due to this method call
...
30 |         println!("Your next task is: {}", ticket.title());
   |                                           ^^^^^^ 
   |                                value used here after move
   |
note: `Ticket::status` takes ownership of the receiver `self`, 
      which moves `ticket`
  --> src/main.rs:12:23
   |
12 |         pub fn status(self) -> String {
   |                       ^^^^
```

In particular, this is the sequence of events that unfold when we call `ticket.status()`:

- `Ticket::status` takes ownership of the `Ticket` instance
- `Ticket::status` extracts `status` from `self` and transfers ownership of `status` back to the caller
- The rest of the `Ticket` instance is discarded (`title` and `description`)

When we try to use `ticket` again via `ticket.title()`, the compiler complains: the `ticket` value is gone now,
we no longer own it, therefore we can't use it anymore.

To build _useful_ accessor methods we need to start working with **references**.

## Borrowing

It is desirable to have methods that can read the value of a variable without taking ownership of it.\
Programming would be quite limited otherwise. In Rust, that's done via **borrowing**.

Whenever you borrow a value, you get a **reference** to it.\
References are tagged with their privileges[^refine]:

- Immutable references (`&`) allow you to read the value, but not to mutate it
- Mutable references (`&mut`) allow you to read and mutate the value

Going back to the goals of Rust's ownership system:

- Data is never mutated while it's being read
- Data is never read while it's being mutated

To ensure these two properties, Rust has to introduce some restrictions on references:

- You can't have a mutable reference and an immutable reference to the same value at the same time
- You can't have more than one mutable reference to the same value at the same time
- The owner can't mutate the value while it's being borrowed
- You can have as many immutable references as you want, as long as there are no mutable references

In a way, you can think of an immutable reference as a "read-only" lock on the value,
while a mutable reference is like a "read-write" lock.

All these restrictions are enforced at compile-time by the borrow checker.

### Syntax

How do you borrow a value, in practice?\
By adding `&` or `&mut` **in front a variable**, you're borrowing its value.
Careful though! The same symbols (`&` and `&mut`) in **front of a type** have a different meaning:
they denote a different type, a reference to the original type.

For example:

```rust
struct Configuration {
    version: u32,
    active: bool,
}

fn main() {
    let config = Configuration {
        version: 1,
        active: true,
    };
    // `b` is a reference to the `version` field of `config`.
    // The type of `b` is `&u32`, since it contains a reference to 
    // a `u32` value.
    // We create a reference by borrowing `config.version`, using 
    // the `&` operator.
    // Same symbol (`&`), different meaning depending on the context!
    let b: &u32 = &config.version;
    //     ^ The type annotation is not necessary, 
    //       it's just there to clarify what's going on
}
```

The same concept applies to function arguments and return types:

```rust
// `f` takes a mutable reference to a `u32` as an argument, 
// bound to the name `number`
fn f(number: &mut u32) -> &u32 {
    // [...]
}
```

## Breathe in, breathe out

Rust's ownership system can be a bit overwhelming at first.\
But don't worry: it'll become second nature with practice.\
And you're going to get a lot of practice over the rest of this chapter, as well as the rest of the course!
We'll revisit each concept multiple times to make sure you get familiar with them
and truly understand how they work.

Towards the end of this chapter we'll explain _why_ Rust's ownership system is designed the way it is.
For the time being, focus on understanding the _how_. Take each compiler error as a learning opportunity!

[^refine]: This is a great mental model to start out, but it doesn't capture the _full_ picture.
We'll refine our understanding of references [later in the course](../07_threads/06_interior_mutability.md).

### 07_setters

# Mutable references

Your accessor methods should look like this now:

```rust
impl Ticket {
    pub fn title(&self) -> &String {
        &self.title
    }

    pub fn description(&self) -> &String {
        &self.description
    }

    pub fn status(&self) -> &String {
        &self.status
    }
}
```

A sprinkle of `&` here and there did the trick!\
We now have a way to access the fields of a `Ticket` instance without consuming it in the process.
Let's see how we can enhance our `Ticket` struct with **setter methods** next.

## Setters

Setter methods allow users to change the values of `Ticket`'s private fields while making sure that its invariants
are respected (i.e. you can't set a `Ticket`'s title to an empty string).

There are two common ways to implement setters in Rust:

- Taking `self` as input.
- Taking `&mut self` as input.

### Taking `self` as input

The first approach looks like this:

```rust
impl Ticket {
    pub fn set_title(mut self, new_title: String) -> Self {
        // Validate the new title [...]
        self.title = new_title;
        self
    }
}
```

It takes ownership of `self`, changes the title, and returns the modified `Ticket` instance.\
This is how you'd use it:

```rust
let ticket = Ticket::new(
    "Title".into(), 
    "Description".into(), 
    "To-Do".into()
);
let ticket = ticket.set_title("New title".into());
```

Since `set_title` takes ownership of `self` (i.e. it **consumes it**), we need to reassign the result to a variable.
In the example above we take advantage of **variable shadowing** to reuse the same variable name: when
you declare a new variable with the same name as an existing one, the new variable **shadows** the old one. This
is a common pattern in Rust code.

`self`-setters work quite nicely when you need to change multiple fields at once: you can chain multiple calls together!

```rust
let ticket = ticket
    .set_title("New title".into())
    .set_description("New description".into())
    .set_status("In Progress".into());
```

### Taking `&mut self` as input

The second approach to setters, using `&mut self`, looks like this instead:

```rust
impl Ticket {
    pub fn set_title(&mut self, new_title: String) {
        // Validate the new title [...]
        
        self.title = new_title;
    }
}
```

This time the method takes a mutable reference to `self` as input, changes the title, and that's it.
Nothing is returned.

You'd use it like this:

```rust
let mut ticket = Ticket::new(
    "Title".into(),
    "Description".into(),
    "To-Do".into()
);
ticket.set_title("New title".into());

// Use the modified ticket
```

Ownership stays with the caller, so the original `ticket` variable is still valid. We don't need to reassign the result.
We need to mark `ticket` as mutable though, because we're taking a mutable reference to it.

`&mut`-setters have a downside: you can't chain multiple calls together.
Since they don't return the modified `Ticket` instance, you can't call another setter on the result of the first one.
You have to call each setter separately:

```rust
ticket.set_title("New title".into());
ticket.set_description("New description".into());
ticket.set_status("In Progress".into());
```

### 08_stack

# Memory layout

We've looked at ownership and references from an operational point of view—what you can and can't do with them.
Now it's a good time to take a look under the hood: let's talk about **memory**.

## Stack and heap

When discussing memory, you'll often hear people talk about the **stack** and the **heap**.\
These are two different memory regions used by programs to store data.

Let's start with the stack.

## Stack

The **stack** is a **LIFO** (Last In, First Out) data structure.\
When you call a function, a new **stack frame** is added on top of the stack. That stack frame stores
the function's arguments, local variables and a few "bookkeeping" values.\
When the function returns, the stack frame is popped off the stack[^stack-overflow].

```text
+-----------------+
| frame for func1 |
+-----------------+
        |
        | func2 is 
        | called
        v
+-----------------+
| frame for func2 |
+-----------------+
| frame for func1 |
+-----------------+
        |
        | func2  
        | returns
        v
+-----------------+
| frame for func1 |
+-----------------+
```

From an operational point of view, stack allocation/de-allocation is **very fast**.\
We are always pushing and popping data from the top of the stack, so we don't need to search for free memory.
We also don't have to worry about fragmentation: the stack is a single contiguous block of memory.

### Rust

Rust will often allocate data on the stack.\
You have a `u32` input argument in a function? Those 32 bits will be on the stack.\
You define a local variable of type `i64`? Those 64 bits will be on the stack.\
It all works quite nicely because the size of those integers is known at compile time, therefore
the compiled program knows how much space it needs to reserve on the stack for them.

### `std::mem::size_of`

You can verify how much space a type would take on the stack
using the [`std::mem::size_of`](https://doc.rust-lang.org/std/mem/fn.size_of.html) function.

For a `u8`, for example:

```rust
// We'll explain this funny-looking syntax (`::<u8>`) later on.
// Ignore it for now.
assert_eq!(std::mem::size_of::<u8>(), 1);
```

1 makes sense, because a `u8` is 8 bits long, or 1 byte.

[^stack-overflow]: If you have nested function calls, each function pushes its data onto the stack when it's called but
it doesn't pop it off until the innermost function returns.
If you have too many nested function calls, you can run out of stack space—the stack is not infinite!
That's called a [**stack overflow**](https://en.wikipedia.org/wiki/Stack_overflow).

### 09_heap

# Heap

The stack is great, but it can't solve all our problems. What about data whose size is not known at compile time?
Collections, strings, and other dynamically-sized data cannot be (entirely) stack-allocated.
That's where the **heap** comes in.

## Heap allocations

You can visualize the heap as a big chunk of memory—a huge array, if you will.\
Whenever you need to store data on the heap, you ask a special program, the **allocator**, to reserve for you
a subset of the heap. We call this interaction (and the memory you reserved) a **heap allocation**.
If the allocation succeeds, the allocator will give you a **pointer** to the start of the reserved block.

## No automatic de-allocation

The heap is structured quite differently from the stack.\
Heap allocations are not contiguous, they can be located anywhere inside the heap.

```
+---+---+---+---+---+---+-...-+-...-+---+---+---+---+---+---+---+
|  Allocation 1 | Free  | ... | ... |  Allocation N |    Free   |
+---+---+---+---+---+---+ ... + ... +---+---+---+---+---+---+---+
```

It's the allocator's job to keep track of which parts of the heap are in use and which are free.
The allocator won't automatically free the memory you allocated, though: you need to be deliberate about it,
calling the allocator again to **free** the memory you no longer need.

## Performance

The heap's flexibility comes at a cost: heap allocations are **slower** than stack allocations.
There's a lot more bookkeeping involved!\
If you read articles about performance optimization you'll often be advised to minimize heap allocations
and prefer stack-allocated data whenever possible.

## `String`'s memory layout

When you create a local variable of type `String`,
Rust is forced to allocate on the heap[^empty]: it doesn't know in advance how much text you're going to put in it,
so it can't reserve the right amount of space on the stack.\
But a `String` is not _entirely_ heap-allocated, it also keeps some data on the stack. In particular:

- The **pointer** to the heap region you reserved.
- The **length** of the string, i.e. how many bytes are in the string.
- The **capacity** of the string, i.e. how many bytes have been reserved on the heap.

Let's look at an example to understand this better:

```rust
let mut s = String::with_capacity(5);
```

If you run this code, memory will be laid out like this:

```
      +---------+--------+----------+
Stack | pointer | length | capacity | 
      |  |      |   0    |    5     |
      +--|------+--------+----------+
         |
         |
         v
       +---+---+---+---+---+
Heap:  | ? | ? | ? | ? | ? |
       +---+---+---+---+---+
```

We asked for a `String` that can hold up to 5 bytes of text.\
`String::with_capacity` goes to the allocator and asks for 5 bytes of heap memory. The allocator returns
a pointer to the start of that memory block.\
The `String` is empty, though. On the stack, we keep track of this information by distinguishing between
the length and the capacity: this `String` can hold up to 5 bytes, but it currently holds 0 bytes of
actual text.

If you push some text into the `String`, the situation will change:

```rust
s.push_str("Hey");
```

```
      +---------+--------+----------+
Stack | pointer | length | capacity |
      |  |      |   3    |    5     |
      +--|  ----+--------+----------+
         |
         |
         v
       +---+---+---+---+---+
Heap:  | H | e | y | ? | ? |
       +---+---+---+---+---+
```

`s` now holds 3 bytes of text. Its length is updated to 3, but capacity remains 5.
Three of the five bytes on the heap are used to store the characters `H`, `e`, and `y`.

### `usize`

How much space do we need to store pointer, length and capacity on the stack?\
It depends on the **architecture** of the machine you're running on.

Every memory location on your machine has an [**address**](https://en.wikipedia.org/wiki/Memory_address), commonly
represented as an unsigned integer.
Depending on the maximum size of the address space (i.e. how much memory your machine can address),
this integer can have a different size. Most modern machines use either a 32-bit or a 64-bit address space.

Rust abstracts away these architecture-specific details by providing the `usize` type:
an unsigned integer that's as big as the number of bytes needed to address memory on your machine.
On a 32-bit machine, `usize` is equivalent to `u32`. On a 64-bit machine, it matches `u64`.

Capacity, length and pointers are all represented as `usize`s in Rust[^equivalence].

### No `std::mem::size_of` for the heap

`std::mem::size_of` returns the amount of space a type would take on the stack,
which is also known as the **size of the type**.

> What about the memory buffer that `String` is managing on the heap? Isn't that
> part of the size of `String`?

No!\
That heap allocation is a **resource** that `String` is managing.
It's not considered to be part of the `String` type by the compiler.

`std::mem::size_of` doesn't know (or care) about additional heap-allocated data
that a type might manage or refer to via pointers, as is the case with `String`,
therefore it doesn't track its size.

Unfortunately there is no equivalent of `std::mem::size_of` to measure the amount of
heap memory that a certain value is allocating at runtime. Some types might
provide methods to inspect their heap usage (e.g. `String`'s `capacity` method),
but there is no general-purpose "API" to retrieve runtime heap usage in Rust.\
You can, however, use a memory profiler tool (e.g. [DHAT](https://valgrind.org/docs/manual/dh-manual.html)
or [a custom allocator](https://docs.rs/dhat/latest/dhat/)) to inspect the heap usage of your program.

[^empty]: `std` doesn't allocate if you create an **empty** `String` (i.e. `String::new()`).
Heap memory will be reserved when you push data into it for the first time.

[^equivalence]: The size of a pointer depends on the operating system too.
In certain environments, a pointer is **larger** than a memory address (e.g. [CHERI](https://web.archive.org/web/20240517051950/https://blog.acolyer.org/2019/05/28/cheri-abi/)).
Rust makes the simplifying assumption that pointers are the same size as memory addresses,
which is true for most modern systems you're likely to encounter.

### 10_references_in_memory

# References

What about references, like `&String` or `&mut String`? How are they represented in memory?

Most references[^fat] in Rust are represented, in memory, as a pointer to a memory location.\
It follows that their size is the same as the size of a pointer, a `usize`.

You can verify this using `std::mem::size_of`:

```rust
assert_eq!(std::mem::size_of::<&String>(), 8);
assert_eq!(std::mem::size_of::<&mut String>(), 8);
```

A `&String`, in particular, is a pointer to the memory location where the `String`'s metadata is stored.\
If you run this snippet:

```rust
let s = String::from("Hey");
let r = &s;
```

you'll get something like this in memory:

```
           --------------------------------------
           |                                    |
      +----v----+--------+----------+      +----|----+
Stack | pointer | length | capacity |      | pointer |
      |  |      |   3    |    5     |      |         |
      +--|  ----+--------+----------+      +---------+
         |          s                           r
         |
         v
       +---+---+---+---+---+
Heap   | H | e | y | ? | ? |
       +---+---+---+---+---+
```

It's a pointer to a pointer to the heap-allocated data, if you will.
The same goes for `&mut String`.

## Not all pointers point to the heap

The example above should clarify one thing: not all pointers point to the heap.\
They just point to a memory location, which _may_ be on the heap, but doesn't have to be.

[^fat]: [Later in the course](../04_traits/06_str_slice.md) we'll talk about **fat pointers**,
i.e. pointers with additional metadata. As the name implies, they are larger than
the pointers we discussed in this chapter, also known as **thin pointers**.

### 11_destructor

# Destructors

When introducing the heap, we mentioned that you're responsible for freeing the memory you allocate.\
When introducing the borrow-checker, we also stated that you rarely have to manage memory directly in Rust.

These two statements might seem contradictory at first.
Let's see how they fit together by introducing **scopes** and **destructors**.

## Scopes

The **scope** of a variable is the region of Rust code where that variable is valid, or **alive**.

The scope of a variable starts with its declaration.
It ends when one of the following happens:

1. the block (i.e. the code between `{}`) where the variable was declared ends
   ```rust
   fn main() {
      // `x` is not yet in scope here
      let y = "Hello".to_string();
      let x = "World".to_string(); // <-- x's scope starts here...
      let h = "!".to_string(); //   |
   } //  <-------------- ...and ends here
   ```
2. ownership of the variable is transferred to someone else (e.g. a function or another variable)
   ```rust
   fn compute(t: String) {
      // Do something [...]
   }

   fn main() {
       let s = "Hello".to_string(); // <-- s's scope starts here...
                   //                    | 
       compute(s); // <------------------- ..and ends here
                   //   because `s` is moved into `compute`
   }
   ```

## Destructors

When the owner of a value goes out of scope, Rust invokes its **destructor**.\
The destructor tries to clean up the resources used by that value—in particular, whatever memory it allocated.

You can manually invoke the destructor of a value by passing it to `std::mem::drop`.\
That's why you'll often hear Rust developers saying "that value has been **dropped**" as a way to state that a value
has gone out of scope and its destructor has been invoked.

### Visualizing drop points

We can insert explicit calls to `drop` to "spell out" what the compiler does for us. Going back to the previous example:

```rust
fn main() {
   let y = "Hello".to_string();
   let x = "World".to_string();
   let h = "!".to_string();
}
```

It's equivalent to:

```rust
fn main() {
   let y = "Hello".to_string();
   let x = "World".to_string();
   let h = "!".to_string();
   // Variables are dropped in reverse order of declaration
   drop(h);
   drop(x);
   drop(y);
}
```

Let's look at the second example instead, where `s`'s ownership is transferred to `compute`:

```rust
fn compute(s: String) {
   // Do something [...]
}

fn main() {
   let s = "Hello".to_string();
   compute(s);
}
```

It's equivalent to this:

```rust
fn compute(t: String) {
    // Do something [...]
    drop(t); // <-- Assuming `t` wasn't dropped or moved 
             //     before this point, the compiler will call 
             //     `drop` here, when it goes out of scope
}

fn main() {
    let s = "Hello".to_string();
    compute(s);
}
```

Notice the difference: even though `s` is no longer valid after `compute` is called in `main`, there is no `drop(s)`
in `main`.
When you transfer ownership of a value to a function, you're also **transferring the responsibility of cleaning it up**.

This ensures that the destructor for a value is called **at most[^leak] once**, preventing
[double free bugs](https://owasp.org/www-community/vulnerabilities/Doubly_freeing_memory) by design.

### Use after drop

What happens if you try to use a value after it's been dropped?

```rust
let x = "Hello".to_string();
drop(x);
println!("{}", x);
```

If you try to compile this code, you'll get an error:

```rust
error[E0382]: use of moved value: `x`
 --> src/main.rs:4:20
  |
3 |     drop(x);
  |          - value moved here
4 |     println!("{}", x);
  |                    ^ value used here after move
```

Drop **consumes** the value it's called on, meaning that the value is no longer valid after the call.\
The compiler will therefore prevent you from using it, avoiding [use-after-free bugs](https://owasp.org/www-community/vulnerabilities/Using_freed_memory).

### Dropping references

What if a variable contains a reference?\
For example:

```rust
let x = 42i32;
let y = &x;
drop(y);
```

When you call `drop(y)`... nothing happens.\
If you actually try to compile this code, you'll get a warning:

```text
warning: calls to `std::mem::drop` with a reference 
         instead of an owned value does nothing
 --> src/main.rs:4:5
  |
4 |     drop(y);
  |     ^^^^^-^
  |          |
  |          argument has type `&i32`
  |
```

It goes back to what we said earlier: we only want to call the destructor once.\
You can have multiple references to the same value—if we called the destructor for the value they point at
when one of them goes out of scope, what would happen to the others?
They would refer to a memory location that's no longer valid: a so-called [**dangling pointer**](https://en.wikipedia.org/wiki/Dangling_pointer),
a close relative of [**use-after-free bugs**](https://owasp.org/www-community/vulnerabilities/Using_freed_memory).
Rust's ownership system rules out these kinds of bugs by design.

[^leak]: Rust doesn't guarantee that destructors will run. They won't, for example, if
you explicitly choose to [leak memory](../07_threads/03_leak.md).

### 12_outro

# Wrapping up

We've covered a lot of foundational Rust concepts in this chapter.\
Before moving on, let's go through one last exercise to consolidate what we've learned.
You'll have minimal guidance this time—just the exercise description and the tests to guide you.
