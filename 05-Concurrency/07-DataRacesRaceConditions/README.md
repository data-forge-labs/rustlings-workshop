# Data Races vs Race Conditions — Cell/RefCell Patterns

> **Test-driven approach**: This project includes a Cargo project with progressive unit tests. Each function in `workshop/src/lib.rs` starts as a `todo!()` stub. As you follow each section, replace `todo!()` with real code and run `cd workshop && cargo test` to watch the pass count grow. Your goal: **all 12 tests pass**.

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
