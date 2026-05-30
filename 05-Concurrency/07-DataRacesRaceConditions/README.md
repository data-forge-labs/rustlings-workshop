# Data Races vs Race Conditions — Cell/RefCell Patterns

> **Test-driven approach**: This project includes a Cargo project with progressive unit tests. Each function in `workshop/src/lib.rs` starts as a `todo!()` stub. As you follow each section, replace `todo!()` with real code and run `cd workshop && cargo test` to watch the pass count grow. Your goal: **all 12 tests pass**.

## Why This Project?

### The Problem

In Python, interior mutability is the default — any variable can be mutated through any reference. This makes it easy to share mutable state but impossible to track at compile time:

```python
data = [1, 2, 3]

# Two "references" to the same list — both can mutate it
r1 = data
r2 = data

r1.append(4)  # data is now [1, 2, 3, 4]
r2.append(5)  # data is now [1, 2, 3, 4, 5]

# No error, no warnings — but threading this would be a disaster
```

```
Python:                 Rust:
Variable ──→ List       Variable ──→ Vec (one owner)
  │         ↑             │
  ├─ r1 ────┘             └─ only ONE reference at a time
  └─ r2 ────┘                 (compile-time enforced)
```

Python's "works by default, breaks in production" approach versus Rust's "breaks at compile time, works in production" philosophy is most visible with shared mutable state.

### The Rust Solution

Rust provides `Cell` and `RefCell` for controlled interior mutability in single-threaded contexts. These types opt into mutation through shared references while preserving safety:

```rust
use std::cell::Cell;

let counter = Cell::new(0usize);

// Clone the Cell reference (both point to same data)
let r1 = &counter;
let r2 = &counter;

r1.set(r1.get() + 1);  // Safe interior mutation
r2.set(r2.get() + 1);  // Both can modify via shared ref

assert_eq!(counter.get(), 2);
```

For multi-threaded contexts, `Cell` is deliberately not `Sync` — the compiler prevents using it across threads, directing you to `Mutex` instead.

## What You'll Learn

| # | Concept | Rust Type / Module | Python Equivalent | Purpose |
|---|---------|--------------------|------------------|---------|
| 1 | Interior Mutability | `Cell<T>` | Direct variable mutation | Mutate through shared ref (Copy types) |
| 2 | Runtime Borrow Checking | `RefCell<T>` | No equivalent | Dynamic borrow rules for non-Copy types |
| 3 | Data Race vs Race Condition | Rust concepts | Same concepts | Understand the key difference |
| 4 | Compile-Time Prevention | Ownership rules | No equivalent | Data races eliminated at compile time |
| 5 | Programmer Responsibility | Race conditions | Same | Race conditions still possible in Rust |

## Concepts at a Glance

- **Interior Mutability (`Cell<T>`)**: Allows mutation through a shared `&Cell<T>` reference for `Copy` types. Python allows this implicitly; Rust requires explicit opt-in. `Cell` is single-threaded only (not `Sync`).
- **Runtime Borrow Checking (`RefCell<T>`)**: Enforces Rust's borrow rules (one mutable or many immutable) at runtime instead of compile time. Violations cause a panic. Python has no equivalent — borrowing violations are simply impossible there.
- **Data Race vs Race Condition**: A data race is unsynchronized concurrent memory access (Rust prevents at compile time). A race condition is a logic error from timing (Rust prevents neither — the programmer must handle it).
- **Compile-Time Prevention**: Rust's type system prevents data races by enforcing `Send`/`Sync` and ownership rules. `Cell` and `RefCell` are deliberately `!Sync` to prevent cross-thread sharing.
- **Programmer Responsibility**: Even with safe Rust, race conditions (like read-check-write without holding the lock) are possible. Rust prevents data races but not all concurrency bugs — the programmer must still design correct logic.

---

## Table of Contents
1. [Introduction](#1-introduction)
2. [Prerequisites](#2-prerequisites)
3. [Concept: Cell — interior mutability for Copy types](#3-concept-cell--interior-mutability-for-copy-types)
4. [Concept: RefCell — runtime borrow checking](#4-concept-refcell--runtime-borrow-checking)
5. [Concept: Race conditions vs data races](#5-concept-race-conditions-vs-data-races)
6. [Putting It All Together](#6-putting-it-all-together)
7. [Exercises](#7-exercises)
8. [Summary](#8-summary)

## 1. Introduction

Rust's ownership rules prevent **data races** (unsynchronized concurrent memory access) at compile time. But **race conditions** (logic errors from timing) can still occur. This project explores Rust's interior mutability types — `Cell` and `RefCell` — which allow mutation through shared references in single-threaded contexts.

**Data engineering context**: In single-threaded ETL pipelines, you may need to mutate a counter or accumulate results inside a closure that only has a shared reference. `Cell` and `RefCell` provide safe interior mutability without the overhead of `Mutex`.

## 2. Prerequisites

- Ownership from [TicketV1](../../02-Ownership/01-TicketV1/README.md)
- `Mutex` from [03-DataRace](../03-DataRace/README.md)

## 3. Concept: Cell — interior mutability for Copy types

### Explanation

`Cell<T>` allows mutation through a shared `&Cell<T>` reference. It only works with `Copy` types (like integers, bools, enums).

```rust
use std::cell::Cell;

let counter = Cell::new(0);
counter.set(5);                      // Replace value
let val = counter.get();             // Read value
counter.replace(10);                 // Replace and return old value
```

### Python comparison

```python
# Python — all types are "interior mutable" by default
counter = 0

def inc():
    global counter  # Need global keyword for reassignment
    counter += 1
```

In Python, mutation through any reference is always allowed. Rust restricts this — `Cell` is the opt-in mechanism.

### Applying to our project

```rust
pub fn cell_counter(ops: usize) -> usize {
    let counter = Cell::new(0usize);
    for _ in 0..ops {
        counter.set(counter.get() + 1);
    }
    counter.get()
}

pub fn cell_string(initial: &str, append: &str) -> String {
    let s = Cell::new(initial.to_string());
    let mut val = s.into_inner();
    val.push_str(append);
    val
}
```

Note: For `String` (non-Copy), we use `into_inner()` or `take()`. `Cell` only works directly with `Copy` types for `get`/`set`, but can hold any type via `take`/`replace`/`into_inner`.

## 4. Concept: RefCell — runtime borrow checking

### Explanation

`RefCell<T>` enforces borrow rules at **runtime** instead of compile time. You can have multiple immutable borrows (`borrow()`) or one mutable borrow (`borrow_mut()`), checked dynamically.

```rust
use std::cell::RefCell;

let data = RefCell::new(vec![1, 2, 3]);

data.borrow_mut().push(4);       // Mutable borrow at runtime
let len = data.borrow().len();   // Immutable borrow at runtime
```

If you violate the rules, you get a **panic** at runtime instead of a compile error:

```rust
let r1 = data.borrow();
let r2 = data.borrow_mut();  // PANIC! Already borrowed immutably
```

### Python comparison

```python
# Python — borrow checking doesn't exist
data = [1, 2, 3]

# All of this works:
r1 = data
r2 = data      # Two mutable references? No problem
data.append(4) # Mutation through original — r1 and r2 still point to it
```

### Applying to our project

`refcell_demo` uses `RefCell` to mutate a vector through a shared reference:

```rust
pub fn refcell_demo(values: Vec<i32>) -> Vec<i32> {
    let result = RefCell::new(Vec::new());
    for &v in &values {
        result.borrow_mut().push(v * 2);
    }
    result.into_inner()
}
```

`refcell_borrow_error` demonstrates the runtime panic:

```rust
pub fn refcell_borrow_error() -> Result<String, String> {
    let data = RefCell::new(String::from("hello"));
    let _r1 = data.borrow();
    // This will panic at runtime
    let _r2 = data.borrow_mut();
    Ok("success".to_string())
}
```

To handle this gracefully, use `try_borrow()` / `try_borrow_mut()` which return `Result`:

```rust
let _r1 = data.borrow();
let r2 = data.try_borrow_mut();  // Returns Err("already borrowed")
match r2 {
    Ok(_) => Ok("success".to_string()),
    Err(e) => Err(format!("borrow error")),
}
```

## 5. Concept: Race conditions vs data races

### Explanation

- **Data race**: Unsynchronized concurrent memory access (compile-time error in Rust, runtime bug in Python).
- **Race condition**: Logic error where the outcome depends on the timing of events.

Rust eliminates data races but not race conditions. You can still have race conditions with atomics or `Mutex`.

`simulate_race_condition()` uses `Cell` with multiple threads (incorrectly — `Cell` is not `Sync`) to demonstrate lost updates:

Since `Cell` is not `Sync` (intentionally), you cannot share it across threads. The demonstration instead uses `Arc<Mutex<usize>>` with incorrect logic to simulate lost updates:

```rust
pub fn simulate_race_condition() -> usize {
    let counter = Arc::new(Mutex::new(0usize));
    let mut handles = vec![];

    for _ in 0..8 {
        let c = Arc::clone(&counter);
        handles.push(thread::spawn(move || {
            for _ in 0..1000 {
                let val = *c.lock().unwrap();
                // Race: another thread might increment between these two lines!
                *c.lock().unwrap() = val + 1;
            }
        }));
    }

    for h in handles { h.join().unwrap(); }
    *counter.lock().unwrap()
}
```

### Python comparison

```python
import threading

counter = 0

def increment():
    global counter
    for _ in range(1000):
        val = counter        # Read
        counter = val + 1    # Write (another thread may have changed counter!)

threads = [threading.Thread(target=increment) for _ in range(8)]
for t in threads: t.start()
for t in threads: t.join()
print(counter)  # Will be much less than 8000 due to race condition
```

This is a race condition — not a data race (the load and store are separate). Rust prevents data races but the programmer must prevent race conditions by using atomic read-modify-write (`fetch_add`) or holding the lock across both read and write.

## 6. Putting It All Together

Implement each function in `workshop/src/lib.rs`:

| Function | Concept | Test Module | Tests |
|---|---|---|---|
| `cell_counter()` | `Cell` in single-threaded context | `step_01_cell` | 3 |
| `cell_string()` | `Cell` with take/into_inner | `step_01_cell` | 3 |
| `refcell_demo()` | `RefCell` for mutable borrow | `step_02_refcell` | 3 |
| `refcell_borrow_error()` | Runtime borrow violation | `step_02_refcell` | 1 |
| `simulate_race_condition()` | Race condition demo | `step_03_race_conditions` | 2 |

## 7. Exercises

**Easy**: Modify `cell_counter` to use `Cell` with `replace()` instead of `get()`/`set()`.

**Medium**: Write a function that uses `RefCell` with `try_borrow_mut` to gracefully handle the case when a borrow already exists.

**Hard**: Implement a small cache using `RefCell<HashMap<K,V>>` that holds recently computed values and returns cached results or computes new ones.

## 8. Summary

| Concept | Rust Type | Python Equivalent |
|---|---|---|
| Interior mutability (Copy) | `Cell<T>` | Direct mutation |
| Runtime borrow checking | `RefCell<T>` | No equivalent |
| Borrow at runtime | `refcell.borrow()` | No equivalent |
| Mutable borrow at runtime | `refcell.borrow_mut()` | No equivalent |
| Data race | Rust eliminates at compile time | GIL + Lock |
| Race condition | Programmer responsibility | Programmer responsibility |
