# Workshop: Advanced Sync — High-Performance Concurrency Primitives

> **Test-driven approach**: This project includes a Cargo project with progressive
> unit tests. Each function in `src/lib.rs` starts as a `todo!()` stub. As you
> follow each section, replace `todo!()` with real code and run `cargo test` to
> watch the pass count grow. Your goal: **all 12 tests pass**.

**Goal**: Implement all functions in `src/lib.rs` to pass all 12 tests.

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
