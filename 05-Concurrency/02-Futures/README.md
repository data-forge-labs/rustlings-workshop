# Async/Await, Tasks, Runtimes — Asynchronous Rust with Tokio

> **Test-driven approach**: This project includes a Cargo project with progressive unit tests. Each function in `workshop/src/lib.rs` starts as a `todo!()` stub. As you follow each section, replace `todo!()` with real code and run `cd workshop && cargo test` to watch the pass count grow. Your goal: **all 5 tests pass**.

## Why Use Async/Await?

**Python pain:** `asyncio` is powerful but easy to misuse — a single `time.sleep(1)` in an async handler blocks the *entire* event loop, and you only discover it when latency spikes in production. There's no compile-time distinction between async and sync functions, and coroutines allocate on the heap every time.

**Rust fix:** Rust's async model is **zero-cost** — a future that is never awaited compiles to *nothing*. The type system prevents blocking calls inside `async fn`:

```rust
pub async fn good_handler() -> String {
    tokio::time::sleep(Duration::from_secs(1)).await;  // yields, doesn't block
    "done".to_string()
}
```

The compiler enforces the boundary — you can't accidentally call `std::thread::sleep` from inside an async function.

## At a Glance

| # | Concept | Rust | Python | Why it matters |
|---|---------|------|--------|----------------|
| 1 | Async Functions | `async fn` | `async def` | Returns a lazy `Future` — zero-cost if never awaited |
| 2 | Awaiting Futures | `future.await` | `await` | Drive a future to completion; yields to the runtime |
| 3 | Task Spawning | `tokio::spawn` | `asyncio.create_task` | Run futures concurrently on the runtime |
| 4 | Async Runtime | `#[tokio::main]` | `asyncio.run()` | Multi-threaded or current-thread executor |
| 5 | Async Sleep | `tokio::time::sleep().await` | `asyncio.sleep` | Non-blocking delay that yields |
| 6 | Task Joining | `JoinHandle` → `.await` | `await task` | Collect results from spawned tasks |
| 7 | Runtime Config | `#[tokio::main(flavor = "multi_thread")]` | `asyncio.run` (always single-thread) | Choose scheduler at the macro level |

---

## Table of Contents
1. [Introduction](#1-introduction)
2. [Prerequisites](#2-prerequisites)
3. [Concept: async fn and .await](#3-concept-async-fn-and-await)
4. [Concept: tokio::spawn — concurrent tasks](#4-concept-tokiospawn--concurrent-tasks)
5. [Concept: Runtime — blocking on async code](#5-concept-runtime--blocking-on-async-code)
6. [Concept: tokio::time — async delays](#6-concept-tokiotime--async-delays)
7. [Putting It All Together](#7-putting-it-all-together)
8. [Exercises](#8-exercises)
9. [Summary](#9-summary)

## 1. Introduction

In Python, `asyncio` provides cooperative multitasking for I/O-bound work. Rust's approach is similar but even more lightweight — futures are zero-cost and the compiler checks that you don't accidentally block the runtime.

You will learn to write async functions, spawn concurrent tasks with Tokio, and build a runtime to drive async code to completion.

**Data engineering context**: Async is essential for high-throughput data pipelines that make many network calls (HTTP APIs, database queries, S3 downloads) without blocking threads.

This project uses **Tokio**, the leading async runtime in Rust. Think of it as `asyncio` but orders of magnitude faster and with no GIL.

## 2. Prerequisites

- Threads basics from [01-Threads](../01-Threads/README.md)
- Basic closures and ownership

## 3. Concept: async fn and .await

### Explanation

In Python, `async def` defines a coroutine. In Rust, `async fn` defines a function that returns a `Future` — a lazy value that does nothing until awaited.

```rust
pub async fn async_hello() -> String {
    "Hello from async!".to_string()
}
```

Calling `async_hello()` returns a `Future` but does not execute. You must `.await` it to drive progress:

```rust
let result = async_hello().await;  // Executes the future
```

### Python comparison

```python
async def async_hello():
    return "Hello from async!"

result = await async_hello()
```

The key difference: Rust futures are **lazy** and **zero-cost**. A future that does not use `.await` compiles to nothing. Python coroutines always allocate.

### Applying to our project

```rust
pub async fn async_hello() -> String {
    "Hello from async!".to_string()
}

pub async fn process_chain() -> String {
    let first = async_hello().await;
    format!("{first} Processed")
}
```

## 4. Concept: tokio::spawn — concurrent tasks

### Explanation

`tokio::spawn` runs a future concurrently on the Tokio runtime, similar to how `thread::spawn` runs a closure on a new thread. Unlike threads, spawned tasks are lightweight and multiplexed onto a thread pool.

```rust
use tokio;

let handle1 = tokio::spawn(async {
    "Task 1".to_string()
});

let handle2 = tokio::spawn(async {
    "Task 2".to_string()
});

let r1 = handle1.await.unwrap();
let r2 = handle2.await.unwrap();
```

### Python comparison

```python
import asyncio

async def task1(): return "Task 1"
async def task2(): return "Task 2"

async def main():
    t1 = asyncio.create_task(task1())
    t2 = asyncio.create_task(task2())
    r1 = await t1
    r2 = await t2
```

`tokio::spawn` is equivalent to `asyncio.create_task`.

### Applying to our project

```rust
pub async fn run_concurrent() -> Vec<String> {
    let h1 = tokio::spawn(async {
        "Hello".to_string()
    });

    let h2 = tokio::spawn(async {
        "World".to_string()
    });

    vec![h1.await.unwrap(), h2.await.unwrap()]
}
```

## 5. Concept: Runtime — blocking on async code

### Explanation

Async code needs a **runtime** to execute. In tests, `#[tokio::test]` provides one. In `main`, you use `#[tokio::main]`. To call async from sync code, use `tokio::runtime::Runtime::block_on`.

```rust
use tokio::runtime::Runtime;

pub fn block_on_hello() -> String {
    let rt = Runtime::new().unwrap();
    rt.block_on(async {
        "Hello from async!".to_string()
    })
}
```

### Python comparison

```python
# Python — no explicit runtime needed
async def hello():
    return "Hello"

result = asyncio.run(hello())
```

`asyncio.run()` is the Python equivalent of `Runtime::block_on`. Rust gives you more control over the runtime configuration (multi-threaded vs current-thread, etc.).

### Applying to our project

```rust
pub fn block_on_hello() -> String {
    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(async_hello())
}
```

## 6. Concept: tokio::time — async delays

### Explanation

`tokio::time::sleep` is the async equivalent of `std::thread::sleep`. Unlike the blocking version, it yields control back to the runtime so other tasks can make progress while waiting.

```rust
use tokio::time::{sleep, Duration};

pub async fn delayed_greeting(seconds: u64) -> String {
    sleep(Duration::from_secs(seconds)).await;
    "Delayed greeting".to_string()
}
```

### Python comparison

```python
import asyncio

async def delayed_greeting(seconds: int):
    await asyncio.sleep(seconds)
    return "Delayed greeting"
```

Both `tokio::time::sleep` and `asyncio.sleep` are cooperative — they yield control so other tasks run.

### Applying to our project

```rust
pub async fn delayed_greeting(seconds: u64) -> String {
    if seconds > 0 {
        tokio::time::sleep(tokio::time::Duration::from_secs(seconds)).await;
    }
    format!("Delayed {} second(s)", seconds)
}
```

## 7. Putting It All Together

Implement each function in `workshop/src/lib.rs` by replacing `todo!()`:

| Function | Concept | Test Module |
|---|---|---|
| `async_hello()` | async fn | `step_01_async_fn` |
| `process_chain()` | .await chaining | `step_01_async_fn` |
| `run_concurrent()` | tokio::spawn | `step_02_spawn` |
| `block_on_hello()` | Runtime::block_on | `step_03_runtime` |
| `delayed_greeting()` | tokio::time::sleep | `step_04_delay` |

Run `cd workshop && cargo test` after each implementation. Note that async test functions use `#[tokio::test]` instead of `#[test]`.

## 8. Exercises

**Easy**: Write an async function `fetch_all()` that spawns 3 tasks, each returning a different string, and collects them into a `Vec<String>`.

**Medium**: Create a function that uses `tokio::time::timeout` to run `delayed_greeting(10)` with a 2-second timeout, returning a default on timeout.

**Hard**: Build a simple async pipeline: spawn a producer task that sends messages through `tokio::sync::mpsc` and a consumer task that receives and processes them.

## 9. Summary

| Concept | Rust | Python |
|---|---|---|
| Async function | `async fn` | `async def` |
| Await future | `.await` | `await` |
| Spawn task | `tokio::spawn` | `asyncio.create_task` |
| Block on runtime | `Runtime::block_on` | `asyncio.run` |
| Async sleep | `tokio::time::sleep` | `asyncio.sleep` |
| Runtime crate | `tokio` | `asyncio` (stdlib) |
