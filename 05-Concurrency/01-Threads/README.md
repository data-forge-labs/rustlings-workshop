# Threads, Channels, Locks — Foundational Concurrency Primitives

> **Test-driven approach**: This project includes a Cargo project with progressive unit tests. Each function in `workshop/src/lib.rs` starts as a `todo!()` stub. As you follow each section, replace `todo!()` with real code and run `cd workshop && cargo test` to watch the pass count grow. Your goal: **all 9 tests pass**.

---

## What Is This Project?

OS threads, channels, and locks — the foundational primitives for concurrent Rust programs.

### Python equivalent

```python
import threading

def worker():
    return "Hello from thread"

t = threading.Thread(target=worker)
t.start()
result = t.join()  # blocks until done
```

In this project you'll learn to build this in Rust — and along the way
you'll discover **`std::thread::spawn`**, **`mpsc` channels**, and **`Mutex` locks**.

## 2. Prerequisites

- Ownership and borrowing from [TicketV1](../../../../02-Ownership/01-TicketV1/README.md)
- `Arc<T>` basics (introduced in [TicketV2](../../../../02-Ownership/03-TicketV2/README.md))
- Closures from [MasterMind](../../../../01-Foundations/04-MasterMind/README.md)

## 3. Concept: Threads — spawn and join

### Explanation

In Python, `threading.Thread(target=fn).start()` spawns a thread but the GIL limits parallelism. In Rust, `std::thread::spawn` creates a real OS thread.

```rust
use std::thread;

let handle = thread::spawn(|| {
    "Hello from thread".to_string()
});

let result = handle.join().unwrap();
```

`spawn` takes a closure. `join()` blocks until the thread finishes and returns a `Result<T, Box<dyn Any + Send>>` — unwrap it to get the value.

### Python comparison

```python
import threading

def worker():
    return "Hello from thread"

t = threading.Thread(target=worker)
t.start()
t.join()
```

Rust closures can return values from threads directly via `join()`. In Python you need a mutable list or a `Queue` to retrieve results.

### Applying to our project

The `spawn_and_join()` function must spawn a thread that returns a greeting string. The thread closure creates the string, and `join().unwrap()` gives it back.

```rust
pub fn spawn_and_join() -> String {
    let handle = thread::spawn(|| {
        String::from("Hello from thread!")
    });
    handle.join().unwrap()
}
```

For `sum_in_parallel`, split the vector into two halves, spawn two threads to sum each half, then add the results:

```rust
pub fn sum_in_parallel(data: Vec<i32>) -> i32 {
    let mid = data.len() / 2;
    let left = data[..mid].to_vec();
    let right = data[mid..].to_vec();

    let h1 = thread::spawn(move || left.iter().sum::<i32>());
    let h2 = thread::spawn(move || right.iter().sum::<i32>());

    h1.join().unwrap() + h2.join().unwrap()
}
```

## 4. Concept: Scoped threads

### Explanation

With `thread::scope`, you can borrow data from the parent thread without `move` closures. The scope ensures all threads finish before the scope exits.

```rust
use std::thread;

let data = vec![1, 2, 3];
thread::scope(|s| {
    s.spawn(|| {
        println!("{:?}", data); // Borrows, not moves
    });
});
// data is still accessible here
```

### Python comparison

Python's `threading.Thread` always requires passing data explicitly. There is no scoped lifetime guarantee.

### Applying to our project

The `scoped_worker` function spawns a scoped thread that processes each element:

```rust
pub fn scoped_worker(data: Vec<i32>) -> Vec<i32> {
    let mut result = vec![0; data.len()];
    thread::scope(|s| {
        for (i, val) in data.iter().enumerate() {
            s.spawn(|| {
                result[i] = val * 2;
            });
        }
    });
    result
}
```

## 5. Concept: Message passing with mpsc

### Explanation

`mpsc` stands for **multiple producer, single consumer**. It is Rust's channel primitive, similar to Python's `queue.Queue`.

```rust
use std::sync::mpsc;
use std::thread;

let (tx, rx) = mpsc::channel();

thread::spawn(move || {
    tx.send("hello".to_string()).unwrap();
});

let msg = rx.recv().unwrap();
```

`tx` (transmitter) can be cloned for multiple producers. `rx` (receiver) has one end.

### Python comparison

```python
import queue

q = queue.Queue()
t = threading.Thread(target=lambda: q.put("hello"))
t.start()
msg = q.get()
```

### Applying to our project

```rust
pub fn channel_send_receive() -> Vec<String> {
    let (tx, rx) = mpsc::channel();
    let tx2 = tx.clone();

    thread::spawn(move || {
        tx.send("Hello".to_string()).unwrap();
    });

    thread::spawn(move || {
        tx2.send("World".to_string()).unwrap();
    });

    let mut msgs = vec![];
    for _ in 0..2 {
        msgs.push(rx.recv().unwrap());
    }
    msgs
}
```

## 6. Concept: Arc\<Mutex\> — shared mutable state

### Explanation

`Arc<T>` enables shared ownership across threads via atomic reference counting. `Mutex<T>` ensures only one thread accesses the data at a time.

```rust
use std::sync::{Arc, Mutex};

let counter = Arc::new(Mutex::new(0usize));
let c = Arc::clone(&counter);

thread::spawn(move || {
    let mut num = c.lock().unwrap();
    *num += 1;
});
```

### Python comparison

```python
import threading

counter = 0
lock = threading.Lock()

def inc():
    global counter
    with lock:
        counter += 1
```

Without the lock in Python, `counter += 1` suffers a race condition. Rust requires the `Mutex` to mutate through a shared reference — the compiler won't let you forget.

### Applying to our project

```rust
pub fn shared_counter(ops: usize) -> usize {
    let counter = Arc::new(Mutex::new(0usize));
    let mut handles = vec![];

    for _ in 0..8 {
        let c = Arc::clone(&counter);
        handles.push(thread::spawn(move || {
            for _ in 0..ops {
                let mut num = c.lock().unwrap();
                *num += 1;
            }
        }));
    }

    for h in handles { h.join().unwrap(); }
    *counter.lock().unwrap()
}
```

## 7. Concept: RwLock — read-write lock

### Explanation

`RwLock<T>` allows multiple readers or one writer at a time. Use it when reads vastly outnumber writes.

```rust
use std::sync::{Arc, RwLock};

let data = Arc::new(RwLock::new(5usize));
let r = data.read().unwrap();  // Multiple readers allowed
let mut w = data.write().unwrap();  // Exclusive
*w += 1;
```

### Python comparison

Python does not have a stdlib `RwLock`. You would simulate it with `threading.Lock`, but readers block each other unnecessarily.

### Applying to our project

```rust
pub fn rwlock_counter(ops: usize) -> usize {
    let counter = Arc::new(RwLock::new(0usize));
    let mut handles = vec![];

    for _ in 0..8 {
        let c = Arc::clone(&counter);
        handles.push(thread::spawn(move || {
            for _ in 0..ops {
                let mut num = c.write().unwrap();
                *num += 1;
            }
        }));
    }

    for h in handles { h.join().unwrap(); }
    *counter.read().unwrap()
}
```

## 8. Putting It All Together

Now implement each function in `workshop/src/lib.rs` by replacing `todo!()` with real code. Run `cd workshop && cargo test` after each section to track your progress.

**Functions to implement:**

| Function | Concept | Section |
|---|---|---|
| `spawn_and_join()` | Thread spawn + join | 3 |
| `sum_in_parallel()` | Parallel sum with threads | 3 |
| `scoped_worker()` | Scoped threads | 4 |
| `channel_send_receive()` | mpsc message passing | 5 |
| `shared_counter()` | Arc\<Mutex\> | 6 |
| `rwlock_counter()` | RwLock | 7 |

## 9. Exercises

**Easy**: Modify `sum_in_parallel` to use 4 threads instead of 2.

**Medium**: Write a function that uses mpsc to stream results from 4 worker threads back to the main thread.

**Hard**: Implement a thread pool where a fixed set of worker threads process jobs from a shared mpsc channel, and results are sent back through a second channel.

## 10. Summary

| Concept | Rust Primitive | Python Equivalent |
|---|---|---|
| Thread spawning | `std::thread::spawn` | `threading.Thread` |
| Scoped threads | `thread::scope` | No equivalent |
| Message passing | `mpsc::channel` | `queue.Queue` |
| Shared mutable state | `Arc<Mutex<T>>` | `threading.Lock` |
| Read-write lock | `Arc<RwLock<T>>` | No stdlib equivalent |
| Wait for thread | `.join()` | `.join()` |

## Exercises

* **Easy** – modify the existing function to handle an extra edge case.
* **Medium** – extend the project with a new helper function that re‑uses the core logic.

