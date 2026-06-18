# Lock-Free Atomics — Atomic Types and Memory Ordering

> **Test-driven approach**: This project includes a Cargo project with progressive unit tests. Each function in `workshop/src/lib.rs` starts as a `todo!()` stub. As you follow each section, replace `todo!()` with real code and run `cd workshop && cargo test` to watch the pass count grow. Your goal: **all 12 tests pass**.

## What Are Atomics?

Lock-free atomic types for high-performance concurrent counters — CPU-level instructions instead of OS mutexes.

### Python equivalent

```python
import threading

counter = 0
lock = threading.Lock()

def increment():
    global counter
    with lock:  # syscall + context switch per increment
        counter += 1
```

```rust
let counter = AtomicUsize::new(0);
counter.fetch_add(1, Ordering::Relaxed);  // single CPU instruction
```

You choose the memory ordering (`Relaxed` / `Acquire` / `Release` / `SeqCst`) for the trade-off between performance and synchronization guarantees.

## At a Glance

| # | Concept | Rust | Python | Why it matters |
|---|---------|------|--------|----------------|
| 1 | Atomic integers | `AtomicUsize`, `AtomicIsize` | `threading.Lock` + `int` | Lock-free counters |
| 2 | Atomic booleans | `AtomicBool` | `threading.Lock` + `bool` | Lock-free flags |
| 3 | Atomic add | `.fetch_add()` | `counter += 1` under lock | Atomic increment/decrement |
| 4 | Atomic store/load | `.store()`, `.load()` | `flag = True` under lock | Atomic read/write |
| 5 | Compare-and-swap | `.compare_exchange()` | N/A | Lock-free CAS primitive |
| 6 | Relaxed ordering | `Ordering::Relaxed` | N/A (GIL provides this) | No ordering guarantees |
| 7 | Acquire/Release | `Acquire` / `Release` | N/A | Happens-before guarantees for message passing |
| 8 | SeqCst | `Ordering::SeqCst` | N/A (GIL provides this) | Strongest ordering — slowest |

---

## Table of Contents
1. [Introduction](#1-introduction)
2. [Prerequisites](#2-prerequisites)
3. [Concept: Atomic types](#3-concept-atomic-types)
4. [Concept: Memory ordering](#4-concept-memory-ordering)
5. [Concept: Atomic operations](#5-concept-atomic-operations)
6. [Putting It All Together](#6-putting-it-all-together)
7. [Exercises](#7-exercises)
8. [Summary](#8-summary)

## 1. Introduction

In Python, there is no equivalent of atomic types. The `threading` module uses locks for all shared mutation. Rust provides `AtomicUsize`, `AtomicBool`, and others — lock-free primitives that are faster than `Mutex` for simple counters and flags.

Atomics are the building blocks of lock-free data structures. They use CPU-level instructions (e.g., `CAS`, `fetch_add`) to perform atomic operations without OS-level locking.

**Data engineering context**: Atomic counters are used for tracking processed records, progress bars, and cumulative statistics in high-throughput parallel pipelines where `Mutex` overhead would be prohibitive.

## 2. Prerequisites

- Threads from [01-Threads](../../01-Threads/README.md)
- `Arc` from [03-DataRace](../../03-DataRace/README.md)

## 3. Concept: Atomic types

### Explanation

Atomic types (in `std::sync::atomic`) support thread-safe operations without a `Mutex`. The most common are `AtomicUsize`, `AtomicIsize`, `AtomicBool`, and `AtomicPtr`.

```rust
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;

let counter = Arc::new(AtomicUsize::new(0));
counter.fetch_add(1, Ordering::Relaxed);
let val = counter.load(Ordering::Relaxed);
```

### Python comparison

Python has no stdlib atomic types. You must use `threading.Lock`:

```python
import threading

counter = 0
lock = threading.Lock()

with lock:
    counter += 1  # Lock required for even a simple increment
```

In Rust, `fetch_add` is a single CPU instruction — no context switch, no lock contention.

### Applying to our project

```rust
pub fn atomic_counter(ops: usize) -> usize {
    let counter = Arc::new(AtomicUsize::new(0));
    let mut handles = vec![];

    for _ in 0..4 {
        let c = Arc::clone(&counter);
        handles.push(thread::spawn(move || {
            for _ in 0..ops {
                c.fetch_add(1, Ordering::Relaxed);
            }
        }));
    }

    for h in handles { h.join().unwrap(); }
    counter.load(Ordering::Relaxed)
}
```

For `AtomicBool`:

```rust
pub fn atomic_flag_toggle() -> bool {
    let flag = Arc::new(AtomicBool::new(false));
    let f = Arc::clone(&flag);

    thread::spawn(move || {
        f.store(true, Ordering::Relaxed);
    }).join().unwrap();

    flag.load(Ordering::Relaxed)
}
```

## 4. Concept: Memory ordering

### Explanation

`Ordering` controls how an atomic operation synchronizes memory across threads. There are five levels:

| Ordering | Guarantee | Cost |
|---|---|---|
| `Relaxed` | No ordering constraints | Fastest |
| `Acquire` | Subsequent reads see previous writes from releasing thread | Medium |
| `Release` | Prior writes are visible to acquiring thread | Medium |
| `AcqRel` | Acquire + Release (for read-modify-write) | Medium |
| `SeqCst` | Sequential consistency — all threads see the same order | Slowest |

### Python comparison

Python has no equivalent. The GIL provides sequential consistency by default — but at the cost of parallelism.

### relaxed_ordering_demo

With `Relaxed`, each thread's increments are atomic, but the order in which different threads see each other's writes is unspecified. Both counters will still reach `40` (4 threads x 10 increments each):

```rust
pub fn relaxed_ordering_demo() -> (usize, usize) {
    let a = Arc::new(AtomicUsize::new(0));
    let b = Arc::new(AtomicUsize::new(0));
    let mut handles = vec![];

    for _ in 0..4 {
        let a = Arc::clone(&a);
        let b = Arc::clone(&b);
        handles.push(thread::spawn(move || {
            for _ in 0..10 {
                a.fetch_add(1, Ordering::Relaxed);
                b.fetch_add(1, Ordering::Relaxed);
            }
        }));
    }

    for h in handles { h.join().unwrap(); }
    (a.load(Ordering::Relaxed), b.load(Ordering::Relaxed))
}
```

### acquire_release_demo

`Acquire`/`Release` pairs create a **happens-before** relationship. A write with `Release` is guaranteed to be visible to a subsequent read with `Acquire` on the same atomic variable:

```rust
pub fn acquire_release_demo() -> (usize, usize) {
    let data = Arc::new(AtomicUsize::new(0));
    let flag = Arc::new(AtomicBool::new(false));
    let data_clone = Arc::clone(&data);
    let flag_clone = Arc::clone(&flag);

    let handle = thread::spawn(move || {
        data_clone.store(42, Ordering::Release);
        flag_clone.store(true, Ordering::Release);
    });

    while !flag.load(Ordering::Acquire) {
        std::hint::spin_loop();
    }
    let val = data.load(Ordering::Acquire);
    handle.join().unwrap();
    (val, flag.load(Ordering::Relaxed) as usize)
}
```

The producer stores `data = 42` then sets `flag = true`, both with `Release`. The consumer spins on `flag` with `Acquire` — when it sees `true`, it is guaranteed to also see `data = 42`.

## 5. Concept: Atomic operations

### Explanation

`AtomicUsize` supports several atomic read-modify-write operations:

- `fetch_add(n, order)` — atomic addition, returns old value
- `fetch_sub(n, order)` — atomic subtraction
- `compare_and_swap(old, new, order)` — atomic CAS
- `swap(val, order)` — atomic store + load
- `load(order)` — atomic read
- `store(val, order)` — atomic write

### Applying to our project

`fetch_add_demo` uses `Relaxed` ordering to sum values across threads:

```rust
pub fn fetch_add_demo(values: Vec<usize>) -> usize {
    let sum = Arc::new(AtomicUsize::new(0));
    let num_threads = 4;
    let chunk_size = (values.len() + num_threads - 1) / num_threads;
    let mut handles = vec![];

    for chunk in values.chunks(chunk_size) {
        let sum = Arc::clone(&sum);
        let chunk = chunk.to_vec();
        handles.push(thread::spawn(move || {
            for v in chunk {
                sum.fetch_add(v, Ordering::Relaxed);
            }
        }));
    }

    for h in handles { h.join().unwrap(); }
    sum.load(Ordering::Relaxed)
}
```

## 6. Putting It All Together

Implement each function in `workshop/src/lib.rs`:

| Function | Concept | Test Module |
|---|---|---|
| `atomic_counter()` | AtomicUsize fetch_add | `step_01_atomic_types` |
| `atomic_flag_toggle()` | AtomicBool store/load | `step_01_atomic_types` |
| `relaxed_ordering_demo()` | Ordering::Relaxed | `step_02_memory_ordering` |
| `acquire_release_demo()` | Ordering::Acquire/Release | `step_02_memory_ordering` |
| `fetch_add_demo()` | fetch_add for parallel sum | `step_03_atomic_operations` |

Note that `atomic_counter`, `relaxed_ordering_demo`, and `fetch_add_demo` already have reference implementations in the stub — review, understand, and ensure they compile.

## 7. Exercises

**Easy**: Replace the `Mutex` in a shared counter with `AtomicUsize` and measure the speed difference.

**Medium**: Implement a spinlock using `AtomicBool` and `compare_exchange`.

**Hard**: Build a lock-free ring buffer (bounded queue) using atomic indices with `Acquire`/`Release` ordering.

## 8. Summary

| Concept | Rust Type | Python Equivalent |
|---|---|---|
| Atomic integer | `AtomicUsize` | No equivalent (use Lock) |
| Atomic boolean | `AtomicBool` | No equivalent |
| Relaxed ordering | `Ordering::Relaxed` | No equivalent |
| Acquire/Release | `Ordering::Acquire`/`Release` | No equivalent |
| Atomic add | `fetch_add()` | No equivalent |
| Atomic CAS | `compare_exchange()` | No equivalent |

## Exercises

* **Easy** – modify the existing function to handle an extra edge case.
* **Medium** – extend the project with a new helper function that re‑uses the core logic.

