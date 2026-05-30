# Send/Sync Traits, RwLock — Thread Safety Foundations

> **Test-driven approach**: This project includes a Cargo project with progressive unit tests. Each function in `workshop/src/lib.rs` starts as a `todo!()` stub. As you follow each section, replace `todo!()` with real code and run `cd workshop && cargo test` to watch the pass count grow. Your goal: **all 20 tests pass**.

## Table of Contents
1. [Introduction](#1-introduction)
2. [Prerequisites](#2-prerequisites)
3. [Concept: Send and Sync marker traits](#3-concept-send-and-sync-marker-traits)
4. [Concept: RwLock — read-write pattern](#4-concept-rwlock--read-write-pattern)
5. [Concept: Parallel sum with scoped threads](#5-concept-parallel-sum-with-scoped-threads)
6. [Concept: Parallel map with scoped threads](#6-concept-parallel-map-with-scoped-threads)
7. [Putting It All Together](#7-putting-it-all-together)
8. [Exercises](#8-exercises)
9. [Summary](#9-summary)

## 1. Introduction

In Python, `concurrent.futures` provides a high-level interface for parallelism, but there is no way to guarantee at compile time that a type is safe to share across threads. Rust has two marker traits — `Send` and `Sync` — that the compiler uses to prevent thread-safety bugs before they happen.

This project explores these traits, demonstrates `RwLock` for concurrent reads with exclusive writes, and builds parallel sum and map functions using scoped threads.

**Data engineering context**: When processing large datasets in parallel, you need to know which data structures are safe to share (`Sync`) and which can be transferred between threads (`Send`). These traits guide the design of parallel data pipelines.

## 2. Prerequisites

- Threads from [01-Threads](../01-Threads/README.md)
- `Arc` and `Mutex` from [03-DataRace](../03-DataRace/README.md)

## 3. Concept: Send and Sync marker traits

### Explanation

`Send` and `Sync` are **auto traits** in Rust — they are automatically implemented for types that are thread-safe.

- **`Send`**: A type is `Send` if ownership can be transferred across threads. Almost all types are `Send`, except `Rc<T>` and raw pointers.
- **`Sync`**: A type is `Sync` if shared references (`&T`) can be shared across threads. Almost all types are `Sync`, except `Cell<T>`, `RefCell<T>`, and `Rc<T>`.

```rust
pub fn is_send<T: Send>(_: &T) -> bool { true }
pub fn is_sync<T: Sync>(_: &T) -> bool { true }
```

### Python comparison

Python has no equivalent of `Send`/`Sync`. The GIL masks most thread-safety bugs, but data races still happen (e.g., sharing a mutable list between threads). In Rust, the compiler catches these at compile time.

### Applying to our project

`is_send` and `is_sync` are generic with trait bounds. They always return `true` because the trait bound ensures the compiler only accepts types that implement the trait. `Rc<i32>` does not implement `Send`, so calling `is_send::<Rc<i32>>` would fail to compile — a compile-time safety check:

```rust
fn test_not_send() {
    let x = Rc::new(42i32);
    // assert!(is_send(&x));  // Would NOT compile: Rc<i32> is not Send
}
```

## 4. Concept: RwLock — read-write pattern

### Explanation

`RwLock` allows **multiple concurrent readers** or **one exclusive writer**. This is ideal for read-heavy workloads.

```rust
use std::sync::RwLock;

let data = RwLock::new(42);

// Multiple readers can hold the lock simultaneously
let r1 = data.read().unwrap();
let r2 = data.read().unwrap();
// Both succeed
drop(r1);
drop(r2);

// Writer requires exclusive access
let mut w = data.write().unwrap();
*w += 1;
```

### rwlock_read_heavy

This function spawns `readers` threads, each performing `ops_per_reader` read operations. The `RwLock` allows all readers to proceed concurrently:

```rust
pub fn rwlock_read_heavy(readers: usize, ops_per_reader: usize) -> usize {
    let counter = Arc::new(RwLock::new(0usize));
    let total_reads = Arc::new(std::sync::atomic::AtomicUsize::new(0));

    thread::scope(|s| {
        for _ in 0..readers {
            let counter = &counter;
            let total_reads = &total_reads;
            s.spawn(|| {
                for _ in 0..ops_per_reader {
                    let _guard = counter.read().unwrap();
                    total_reads.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                }
            });
        }
    });

    total_reads.load(std::sync::atomic::Ordering::Relaxed)
}
```

### rwlock_write_once

This function spawns `ops` writer threads, each doing one write. Writers are serialized by the `RwLock`:

```rust
pub fn rwlock_write_once(ops: usize) -> usize {
    let counter = Arc::new(RwLock::new(0usize));

    thread::scope(|s| {
        for _ in 0..ops {
            let counter = &counter;
            s.spawn(|| {
                let mut guard = counter.write().unwrap();
                *guard += 1;
            });
        }
    });

    *counter.read().unwrap()
}
```

### Python comparison

```python
import threading

# Python has no RwLock — readers block each other
lock = threading.Lock()
with lock:  # Blocks all other threads, even readers
    read_data()
```

## 5. Concept: Parallel sum with scoped threads

### Explanation

`thread::scope` allows borrowing references across threads without `move` closures. Combine it with `chunks` for data parallelism:

```rust
pub fn parallel_sum(data: Vec<i32>) -> i32 {
    if data.is_empty() { return 0; }

    thread::scope(|s| {
        let num_threads = thread::available_parallelism()
            .map(|n| n.get()).unwrap_or(4);
        let chunk_size = (data.len() + num_threads - 1) / num_threads;

        let mut handles = Vec::new();
        for chunk in data.chunks(chunk_size) {
            handles.push(s.spawn(|| chunk.iter().sum::<i32>()));
        }

        handles.into_iter().map(|h| h.join()).sum()
    })
}
```

### Python comparison

```python
from concurrent.futures import ThreadPoolExecutor

def parallel_sum(data: list[int]) -> int:
    with ThreadPoolExecutor() as ex:
        futures = [ex.submit(sum, data[i::4]) for i in range(4)]
        return sum(f.result() for f in futures)
```

## 6. Concept: Parallel map with scoped threads

### Explanation

Same pattern as `parallel_sum`, but applies a mapping function to each element:

```rust
pub fn parallel_map(data: Vec<i32>, mapper: fn(i32) -> i32) -> Vec<i32> {
    if data.is_empty() { return Vec::new(); }

    thread::scope(|s| {
        let num_threads = thread::available_parallelism()
            .map(|n| n.get()).unwrap_or(4);
        let chunk_size = (data.len() + num_threads - 1) / num_threads;

        let mut handles = Vec::new();
        for chunk in data.chunks(chunk_size) {
            handles.push(s.spawn(|| {
                chunk.iter().map(|&x| mapper(x)).collect::<Vec<i32>>()
            }));
        }

        let mut results = Vec::with_capacity(data.len());
        for handle in handles {
            results.extend(handle.join());
        }
        results
    })
}
```

### Python comparison

```python
from concurrent.futures import ThreadPoolExecutor

def parallel_map(data: list[int], mapper):
    with ThreadPoolExecutor() as ex:
        return list(ex.map(mapper, data))
```

## 7. Putting It All Together

Implement each function in `workshop/src/lib.rs`:

| Function | Concept | Test Module | Tests |
|---|---|---|---|
| `is_send()` | Send trait bound | `step_01_send_sync` | 6 |
| `is_sync()` | Sync trait bound | `step_01_send_sync` | 6 |
| `rwlock_read_heavy()` | RwLock concurrent reads | `step_02_rwlock` | 3 |
| `rwlock_write_once()` | RwLock exclusive writes | `step_02_rwlock` | 3 |
| `parallel_sum()` | Parallel reduce | `step_03_parallelism` | 4 |
| `parallel_map()` | Parallel transform | `step_03_parallelism` | 4 |

## 8. Exercises

**Easy**: Change `parallel_map` to use a configurable number of threads instead of `available_parallelism()`.

**Medium**: Implement a `parallel_filter` function using scoped threads that returns elements satisfying a predicate.

**Hard**: Build a parallel word count that reads lines from a `Vec<&str>`, counts words per chunk, and merges results using a `HashMap` behind an `Arc<RwLock<HashMap>>`.

## 9. Summary

| Concept | Rust | Python Equivalent |
|---|---|---|
| Send trait | `T: Send` | No equivalent |
| Sync trait | `T: Sync` | No equivalent |
| RwLock | `std::sync::RwLock` | No stdlib equivalent |
| Scoped threads | `thread::scope` | No equivalent |
| Parallel sum | Scoped threads + chunks | `concurrent.futures` |
| Parallel map | Scoped threads + mapper | `executor.map()` |
