# Section 5: Concurrency — Beyond Python's GIL

*Python threads are limited by the GIL. Rust gives you true parallelism with compile-time safety guarantees.*

---

## Why This Section?

### The Problem — Python's GIL Cage

Python's Global Interpreter Lock (GIL) means your multi-threaded code **doesn't actually run in parallel**:

```python
import threading
import time

counter = 0

def increment(n):
    global counter
    for _ in range(n):
        counter += 1  # Protected by GIL, but only ONE thread runs at a time

# 8 threads, but only 1 runs at any instant
threads = [threading.Thread(target=increment, args=(10_000_000,))
           for _ in range(8)]

start = time.time()
for t in threads: t.start()
for t in threads: t.join()
print(f"Time: {time.time() - start:.2f}s")  # Same as single-threaded!
```

```
┌─────────────────────────────────────────────────────┐
│  Python Thread Execution (GIL)                       │
│                                                      │
│  Thread 1  ████████░░░░░░░░░░░░░░░░░░░░░░░░░░░      │
│  Thread 2  ░░░░░░░░████████░░░░░░░░░░░░░░░░░░░      │
│  Thread 3  ░░░░░░░░░░░░░░░░████████░░░░░░░░░░░      │
│  Thread 4  ░░░░░░░░░░░░░░░░░░░░░░░░████████░░░      │
│                                                      │
│  CPU 0     ████████████████████████████████████      │
│                                                      │
│  One thread runs at a time → NO speedup              │
└─────────────────────────────────────────────────────┘
```

**Consequences for data engineering:**

| Scenario | Python | Rust |
|----------|--------|------|
| Process 10 large CSV files | Sequential (GIL) | Parallel (all cores) |
| Real-time API with many clients | `asyncio` (co-op) | `tokio` (true async I/O) |
| Shared counter | Needs `multiprocessing` | `AtomicU64` — lock-free |
| Pipeline with 4 stages | `queue.Queue` + processes | `mpsc` channels + threads |
| Concurrent hashmap | Lock everything | `Arc<RwLock<HashMap>>` |

### The Rust Solution — True Parallelism + Safety

```rust
use std::thread;
use std::sync::Arc;

let data = Arc::new(vec![1, 2, 3, 4, 5, 6, 7, 8]);
let mut handles = vec![];

for i in 0..4 {
    let data = Arc::clone(&data);
    handles.push(thread::spawn(move || {
        let sum: i32 = data[i*2..(i+1)*2].iter().sum();
        println!("Thread {} sum: {}", i, sum);
    }));
}
```

```
┌─────────────────────────────────────────────────────┐
│  Rust Thread Execution (True Parallelism)            │
│                                                      │
│  Thread 1  ████████████████████████████████████      │
│  Thread 2  ████████████████████████████████████      │
│  Thread 3  ████████████████████████████████████      │
│  Thread 4  ████████████████████████████████████      │
│                                                      │
│  CPU 0     ████████████████████████████████████      │
│  CPU 1     ████████████████████████████████████      │
│  CPU 2     ████████████████████████████████████      │
│  CPU 3     ████████████████████████████████████      │
│                                                      │
│  All threads run simultaneously → 4x speedup         │
└─────────────────────────────────────────────────────┘
```

And crucially: **Rust prevents data races at compile time**. Python can't catch data races — you discover them in production after hours of debugging. Rust's borrow checker rejects them before the code compiles.

---

## What You'll Learn

| # | Concept | Rust Type / Module | Python Equivalent | Purpose |
|---|---------|--------------------|------------------|---------|
| 1 | OS threads | `std::thread` | `threading.Thread` | True parallel execution |
| 2 | Scoped threads | `std::thread::scope` | `concurrent.futures` | Safe thread spawning with borrows |
| 3 | Message passing | `std::sync::mpsc` | `queue.Queue` | Send data between threads |
| 4 | Mutual exclusion | `Mutex<T>` | `threading.Lock` | Protect shared data (one writer) |
| 5 | Read-write lock | `RwLock<T>` | `threading.RLock` | Many readers or one writer |
| 6 | Atomic reference counting | `Arc<T>` | N/A (GC) | Thread-safe shared ownership |
| 7 | Atomic primitives | `AtomicU64`, `AtomicBool`, etc. | N/A | Lock-free concurrent counters |
| 8 | Memory ordering | `Ordering::Relaxed`, `Acquire`, `Release` | N/A | Control visibility of atomic ops |
| 9 | Data parallelism | `rayon` crate | `concurrent.futures` | Automatic parallel iterators |
| 10 | Async functions | `async fn` | `async def` | Concurrent I/O without threads |
| 11 | Async runtime | `tokio` | `asyncio` | Event-driven I/O executor |
| 12 | Async tasks | `tokio::spawn` | `asyncio.create_task` | Lightweight concurrent tasks |
| 13 | Interior mutability | `Cell<T>`, `RefCell<T>` | N/A | Single-threaded mutation behind `&` |
| 14 | Send trait | `Send` | N/A | Types safe to send between threads |
| 15 | Sync trait | `Sync` | N/A | Types safe to share between threads |
| 16 | Deadlock prevention | Lock ordering | N/A | Systematic deadlock avoidance |

---

## Concepts at a Glance

### 1. `std::thread` — OS Threads

```rust
use std::thread;

let handle = thread::spawn(|| {
    println!("Hello from a thread!");
});
handle.join().unwrap();  // wait for thread to finish
```

In Python: `threading.Thread(target=fn).start()`

### 2. Scoped Threads — Safe Borrowing

```rust
use std::thread;

let numbers = vec![1, 2, 3];
thread::scope(|s| {
    s.spawn(|| {
        println!("{}", numbers.len());  // borrows 'numbers'
    });
});  // numbers still available here
```

### 3. `mpsc` — Message Passing Channels

```rust
use std::sync::mpsc;

let (tx, rx) = mpsc::channel();
thread::spawn(move || {
    tx.send(42).unwrap();
});
println!("Received: {}", rx.recv().unwrap());
```

In Python: `queue.Queue`

### 4. `Mutex<T>` — Shared State

```rust
use std::sync::{Arc, Mutex};

let counter = Arc::new(Mutex::new(0));
let mut handles = vec![];

for _ in 0..10 {
    let counter = Arc::clone(&counter);
    handles.push(thread::spawn(move || {
        let mut num = counter.lock().unwrap();
        *num += 1;
    }));
}
```

### 5. `Arc<T>` — Thread-Safe Reference Counting

Python's GC handles shared references automatically. Rust uses `Arc` (Atomic Reference Counting):

```
  Arc layout:
  ┌──────────────┐
  │  Arc<T>      │     ┌──────────────┐
  │  ptr ────────┼─────►  T (data)    │
  │  ref_count   │     └──────────────┘
  └──────────────┘
       share via Clone (increment ref_count)
```

### 6. `rayon` — Data Parallelism

The easiest way to parallelize data processing:

```rust
use rayon::prelude::*;

let numbers: Vec<i64> = (0..1_000_000).collect();
let sum: i64 = numbers.par_iter().sum();  // automatic parallel!
let max = numbers.par_iter().max();
```

In Python: `concurrent.futures.ProcessPoolExecutor`

### 7. `async` / `.await` — Async I/O

```rust
use tokio::fs::File;
use tokio::io::AsyncReadExt;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut file = File::open("data.csv").await?;
    let mut contents = String::new();
    file.read_to_string(&mut contents).await?;
    Ok(())
}
```

### 8. `Send` and `Sync` — Safety Markers

These marker traits tell the compiler what's safe to share:

```
  ┌─────────────────────────────────────────┐
  │  Send: type can be TRANSFERRED between  │
  │        threads (ownership moves)         │
  │         e.g., Arc<T>, Mutex<T>, Box<T>   │
  ├─────────────────────────────────────────┤
  │  Sync: type can be SHARED between       │
  │        threads (&T is Send)             │
  │         e.g., Arc<T>, Mutex<T>, i32     │
  └─────────────────────────────────────────┘
```

### Thread Safety Comparison

| Rust Type | Send? | Sync? | Python Equivalent |
|-----------|-------|-------|-------------------|
| `Rc<T>` | No | No | Regular Python ref |
| `Arc<T>` | Yes | Yes | N/A (GC handles) |
| `Mutex<T>` | Yes | Yes | `threading.Lock` |
| `RefCell<T>` | No | No | Regular Python object |
| `AtomicU64` | Yes | Yes | N/A |
| `mpsc::Sender` | Yes | No | `queue.Queue` |

---

## Prerequisites

- Completed [Section 3: Collections](../03-Collections/README.md)
- Understand ownership deeply from Section 2

## Projects in This Section

| # | Project | Concepts | Format |
|---|---------|----------|--------|
| 7 | **Threads** — threads, channels, locks | `std::thread`, `'static`, scoped threads, `mpsc`, interior mutability, `Mutex`/`Arc`, `RwLock`, `Sync` | Tutorial |
| 8 | **Futures** — async/await, tasks, runtimes | `async fn`, `.await`, `tokio`, `Future` trait, spawning, cancellation | Tutorial |
| 34 | **DataRace** — preventing data races | `Mutex`, `Arc`, `MutexGuard`, shared-state concurrency | Project |
| 44 | **Atomics** — lock-free atomics | Atomic types, memory ordering (`Relaxed`, `Acquire`, `Release`, `SeqCst`) | Project |
| 45 | **DistributedChallenges** — consistency | Eventual vs strong consistency, CAP theorem | Project |
| 46 | **ConcurrencyParallelism** — Send/Sync, RwLock | `Send`/`Sync` traits, `Mutex`, `RwLock`, `Arc` | Project |
| 47 | **DataRacesRaceConditions** — data races vs race conditions | Data races, race conditions, `Cell`/`RefCell` | Project |
| 48 | **DiningPhilosophers** — deadlock prevention | `Mutex`, ordered lock acquisition, thread synchronization | Project |
| 49 | **DistributedComputing** — Rust for distributed systems | GC overhead, compiled vs interpreted, distributed challenges | Reflection |
| 50 | **RayonChallenge** — data parallelism with Rayon | `rayon` parallel iterators, speedup benchmarking | Project |
| 51 | **SendSync** — Send and Sync marker traits | `Send`, `Sync`, thread safety markers, `unsafe impl` | Project |
| 52 | **ConcurrencyLessonReflection** — concurrency review | Ownership + concurrency, data-race freedom, `mpsc` | Reflection |

## Learning Path

1. Study **7-Threads** tutorial for threading fundamentals
2. Study **8-Futures** tutorial for async/await patterns
3. Build **03-DataRace** to see Rust prevent data races at compile time
4. Explore **04-Atomics** through **11-SendSync** for advanced concurrency
5. Finish with **10-RayonChallenge** (data parallelism)
