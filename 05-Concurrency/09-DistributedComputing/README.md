# Rust for Distributed Systems — GC Overhead, Compiled vs Interpreted

> **Test-driven approach**: This project includes a Cargo project with progressive unit tests. Each function in `workshop/src/lib.rs` starts as a `todo!()` stub. As you follow each section, replace `todo!()` with real code and run `cd workshop && cargo test` to watch the pass count grow. Your goal: **all 14 tests pass**.

## Why Skip the Garbage Collector?

**Python pain:** The GC can stop execution at *any* time, causing unpredictable latency spikes in distributed pipelines:

```
Python (GC):  ▁▁▁▁▁▄▁▁▁▁▇▁▁▁▁▁▅▁▁▁▁▁▇▁▁▁▁▁▄▁▁▁▁
              ^^^    ^    ^^^    ^    ^    ^^^     GC pauses
Rust (no GC): ▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁
              flat, predictable latency
```

**Rust fix:** Ownership + `Drop` frees memory at compile-time-known points — no stop-the-world pauses, no tracing collector. Iterator chains compile to the same assembly as hand-written loops, so the abstractions have zero runtime cost:

```rust
pub fn measure_allocation_overhead(count: usize) -> usize {
    for _ in 0..count {
        let mut v: Vec<u64> = Vec::with_capacity(1000);
        for j in 0..1000 { v.push(j as u64); }
        // v dropped here — deterministic, no GC pause
    }
    count
}
```

## At a Glance

| # | Concept | Rust | Python | Why it matters |
|---|---------|------|--------|----------------|
| 1 | No GC Pauses | ownership + `Drop` | GC stop-the-world | Predictable tail latency |
| 2 | Deterministic Drop | RAII | refcount + tracing GC | Memory freed at scope exit |
| 3 | Zero-Cost Abstractions | iterator compilation | generator objects | No overhead for high-level patterns |
| 4 | Compute Throughput | native compilation | interpreter overhead | Faster CPU-bound processing |
| 5 | Cache Efficiency | tight loops, no bounds checks | bytecode dispatch | Better cache utilization |
| 6 | Simulated GC Pause | busy-work loop | `gc.collect()` | Model the impact of GC |

---

## Table of Contents
1. [Introduction](#1-introduction)
2. [Prerequisites](#2-prerequisites)
3. [Concept: Allocation overhead — no GC pauses](#3-concept-allocation-overhead--no-gc-pauses)
4. [Concept: Compute-intensive workloads — zero-cost abstractions](#4-concept-compute-intensive-workloads--zero-cost-abstractions)
5. [Concept: Simulating GC pauses](#5-concept-simulating-gc-pauses)
6. [Concept: Throughput comparison — compiled vs interpreted](#6-concept-throughput-comparison--compiled-vs-interpreted)
7. [Putting It All Together](#7-putting-it-all-together)
8. [Exercises](#8-exercises)
9. [Summary](#9-summary)

## 1. Introduction

In Python, the Garbage Collector (GC) periodically pauses execution to reclaim memory. These **GC pauses** can cause latency spikes in distributed systems. Rust has no GC — memory is reclaimed deterministically at compile time via ownership and lifetimes.

This project benchmarks the differences that matter for distributed data systems: allocation overhead, compute throughput, and the impact of GC pauses.

**Data engineering context**: Distributed data pipelines (Kafka consumers, stream processors, real-time aggregators) must maintain low and predictable latency. GC pauses in Python/JVM can cause tail latency spikes. Rust's no-GC model provides deterministic performance.

## 2. Prerequisites

- Basic Rust syntax and ownership
- Understanding of `Vec` and iterators

## 3. Concept: Allocation overhead — no GC pauses

### Explanation

`measure_allocation_overhead` allocates and drops `count` Vecs to simulate heap activity. In Rust, each allocation is freed immediately when the `Vec` goes out of scope. In Python, the GC would trace live objects before freeing unreachable memory.

```rust
pub fn measure_allocation_overhead(count: usize) -> usize {
    for _ in 0..count {
        let mut v: Vec<u64> = Vec::with_capacity(1000);
        for j in 0..1000 {
            v.push(j as u64);
        }
    }
    count
}
```

### Python comparison

```python
def measure_allocation_overhead(count: int) -> int:
    import gc
    for _ in range(count):
        v = [j for j in range(1000)]
        # In CPython: reference counting frees immediately for non-cyclic
        # But cycles require the GC to run
    return count
```

Python's reference counting frees non-cyclic objects immediately, but cyclic garbage requires a full GC cycle. Rust frees everything at `drop()` — no tracing, no pauses.

## 4. Concept: Compute-intensive workloads — zero-cost abstractions

### Explanation

Rust's **zero-cost abstractions** mean that high-level patterns (iterators, closures) compile to the same machine code as hand-written loops. The `compute_intensive` function calculates Fibonacci numbers iteratively:

```rust
pub fn compute_intensive(iterations: usize) -> u64 {
    if iterations == 0 { return 0; }
    let mut a = 0u64;
    let mut b = 1u64;
    for _ in 1..iterations {
        let next = a.saturating_add(b);
        a = b;
        b = next;
    }
    b
}
```

`zero_cost_abstraction_demo` uses a functional chain that compiles to optimized assembly:

```rust
pub fn zero_cost_abstraction_demo(values: Vec<i32>) -> i32 {
    values.iter().filter(|&&x| x % 2 == 0).sum()
}
```

### Python comparison

```python
def compute_intensive(iterations: int) -> int:
    a, b = 0, 1
    for _ in range(1, iterations):
        a, b = b, a + b
    return b

def zero_cost_abstraction_demo(values: list[int]) -> int:
    return sum(x for x in values if x % 2 == 0)
```

In Python, the generator expression allocates generator objects and frames. In Rust, the iterator chain compiles to a tight loop with no overhead — the abstractions truly cost nothing.

## 5. Concept: Simulating GC pauses

### Explanation

`simulate_gc_pause` processes work items and introduces a simulation of GC pauses every 3 items. In a real GC'd language, these pauses happen unpredictably — this function models the effect:

```rust
pub fn simulate_gc_pause(work_items: Vec<u64>) -> u64 {
    let mut total = 0u64;
    for (i, item) in work_items.iter().enumerate() {
        total += item;
        if (i + 1) % 3 == 0 {
            // Simulate a GC "pause" with busy-work
            let mut pause: u64 = 0;
            for _ in 0..1000 {
                pause = pause.wrapping_add(1);
            }
        }
    }
    total
}
```

### Python comparison

```python
import gc

def simulate_gc_pause(work_items: list[int]) -> int:
    total = 0
    for i, item in enumerate(work_items):
        total += item
        if (i + 1) % 3 == 0:
            # Python GC might pause at ANY point, not just here
            gc.collect()  # Force a GC cycle
    return total
```

## 6. Concept: Throughput comparison — compiled vs interpreted

### Explanation

`compare_throughput` computes a simulated speedup ratio for compiled code (Rust) over interpreted code (Python). The ratio increases with data size because compiled code benefits from better cache utilization, inlining, and no interpreter overhead:

```rust
pub fn compare_throughput(data_sizes: Vec<usize>) -> Vec<f64> {
    data_sizes
        .iter()
        .map(|&size| {
            let base_ratio = 1.5;
            let size_factor = (size as f64).sqrt() * 0.1;
            base_ratio + size_factor
        })
        .collect()
}
```

Larger data sizes yield higher speedup ratios because:
1. **Cache efficiency** — Rust's tight loops fit in CPU cache better
2. **No interpreter overhead** — No bytecode dispatch per iteration
3. **No bounds checking** — Iterators eliminate redundant checks
4. **No GC pauses** — No stop-the-world events

### Python comparison

```python
def compare_throughput(data_sizes: list[int]) -> list[float]:
    return [1.5 + (size ** 0.5) * 0.1 for size in data_sizes]
```

## 7. Putting It All Together

Implement each function in `workshop/src/lib.rs`:

| Function | Concept | Test Module | Tests |
|---|---|---|---|
| `measure_allocation_overhead()` | Stack vs heap, deterministic drop | `step_01_memory_efficiency` | 2 |
| `simulate_gc_pause()` | Modeling GC interruption | `step_01_memory_efficiency` | 3 |
| `compute_intensive()` | CPU-bound computation | `step_02_compute_performance` | 3 |
| `compare_throughput()` | Compiled vs interpreted ratio | `step_02_compute_performance` | 2 |
| `zero_cost_abstraction_demo()` | Iterator chain performance | `step_03_zero_cost` | 4 |

## 8. Exercises

**Easy**: Extend `compare_throughput` to cap the ratio at 10x to model Amdahl's law limits.

**Medium**: Write a benchmark function that times `compute_intensive` vs a similar Python snippet using `std::time::Instant` and compares results.

**Hard**: Build a small distributed message processor that receives batches, processes them through `compute_intensive`, and measures p99 latency. Compare expected behavior with a simulated GC pause.

## 9. Summary

| Concept | Rust | Python |
|---|---|---|
| Memory management | Ownership (compile-time) | GC (stop-the-world pauses) |
| Allocation cost | Deterministic drop | Reference counting + GC |
| Iterator overhead | Zero-cost (compiles to loop) | Object allocation for generators |
| Compute throughput | Native compiled speed | Interpreter overhead |
| Tail latency | Predictable | GC pauses cause spikes |
