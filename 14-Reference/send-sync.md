# Send and Sync — Thread Safety Markers

## The Two Traits

```rust
// Safe to transfer ownership to another thread
pub unsafe auto trait Send { }

// Safe to share &T across threads
pub unsafe auto trait Sync { }
```

| Trait | Meaning | Equivalent Question |
|-------|---------|---------------------|
| `Send` | Owned value can cross threads | "Can I give this to another thread?" |
| `Sync` | Reference can be shared across threads | "Can multiple threads look at this?" |

## Auto Traits

`Send` and `Sync` are **auto traits** — the compiler derives them automatically for most types.

A type is:
- **Send** if all its fields are Send
- **Sync** if all its fields are Sync (`T: Sync` ⇔ `&T: Send`)

### Common Types

| Type | `Send` | `Sync` | Reason |
|------|--------|--------|--------|
| `i32`, `f64`, `bool` | ✅ | ✅ | Primitive types |
| `String`, `Vec<T>` | ✅ | ✅ | Owned heap data |
| `&T` | ✅ | ✅ | Shared reference |
| `&mut T` | ✅ | ❌ | Mutable references can't be shared |
| `Rc<T>` | ❌ | ❌ | Non-atomic ref count |
| `Arc<T>` | ✅ | ✅ | Atomic ref count |
| `RefCell<T>` | ✅ | ❌ | Runtime borrow checking, not thread-safe |
| `Mutex<T>` | ✅ | ✅ | Thread-safe interior mutability |
| `RwLock<T>` | ✅ | ✅ | Thread-safe read/write lock |
| `Cell<T>` | ✅ | ❌ | Not Sync (but Send) |
| `AtomicBool`, `AtomicI32` | ✅ | ✅ | Atomic operations |
| `mpsc::Sender<T>` | ✅ | ❌ | Clone to get multiple senders |
| `mpsc::Receiver<T>` | ❌ | ❌ | Single consumer |

## Manual Implementation

`unsafe impl Send` or `unsafe impl Sync` is needed for:
- Raw pointers (`*const T`, `*mut T`) — not Send/Sync by default
- FFI types where the C library guarantees thread safety

```rust
struct MyRc<T>(*mut T);

// Safety: we use atomic operations internally
unsafe impl<T: Send> Send for MyRc<T> {}
unsafe impl<T: Sync> Sync for MyRc<T> {}
```

**Always document why the manual impl is safe.**

## `Rc` vs `Arc`, `RefCell` vs `Mutex`

```
Single-threaded              Multi-threaded
─────────────────             ─────────────────
Rc<T>         ❌ Send/Sync   Arc<T>          ✅ Send/Sync
RefCell<T>    ✅ Send ❌ Sync Mutex<T>        ✅ Send/Sync
Cell<T>       ✅ Send ❌ Sync AtomicI32       ✅ Send/Sync
Box<T>        ✅ Send/Sync   Box<T>           ✅ Send/Sync
```

### Decision tree

```
Need shared ownership?
├── Single thread → Rc<T>
└── Multiple threads → Arc<T>

Need interior mutability?
├── Single thread
│   ├── Copy type → Cell<T>
│   └── Non-copy → RefCell<T>
├── Multiple threads
│   ├── Many readers, few writers → RwLock<T>
│   └── Simple mutual exclusion → Mutex<T>
└── Atomic-safe primitive → AtomicBool/I32/U64
```

## Common Patterns and Pitfalls

### Pattern: Arc<Mutex<T>>

```rust
use std::sync::{Arc, Mutex};

let shared: Arc<Mutex<Vec<i32>>> = Arc::new(Mutex::new(vec![]));
let clone = Arc::clone(&shared);

std::thread::spawn(move || {
    let mut data = clone.lock().unwrap();
    data.push(42);
});

let data = shared.lock().unwrap();
assert_eq!(data[0], 42);
```

### Pitfall: Holding a Mutex Guard Across an `.await`

```rust
// BAD — will deadlock or panic
async fn bad() {
    let data = Arc::new(Mutex::new(0));
    let guard = data.lock().unwrap();
    some_async_fn().await;  // guard held across yield point!
    drop(guard);
}

// GOOD — scope the lock
async fn good(data: &Mutex<i32>) {
    {
        let mut guard = data.lock().unwrap();
        *guard += 1;
    } // guard dropped before .await
    some_async_fn().await;
}
```

### Pitfall: Rc Across a Thread

```rust
use std::rc::Rc;
// let r = Rc::new(5);
// std::thread::spawn(move || { println!("{}", r); });
// ERROR: Rc<i32> cannot be sent between threads safely
```

## Python Comparison

| Python | Rust |
|--------|------|
| Every object can be shared between threads (GIL protects) | Only `Sync` types can be shared |
| `threading.Lock` | `std::sync::Mutex` |
| No compile-time thread safety | Send/Sync enforce thread safety at compile time |
| GIL prevents true parallelism for CPU work | True multi-threaded execution |
| `queue.Queue` is thread-safe | `mpsc::channel` or `crossbeam::channel` |
| No concept of Rc/Arc (all GC'd) | Explicit choice: `Rc` (single-thread) vs `Arc` (multi-thread) |
