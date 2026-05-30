# Workshop: Dining Philosophers

**Goal**: Study the implemented `src/lib.rs` and pass all 7 tests.

## Functions to Study

### `create_forks`
- **Signature**: `pub fn create_forks(count: usize) -> Vec<Arc<Mutex<Fork>>>`
- **Task**: Create `count` forks wrapped in `Arc<Mutex<Fork>>`. Already implemented.

### `create_philosopher`
- **Signature**: `pub fn create_philosopher(id: u32, name: &str, left: Arc<Mutex<Fork>>, right: Arc<Mutex<Fork>>) -> Philosopher`
- **Task**: Create a philosopher with given id, name, and fork references. Already implemented.

### `try_lock_both`
- **Signature**: `pub fn try_lock_both(left: &Arc<Mutex<Fork>>, right: &Arc<Mutex<Fork>>) -> bool`
- **Task**: Try to lock both forks without blocking; return `true` on success. Already implemented.

### `lock_ordered`
- **Signature**: `pub fn lock_ordered(id: u32, left: &Arc<Mutex<Fork>>, right: &Arc<Mutex<Fork>>) -> bool`
- **Task**: Lock forks in a fixed order based on philosopher ID to prevent circular wait. Already implemented.

### Other items
- `Fork` struct with `take()` and `free()` methods
- `Philosopher` struct with `eat()`, `think()`, `get_forks_try_lock()`, `get_forks_odd_even()` methods
- `start_dining()` with Ctrl-C handler

## Test Modules

| Module | Tests | What It Tests |
|--------|-------|---------------|
| step_01_fork | 2 | Fork take/free and panic on double-take |
| step_02_philosopher | 2 | Philosopher and fork creation |
| step_03_deadlock_prevention | 3 | Try-lock and ordered-lock strategies |

## How to Run Tests
```bash
cargo test
```
