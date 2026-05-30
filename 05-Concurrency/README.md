# Section 5: Concurrency — Beyond Python's GIL

*Python threads are limited by the GIL. Rust gives you true parallelism with compile-time safety.*

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
5. Finish with **10-RayonChallenge** (data parallelism) and ****

## Key Concepts Covered

| Rust Concept | Python Equivalent | Why It Matters |
|---|---|---|
| `std::thread` | `threading.Thread` | True parallelism (no GIL) |
| `async` / `.await` | `async` / `await` | Async I/O |
| `tokio` | `asyncio` | Async runtime |
| `Mutex<T>` | `threading.Lock` | Mutual exclusion |
| `Arc<T>` | N/A (GC handles this) | Thread-safe ref counting |
| `RwLock<T>` | `threading.RLock` | Read-write lock |
| `mpsc` channel | `queue.Queue` | Message passing |
| `Rayon` | `concurrent.futures` | Data parallelism |
| `Send` / `Sync` | N/A | Thread safety markers |
| `Atomic*` | N/A | Lock-free primitives |
