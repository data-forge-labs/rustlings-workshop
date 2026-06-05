# Rust Concurrency Model — Quick Reference

## Fearless Concurrency

Rust guarantees **no data races at compile time** through ownership + the type system. If your code compiles, it is free of data races (though not necessarily of deadlocks).

## Thread Safety: `Send` and `Sync`

| Trait | Meaning | Example |
|-------|---------|---------|
| `Send` | Ownership can be transferred to another thread | `Arc<T>`, `Mutex<T>`, `mpsc::Sender<T>` |
| `Sync` | Shared reference `&T` can be accessed from multiple threads | `Mutex<T>`, `RwLock<T>`, `AtomicBool` |
| `!Send` | Not safe to transfer between threads | `Rc<T>`, `RefCell<T>` |
| `!Sync` | Not safe to share across threads | `RefCell<T>`, `Cell<T>` |

See `send-sync.md` for deep coverage.

## Shared-State Concurrency

### `Arc<T>` + `Mutex<T>`

```rust
use std::sync::{Arc, Mutex};
let data = Arc::new(Mutex::new(vec![1, 2, 3]));
let mut guard = data.lock().unwrap();
guard.push(4); // guard auto-drops when scope ends
```

### `RwLock<T>`

Multiple readers OR one writer — like Python's `threading.RLock` but with shared reads.

```rust
use std::sync::RwLock;
let lock = RwLock::new(5);
{
    let r = lock.read().unwrap();   // multiple readers OK
    let r2 = lock.read().unwrap();
}
let mut w = lock.write().unwrap();  // exclusive
*w += 1;
```

## Message-Passing: `mpsc` Channels

```rust
use std::sync::mpsc;
let (tx, rx) = mpsc::channel();
tx.send(42).unwrap();
assert_eq!(rx.recv().unwrap(), 42);
```

- **`mpsc`**: multi-producer, single-consumer
- `tx.clone()` for multiple senders
- `rx.try_recv()` for non-blocking receive

## Async/Await with Tokio

```rust
#[tokio::main]
async fn main() {
    let handle = tokio::spawn(async {
        // concurrent work
    });
    handle.await.unwrap();
}
```

- **Tokio** is the standard async runtime (like Python's `asyncio`)
- Multi-threaded by default (worker threads)
- `tokio::spawn` for tasks, `tokio::sync` for channels/mutexes
- Use `tokio::fs` and `tokio::io` for non-blocking I/O

## Data Parallelism with Rayon

```rust
use rayon::prelude::*;

let sum: i32 = vec![1, 2, 3, 4, 5]
    .par_iter()
    .map(|x| x * 2)
    .sum();
```

- `.par_iter()` — parallel iterator (like `concurrent.futures` + map)
- `.par_sort()` — parallel sort
- Automatically uses all CPU cores
- Good for CPU-bound data processing

## Atomics & Memory Ordering

| Type | Use |
|------|-----|
| `AtomicBool` | Flag shared across threads |
| `AtomicI32` / `AtomicU64` | Counter |
| `AtomicPtr` | Lock-free linked structures |

Memory ordering (from weakest to strongest):
- `Relaxed` — no ordering guarantees (counter)
- `Acquire` / `Release` — for locks, store-load pairs
- `AcqRel` — combine acquire + release
- `SeqCst` — strongest, sequential consistency

```rust
use std::sync::atomic::{AtomicBool, Ordering};
static FLAG: AtomicBool = AtomicBool::new(false);
FLAG.store(true, Ordering::SeqCst);
assert!(FLAG.load(Ordering::SeqCst));
```

## Deadlock Prevention

1. **Always lock in the same order** across all threads.
2. **Use `try_lock`** with timeout instead of `lock`.
3. **Minimize lock scope** — lock, do work, drop.
4. **Prefer message-passing** over shared state when possible.
5. **Use a lock-free data structure** (e.g., `crossbeam` channels, `dashmap`).

## Python → Rust Concurrency

| Python | Rust |
|--------|------|
| `threading.Thread` | `std::thread::spawn` |
| `threading.Lock` | `std::sync::Mutex` |
| `threading.RLock` | `std::sync::RwLock` |
| `asyncio` | `tokio` |
| `concurrent.futures.ProcessPoolExecutor` | `rayon::par_iter()` |
| `queue.Queue` | `std::sync::mpsc` or `crossbeam::channel` |
| `multiprocessing` | Not directly equivalent (Rust threads are OS threads) |
| GIL prevents true parallelism | Full parallel execution |
