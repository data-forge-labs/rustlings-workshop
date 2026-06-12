# Workshop: Send + Sync

**Goal**: Implement all functions in `src/lib.rs` to pass all 10 tests.

## Functions to Implement

### `verify_send`
- **Signature**: `pub fn verify_send<T: Send>(val: T) -> T`
- **Task**: Return the value unchanged, with a `Send` bound on the generic type.
- **Tests**: test_verify_send_with_integer, test_verify_send_with_string

### `verify_sync`
- **Signature**: `pub fn verify_sync<T: Sync>(val: T) -> T`
- **Task**: Return the value unchanged, with a `Sync` bound on the generic type.
- **Tests**: test_verify_sync_with_integer, test_verify_sync_with_mutex

### `verify_send_sync`
- **Signature**: `pub fn verify_send_sync<T: Send + Sync>(val: T) -> T`
- **Task**: Return the value unchanged, with both `Send` and `Sync` bounds.
- **Tests**: (Used in the wrapper test module)

### `create_thread_safe_wrapper`
- **Signature**: `pub fn create_thread_safe_wrapper(val: i32) -> Wrapper`
- **Task**: Create a `Wrapper` with the given value (Wrapper is `unsafe impl Send + Sync`).
- **Tests**: test_create_thread_safe_wrapper

### `demonstrate_mutex_send_sync`
- **Signature**: `pub fn demonstrate_mutex_send_sync() -> bool`
- **Task**: Return `true` to show `Arc<Mutex<i32>>` is Send + Sync.
- **Tests**: test_mutex_send_sync

## Structs

### `Wrapper(pub i32)`
- Explicitly implements `Send` and `Sync` via `unsafe impl`.

## Test Modules

| Module | Tests | What It Tests |
|--------|-------|---------------|
| step_01_send_trait | 4 | Send trait bound on generic functions |
| step_02_sync_trait | 2 | Sync trait bound on generic functions |
| step_03_unsafe_impl | 4 | Unsafe Send/Sync impl and Mutex verification |

## How to Run Tests
```bash
cargo test
```

## Exercises

* **Easy** – modify the existing function to handle an extra edge case.
* **Medium** – extend the project with a new helper function that re‑uses the core logic.

