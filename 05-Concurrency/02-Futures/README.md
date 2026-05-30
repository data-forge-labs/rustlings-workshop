# Async/Await, Tasks, Runtimes — Asynchronous Rust with Tokio

> **Test-driven approach**: This project includes a Cargo project with progressive unit tests. Each function in `workshop/src/lib.rs` starts as a `todo!()` stub. As you follow each section, replace `todo!()` with real code and run `cd workshop && cargo test` to watch the pass count grow. Your goal: **all 5 tests pass**.

## Why This Project?

### The Problem

Python's `asyncio` is powerful but easy to misuse. Accidentally using `time.sleep()` instead of `asyncio.sleep()` blocks the entire event loop. There is no compile-time distinction between async and sync functions:

```python
import asyncio
import time

async def bad_handler():
    time.sleep(1)  # Oops — blocks the entire event loop for 1 second!
    return "done"

# Only discovered when latency spikes in production
```

```
Python asyncio:     Bad handler calls time.sleep(1)
┌─────────────────────────────────────────────────────┐
│ Event Loop:  | Task1 | ████████ sleep ████████ | ...│
│              The ENTIRE loop is blocked             │
└─────────────────────────────────────────────────────┘
```

Python also allocates coroutine objects on the heap every time you create an async task, adding GC pressure.

### The Rust Solution

Rust's async model is **zero-cost** — futures that are not awaited compile to nothing. The type system prevents blocking calls inside async functions:

```rust
use tokio::time::{sleep, Duration};

pub async fn good_handler() -> String {
    sleep(Duration::from_secs(1)).await;  // Yields — doesn't block
    "done".to_string()
}

pub fn sync_blocking() -> String {
    std::thread::sleep(Duration::from_secs(1));  // Only in sync code
    "done".to_string()
}
```

The compiler won't let you call `std::thread::sleep` inside an `async fn` without explicit wrapping — the distinction is enforced.

## What You'll Learn

| # | Concept | Rust Type / Module | Python Equivalent | Purpose |
|---|---------|--------------------|------------------|---------|
| 1 | Async Functions | `async fn` | `async def` | Define async operations |
| 2 | Awaiting Futures | `.await` | `await` | Drive a future to completion |
| 3 | Task Spawning | `tokio::spawn` | `asyncio.create_task` | Run futures concurrently |
| 4 | Async Runtime | `tokio::runtime::Runtime` | `asyncio.run()` | Drive async code to completion |
| 5 | Async Sleep | `tokio::time::sleep` | `asyncio.sleep` | Non-blocking delay |
| 6 | Task Joining | `.await` on `JoinHandle` | `await` on task | Collect results from spawned tasks |
| 7 | Runtime Configuration | `#[tokio::main]` | `asyncio.run()` | Multi-threaded vs current-thread |

## Concepts at a Glance

- **Async Functions (`async fn`)**: Returns a `Future` — a lazy computation that does nothing until awaited. Python's `async def` returns a coroutine that is eagerly allocated. Rust futures are zero-cost: an unused future compiles to nothing.
- **Awaiting Futures (`.await`)**: Drives the future to completion, yielding control back to the runtime if the future is not ready. Same syntax and semantics as Python's `await` but Rust's `.await` is a method call, not a keyword.
- **Task Spawning (`tokio::spawn`)**: Spawns a future to run concurrently on the Tokio runtime. Equivalent to `asyncio.create_task`. Spawned tasks are multiplexed onto a thread pool — lightweight, no per-task thread overhead.
- **Async Runtime (`tokio::runtime::Runtime`)**: The executor that drives futures to completion. `Runtime::block_on` is like `asyncio.run()`, but Rust gives you finer control (current-thread vs multi-thread scheduler).
- **Async Sleep (`tokio::time::sleep`)**: Non-blocking delay that yields to the runtime. Like `asyncio.sleep()`. Using `std::thread::sleep` instead would block the entire thread, preventing other tasks from running.
- **Task Joining**: Awaiting a `JoinHandle` returns the task's result. Same pattern as Python's `await task` but with typed error handling via `Result`.
- **Runtime Configuration**: `#[tokio::main]` macro sets up a multi-threaded runtime by default. Python's `asyncio.run()` is always single-threaded.

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
