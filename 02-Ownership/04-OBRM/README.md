# Ownership-Based Resource Management — RAII and the Drop Trait

> **Test-driven approach**: This project includes a Cargo project with progressive
> unit tests. Each function in `workshop/src/lib.rs` starts as a `todo!()` stub. As you
> follow each section, replace `todo!()` with real code and run `cd workshop && cargo test` to
> watch the pass count grow. Your goal: **all 11 tests pass**.

---

## Why RAII for Data Pipelines?

**Python pain:** File handles, DB connections, and sockets need to be closed. Forget `with` and the resource leaks. Early returns need careful `try/finally` placement. `__del__` is non-deterministic — the GC decides when it runs, *if ever*. In a production ETL pipeline, a single forgotten `f.close()` can exhaust file descriptors and crash the job.

**Rust fix:** **Ownership-Based Resource Management (RAII)** ties resource lifetime to variable scope. Acquire on creation, release automatically when the owner goes out of scope — no `with`, no `try/finally`, no GC. The compiler inserts cleanup at every scope exit, even on early return or panic:

```rust
{
    let f = File::open("data.csv")?;
    // use f...
} // f is automatically closed here — guaranteed by the compiler
```

## At a Glance

| # | Concept | Rust | Python | Why it matters |
|---|---------|------|--------|----------------|
| 1 | RAII | resource acquired in ctor, released in `Drop` | `with` statement (manual) | Lifetime tied to scope — no leaks by default |
| 2 | `Drop` Trait | `impl Drop for Resource { fn drop(&mut self) { ... } }` | `__del__` (non-deterministic) / `__exit__` (manual) | Both *automatic* and *deterministic* — the best of both |
| 3 | Ownership Transfer | move transfers cleanup responsibility | passing a handle keeps the caller's reference | Caller is *forced* to give up the handle when it moves |
| 4 | Borrowing | `&Resource` / `&mut Resource` | all references are equal | Borrower can't outlive the owner; owner can't close while borrowed |
| 5 | Scope-based Cleanup | every `}` runs `Drop` for all locals | `with` (only if used) | Works even on early return and panic — zero escape hatches |
| 6 | Resource Lifecycle | `closed → open → closed` tracked in struct | `__enter__` / `__exit__` protocol | The `Drop` impl closes if still open — the last line of defense |

---

## Table of Contents

1. [Introduction](#1-introduction)
2. [Prerequisites](#2-prerequisites)
3. [Concept: Context Managers in Python vs RAII in Rust](#3-concept-context-managers-in-python-vs-raii-in-rust)
4. [Concept: The Drop Trait — Automatic Cleanup](#4-concept-the-drop-trait--automatic-cleanup)
5. [Concept: Ownership and Resource Lifecycle](#5-concept-ownership-and-resource-lifecycle)
6. [Concept: Borrowing Resources](#6-concept-borrowing-resources)
7. [Putting It All Together](#7-putting-it-all-together)
8. [Exercises](#8-exercises)
9. [Summary](#9-summary)

---

## 1. Introduction

Every data engineer deals with resources: file handles, database connections,
network sockets, memory buffers. In Python, you manage these with context
managers (`with` statements). In Rust, the compiler manages them for you
automatically using **Ownership-Based Resource Management (OBRM)** — also known
as **RAII (Resource Acquisition Is Initialization)**.

In this workshop, you will build a `Resource` type that simulates a managed
system resource (like a file handle or database connection). You will:

- Implement `open` and `close` lifecycle methods
- Implement the `Drop` trait so resources clean up **automatically** when they
  go out of scope
- Transfer ownership of resources between functions
- Borrow resources without taking ownership
- See how Rust's compiler guarantees no resource leaks — no `with`, no
  `try/finally`, no manual `close()` calls needed

**Data-engineering motivation**: Imagine an ETL pipeline that opens 10,000 CSV
files. In Python, forgetting a single `f.close()` or an exception bypassing your
`with` block can leak file handles. In Rust, every resource is guaranteed to
close when its owner goes out of scope — even if you `return` early or `panic!`.
For production data pipelines, this determinism is invaluable.

---

## 2. Prerequisites

Before starting, you should be comfortable with:

- **Basic Rust syntax**: functions, variables, `println!`
  ([01-Foundations/01-Intro](../../01-Foundations/01-Intro/README.md))

  ([01-Foundations/04-MasterMind](../../01-Foundations/04-MasterMind/README.md))
- **Ownership basics**: moves, copies, borrowing
  ([02-Ownership/01-TicketV1](../../02-Ownership/01-TicketV1/README.md))

---

## 3. Concept: Context Managers in Python vs RAII in Rust

### How Python Does It

In Python, you use a context manager to ensure a resource is cleaned up:

```python
# Python — manual resource management with context manager
with open("data.csv", "r") as f:
    data = f.read()
# f is automatically closed here — but only because of the `with` block
```

The `with` statement calls `__enter__` at the start and `__exit__` at the end
(even if an exception occurs). This is good, but it has limitations:

- You must remember to use `with` — nothing enforces it
- If you assign the file to a variable without `with`, you must close it
  manually or rely on `__del__` (which is non-deterministic)
- Early returns require careful placement of cleanup code

Here's what happens when you forget:

```python
# Python — easy to forget cleanup
f = open("data.csv", "r")
data = f.read()
# f is still open! It will eventually be garbage-collected, but when?
```

### How Rust Does It — RAII

In Rust, resource cleanup is tied to **ownership** and **scope**. When a value
goes out of scope, its destructor runs automatically. You do not need a `with`
statement — the compiler does it for you:

```rust
// Rust — automatic cleanup via RAII
{
    let f = File::open("data.csv")?;
    // use f...
} // f is automatically closed here — guaranteed by the compiler
```

This pattern is called **RAII (Resource Acquisition Is Initialization)**:
- **Acquisition** happens when you create the value (`File::open`)
- **Release** happens when the value goes out of scope (compiler-inserted `drop`)

### Comparison

| Aspect | Python (context manager) | Rust (RAII) |
|---|---|---|
| Enforced by | Convention / `with` keyword | Compiler — always |
| Cleanup trigger | End of `with` block | End of scope (`}`) |
| Early return | `__exit__` runs | `drop` runs |
| Exception/panic | `__exit__` runs | `drop` runs (unwind) |
| Manual bypass | Easy (`open()` without `with`) | Impossible (ownership rules) |
| Deterministic | Yes (with `with`) | Yes (always) |
| Multiple resources | Nested `with` blocks | Multiple values, each drops |

---

## 4. Concept: The Drop Trait — Automatic Cleanup

### What is `Drop`?

The `Drop` trait is Rust's mechanism for defining what happens when a value
goes out of scope. It has a single method:

```rust
pub trait Drop {
    fn drop(&mut self);
}
```

The compiler automatically calls `drop()` when a value is no longer needed.
You **cannot** call it manually (well, you can with `std::mem::drop`, but
that's a different story — more on that later).

### Python Comparison: `__del__` vs `__exit__`

Python has two related concepts:

```python
# Python __del__ — unreliable, non-deterministic
class Resource:
    def __del__(self):
        print("Cleanup (maybe)")  # Called by GC — when? who knows!

# Python __exit__ — reliable but requires `with`
class Resource:
    def __enter__(self):
        return self
    def __exit__(self, *args):
        print("Cleanup (guaranteed)")  # Called when `with` block ends
```

Rust's `Drop` combines the reliability of `__exit__` with the automatic nature
of `__del__` — and it's guaranteed at compile time.

### Our `Resource` Struct

Here is the `Resource` struct in this project:

```rust
pub struct Resource {
    pub id: u32,
    pub is_open: bool,
}
```

It simulates a managed resource like a file handle. The `id` uniquely identifies
it, and `is_open` tracks whether it's currently "open" (acquired).

### Step 1: Implement `new` and `close`

```rust
impl Resource {
    pub fn new(id: u32) -> Self {
        Resource { id, is_open: true }
    }

    pub fn close(&mut self) {
        if self.is_open {
            println!("Resource {}: closing", self.id);
            self.is_open = false;
        }
    }

    pub fn is_open(&self) -> bool {
        self.is_open
    }
}
```

- `new(id)` creates a resource in the "open" state — this is **Acquisition**
- `close()` transitions it to "closed" — it is safe to call multiple times
- `is_open()` lets callers check the current state

### Step 2: Implement `Drop`

```rust
impl Drop for Resource {
    fn drop(&mut self) {
        if self.is_open {
            println!("Resource {}: auto-closing via Drop!", self.id);
            self.close();
        }
    }
}
```

Now, **every time a `Resource` goes out of scope**, it is automatically closed.
No manual `close()` calls needed. The compiler guarantees this.

### Seeing It in Action

```rust
fn raii_demo() -> Vec<String> {
    let mut messages = Vec::new();

    {
        let res = Resource::new(1);  // Resource is opened
        messages.push(format!("Resource {} created", res.id));
        // res is used here...
    }  // <-- res goes out of scope, drop() is called automatically

    messages.push("Resource 1 was dropped".to_string());
    messages
}
```

The output when you run this:

```
Resource 1 created
Resource 1: auto-closing via Drop!
Resource 1 was dropped
```

### Resource Lifecycle Diagram

```
┌─────────────────────────────────────────────────────┐
│                   Scope of `res`                     │
│                                                      │
│   let res = Resource::new(1);                        │
│   ┌──────────────────────────────────────┐           │
│   │  Resource { id: 1, is_open: true }   │           │
│   └──────────────────────────────────────┘           │
│         │                                            │
│         ▼                                            │
│   ... use res ...                                    │
│         │                                            │
│         ▼                                            │
│   }  ←── res goes out of scope                       │
│         │                                            │
│         ▼                                            │
│   drop(&mut self) is called automatically            │
│         │                                            │
│         ▼                                            │
│   Resource { id: 1, is_open: false }                 │
│         │                                            │
│         ▼                                            │
│   Memory is freed                                    │
└─────────────────────────────────────────────────────┘
```

---

## 5. Concept: Ownership and Resource Lifecycle

> **Recap**: Basic ownership (move, copy, single-owner rule) was taught in [01-TicketV1 §9 — Ownership](../01-TicketV1/README.md#9-concept-ownership--the-key-to-rust). Read that first if you have not.

This section focuses on the **RAII twist**: when a value owns a *resource* (file handle, socket, DB connection, timer), moving the value transfers **cleanup responsibility** along with it.

### Moving a Resource Transfers Cleanup Responsibility

```rust
fn take_ownership(r: Resource) {
    // r owns the resource now
    println!("Got resource {}", r.id);
} // r goes out of scope here — Drop runs, resource is cleaned up
```

This is the same move rule as before — but now the value it moves is something that needs *deterministic* cleanup, not just memory. The compiler still prevents use-after-move, double-free, and dangling references, but the practical effect is "the file is closed exactly once, at exactly the right moment".

### Why This Matters for Data Engineering

Consider an ETL pipeline that processes files in sequence:

```rust
fn etl_pipeline() {
    let raw = open_csv("raw_data.csv");   // Owned by etl_pipeline
    let cleaned = transform(raw);          // Ownership moves to transform
    // raw is no longer valid — can't accidentally re-read it
    let loaded = load(cleaned);            // Ownership moves to load
    // cleaned is no longer valid
} // loaded goes out of scope, file is closed
```

Each function takes ownership, processes the data, and passes it along. The
compiler ensures files are closed exactly once, at the right time, with zero
runtime overhead.

---

## 6. Concept: Borrowing Resources

> **Recap**: Basic borrowing (`&T` vs `&mut T`, aliasing rules) was taught in [01-TicketV1 §11 — References and Borrowing](../01-TicketV1/README.md#11-concept-references-and-borrowing). Read that first if you have not.

This section focuses on the **RAII twist for borrowing**: when you lend out a resource through a reference, the *original owner* is still responsible for cleanup. The borrower's `Drop` is **not** called.

```rust
fn borrow_resource(res: &Resource) -> u32 {
    println!("Borrowing resource {}", res.id);
    res.id
}  // res (the reference) goes out of scope, but the Resource is NOT dropped.
   // The original owner is still responsible.
```

For a data engineer this means: you can pass `&file_handle` into a function that needs to read from it, and the caller's `Drop` will close the file at the right time. The borrower cannot accidentally close or consume it.

See [01-TicketV1 §11](../01-TicketV1/README.md#11-concept-references-and-borrowing) for the full borrowing rules (`&T` vs `&mut T`, the "one mutable OR many immutable" rule, deref coercion).

---

## 7. Putting It All Together

Now let's build the complete project. Open `workshop/src/lib.rs` and follow these steps.

### Step 1: Implement `Resource` Methods

Replace the `todo!()` in `Resource::new`, `close`, and `is_open`:

```rust
impl Resource {
    pub fn new(id: u32) -> Self {
        Resource { id, is_open: true }
    }

    pub fn close(&mut self) {
        if self.is_open {
            self.is_open = false;
        }
    }

    pub fn is_open(&self) -> bool {
        self.is_open
    }
}
```

Run `cd workshop && cargo test` — the first 3 tests (`step_01_resource_lifecycle`) should pass.

### Step 2: Implement `Drop` for `Resource`

```rust
impl Drop for Resource {
    fn drop(&mut self) {
        if self.is_open {
            self.close();
        }
    }
}
```

### Step 3: Implement `raii_demo`

This function demonstrates RAII by creating a resource inside a block scope and
logging its lifecycle:

```rust
pub fn raii_demo() -> Vec<String> {
    let mut messages = Vec::new();

    {
        let res = Resource::new(1);
        messages.push(format!("Resource {} opened", res.id));
    } // drop() is called here

    messages.push("Resource was auto-closed".to_string());
    messages
}
```

Run `cd workshop && cargo test` — `step_02_raii_demo` tests should now pass.

### Step 4: Implement `ownership_transfer` and `borrow_resource`

```rust
pub fn ownership_transfer() -> u32 {
    let res = Resource::new(1);
    // In a real scenario, you'd move this to another function
    // Here we just return the id and let Drop handle cleanup
    res.id
}

pub fn borrow_resource(res: &Resource) -> u32 {
    res.id
}
```

Run `cd workshop && cargo test` — `step_03_ownership` tests should pass.

### Step 5: Implement `obrm_concepts`

```rust
pub fn obrm_concepts() -> Vec<&'static str> {
    vec![
        "RAII (Resource Acquisition Is Initialization)",
        "Drop trait — deterministic cleanup",
        "Ownership transfer — move semantics",
        "Borrowing — non-owning references",
    ]
}
```

Run `cd workshop && cargo test` — all 11 tests should pass.

### Complete `lib.rs`

Here is the complete implementation for reference:

```rust
pub struct Resource {
    pub id: u32,
    pub is_open: bool,
}

impl Resource {
    pub fn new(id: u32) -> Self {
        Resource { id, is_open: true }
    }

    pub fn close(&mut self) {
        if self.is_open {
            self.is_open = false;
        }
    }

    pub fn is_open(&self) -> bool {
        self.is_open
    }
}

impl Drop for Resource {
    fn drop(&mut self) {
        if self.is_open {
            self.close();
        }
    }
}

pub fn raii_demo() -> Vec<String> {
    let mut messages = Vec::new();
    {
        let res = Resource::new(1);
        messages.push(format!("Resource {} opened", res.id));
    }
    messages.push("Resource was auto-closed".to_string());
    messages
}

pub fn ownership_transfer() -> u32 {
    let res = Resource::new(1);
    res.id
}

pub fn borrow_resource(res: &Resource) -> u32 {
    res.id
}

pub fn obrm_concepts() -> Vec<&'static str> {
    vec![
        "RAII (Resource Acquisition Is Initialization)",
        "Drop trait — deterministic cleanup",
        "Ownership transfer — move semantics",
        "Borrowing — non-owning references",
    ]
}
```

### Run the Program

```bash
cd workshop && cargo run
```

You should see output like:

```
--- OBRM Demo ---
Resource 1 opened
Resource was auto-closed
Ownership transfer count: 1
Borrowed resource id: 42
Resource still open: true

OBRM Concepts:
  - RAII (Resource Acquisition Is Initialization)
  - Drop trait — deterministic cleanup
  - Ownership transfer — move semantics
  - Borrowing — non-owning references
```

---

## 8. Exercises

### Easy: Add a `name` Field

Add a `name: String` field to `Resource`. Update `new` to accept a name, and
modify `close` and `drop` to print the name instead of the ID. Update all
callers in `main.rs` and the test expectations.

**Solution hint**: You'll need to change `Resource::new(1)` to
`Resource::new(1, "file.csv")` and update the `new` function signature.

### Medium: Logging Resource

Create a `Logger` resource that wraps a `Vec<String>` and writes to it on
creation and destruction. Implement `Drop` so that when the logger goes out of
scope, it prints "Log saved with N entries". The logger should auto-flush on
drop.

```rust
pub struct Logger {
    pub entries: Vec<String>,
}

impl Logger {
    pub fn new() -> Self {
        Logger { entries: Vec::new() }
    }

    pub fn log(&mut self, msg: &str) {
        self.entries.push(msg.to_string());
    }
}

impl Drop for Logger {
    fn drop(&mut self) {
        println!("Log saved with {} entries", self.entries.len());
    }
}
```

Write a test that verifies the log message appears when a `Logger` goes out of
scope.

### Hard: Database Connection Pool Manager

Create a `PooledConnection` struct that simulates a database connection from a
pool. When a `PooledConnection` is dropped, it should automatically return
itself to the pool (increment a counter) rather than truly closing.

```rust
pub struct Pool {
    pub available: u32,
}

pub struct PooledConnection {
    pub id: u32,
    pub pool: *const Pool,  // raw pointer — we'll learn safer patterns later
}
```

Implement `Drop for PooledConnection` that increments `pool.available`. Write
tests to verify that connections are returned to the pool on drop, even if an
error occurs.

**Note**: This exercise introduces interior mutability concepts. Feel free to
use `RefCell` if you've seen it, or just use a raw pointer with `unsafe` for now.

---

## 9. Summary

| Concept | Description | Where Used |
|---|---|---|
| **RAII** | Resource Acquisition Is Initialization — tie resource lifetime to object lifetime | `Resource::new()` opens, `Drop::drop()` closes |
| **Drop trait** | Rust's deterministic destructor mechanism | `impl Drop for Resource` |
| **Ownership transfer** | Moving a value moves cleanup responsibility | `ownership_transfer()` |
| **Borrowing** | Using a reference (`&T`) to access without taking ownership | `borrow_resource(&res)` |
| **Move semantics** | Default in Rust — values are moved, not copied | Passing `Resource` to a function |
| **Scope-based cleanup** | Resources are cleaned up when `}` is reached, even on early return or panic | All examples |

### Key Takeaways for Data Engineers

1. **Forget `with` blocks** — Rust's RAII handles cleanup automatically, with
   zero runtime cost, at every scope boundary.
2. **Move to transfer** — When a pipeline stage is done with a resource, pass
   ownership to the next stage. The old stage can't accidentally use it.
3. **Borrow to inspect** — Need to check a resource's state without consuming
   it? Use `&Resource`. The original owner keeps cleanup responsibility.
4. **Deterministic > Best-effort** — Python's `__del__` is unreliable; Rust's
   `Drop` is guaranteed. For production data systems processing millions of
   files, this matters.

### Further Reading

- [The Rust Book, Ch. 15.3: The Drop Trait](https://doc.rust-lang.org/book/ch15-03-drop.html)
- [Rustnomicon: Drop](https://doc.rust-lang.org/nomicon/drop.html)
- [std::ops::Drop documentation](https://doc.rust-lang.org/std/ops/trait.Drop.html)
- [RAII in C++ (inspiration for Rust's approach)](https://en.cppreference.com/w/cpp/language/raii)
- Previous workshop: [02-Ownership/01-TicketV1](../01-TicketV1/README.md) —
  ownership fundamentals
- Next workshop: [02-Ownership/05-OwnershipLifetimes](../05-OwnershipLifetimes/README.md)
  — lifetimes and the borrow checker
