# Data Parallelism with Rayon — Parallel Iterators and Speedup

> **Test-driven approach**: This project includes a Cargo project with progressive unit tests. Each function in `workshop/src/lib.rs` starts as a `todo!()` stub. As you follow each section, replace `todo!()` with real code and run `cd workshop && cargo test` to watch the pass count grow. Your goal: **all 11 tests pass**.

## Table of Contents
1. [Introduction](#1-introduction)
2. [Prerequisites](#2-prerequisites)
3. [Concept: par_iter — parallel iterators](#3-concept-par_iter--parallel-iterators)
4. [Concept: Parallel filter and frequency](#4-concept-parallel-filter-and-frequency)
5. [Concept: CPU count and speedup](#5-concept-cpu-count-and-speedup)
6. [Putting It All Together](#6-putting-it-all-together)
7. [Exercises](#7-exercises)
8. [Summary](#8-summary)

## 1. Introduction

**Rayon** is a Rust library that makes data parallelism trivial. With a single `.par_iter()` call, sequential code becomes parallel. Under the hood, Rayon manages a thread pool and splits work automatically.

In Python, `concurrent.futures.ProcessPoolExecutor` provides similar functionality but requires explicit task submission and result collection. Rayon's design is cleaner: replace `.iter()` with `.par_iter()`.

**Data engineering context**: When processing large datasets (CSV parsing, log analysis, feature extraction), Rayon lets you parallelize with minimal code changes. It is one of the most popular Rust crates for data engineering.

## 2. Prerequisites

- Iterators from [Rust Iterators](../../03-Collections/12-RustIterators/README.md)
- Threads from [01-Threads](../01-Threads/README.md)

## 3. Concept: par_iter — parallel iterators

### Explanation

Rayon provides `par_iter()`, `par_iter_mut()`, and `into_par_iter()` as drop-in replacements for sequential iterators:

```rust
use rayon::prelude::*;

fn parallel_sum(data: Vec<i32>) -> i32 {
    data.par_iter().sum()
}

fn parallel_increment(data: Vec<i32>) -> Vec<i32> {
    data.par_iter().map(|&x| x + 1).collect()
}
```

### Python comparison

```python
from concurrent.futures import ProcessPoolExecutor

def parallel_increment(data: list[int]) -> list[int]:
    with ProcessPoolExecutor() as ex:
        return list(ex.map(lambda x: x + 1, data))
```

Rayon is simpler: no executor context, no explicit mapping. The parallel overhead is handled automatically.

### Applying to our project

```rust
pub fn parallel_sum(data: Vec<i32>) -> i32 {
    data.par_iter().sum()
}

pub fn parallel_increment(data: Vec<i32>) -> Vec<i32> {
    data.into_par_iter().map(|x| x + 1).collect()
}
```

## 4. Concept: Parallel filter and frequency

### Explanation

Rayon supports all standard iterator adapters in parallel:

```rust
pub fn parallel_filter(data: Vec<i32>, threshold: i32) -> Vec<i32> {
    data.into_par_iter()
        .filter(|&x| x > threshold)
        .collect()
}
```

For `parallel_frequency`, use `par_iter()` with a fold that builds a `HashMap`:

```rust
pub fn parallel_frequency<'a>(text: Vec<&'a str>) -> HashMap<&'a str, usize> {
    text.into_par_iter()
        .fold(HashMap::new, |mut map, word| {
            *map.entry(word).or_insert(0) += 1;
            map
        })
        .reduce(HashMap::new, |mut a, b| {
            for (k, v) in b {
                *a.entry(k).or_insert(0) += v;
            }
            a
        })
}
```

### Python comparison

```python
from collections import Counter

def parallel_frequency(text: list[str]) -> dict[str, int]:
    # Python's Counter is single-threaded
    return dict(Counter(text))
```

## 5. Concept: CPU count and speedup

### Explanation

Rayon uses `rayon::current_num_threads()` to report the thread pool size, typically equal to the number of CPU cores. `num_cpus` provides `num_cpus::get()` for the same value.

```rust
pub fn cpu_count() -> usize {
    num_cpus::get()
}
```

`compute_speedup` estimates how much faster parallel execution is compared to sequential execution, modeled by Amdahl's law: the speedup depends on the parallelizable fraction of the workload. Larger datasets yield better speedup because the parallel fraction dominates.

```rust
pub fn compute_speedup(data_size: usize) -> f64 {
    if data_size == 0 { return 1.0; }
    let parallel_fraction = 0.95;  // 95% parallelizable
    let num_cores = cpu_count() as f64;
    let p = parallel_fraction;
    1.0 / ((1.0 - p) + p / num_cores)
}
```

### Python comparison

```python
import os

def cpu_count() -> int:
    return os.cpu_count() or 1
```

## 6. Putting It All Together

Implement each function in `workshop/src/lib.rs`:

| Function | Concept | Test Module | Tests |
|---|---|---|---|
| `parallel_sum()` | par_iter sum | `step_01_par_iter` | 2 |
| `parallel_increment()` | par_iter map/collect | `step_01_par_iter` | 2 |
| `parallel_filter()` | par_iter filter/collect | `step_02_par_filter` | 3 |
| `parallel_frequency()` | par_iter fold/reduce | `step_03_parallel_workload` | 2 |
| `cpu_count()` | Number of cores | `step_03_parallel_workload` | 1 |
| `compute_speedup()` | Amdahl's law speedup | `step_03_parallel_workload` | 2 |

Add `rayon` and `num_cpus` to your `Cargo.toml` if not already present:

```toml
[dependencies]
rayon = "1.5.3"
num_cpus = "1.13.1"
```

## 7. Exercises

**Easy**: Change `parallel_increment` to multiply each element by 3 instead of incrementing.

**Medium**: Implement `parallel_flat_map` that splits each word into characters and collects them into a flat `Vec<char>`.

**Hard**: Benchmark `parallel_sum` against a sequential `sum()` using `std::time::Instant` with a large dataset (10M elements). Report the speedup ratio.

## 8. Summary

| Concept | Rust (Rayon) | Python Equivalent |
|---|---|---|
| Parallel iterate | `par_iter()` | `executor.map()` |
| Parallel sum | `par_iter().sum()` | `sum()` (sequential only) |
| Parallel filter | `par_iter().filter()` | `filter()` (sequential) |
| Parallel frequency | `par_iter().fold().reduce()` | `Counter()` (sequential) |
| CPU count | `num_cpus::get()` | `os.cpu_count()` |
| Speedup model | Amdahl's law | No standard equivalent |
