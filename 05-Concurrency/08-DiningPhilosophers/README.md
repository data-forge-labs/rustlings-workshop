# Deadlock Prevention — Ordered Lock Acquisition

> **Test-driven approach**: This project includes a Cargo project with progressive unit tests. Each function in `workshop/src/lib.rs` starts as a `todo!()` stub. As you follow each section, replace `todo!()` with real code and run `cd workshop && cargo test` to watch the pass count grow. Your goal: **all 7 tests pass**.

## Why Order Lock Acquisition?

Ownership note: In Rust, values like `String` and `Vec` live on the heap, while primitive values (e.g., `i32`, `bool`) live on the stack. Ownership rules govern when heap data is cleaned up.


---
---

## Table of Contents
1. [Introduction](#1-introduction)
2. [Prerequisites](#2-prerequisites)
3. [Concept: The dining philosophers problem](#3-concept-the-dining-philosophers-problem)
4. [Concept: Deadlock conditions](#4-concept-deadlock-conditions)
5. [Concept: try_lock — breaking hold-and-wait](#5-concept-try_lock--breaking-hold-and-wait)
6. [Concept: Ordered lock acquisition — breaking circular wait](#6-concept-ordered-lock-acquisition--breaking-circular-wait)
7. [Putting It All Together](#7-putting-it-all-together)
8. [Exercises](#8-exercises)
9. [Summary](#9-summary)

## 1. Introduction

The Dining Philosophers problem is a classic concurrency example that demonstrates **deadlock** — a state where multiple threads are permanently blocked, each waiting for resources held by another. This project implements the problem and two deadlock prevention strategies.

**Data engineering context**: Deadlock can occur when multiple threads compete for resources (file handles, database connections, locks on partitions). Understanding prevention strategies is essential for building robust concurrent data pipelines.

In Python, deadlocks happen the same way with `threading.Lock`. The prevention techniques are language-independent, but Rust's type system makes you think about ownership of each resource explicitly.

## 2. Prerequisites

- `Mutex` and `Arc` from [03-DataRace](../../03-DataRace/README.md)
- Thread spawning from [01-Threads](../../01-Threads/README.md)

## 3. Concept: The dining philosophers problem

### Explanation

Five philosophers sit at a round table with five forks. Each philosopher needs **two forks** to eat. They alternate between thinking and eating. If all philosophers pick up their left fork simultaneously, they all wait forever for the right fork — deadlock.

```rust
pub struct Fork {
    pub id: u32,
    pub state: ForkState,
}

pub struct Philosopher {
    pub id: u32,
    pub name: String,
    pub left_hand: Arc<Mutex<Fork>>,
    pub right_hand: Arc<Mutex<Fork>>,
}
```

### Python comparison

```python
import threading

class Fork:
    def __init__(self, id):
        self.id = id
        self.lock = threading.Lock()

class Philosopher(threading.Thread):
    def __init__(self, id, name, left_fork, right_fork):
        super().__init__()
        self.id = id
        self.name = name
        self.left_fork = left_fork
        self.right_fork = right_fork
```

## 4. Concept: Deadlock conditions

### Explanation

Four conditions are necessary for deadlock:

1. **Mutual exclusion** — resources are non-shareable (a fork can be held by only one philosopher)
2. **Hold and wait** — a thread holds at least one resource while waiting for another
3. **No preemption** — resources cannot be forcibly taken from a thread
4. **Circular wait** — a cycle of threads, each holding a resource the next one needs

Breaking any one condition prevents deadlock.

## 5. Concept: try_lock — breaking hold-and-wait

### Explanation

`Mutex::try_lock()` returns immediately with either `Ok(guard)` or `Err(TryLockError)` instead of blocking. This breaks **hold and wait**: a philosopher either gets both forks or neither.

```rust
pub fn try_lock_both(left: &Arc<Mutex<Fork>>, right: &Arc<Mutex<Fork>>) -> bool {
    let _left_guard = match left.try_lock() {
        Ok(g) => g,
        Err(_) => return false,
    };
    match right.try_lock() {
        Ok(_) => true,
        Err(_) => false,
    }
}
```

### Python comparison

```python
def try_lock_both(left, right):
    if left.acquire(blocking=False):
        if right.acquire(blocking=False):
            return True
        left.release()
    return False
```

## 6. Concept: Ordered lock acquisition — breaking circular wait

### Explanation

**Ordered lock acquisition** breaks **circular wait** by enforcing a global order for acquiring locks. Philosophers with even IDs pick up the left fork first; odd IDs pick up the right fork first.

```rust
pub fn lock_ordered(id: u32, left: &Arc<Mutex<Fork>>, right: &Arc<Mutex<Fork>>) -> bool {
    if id % 2 == 0 {
        let _first = left.lock().unwrap();
        let _second = right.lock().unwrap();
    } else {
        let _first = right.lock().unwrap();
        let _second = left.lock().unwrap();
    }
    true
}
```

### Python comparison

```python
def lock_ordered(philosopher_id, left_fork, right_fork):
    if philosopher_id % 2 == 0:
        left_fork.acquire()
        right_fork.acquire()
    else:
        right_fork.acquire()
        left_fork.acquire()
```

This prevents a circular wait because the order of acquisition breaks the cycle: the last philosopher picks up the right fork first (same as the first philosopher), eliminating the circular dependency.

### Diagram

```
Standard (deadlock):
  P0: left(F1) -> right(F2)
  P1: left(F2) -> right(F3)
  P2: left(F3) -> right(F4)
  P3: left(F4) -> right(F5)
  P4: left(F5) -> right(F1)  <-- Cycle!

Ordered (no deadlock):
  P0: left(F1) -> right(F2)  (even: left first)
  P1: right(F2) -> left(F1)  (odd: right first)
  P2: left(F3) -> right(F4)  (even: left first)
  P3: right(F4) -> left(F3)  (odd: right first)
  P4: right(F1) -> left(F5)  (odd: right first, breaks the cycle)
```

## 7. Putting It All Together

### Functions to implement

| Function | Concept | Test Module | Tests |
|---|---|---|---|
| `create_forks()` | Create N forks as Arc\<Mutex\<Fork\>\> | `step_02_philosopher` | 1 |
| `create_philosopher()` | Create a Philosopher struct | `step_02_philosopher` | 1 |
| `try_lock_both()` | try_lock to avoid deadlock | `step_03_deadlock_prevention` | 2 |
| `lock_ordered()` | Ordered lock acquisition | `step_03_deadlock_prevention` | 1 |

The `Fork` and `ForkState` types are already defined. The `Philosopher::eat()` and `Philosopher::think()` methods are also provided — they use `get_forks_odd_even` by default.

### Running the simulation

```bash
cd workshop && cargo run
```

This runs the full dining simulation with 5 philosophers eating and thinking in a loop. Press Ctrl-C to stop and see statistics.

## 8. Exercises

**Easy**: Add a 6th philosopher and fork to the simulation.

**Medium**: Implement a third prevention strategy: limit the number of philosophers that can eat simultaneously using a semaphore (use `tokio::sync::Semaphore` or a simple counter with `Mutex`).

**Hard**: Modify the simulation to detect deadlock: if no philosopher has eaten in 5 seconds, print a warning and abort.

## 9. Summary

| Concept | Rust Implementation | Python Equivalent |
|---|---|---|
| Mutual exclusion lock | `Mutex<T>` | `threading.Lock` |
| Non-blocking lock attempt | `try_lock()` | `lock.acquire(blocking=False)` |
| Deadlock prevention 1 | `try_lock_both` (breaks hold-and-wait) | Same logic |
| Deadlock prevention 2 | `lock_ordered` (breaks circular wait) | Same logic |
| Thread synchronization | `Mutex` + `Arc` | `Lock` + no Arc needed |

## Exercises

* **Easy** – modify the existing function to handle an extra edge case.
* **Medium** – extend the project with a new helper function that re‑uses the core logic.

