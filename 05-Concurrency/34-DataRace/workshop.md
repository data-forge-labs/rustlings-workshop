# Rust for Python Data Engineers — Data Races & Concurrency

*Learn how Rust prevents data races at compile time — a superpower Python lacks. Master threads, Mutex, Arc, and RwLock for safe parallel data processing.*

---

## Table of Contents

1. [Project Overview](#1-project-overview)
2. [Prerequisites](#2-prerequisites)
3. [Concept: Fearless Concurrency](#3-concept-fearless-concurrency)
4. [Concept: Threads with `thread::spawn`](#4-concept-threads-with-threadspawn)
5. [Concept: Move Closures — Sending Data to Threads](#5-concept-move-closures--sending-data-to-threads)
6. [Concept: `Arc<T>` — Shared Ownership Across Threads](#6-concept-arct--shared-ownership-across-threads)
7. [Concept: `Mutex<T>` — Mutual Exclusion](#7-concept-mutext--mutual-exclusion)
8. [Concept: `RwLock<T>` — Read-Write Lock](#8-concept-rwlockt--read-write-lock)
9. [Concept: `Condvar` — Condition Variables](#9-concept-condvar--condition-variables)
10. [Building Step by Step](#10-building-step-by-step)
11. [Exercises](#11-exercises)
12. [Summary](#12-summary)

---

## 1. Project Overview

We'll explore concurrent data access by:
1. Spawning multiple threads to modify shared data
2. Using `Mutex` and `Arc` for safe shared mutation
3. Comparing `RwLock` for read-heavy workloads
4. Using condition variables for thread coordination

### Python's Problem

```python
# Python — data race waiting to happen (GIL helps but doesn't eliminate)
import threading

data = [1, 2, 3]

def increment(i):
    data[i] += 1  # Two threads might read the same value!

threads = [threading.Thread(target=increment, args=(i,)) for i in range(3)]
for t in threads: t.start()
for t in threads: t.join()
print(data)  # Might be [2, 3, 4], or [1, 3, 4], or ...
```

```rust
// Rust — data race caught at compile time
use std::thread;

let mut data = vec![1, 2, 3];

for i in 0..3 {
    thread::spawn(move || {
        data[i] += 1;  // ❌ ERROR: cannot share data across threads!
    });
}
// Compiler: "data is moved into the closure"
```

---

## 2. Prerequisites

- Completed ownership ([TicketV1](../02-Ownership/3-TicketV1/workshop.md))
- Understand closures (briefly covered in [MasterMind](../01-Foundations/1-MasterMind/master_mind.md))

---

## 3. Concept: Fearless Concurrency

### The Concurrency Pyramid

```
            ┌──────────────────────┐
            │    Deadlocks         │ ← You handle these
            ├──────────────────────┤
            │    Race Conditions   │ ← Rust prevents with ownership
            ├──────────────────────┤
            │    Data Races        │ ← Rust prevents at COMPILE TIME
            ├──────────────────────┤
            │    Parallelism       │ ← Rust enables with std::thread
            └──────────────────────┘
```

### What Is a Data Race?

> A **data race** occurs when two or more threads access the same memory location concurrently, **and at least one access is a write**, and there's no synchronization.

Python + GIL prevents pure data races (only one thread runs at a time), but race conditions (wrong result due to timing) are still possible. Rust prevents both.

### Rust's Guarantee

> **Rust's type system guarantees that data races are impossible at compile time.**

```rust
let mut x = 5;

// ❌ Cannot have a reference and mutate concurrently
let r = &x;
x = 6;  // ERROR: cannot assign to `x` because it is borrowed
println!("{}", r);

// Same rule applies across threads — the compiler enforces it everywhere
```

---

## 4. Concept: Threads with `thread::spawn`

### Creating a Thread

```rust
use std::thread;
use std::time::Duration;

fn main() {
    // Spawn a new thread
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("Spawned thread: {i}");
            thread::sleep(Duration::from_millis(1));
        }
    });

    // Main thread continues
    for i in 1..5 {
        println!("Main thread: {i}");
        thread::sleep(Duration::from_millis(1));
    }

    // Wait for spawned thread to finish
    handle.join().unwrap();
}
```

### Scoped Threads

```rust
use std::thread;

fn main() {
    let data = vec![1, 2, 3];

    // Scoped threads can borrow from the parent
    thread::scope(|scope| {
        scope.spawn(|| {
            println!("From thread: {:?}", data);  // Borrows data!
        });
        scope.spawn(|| {
            println!("From thread 2: {:?}", data);
        });
    });  // All threads finish here

    // data is still available here
    println!("Back in main: {:?}", data);
}
```

### Python vs Rust Threading

```python
import threading

def worker(n):
    print(f"Worker {n}")

threads = [threading.Thread(target=worker, args=(i,)) for i in range(5)]
for t in threads: t.start()
for t in threads: t.join()
```

```rust
use std::thread;

let handles: Vec<_> = (0..5).map(|i| {
    thread::spawn(move || {
        println!("Worker {i}");
    })
}).collect();

for h in handles {
    h.join().unwrap();
}
```

---

## 5. Concept: Move Closures — Sending Data to Threads

### The Problem

```rust
let data = vec![1, 2, 3];

thread::spawn(|| {
    println!("{:?}", data);  // ❌ data might outlive the thread
});
// ERROR: closure may outlive the current function
// data is borrowed, but the compiler can't guarantee data lives long enough
```

### The Solution: `move`

```rust
let data = vec![1, 2, 3];

thread::spawn(move || {       // move = transfer ownership to the thread
    println!("{:?}", data);   // ✅ data is moved into the closure
});
// data is no longer available in main — it belongs to the thread now
```

### Memory Flow

```
Main thread:                    Spawned thread:
┌──────────────────────┐       ┌──────────────────────┐
│ data (Vec<i32>)      │       │                      │
│   ptr, len, cap ─────┼──┐   │                      │
│                      │  │   │                      │
│ thread::spawn(move   │  │   │ closure { data: Vec } │
│   || { ... data })   │  │   │   ptr, len, cap ←─────┘
└──────────────────────┘  │   └──────────────────────┘
                          │
                          ▼
                   Heap:
                  [1, 2, 3]
                  (owned by the thread)
```

---

## 6. Concept: `Arc<T>` — Shared Ownership Across Threads

### The Problem

```rust
let data = vec![1, 2, 3];

for _ in 0..3 {
    thread::spawn(move || {
        // Each thread gets its OWN copy of data
        // We want them to SHARE the same data!
    });
}
```

### `Rc` (Single-Threaded Reference Counting)

```rust
use std::rc::Rc;

let data = Rc::new(vec![1, 2, 3]);
let d1 = Rc::clone(&data);  // Increments reference count
let d2 = Rc::clone(&data);  // Increments reference count
// When the last Rc is dropped, data is freed
```

### `Arc` (Atomic Reference Counting — Thread-Safe)

```rust
use std::sync::Arc;
use std::thread;

let data = Arc::new(vec![1, 2, 3]);

let mut handles = vec![];
for _ in 0..3 {
    let data = Arc::clone(&data);  // Thread-safe clone
    handles.push(thread::spawn(move || {
        println!("{:?}", data);  // All threads share the same data
    }));
}

for h in handles {
    h.join().unwrap();
}
// data is freed when all Arcs are dropped
```

### `Arc` vs `Rc`

| | `Rc<T>` | `Arc<T>` |
|---|---|---|
| Thread-safe | No | Yes |
| Performance | Faster | Slower (atomic operations) |
| Use case | Single-threaded | Multi-threaded |
| Cost | Reference counting | Atomic reference counting |

### `Arc` Memory Diagram

```
Arc created:                 After Arc::clone():
refcount: 1    data          refcount: 2    data
┌─────────┐   ┌──────────┐  ┌─────────┐   ┌──────────┐
│ Arc A   │──→│ [1, 2, 3]│  │ Arc A   │──→│ [1, 2, 3]│
└─────────┘   └──────────┘  │ Arc B   │──→│          │
                            └─────────┘   └──────────┘
```

---

## 7. Concept: `Mutex<T>` — Mutual Exclusion

### What Is a Mutex?

A **Mutex** (Mutual Exclusion) ensures only one thread can access the data at a time.

```rust
use std::sync::Mutex;

let counter = Mutex::new(0);  // Create a mutex-guarded integer

{
    let mut num = counter.lock().unwrap();  // Acquire lock
    *num += 1;                              // Modify data
}  // Lock is released here (when MutexGuard is dropped)

println!("{:?}", counter);  // Mutex { data: 1 }
```

### Mutex + Arc: The Standard Pattern

```rust
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        handles.push(thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        }));
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());  // 10
}
```

### Python vs Rust: Shared Counter

```python
# Python — still need Lock for correct results
import threading

counter = 0
lock = threading.Lock()
def increment():
    global counter
    for _ in range(100000):
        with lock:    # Without lock: counter will be wrong
            counter += 1  # Without lock: race condition!

# Run with lock: counter = 1000000 (10 threads × 100000)
# Run without lock: counter ≈ 300000 (wrong!)
```

```rust
// Rust — Mutex + Arc, always correct
use std::sync::{Arc, Mutex};
use std::thread;

let counter = Arc::new(Mutex::new(0));
let mut handles = vec![];

for _ in 0..10 {
    let c = Arc::clone(&counter);
    handles.push(thread::spawn(move || {
        for _ in 0..100000 {
            *c.lock().unwrap() += 1;  // Always correct
        }
    }));
}

for h in handles { h.join().unwrap(); }
println!("{}", *counter.lock().unwrap());  // 1,000,000
```

### MutexGuard — Automatic Unlock

```rust
{
    let mut guard = counter.lock().unwrap();  // Lock acquired
    *guard += 1;
    // guard is dropped here → lock released automatically
}  // Never forget to unlock!

// No finally block needed — Rust's Drop handles it
```

---

## 8. Concept: `RwLock<T>` — Read-Write Lock

### When Reads > Writes

`RwLock` allows:
- **Multiple readers** simultaneously (no blocking for reads)
- **OR one writer** (exclusive access for writes)

```rust
use std::sync::{Arc, RwLock};
use std::thread;

fn main() {
    let data = Arc::new(RwLock::new(vec![1, 2, 3]));
    let mut handles = vec![];

    // Writer thread
    let data_ref = Arc::clone(&data);
    handles.push(thread::spawn(move || {
        let mut w = data_ref.write().unwrap();  // Exclusive write lock
        w.push(4);
    }));

    // Reader threads (can run concurrently)
    for _ in 0..5 {
        let data_ref = Arc::clone(&data);
        handles.push(thread::spawn(move || {
            let r = data_ref.read().unwrap();  // Shared read lock
            println!("Read: {:?}", r);
        }));  // Multiple readers don't block each other
    }

    for h in handles { h.join().unwrap(); }
}
```

### Mutex vs RwLock

| Aspect | `Mutex<T>` | `RwLock<T>` |
|---|---|---|
| Multiple readers | No (exclusive only) | Yes (shared) |
| One writer | Yes | Yes |
| Best for | Read + Write mixed | Read-heavy workloads |
| Overhead | Lower | Higher (more complex) |
| Starvation risk | Low | Possible (many readers block writer) |
| Mutex poisoning | Yes | Yes |

### Python vs Rust

```python
# Python — threading.Lock handles both
import threading
lock = threading.Lock()
# No distinction between read and write locks
with lock:
    read_data()  # Blocks other readers unnecessarily!
with lock:
    write_data()
```

```rust
// Rust — RwLock allows concurrent reads
use std::sync::RwLock;
let lock = RwLock::new(data);
let r1 = lock.read().unwrap();  // ✅ Multiple readers
let r2 = lock.read().unwrap();  // ✅ Allowed concurrently
drop(r1); drop(r2);  // Release before writing
let mut w = lock.write().unwrap();  // ✅ Exclusive
```

---

## 9. Concept: `Condvar` — Condition Variables

### Coordinating Threads

A `Condvar` lets threads wait for a condition to become true:

```rust
use std::sync::{Arc, Condvar, Mutex};
use std::thread;

fn main() {
    let pair = Arc::new((Mutex::new(false), Condvar::new()));
    let pair2 = Arc::clone(&pair);

    // Worker thread
    thread::spawn(move || {
        let (lock, cvar) = &*pair2;
        let mut started = lock.lock().unwrap();
        *started = true;
        cvar.notify_one();  // Signal the main thread
        println!("Worker: notified main");
    });

    // Main thread waits for signal
    let (lock, cvar) = &*pair;
    let mut started = lock.lock().unwrap();
    while !*started {
        started = cvar.wait(started).unwrap();  // Wait for notification
    }
    println!("Main: worker started!");
}
```

---

## 10. Building Step by Step

### Step 1: Create the Project

```bash
cargo new data-race
cd data-race
```

### Step 2: Basic Thread Example

```rust
use std::thread;

fn main() {
    let mut handles = vec![];

    for i in 0..5 {
        handles.push(thread::spawn(move || {
            println!("Thread {i} running");
        }));
    }

    for h in handles {
        h.join().unwrap();
    }

    println!("All threads done!");
}
```

### Step 3: Shared State with Arc + Mutex

```rust
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let data = Arc::new(Mutex::new(vec![1, 2, 3]));

    let handles: Vec<_> = (0..3).map(|i| {
        let data = Arc::clone(&data);
        thread::spawn(move || {
            let mut d = data.lock().unwrap();
            d[i] += 1;
            println!("Thread {i} modified: {:?}", d);
        })
    }).collect();

    for h in handles {
        h.join().unwrap();
    }

    println!("Final: {:?}", data.lock().unwrap());
}
```

### Step 4: RwLock (Read-Heavy)

```rust
use std::sync::{Arc, RwLock};
use std::thread;

fn main() {
    let data = Arc::new(RwLock::new(vec![1, 2, 3]));

    // Writers
    let mut handles = vec![];
    for i in 0..3 {
        let data = Arc::clone(&data);
        handles.push(thread::spawn(move || {
            let mut w = data.write().unwrap();
            w[i] += 1;
        }));
    }

    // Readers (can run simultaneously)
    for _ in 0..10 {
        let data = Arc::clone(&data);
        handles.push(thread::spawn(move || {
            let r = data.read().unwrap();
            println!("Read: {:?}", r);
        }));
    }

    for h in handles { h.join().unwrap(); }
    println!("Final: {:?}", data.read().unwrap());
}
```

---

## 11. Exercises

### Exercise 1: Safe Counter

Create a function that increments a shared counter 1000 times across 10 threads:

```rust
fn safe_increment() -> i32 {
    // Use Arc<Mutex<i32>>
}
```

### Exercise 2: Fix the Data Race

What's wrong with this code? Fix it:

```rust
let v = vec![1, 2, 3];
let mut handles = vec![];

for i in 0..3 {
    handles.push(thread::spawn(move || {
        println!("{}", v[i]);
    }));
}
```

### Exercise 3: Thread Pool Pattern

Process a list of items in parallel:

```rust
fn parallel_process(items: Vec<i32>) -> Vec<i32> {
    // Spawn threads to process items in parallel
    // Each thread doubles its assigned items
    // Return the combined results
}
```

---

## 12. Summary

| Concept | Description | Python Equivalent |
|---|---|---|
| `thread::spawn` | Create a new thread | `threading.Thread` |
| `move` closure | Transfer ownership to thread | N/A (implicit) |
| `Arc<T>` | Thread-safe reference counting | N/A (GC handles this) |
| `Mutex<T>` | Mutual exclusion for writes | `threading.Lock` |
| `RwLock<T>` | Read-write lock | N/A (no stdlib equivalent) |
| `Condvar` | Thread signaling | `threading.Condition` |
| `.join()` | Wait for thread | `.join()` |
| `thread::scope` | Scoped threads for borrowing | N/A |

### Key Data Engineering Takeaway

**Rust prevents data races at compile time.** This is revolutionary for data engineers who deal with:

1. **Parallel data processing** — Partition a dataset and process chunks in parallel with confidence
2. **Shared state** — Accumulate results from multiple threads without locks (use atomics or message passing)
3. **Pipeline stages** — Each stage runs in its own thread, passing data via channels

> "If it compiles, it doesn't have data races" — Rust's fearless concurrency promise.

### Next Project

Proceed to [35-SafeAndUnsafe](../07-Security/35-SafeAndUnsafe/workshop.md) for **unsafe Rust** and when to use it.
