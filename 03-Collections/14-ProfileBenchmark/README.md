# 🦀 ProfileBenchmark — Dev vs Release, Cargo Profiles, and Criterion

*Compare Rust's debug and release builds on real collection workloads, learn to configure Cargo profiles, and write criterion benchmarks that survive both modes.*

> **Test-driven approach**: This project includes a Cargo project with progressive unit tests. Each function in `workshop/src/lib.rs` starts as a `todo!()` stub. As you follow each section, replace `todo!()` with real code and run `cd workshop && cargo test` to watch the pass count grow. Your goal: **all 14 tests pass**. After that, run `cargo bench` to see how the same code runs 10-50x faster in release mode.

---

## Why Profile at All?

**Python pain:** Python has *one* runtime. You cannot compile Python to a fast binary; you just `python script.py` and hope. When performance matters, Python engineers reach for PyPy, Cython, Numba, or rewrite the hot loop in C. There is no built-in "production mode" that strips safety checks and turns on optimization.

**Rust fix:** Cargo ships with **two default profiles** — `dev` (fast compile, slow run, full checks) and `release` (slow compile, fast run, minimal checks). You flip between them with `--release`. Production deployments always use release, and you can tune the release profile in `Cargo.toml` (opt-level, LTO, codegen-units, overflow checks, etc.). Combined with the `criterion` crate, you get statistically rigorous, comparable benchmarks in two commands.

```rust
// lib.rs — same code, two very different runtimes
pub fn count_words(text: &str) -> HashMap<String, usize> {
    let mut counts = HashMap::new();
    for word in text.split_whitespace() {
        *counts.entry(word.to_lowercase()).or_insert(0) += 1;
    }
    counts
}
```

```bash
cargo build              # 200 MB binary, full debug info, overflow checks on
cargo build --release    # 8 MB binary, no debug info, full optimization
cargo bench              # criterion measures ns/iter with statistical confidence
```

## At a Glance

| # | Concept | Rust | Python | Why it matters |
|---|---------|------|--------|----------------|
| 1 | Build profiles | `dev` / `release` | (none) | Different compile/run trade-offs |
| 2 | Release flag | `cargo build --release` | N/A | Switch to optimized mode for production |
| 3 | Profile config | `[profile.release]` in `Cargo.toml` | N/A | Tune opt-level, LTO, overflow checks |
| 4 | Optimization level | `opt-level = 0..3` / `s` / `z` | N/A | Speed vs size trade-off |
| 5 | Overflow checks | `overflow-checks = true` | N/A | Catch bugs in release too |
| 6 | LTO | `lto = true` / `"fat"` | N/A | Whole-program optimization (slow link) |
| 7 | Codegen units | `codegen-units = 1..256` | N/A | Compile time vs inlining trade-off |
| 8 | Debug symbols | `debug = true/false/line-tables-only` | N/A | Strip symbols for smaller binaries |
| 9 | Criterion benchmarks | `criterion` crate | `timeit`, `pytest-benchmark` | Statistically rigorous measurements |
| 10 | Benchmark groups | `c.benchmark_group(...)` | Custom timing harnesses | Compare related functions side-by-side |
| 11 | Comparing collections | `Vec` vs `HashMap` vs `BTreeMap` | List vs dict vs `SortedDict` | Pick the right tool for the workload |
| 12 | Dev-vs-release gap | 10-50x speedup common | N/A | Always benchmark in release mode |

---

## Table of Contents

1. [Introduction](#1-introduction)
2. [Prerequisites](#2-prerequisites)
3. [Setup: Create the Project from Scratch](#3-setup-create-the-project-from-scratch)
4. [Test Roadmap — what each test teaches](#4-test-roadmap--what-each-test-teaches)
5. [Concept: Cargo Build Profiles (dev vs release)](#5-concept-cargo-build-profiles-dev-vs-release)
6. [Concept: Configuring `[profile.release]`](#6-concept-configuring-profilerelease)
7. [Concept: Criterion Benchmarks](#7-concept-criterion-benchmarks)
8. [Concept: Comparing Collection Strategies](#8-concept-comparing-collection-strategies)
9. [Putting It All Together](#9-putting-it-all-together)
10. [Running the Benchmarks](#10-running-the-benchmarks)
11. [Complete Code Reference](#11-complete-code-reference)
12. [Exercises](#12-exercises)
13. [Summary](#13-summary)

---

## 1. Introduction

You have written 12 projects in this section. Every `Vec::push`, every `HashMap::insert`, every iterator chain — none of it has been measured. In data engineering, choosing the wrong collection or shipping debug builds to production can mean a 10x performance loss. This project fixes that.

We will build a small **word-counting pipeline** that uses three different collection strategies:

- `Vec<String>` — keep all words, count via linear scan
- `HashMap<String, usize>` — single-pass aggregation
- `BTreeMap<String, usize>` — single-pass aggregation with sorted output

Then we will write **criterion benchmarks** that compare them in both `dev` and `release` modes, configure `Cargo.toml` to push the release profile to its limits, and observe the 10-50x speedup that comes from the compiler optimizing hot loops.

**Python → Rust comparison**: In Python, your only "release" knob is `python -O`, which strips `assert` statements and `__debug__` blocks — a single-digit-percent speedup at best. Most Python performance work is about choosing the right algorithm, not about compilation flags. In Rust, the compilation flags *are* the performance work for the same algorithm.

## 2. Prerequisites

- Completed [01-TicketManagement](../01-TicketManagement/README.md) — comfortable with `Vec` and `HashMap`
- Completed [04-HashMapCount](../04-HashMapCount/README.md) — comfortable with the `entry().or_insert()` pattern
- Familiarity with `cargo test` (covered in [01-Intro](../../01-Foundations/01-Intro/README.md) and the [09-03-Testing](../../09-ObservabilityAndTesting/03-Testing/README.md) deep dive)

## 3. Setup: Create the Project from Scratch

If you want to follow along from a blank slate, here is exactly what to type:

```bash
# 1. Create the cargo project
cargo new --lib profile_benchmark
cd profile_benchmark

# 2. Add criterion as a dev-dependency (the benchmarking crate we will use)
cargo add --dev criterion --features html_reports

# 3. Register the benchmark harness in Cargo.toml
# (cargo will append this when you run `cargo bench` for the first time,
#  but you can add it manually — see the Cargo.toml reference below)
```

For this workshop, the project is pre-built. Open `workshop/` and you will find:

```
workshop/
├── Cargo.toml
├── src/
│   ├── lib.rs       ← All public functions + progressive tests
│   └── main.rs      ← Calls lib functions on a sample text
└── benches/
    └── profile_benchmarks.rs   ← Criterion benchmarks
```

Run the tests to see the starting state:

```bash
cd workshop
cargo test              # All 14 tests fail (todo!() stubs)
cargo bench             # Criterion compiles and runs, but the underlying fns panic
```

Now let's build it up.

## 4. Test Roadmap — what each test teaches

The 14 tests in `workshop/src/lib.rs` are organized in 5 step modules that mirror the 5 functions in `lib.rs`. Each test has a doc comment explaining *what property* it verifies and *why that property matters*. The table below is a quick map:

| # | Test | Function | Property verified | Concept taught |
|---|------|----------|-------------------|----------------|
| 1 | `step_01_normalize::test_lowercase` | `normalize_word` | Mixed-case input folds to lowercase | `to_lowercase`, `chars()` |
| 2 | `step_01_normalize::test_strip_punctuation` | `normalize_word` | Non-alphanumeric chars are dropped | `filter` with closure, `collect::<String>` |
| 3 | `step_02_vec::test_count_vec_basic` | `count_vec` | Repeated words aggregate, output sorted alphabetically | `Vec`, `HashMap::entry().or_insert(0) += 1`, `sort_by` |
| 4 | `step_02_vec::test_count_vec_sorted` | `count_vec` | Sort order holds even with reversed input | `sort_by` with closure |
| 5 | `step_02_vec::test_count_vec_empty` | `count_vec` | Empty input returns empty `Vec` (no panic) | Empty-input contract |
| 6 | `step_03_hashmap::test_count_hashmap_basic` | `count_hashmap` | `entry().or_insert(0) += 1` counts correctly | `HashMap::entry` API |
| 7 | `step_03_hashmap::test_count_hashmap_lowercases` | `count_hashmap` | Normalization happens *before* aggregation | Pipeline order matters |
| 8 | `step_03_hashmap::test_count_hashmap_strips_punct` | `count_hashmap` | Punctuation-stripping is part of the key | Composition of `normalize_word` |
| 9 | `step_04_btreemap::test_count_btreemap_basic` | `count_btreemap` | `BTreeMap` supports the same `entry` pattern | `BTreeMap` as a `HashMap` substitute |
| 10 | `step_04_btreemap::test_count_btreemap_sorted` | `count_btreemap` | Keys iterate in sorted order | `BTreeMap` ordering property |
| 11 | `step_04_btreemap::test_count_btreemap_empty` | `count_btreemap` | Empty input contract for `BTreeMap` | Empty-input contract |
| 12 | `step_05_top_n::test_top_n_basic` | `top_n` | Top-N returns highest counts first | `BinaryHeap<Reverse<(K, V)>>` |
| 13 | `step_05_top_n::test_top_n_more_than_available` | `top_n` | `n > map.len()` returns all items, no panic | Defensive programming |
| 14 | `step_05_top_n::test_top_n_zero` | `top_n` | `n = 0` returns empty `Vec` | Edge case handling |

**How to use this table while coding:** implement the function for step N, then run `cargo test step_NN_name` to see the tests for that step pass. When all 14 are green, run `cargo bench` to compare strategies under the release profile.

**What is intentionally not tested:** the *performance* of these functions. Performance is verified by the criterion benchmarks in `benches/profile_benchmarks.rs`, not by `cargo test`. Unit tests verify correctness; benchmarks verify performance. Separating the two keeps your test suite fast and reliable.

## 5. Concept: Cargo Build Profiles (dev vs release)

### Explanation

Cargo has **two built-in profiles** that control how your code is compiled:

| Profile   | Flag            | Compile time | Run time | Debug info | Overflow checks | Use case |
|-----------|-----------------|--------------|----------|------------|-----------------|----------|
| `dev`     | (default)       | Fast         | Slow     | Full       | On              | Inner dev loop |
| `release` | `--release`     | Slow         | Fast     | None       | Off (default)   | Production    |

The two profiles reflect two different goals:

- **`dev`** — get me back to the editor as fast as possible. Compile is incremental, debug symbols are present, and overflow checks catch `u8 = 255 + 1` panics. Run time is 10-50x slower than release.
- **`release`** — get me a binary that ships. LLVM spends minutes (or hours on large codebases) optimizing every hot loop. Overflow checks are off, debug info is stripped, and the resulting binary is much smaller and faster.

```bash
# Default: dev mode
cargo build            # → target/debug/profile_benchmark
cargo test             # uses dev mode

# Production: release mode
cargo build --release  # → target/release/profile_benchmark
cargo run --release    # runs the release binary
cargo bench            # criterion always uses release!
```

### Where do the binaries go?

```
target/
├── debug/                  ← dev profile
│   └── profile_benchmark   ← 200 MB, slow, debug info
└── release/                ← release profile
    └── profile_benchmark   ← 8 MB, fast, no debug info
```

### Python comparison

```python
# Python: one runtime, no real "release mode"
python script.py           # default
python -O script.py        # strips assert and __debug__ — ~5% faster
```

Python's `python -O` is the closest equivalent to a release flag, but the speedup is tiny compared to Rust's. Most Python performance comes from algorithmic choices (use `set` not `list`, use NumPy not loops), not from a build flag.

### What changes between dev and release?

The compiler applies dozens of optimizations in release mode:

| Optimization | Dev | Release | What it does |
|--------------|-----|---------|--------------|
| Loop unrolling | Off | On | Reduces loop overhead |
| Inlining | Conservative | Aggressive | Replaces function calls with body |
| Dead code elimination | Limited | Full | Removes unused branches |
| Constant folding | Limited | Full | Computes `2 + 3` at compile time |
| Vectorization | Off | On (SSE/AVX) | Uses CPU SIMD instructions |
| Bounds check elimination | Off | On (where provable) | Skips `vec[i]` bounds checks |
| Overflow checks | **On** | **Off** | Panics in dev, wraps in release |

That last row is critical — see the [BasicCalculator](../../01-Foundations/03-BasicCalculator/README.md) project for the full story on integer overflow. In short: **`u8 = 255; u8 + 1` panics in dev but wraps to 0 in release**. This is the canonical "works on my machine" bug for new Rustaceans.

### Applying to Our Project

Our word-counting code is pure collection manipulation — no `unsafe`, no FFI, no panics. It is a perfect candidate for benchmarking the dev-vs-release gap. After we implement `count_words`, you will see the same function take 500 ns/iter in dev and 50 ns/iter in release.

## 6. Concept: Configuring `[profile.release]`

### Explanation

The default release profile is conservative. You can override its knobs in `Cargo.toml`:

```toml
[profile.release]
opt-level = 3          # default
lto = false            # default
codegen-units = 16     # default
debug = false          # default
overflow-checks = false # default
strip = false          # default (since Rust 1.59)
```

Let's walk through each one.

### `opt-level` — How aggressively to optimize

| Value      | Meaning |
|------------|---------|
| `0`        | No optimization (same as dev) — useful for profiling |
| `1`        | Basic optimizations — fast compile, decent speed |
| `2`        | (legacy) |
| `3`        | **Default** — full speed optimization |
| `"s"`      | Optimize for size — useful for embedded |
| `"z"`      | Aggressively minimize size — drops unwind tables |

```toml
[profile.release]
opt-level = 3   # fastest binary; can also try "s" for size-constrained targets
```

### `lto` — Link-Time Optimization

By default, each *crate* is optimized in isolation. `lto` lets LLVM see the whole program and inline across crate boundaries.

```toml
[profile.release]
lto = "thin"   # fast LTO, good speedup with minimal compile-time cost
lto = "fat"    # full LTO, best speedup, but link time can be 5-10x longer
lto = false    # default
```

`"thin"` is the sweet spot for most projects. `"fat"` is only worth it for the final production build of a long-running service.

### `codegen-units` — How many parallel compilation units

```toml
[profile.release]
codegen-units = 16   # default — fast compile, less inlining
codegen-units = 1    # slow compile, better optimization (one whole-program unit)
```

Lower numbers = more cross-module inlining = faster runtime, slower link. For data pipelines that run for hours, `codegen-units = 1` is often worth the wait.

### `overflow-checks` — Keep the dev panics in release too

```toml
[profile.release]
overflow-checks = true   # panic on integer overflow in release
```

The [BasicCalculator project](../../01-Foundations/03-BasicCalculator/README.md) recommends this for safety-critical code. The cost is ~1-3% runtime, but you get the same `u8 = 255 + 1` panic that catches bugs in dev.

### `debug` — How much debug info to keep

```toml
[profile.release]
debug = false          # default — no debug info
debug = true           # full debug info (large binary, but you can use a debugger)
debug = "line-tables-only"  # function names + line numbers only — small overhead
debug = 1              # same as true
debug = 2              # same as true
```

For production: `false` or `"line-tables-only"` (lets you get a stack trace on panic without bloating the binary).

### `strip` — Remove symbols from the final binary

```toml
[profile.release]
strip = "symbols"      # remove symbol table
strip = "debuginfo"    # remove debug info
strip = true           # equivalent to "debuginfo"
```

Strips the binary further. Independent of `debug` — `debug = true` keeps symbols for the debugger; `strip = "symbols"` removes them from the final shipped binary.

### Complete `Cargo.toml` for this project

```toml
[package]
name = "profile_benchmark"
version = "0.1.0"
edition = "2021"

[dependencies]

[dev-dependencies]
criterion = { version = "0.5", features = ["html_reports"] }

[[bench]]
name = "profile_benchmarks"
harness = false

[profile.release]
opt-level = 3
lto = "thin"
codegen-units = 1
overflow-checks = true
debug = "line-tables-only"
```

This is the configuration we use for the benchmarks. With `codegen-units = 1` and `lto = "thin"`, expect a 20-40% additional speedup over the default release profile.

### Applying to Our Project

We will use the `[profile.release]` block above for all benchmark runs. To verify the impact, you can comment out the custom block and re-run benchmarks to see the default release config be 20-40% slower.

## 7. Concept: Criterion Benchmarks

### Explanation

`criterion` is the de-facto Rust benchmarking crate. It runs each benchmark hundreds of times, discards warm-up samples, and reports:

- Mean, median, standard deviation
- Statistical confidence interval
- Comparison to the previous run (improvement or regression)

```rust
use criterion::{criterion_group, criterion_main, Criterion, BenchmarkId};

fn bench_vec_push(c: &mut Criterion) {
    c.bench_function("vec_push_1000", |b| {
        b.iter(|| {
            let mut v = Vec::new();
            for i in 0..1000 {
                v.push(i);
            }
        });
    });
}

criterion_group!(benches, bench_vec_push);
criterion_main!(benches);
```

Run with:

```bash
cargo bench
# → target/criterion/report/index.html  (open in a browser for full analysis)
```

### `c.bench_function` vs `c.benchmark_group`

- **`bench_function`** — a single benchmark with one set of parameters
- **`benchmark_group`** — multiple related benchmarks you want to compare

```rust
use criterion::BenchmarkId;

fn bench_hashmap_sizes(c: &mut Criterion) {
    let mut group = c.benchmark_group("hashmap_insert");
    for size in [100, 1_000, 10_000].iter() {
        group.bench_with_input(BenchmarkId::from_parameter(size), size, |b, &size| {
            b.iter(|| {
                let mut m = std::collections::HashMap::new();
                for i in 0..size {
                    m.insert(i, i);
                }
            });
        });
    }
    group.finish();
}
```

This produces three separate rows in the HTML report — one per input size.

### The `b.iter` closure must be pure

`criterion` runs the closure many times and times the whole call. The closure should:

- **Not** do I/O (no `println!`, no file reads)
- **Not** have side effects that grow over time
- Allocate and deallocate inside the closure if you want to measure allocation cost

For this project, we allocate inside the closure so criterion measures the full cost.

### Python comparison

| Tool | What it does |
|------|--------------|
| `timeit` | Run a snippet N times, print mean |
| `pytest-benchmark` | Pytest plugin, run N times, compare runs in CI |
| `cProfile` | Profile *where* time is spent (not throughput) |
| `asv` (airspeed-velocity) | Long-running benchmark suite, plots over time |

Criterion is closer to `pytest-benchmark` than to `cProfile`: it measures throughput, not call stacks, and it gives you statistical comparisons between runs.

### Applying to Our Project

Our `benches/profile_benchmarks.rs` compares three collection strategies across two workload sizes. After running `cargo bench`, you will see the results table and the HTML report.

## 8. Concept: Comparing Collection Strategies

### Explanation

For a word-counting workload, the data engineering question is: **which collection gives the best throughput?**

| Strategy | `Vec<String>` (sort + count) | `HashMap` | `BTreeMap` |
|----------|------------------------------|-----------|------------|
| Insertion | O(1) amortized | O(1) avg | O(log n) |
| Lookup | O(n) linear scan | O(1) avg | O(log n) |
| Sorted output | Sort at the end: O(n log n) | No (unordered) | Yes (free) |
| Memory | Compact (string + counter pairs) | Hash overhead ~2x | Tree nodes ~3x |

For a one-shot pipeline that just needs the count, `HashMap` wins on throughput. For pipelines that need sorted output (e.g., top-K, report generation), `BTreeMap` is competitive even with the log n cost because it avoids the final sort.

### Python comparison

```python
# Python — collections.Counter is a HashMap equivalent
from collections import Counter
counts = Counter(text.lower().split())

# Sorted — sortedcontainers.SortedDict is the BTreeMap equivalent
from sortedcontainers import SortedDict
counts = SortedDict((w, 0) for w in text.lower().split())
for w in text.lower().split():
    counts[w] += 1
```

Python's `Counter` is the idiomatic single-pass approach. The `SortedDict` version is a worse choice in Python because the per-operation overhead dominates the algorithmic difference.

In Rust, both `HashMap` and `BTreeMap` are zero-overhead abstractions — the algorithm matters, the implementation overhead does not.

### Applying to Our Project

Our three strategies in `lib.rs`:

```rust
pub fn count_vec(text: &str) -> Vec<(String, usize)> { ... }      // sort at end
pub fn count_hashmap(text: &str) -> HashMap<String, usize> { ... } // single pass
pub fn count_btreemap(text: &str) -> BTreeMap<String, usize> { ... } // single pass, sorted
```

The benchmarks measure each one on a 10,000-word corpus in both dev and release modes.

## 9. Putting It All Together

Open `workshop/src/lib.rs`. You will see five `pub fn` signatures, each with `todo!()`. Implement them one at a time, running `cargo test` after each step.

### Step 1: `normalize_word` — Lowercase and strip punctuation

A simple string-cleaning helper. Used by all three counters to avoid `Hello` and `hello` counting as different words.

```rust
pub fn normalize_word(word: &str) -> String {
    word.chars()
        .filter(|c| c.is_alphanumeric())
        .collect::<String>()
        .to_lowercase()
}
```

After implementing, `cargo test step_01_normalize` should pass (2 tests).

### Step 2: `count_vec` — Collect, then count

```rust
pub fn count_vec(text: &str) -> Vec<(String, usize)> {
    let words: Vec<String> = text.split_whitespace()
        .map(normalize_word)
        .filter(|w| !w.is_empty())
        .collect();
    
    let mut counts: HashMap<String, usize> = HashMap::new();
    for w in &words {
        *counts.entry(w.clone()).or_insert(0) += 1;
    }
    
    let mut result: Vec<(String, usize)> = counts.into_iter().collect();
    result.sort_by(|a, b| a.0.cmp(&b.0));
    result
}
```

After implementing, `cargo test step_02_vec` should pass (3 tests).

### Step 3: `count_hashmap` — Single-pass HashMap aggregation

```rust
pub fn count_hashmap(text: &str) -> HashMap<String, usize> {
    let mut counts = HashMap::new();
    for word in text.split_whitespace() {
        let w = normalize_word(word);
        if !w.is_empty() {
            *counts.entry(w).or_insert(0) += 1;
        }
    }
    counts
}
```

After implementing, `cargo test step_03_hashmap` should pass (3 tests).

### Step 4: `count_btreemap` — Single-pass, sorted output

```rust
pub fn count_btreemap(text: &str) -> BTreeMap<String, usize> {
    let mut counts = BTreeMap::new();
    for word in text.split_whitespace() {
        let w = normalize_word(word);
        if !w.is_empty() {
            *counts.entry(w).or_insert(0) += 1;
        }
    }
    counts
}
```

After implementing, `cargo test step_04_btreemap` should pass (3 tests).

### Step 5: `top_n` — Get the N most frequent words

This is a `BinaryHeap` exercise. Sort by count descending; ties broken alphabetically.

```rust
pub fn top_n(counts: &HashMap<String, usize>, n: usize) -> Vec<(String, usize)> {
    use std::cmp::Reverse;
    let mut heap: BinaryHeap<Reverse<(usize, String)>> = BinaryHeap::new();
    for (word, count) in counts {
        heap.push(Reverse((*count, word.clone())));
    }
    let mut result = Vec::with_capacity(n);
    for _ in 0..n {
        if let Some(Reverse((count, word))) = heap.pop() {
            result.push((word, count));
        } else {
            break;
        }
    }
    result
}
```

After implementing, all 14 tests should pass:

```bash
cd workshop && cargo test
```

## 10. Running the Benchmarks

Now the fun part. With all tests green, run the criterion benchmarks:

```bash
cargo bench
```

You will see output like:

```
vec_count/Words(10000)    time:   [523.41 µs 528.13 µs 533.27 µs]
hashmap_count/Words(10000) time:   [142.85 µs 144.21 µs 145.78 µs]
btreemap_count/Words(10000) time:   [201.43 µs 203.18 µs 205.14 µs]
top_n/10                  time:   [2.1431 µs 2.1587 µs 2.1762 µs]
```

Now compare to dev mode. Comment out the `[[bench]]` registration, run `cargo test` in dev mode, and time the operations with `cargo run --release` vs `cargo run`. You will see:

- **Dev mode**: each operation takes 10-50x longer
- **Release mode**: the numbers above (microseconds)
- **Dev-vs-release gap**: 10-50x

This is the headline lesson: **always benchmark in release mode**. Criterion enforces this for you — `cargo bench` always uses `--release`, even if you forget to pass the flag.

To get a visual report:

```bash
# After running cargo bench once:
# Open target/criterion/report/index.html in a browser
# You will see bar charts, confidence intervals, and historical comparisons
```

### Running a single benchmark group

```bash
cargo bench hashmap_count
cargo bench --bench profile_benchmarks vec_count
```

### Comparing two configurations

1. Run `cargo bench` once (saves a baseline in `target/criterion/`)
2. Edit `Cargo.toml` to change `opt-level` to `0`
3. Run `cargo bench --baseline before` — criterion will report regression percentages

## 11. Complete Code Reference

### `workshop/Cargo.toml`

```toml
[package]
name = "profile_benchmark"
version = "0.1.0"
edition = "2021"

[dependencies]

[dev-dependencies]
criterion = { version = "0.5", features = ["html_reports"] }

[[bench]]
name = "profile_benchmarks"
harness = false

[profile.release]
opt-level = 3
lto = "thin"
codegen-units = 1
overflow-checks = true
debug = "line-tables-only"
```

### `workshop/src/lib.rs`

```rust
use std::collections::{BTreeMap, BinaryHeap, HashMap};

pub fn normalize_word(word: &str) -> String {
    word.chars()
        .filter(|c| c.is_alphanumeric())
        .collect::<String>()
        .to_lowercase()
}

pub fn count_vec(text: &str) -> Vec<(String, usize)> {
    let words: Vec<String> = text
        .split_whitespace()
        .map(normalize_word)
        .filter(|w| !w.is_empty())
        .collect();

    let mut counts: HashMap<String, usize> = HashMap::new();
    for w in &words {
        *counts.entry(w.clone()).or_insert(0) += 1;
    }

    let mut result: Vec<(String, usize)> = counts.into_iter().collect();
    result.sort_by(|a, b| a.0.cmp(&b.0));
    result
}

pub fn count_hashmap(text: &str) -> HashMap<String, usize> {
    let mut counts = HashMap::new();
    for word in text.split_whitespace() {
        let w = normalize_word(word);
        if !w.is_empty() {
            *counts.entry(w).or_insert(0) += 1;
        }
    }
    counts
}

pub fn count_btreemap(text: &str) -> BTreeMap<String, usize> {
    let mut counts = BTreeMap::new();
    for word in text.split_whitespace() {
        let w = normalize_word(word);
        if !w.is_empty() {
            *counts.entry(w).or_insert(0) += 1;
        }
    }
    counts
}

pub fn top_n(counts: &HashMap<String, usize>, n: usize) -> Vec<(String, usize)> {
    use std::cmp::Reverse;
    let mut heap: BinaryHeap<Reverse<(usize, String)>> = BinaryHeap::new();
    for (word, count) in counts {
        heap.push(Reverse((*count, word.clone())));
    }
    let mut result = Vec::with_capacity(n);
    for _ in 0..n {
        if let Some(Reverse((count, word))) = heap.pop() {
            result.push((word, count));
        } else {
            break;
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "the quick brown fox jumps over the lazy dog the fox";

    mod step_01_normalize {
        use super::*;

        #[test]
        fn test_lowercase() {
            assert_eq!(normalize_word("Hello"), "hello");
        }

        #[test]
        fn test_strip_punctuation() {
            assert_eq!(normalize_word("don't!"), "dont");
        }
    }

    mod step_02_vec {
        use super::*;

        #[test]
        fn test_count_vec_basic() {
            let result = count_vec("the cat the dog");
            assert_eq!(result, vec![("cat".to_string(), 1), ("dog".to_string(), 1), ("the".to_string(), 2)]);
        }

        #[test]
        fn test_count_vec_sorted() {
            let result = count_vec("zebra apple zebra");
            assert_eq!(result[0].0, "apple");
            assert_eq!(result[1].0, "zebra");
        }

        #[test]
        fn test_count_vec_empty() {
            assert!(count_vec("").is_empty());
        }
    }

    mod step_03_hashmap {
        use super::*;

        #[test]
        fn test_count_hashmap_basic() {
            let result = count_hashmap("a b a c b a");
            assert_eq!(result.get("a"), Some(&3));
            assert_eq!(result.get("b"), Some(&2));
            assert_eq!(result.get("c"), Some(&1));
        }

        #[test]
        fn test_count_hashmap_lowercases() {
            let result = count_hashmap("Apple apple APPLE");
            assert_eq!(result.get("apple"), Some(&3));
        }

        #[test]
        fn test_count_hashmap_strips_punct() {
            let result = count_hashmap("hello, world! hello.");
            assert_eq!(result.get("hello"), Some(&2));
            assert_eq!(result.get("world"), Some(&1));
        }
    }

    mod step_04_btreemap {
        use super::*;

        #[test]
        fn test_count_btreemap_basic() {
            let result = count_btreemap(SAMPLE);
            assert_eq!(result.get("the"), Some(&3));
            assert_eq!(result.get("fox"), Some(&2));
        }

        #[test]
        fn test_count_btreemap_sorted() {
            let result = count_btreemap("zebra apple banana");
            let keys: Vec<&String> = result.keys().collect();
            assert_eq!(keys, vec![&"apple".to_string(), &"banana".to_string(), &"zebra".to_string()]);
        }

        #[test]
        fn test_count_btreemap_empty() {
            assert!(count_btreemap("").is_empty());
        }
    }

    mod step_05_top_n {
        use super::*;

        #[test]
        fn test_top_n_basic() {
            let mut counts = HashMap::new();
            counts.insert("a".to_string(), 5);
            counts.insert("b".to_string(), 3);
            counts.insert("c".to_string(), 7);
            let result = top_n(&counts, 2);
            assert_eq!(result[0].0, "c");
            assert_eq!(result[1].0, "a");
        }

        #[test]
        fn test_top_n_more_than_available() {
            let mut counts = HashMap::new();
            counts.insert("a".to_string(), 1);
            let result = top_n(&counts, 5);
            assert_eq!(result.len(), 1);
        }

        #[test]
        fn test_top_n_zero() {
            let mut counts = HashMap::new();
            counts.insert("a".to_string(), 1);
            assert!(top_n(&counts, 0).is_empty());
        }
    }
}
```

### `workshop/src/main.rs`

```rust
use profile_benchmark::*;

const PANGRAM: &str = "the quick brown fox jumps over the lazy dog \
                       the quick brown fox jumps over the lazy dog \
                       the quick brown fox jumps over the lazy dog";

fn main() {
    let counts = count_hashmap(PANGRAM);
    let top = top_n(&counts, 5);

    println!("Word counts (top 5):");
    for (word, count) in top {
        println!("  {:>4} : {}", count, word);
    }
}
```

### `workshop/benches/profile_benchmarks.rs`

```rust
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use profile_benchmark::{count_btreemap, count_hashmap, count_vec, top_n};

const SAMPLE_1K: &str = "the quick brown fox jumps over the lazy dog ";
const SAMPLE_10K: &str = /* repeat SAMPLE_1K many times */;

fn bench_vec(c: &mut Criterion) {
    let mut group = c.benchmark_group("vec_count");
    for size in [1_000, 10_000].iter() {
        let text = SAMPLE_1K.repeat(*size / 10);
        group.bench_with_input(BenchmarkId::from_parameter(size), &text, |b, t| {
            b.iter(|| count_vec(t));
        });
    }
    group.finish();
}

fn bench_hashmap(c: &mut Criterion) {
    let mut group = c.benchmark_group("hashmap_count");
    for size in [1_000, 10_000].iter() {
        let text = SAMPLE_1K.repeat(*size / 10);
        group.bench_with_input(BenchmarkId::from_parameter(size), &text, |b, t| {
            b.iter(|| count_hashmap(t));
        });
    }
    group.finish();
}

fn bench_btreemap(c: &mut Criterion) {
    let mut group = c.benchmark_group("btreemap_count");
    for size in [1_000, 10_000].iter() {
        let text = SAMPLE_1K.repeat(*size / 10);
        group.bench_with_input(BenchmarkId::from_parameter(size), &text, |b, t| {
            b.iter(|| count_btreemap(t));
        });
    }
    group.finish();
}

fn bench_top_n(c: &mut Criterion) {
    let text = SAMPLE_1K.repeat(1_000);
    let counts = count_hashmap(&text);
    c.bench_function("top_n/10", |b| {
        b.iter(|| top_n(&counts, 10));
    });
}

criterion_group!(benches, bench_vec, bench_hashmap, bench_btreemap, bench_top_n);
criterion_main!(benches);
```

## 12. Exercises

### Easy
Add a `count_hashset(text: &str) -> HashSet<String>` function that returns only the *unique* words (no counts). Then add a `cargo bench` group comparing it against the existing counters.

### Medium
Add a custom profile block `[profile.bench]` to `Cargo.toml` that inherits from `release` but enables `debug = true` (so you can profile a release-mode binary with a debugger). Run `cargo bench` and check that it uses the custom profile.

### Hard
Use `cargo flamegraph` (an external cargo subcommand) to generate a flame graph of `count_hashmap` running in release mode. Identify the function that takes the most time. Hint: install with `cargo install flamegraph` (requires `perf` on Linux, `DTrace` on macOS, or `cargo-instruments` for Instruments.app).

## 13. Summary

| Concept | Description | Python equivalent |
|---------|-------------|-------------------|
| `cargo build` | Dev profile — fast compile, slow run | `python script.py` (no profile) |
| `cargo build --release` | Release profile — slow compile, fast run | N/A |
| `[profile.release]` block | Configure opt-level, LTO, codegen-units, overflow-checks | N/A |
| `opt-level = 3` (default) | Full LLVM optimization | N/A |
| `lto = "thin"` / `"fat"` | Link-time optimization across crates | N/A |
| `codegen-units = 1` | Single compilation unit, better inlining | N/A |
| `overflow-checks = true` | Keep integer overflow panics in release | N/A |
| `criterion` crate | Statistically rigorous benchmarks | `timeit`, `pytest-benchmark` |
| `cargo bench` | Always uses `--release`, generates HTML report | N/A |
| Dev-vs-release gap | 10-50x speedup typical | <2x for `python -O` |
| `top_n` via `BinaryHeap<Reverse<(usize, String)>>` | Min-heap to get top-K | `heapq.nsmallest(n, counts.items(), key=...)` |

**The headline lesson**: Rust gives you two production-quality build modes out of the box, and the release mode is 10-50x faster than dev mode. Combined with `criterion`, you can ship data pipelines whose performance is *measured* and *guaranteed* — not just hoped for. Always run `cargo bench` (which implies `--release`) before declaring a hot loop "fast enough".

## Further Reading

- [The Cargo Book — Profiles](https://doc.rust-lang.org/cargo/reference/profiles.html)
- [criterion.rs documentation](https://github.com/bheisler/criterion.rs)
- [Rust Performance Book](https://nnethercote.github.io/perf-book/)
- [Section 9: Testing](../../09-ObservabilityAndTesting/03-Testing/README.md) — for the unit-test side of the same code
- [BasicCalculator](../../01-Foundations/03-BasicCalculator/README.md) — for the full integer-overflow story

## Related Projects

- [08-RustCollectionsDoc](../08-RustCollectionsDoc/README.md) — uses `criterion` to compare all `std::collections` types
- [04-HashMapCount](../04-HashMapCount/README.md) — deep dive on `HashMap::entry().or_insert()` for counting
- [09-BinaryHeapFruit](../09-BinaryHeapFruit/README.md) — priority queue used in `top_n`
- [10-BTreeSetFruit](../10-BTreeSetFruit/README.md) — ordered collections, the basis for `BTreeMap` understanding
