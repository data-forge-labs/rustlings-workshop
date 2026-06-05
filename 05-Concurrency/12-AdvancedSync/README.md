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
