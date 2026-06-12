# ⚡ Advanced Sync — High-Performance Concurrency Primitives

*Subtitle: when `std::sync` isn't fast enough — `parking_lot`, `crossbeam`, `arc_swap`.*

> **Test-driven approach**: This project includes a Cargo project with progressive
> unit tests. Each function in `src/lib.rs` starts as a `todo!()` stub. As you
> follow each section, replace `todo!()` with real code and run `cargo test` to
> watch the pass count grow. Your goal: **all 12 tests pass**.

---

## Why `parking_lot` and `crossbeam`?

**Python pain:** Python's GIL serialises CPU work. When you want real threads
in a systems language, `std::sync::Mutex` is fine — but the implementation
uses a `pthread_mutex_t` plus a `std::sync::Once` to handle poisoning, which
adds overhead on the hot path. Production code with millions of lock/unlock
pairs a second feels it.

**Rust fix:** `parking_lot` replaces the OS mutex with a futex on Linux and
inline assembly on x86, plus a 30-byte guard. It is **2-3× faster** for
uncontended locks and never poisons. `crossbeam` gives you a true MPMC
channel (vs `std::sync::mpsc` which is SPSC-friendly in design). `arc_swap`
is a lock-free atomic pointer swap — readers never block writers.

```rust
let m = parking_lot::Mutex::new(0);
*m.lock() += 1;  // no Result, no poisoning, ~2-3x faster
```

## At a Glance

| # | Concept | Rust | Python | Why it matters |
|---|---------|------|--------|----------------|
| 1 | `parking_lot::Mutex` | no poisoning, ~2-3× faster | `threading.Lock` | Hot-path performance |
| 2 | `parking_lot::RwLock` | writer-starves-free, smaller | `threading.RLock` | Many-readers workloads |
| 3 | `crossbeam_channel` | true MPMC, bounded + unbounded | `queue.Queue` | Multiple producers + consumers |
| 4 | `arc_swap` | lock-free atomic swap | `contextvars.ContextVar` | Hot-reload config, versioned snapshots |
| 5 | `try_lock` | returns `None` if locked | n/a | Non-blocking read attempt |
| 6 | `Arc<Mutex<T>>` | `parking_lot::Mutex` is `!Sync` over `T` | n/a | Combine with `Arc` for shared state |
| 7 | Reader guard | `lock.read()` returns `RwLockReadGuard` | `RLock` context manager | RAII lock release |
| 8 | Writer guard | `lock.write()` returns `RwLockWriteGuard` | n/a | Exclusive access |

---

> **Test-driven approach**: This project includes a Cargo project with progressive
> unit tests. Each function in `src/lib.rs` starts as a `todo!()` stub. As you
> follow each section, replace `todo!()` with real code and run `cargo test` to
> watch the pass count grow. Your goal: **all 12 tests pass**.

**Goal**: Implement all functions in `src/lib.rs` to pass all 12 tests.


---

## Setup: Create the Project from Scratch

This is a hands-on workshop. You will write the code yourself following the steps below.

### 1. Create the new Cargo project

```bash
cargo new --lib advanced_sync_workshop
cd advanced_sync_workshop
```

### 2. Add the dependencies

Open `Cargo.toml` and replace whatever is there with this:

```toml
[package]
name = "advanced_sync_workshop"
version = "0.1.0"
edition = "2024"

[dependencies]
parking_lot = "0.12"
crossbeam-channel = "0.5"
arc-swap = "1"
triomphe = "0.1"

```

### 3. Copy the test stubs as your starting point

This project follows a **test-driven** approach. Each function in `src/lib.rs` starts as a `todo!()` stub, and progressive tests guide your implementation.

```bash
cp "05-Concurrency/12-AdvancedSync/workshop/src/lib.rs" src/lib.rs
cp "05-Concurrency/12-AdvancedSync/workshop/src/main.rs" src/main.rs
```

### 4. Run the tests to see them fail (this is expected!)

```bash
cargo test
```

You should see all tests fail with the message "not yet implemented". That's the starting point — you are about to make them pass.

### 5. Follow the step-by-step sections below

Each section below corresponds to a step module in the test file. Implement the function(s) described, then run:

```bash
cargo test step_XX_name
```

to watch the pass count grow. The test module names match the section headings.

## Functions to Implement

### Step 1 — `parking_lot::Mutex`

#### `with_mutex`
- **Signature**: `pub fn with_mutex<F: FnOnce(&mut i32) -> R, R>(m: &Mutex<i32>, f: F) -> R`
- **Task**: `let mut g = m.lock(); f(&mut g)`.

#### `try_with_mutex`
- **Signature**: `pub fn try_with_mutex<F: FnOnce(&mut i32) -> R, R>(m: &Mutex<i32>, f: F) -> Option<R>`
- **Task**: `m.try_lock().map(|mut g| f(&mut g))`. Returns `None` if locked.

#### `update_counter`
- **Signature**: `pub fn update_counter(counter: &Mutex<i32>, delta: i32) -> i32`
- **Task**: `let mut g = counter.lock(); *g += delta; *g`.

### Step 2 — `parking_lot::RwLock`

#### `read_under_rwlock`
- **Signature**: `pub fn read_under_rwlock<F: FnOnce(&i32) -> R, R>(lock: &parking_lot::RwLock<i32>, f: F) -> R`
- **Task**: `f(&*lock.read())`.

#### `write_under_rwlock`
- **Signature**: `pub fn write_under_rwlock<F: FnOnce(&mut i32) -> R, R>(lock: &parking_lot::RwLock<i32>, f: F) -> R`
- **Task**: `f(&mut *lock.write())`.

### Step 3 — `crossbeam_channel`

#### `crossbeam_send`
- **Signature**: `pub fn crossbeam_send(tx: &crossbeam_channel::Sender<i32>, value: i32) -> Result<(), &'static str>`
- **Task**: `tx.send(value).map_err(|_| "channel closed")`.

#### `crossbeam_collect`
- **Signature**: `pub fn crossbeam_collect(rx: crossbeam_channel::Receiver<i32>, n: usize) -> Vec<i32>`
- **Task**: `rx.iter().take(n).collect()`.

### Step 4 — `arc_swap`

#### `arc_swap_load`
- **Signature**: `pub fn arc_swap_load(swap: &arc_swap::ArcSwap<String>) -> arc_swap::Guard<Arc<String>>`
- **Task**: `swap.load()`.

#### `arc_swap_store`
- **Signature**: `pub fn arc_swap_store(swap: &arc_swap::ArcSwap<String>, value: String)`
- **Task**: `swap.store(Arc::new(value))`.

## Test Modules

| Module | Tests | What It Tests |
|--------|-------|---------------|
| step_01_parking_lot_mutex | 4 | with/try/update + 10×100 concurrent |
| step_02_rwlock | 3 | read/write + 4 concurrent readers |
| step_03_crossbeam | 2 | send/recv + 3×10 MPMC |
| step_04_arc_swap | 3 | store/load + 4 readers × 100 reads, no lock contention |

## How to Run Tests
```bash
cargo test
```

## Exercises

* **Easy** – modify the existing function to handle an extra edge case.
* **Medium** – extend the project with a new helper function that re‑uses the core logic.

