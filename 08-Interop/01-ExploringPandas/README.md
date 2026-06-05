# Project 57: Rust meets pandas — DataFrame operations and Python/Rust interop

> **Test-driven approach**: This project includes a Cargo project with progressive
> unit tests. Each function in `workshop/src/lib.rs` starts as a `todo!()` stub. As you
> follow each section, replace `todo!()` with real code and run `cd workshop && cargo test` to
> watch the pass count grow. Your goal: **all 17 tests pass**.

## Why Build DataFrame Operations from Primitives?

**Python pain:** Pandas DataFrames copy data, coerce types at runtime (`"123"` might become int or stay str), and have no compile-time validation — bugs surface in production at 3 AM. For 10GB ETL jobs, the 2x memory multiplier wastes money.

**Rust fix:** Build the same operations from explicit, zero-cost primitives — no hidden allocations, no type surprises, compile-time checked:

```rust
use serde::Deserialize;
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct FruitRecord {
    pub fruit: String,
    pub year: u32,      // u32 can never become NaN
    pub price: f64,
}
```

A CSV containing `"abc"` in the `price` column won't compile. Pandas-level expressiveness with C-level performance and Rust-level safety.

---

## At a Glance

| # | Concept | Rust | Python | Why it matters |
|---|---------|------|--------|----------------|
| 1 | Serde derive | `#[derive(Serialize, Deserialize)]` | pydantic `BaseModel` | Typed data structures from CSV schema |
| 2 | CSV deserialization | `csv::Reader::deserialize()` | `pd.read_csv()` | Stream CSV rows into typed structs |
| 3 | HashMap entry API | `entry().or_insert()` | `defaultdict` / `dict.setdefault()` | GroupBy aggregation with sum/count |
| 4 | Iterator filter | `.iter().filter().cloned().collect()` | `df[df["price"] > x]` | Filter records by predicate |
| 5 | `f64` ordering | `.sort_by(\|a, b\| a.partial_cmp(b))` | `df.describe()` | Compute min/max/mean/count |
| 6 | CSV serialization | `csv::Writer::serialize()` | `df.to_csv()` | Write structs back to CSV format |
| 7 | `Result` errors | `Result<Vec<T>, String>` | `try/except` | Propagate parse and IO errors |

---

## Concepts at a Glance

**1. Serde derive macros** — Python's pydantic uses `BaseModel` with type annotations; Rust's `#[derive(Serialize, Deserialize)]` generates serialisation code at compile time. Both map fields by name, but Rust rejects type mismatches at compile time rather than at runtime.

**2. CSV deserialization** — `pd.read_csv()` loads everything into memory. Rust's `csv::Reader::deserialize()` returns a lazy iterator — rows are parsed on demand, streaming-friendly. Type mismatches fail immediately at the first bad row, not silently with NaN coercion.

**3. HashMap entry API** — Python's `defaultdict(float)` auto-initialises missing keys. Rust's `entry(key).or_insert(default)` does the same — it returns a mutable reference to the value (inserting a default if absent), which you then update in place.

**4. Iterator filter** — Pandas boolean indexing (`df[df["price"] > x]`) is concise but opaque. Rust's `.iter().filter(|r| r.price > threshold).collect()` makes every step explicit and is lazy — nothing allocates until `.collect()`.

**5. f64 partial_cmp** — Python's `list.sort()` works on floats. Rust requires `partial_cmp` because `f64` has NaN which breaks total ordering. The compiler forces you to handle an edge case Python silently gets wrong.

**6. CSV serialization** — `df.to_csv()` writes the entire DataFrame. Rust's `csv::Writer::serialize()` writes rows one at a time to any `Write` sink — files, buffers, network streams. Serde derives ensure output matches input structure.

**7. Result error handling** — Python raises exceptions. Rust's `Result` is a return value — callers must handle errors (even if just with `?` to propagate). This makes error paths visible in the function signature.

---

## Table of Contents

1. [Introduction](#1-introduction)
2. [Prerequisites](#2-prerequisites)
3. [Concept: CSV Reading with Serde](#3-concept-csv-reading-with-serde)
4. [Concept: GroupBy — HashMap Aggregation](#4-concept-groupby--hashmap-aggregation)
5. [Concept: Filtering with Iterators](#5-concept-filtering-with-iterators)
6. [Concept: Summary Statistics](#6-concept-summary-statistics)
7. [Concept: CSV Writing](#7-concept-csv-writing)
8. [Putting It All Together](#8-putting-it-all-together)
9. [Complete Code Reference](#9-complete-code-reference)
10. [Summary](#10-summary)

## 1. Introduction

You know pandas. You've written `df.groupby("fruit")["price"].mean()` a hundred
times. This workshop maps those pandas operations directly to idiomatic Rust,
using the `csv` and `serde` crates.

You'll build a fruit-price analysis library that:

- Reads CSV data into typed Rust structs (like `pd.read_csv`)
- Groups records and computes mean prices (like `df.groupby(...).mean()`)
- Filters records by price threshold (like `df[df["price"] > x]`)
- Computes min/max/mean/count statistics (like `df.describe()`)
- Writes records back to CSV (like `df.to_csv()`)

**Python -> Rust**: In Python, pandas is a black-box DataFrame engine. In Rust,
you build these operations explicitly with iterators, HashMaps, and serde
derives. You get full control over memory layout, error handling, and
performance — no hidden overhead.

## 2. Prerequisites

- [01-Foundations/04-MasterMind](../../01-Foundations/04-MasterMind/README.md) --
  structs, Vec, Option
- [02-Ownership/03-TicketV2](../../02-Ownership/03-TicketV2/README.md) -- Result,
  error handling with `?`
- [03-Collections/04-HashMapCount](../../03-Collections/04-HashMapCount/README.md) --
  HashMap entry API
- [03-Collections/12-RustIterators](../../03-Collections/12-RustIterators/README.md) --
  iterators, closures
- Installed: Rust toolchain, `cargo`

## 3. Concept: CSV Reading with Serde

### Explanation

In Python:
```python
import pandas as pd
df = pd.read_csv("fruits.csv")
```

pandas infers column types, handles missing values, and returns a DataFrame. In
Rust, you define the shape of each row as a struct and let `serde` deserialise
it automatically. The `csv` crate streams rows without loading the whole file
into memory.

```rust
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct FruitRecord {
    pub fruit: String,
    pub year: u32,
    pub price: f64,
}
```

The `#[derive(Serialize, Deserialize)]` macro generates the CSV
serialisation/deserialisation code for you — similar to pandas' type inference,
but explicit at compile time.

The reader works on any `Read` source (bytes, files, stdin):

```rust
let mut reader = csv::Reader::from_reader(bytes);
for result in reader.deserialize() {
    match result {
        Ok(record) => records.push(record),
        Err(e) => return Err(format!("CSV parse error: {}", e)),
    }
}
```

**Python comparison**: `pd.read_csv()` returns a DataFrame; Rust's
`csv::Reader::deserialize()` returns an iterator of `Result<FruitRecord>`.
pandas may quietly coerce types; Rust fails at the first type mismatch — no
surprise NaN values downstream.

### Applying to Our Project

The `read_fruits()` function accepts `&[u8]` (a byte slice), creates a CSV
reader, and deserialises each row into `FruitRecord`. Errors are propagated as
`Result<Vec<FruitRecord>, String>`.

## 4. Concept: GroupBy — HashMap Aggregation

### Explanation

In Python:
```python
df.groupby("fruit")["price"].mean()
```

pandas splits by unique values in the "fruit" column, computes the mean of
"price" per group, and returns a Series.

In Rust, the same logic uses a `HashMap<&str, (f64, usize)>` where each entry
accumulates `(sum, count)`. After iteration, divide sum by count:

```rust
let mut sums: HashMap<&str, (f64, usize)> = HashMap::new();
for r in records {
    let entry = sums.entry(&r.fruit).or_insert((0.0, 0));
    entry.0 += r.price;
    entry.1 += 1;
}
// Convert to Vec<(String, f64)> by dividing sum / count
```

```
┌───────────────────────────────────────────┐
│            HashMap<&str, (f64, usize)>     │
├──────────┬────────────────────────────────┤
│ "Apple"  │  sum=2.5  count=2              │
│ "Banana" │  sum=0.8  count=1              │
└──────────┴────────────────────────────────┘
        │  map: (fruit, sum/count)
        ▼
   Vec("Apple", 1.25), ("Banana", 0.80)
```

**Python comparison**: pandas does groupby internally with C-optimised code;
Rust's HashMap approach is explicit but equally fast. The `.entry()` API is
similar to `defaultdict(float)` in Python — it inserts a default value if the
key is missing, then returns a mutable reference.

### Applying to Our Project

Two functions implement groupby:

- `mean_price_per_fruit()` — groups by the `fruit` field
- `mean_price_per_year()` — groups by the `year` field

Both follow the same pattern: accumulate sums and counts in a HashMap, then
convert to a sorted `Vec`.

## 5. Concept: Filtering with Iterators

### Explanation

In Python:
```python
df[df["price"] > 1.0]
```

pandas boolean indexing filters rows where the condition is True.

In Rust, the equivalent is the `filter()` iterator adaptor:

```rust
records
    .iter()
    .filter(|r| r.price > threshold)
    .cloned()
    .collect()
```

This chains three operations:

1. `.iter()` — creates an iterator over references to `FruitRecord`
2. `.filter(|r| r.price > threshold)` — keeps only records where the closure
   returns `true` (lazy — no allocation yet)
3. `.cloned()` — converts `&FruitRecord` to `FruitRecord` (requires `Clone`)
4. `.collect()` — materialises the filtered items into a `Vec`

**Python comparison**: `filter()` in Rust is exactly `itertools.filter()` in
Python. The key difference: Rust iterators are lazy by default — nothing
happens until you call `.collect()`. This is like generator expressions in
Python: `(r for r in records if r.price > threshold)`.

### Applying to Our Project

`filter_by_price(records, 1.5)` returns a new `Vec<FruitRecord>` containing
only records where `price > 1.5`. The function works on any `&[FruitRecord]`
slice, including sub-slices and empty arrays.

## 6. Concept: Summary Statistics

### Explanation

In Python:
```python
df["price"].describe()
# count    3.0
# mean     2.0
# min      1.0
# max      3.0
```

In Rust, we compute min, max, mean, and count manually:

```rust
let count = records.len();
let mut prices: Vec<f64> = records.iter().map(|r| r.price).collect();
prices.sort_by(|a, b| a.partial_cmp(b).unwrap());
let min = prices[0];
let max = prices[prices.len() - 1];
let sum: f64 = prices.iter().sum();
let mean = sum / count as f64;
```

We collect prices into a `Vec<f64>`, sort them (using `partial_cmp` because
`f64` does not implement `Ord` — NaN values break total ordering), then read
the first and last elements.

**Python comparison**: pandas `.describe()` returns a full statistical summary
(count, mean, std, min, 25%, 50%, 75%, max). Our Rust version returns a 4-tuple
`(min, max, mean, count)` — simpler but directly comparable. The sorting step
is explicit in Rust; pandas hides it behind its C engine.

### Applying to Our Project

`summary_stats()` returns `(f64, f64, f64, usize)` representing
(min_price, max_price, mean_price, count). For empty input, it returns
`(0.0, 0.0, 0.0, 0)`.

## 7. Concept: CSV Writing

### Explanation

In Python:
```python
df.to_csv("output.csv", index=False)
```

In Rust, we use `csv::Writer` and serde serialization:

```rust
let mut wtr = csv::Writer::from_writer(Vec::new());
for r in records {
    wtr.serialize(r).map_err(|e| format!("CSV write error: {}", e))?;
}
wtr.flush()?;
String::from_utf8(wtr.into_inner()?)?
```

We write to a `Vec<u8>` buffer, then convert to `String`. This lets us return
the CSV as a string instead of writing to a file — useful for testing and for
passing data around without filesystem I/O.

**Python comparison**: The round-trip works the same way: struct -> CSV string
-> struct. With serde, the struct field names become CSV column headers, just
like pandas uses the DataFrame column names.

### Applying to Our Project

`write_fruits()` serialises a slice of `FruitRecord` into a CSV-formatted
`String`. The test suite verifies the round-trip: write -> read -> assert
equality.

## 8. Putting It All Together

Open `workshop/src/lib.rs`. You'll find these functions with `todo!()`:

1. `read_fruits(bytes: &[u8]) -> Result<Vec<FruitRecord>, String>` — parse CSV bytes
2. `mean_price_per_fruit(records: &[FruitRecord]) -> Vec<(String, f64)>` — groupby fruit
3. `mean_price_per_year(records: &[FruitRecord]) -> Vec<(u32, f64)>` — groupby year
4. `filter_by_price(records: &[FruitRecord], threshold: f64) -> Vec<FruitRecord>` — filter
5. `write_fruits(records: &[FruitRecord]) -> Result<String, String>` — write CSV
6. `summary_stats(records: &[FruitRecord]) -> (f64, f64, f64, usize)` — statistics

Implement them one at a time. After each step, run:

```bash
cd workshop && cargo test
```

Tests are grouped into four modules matching the sections above:
`step_01_csv_io`, `step_02_groupby`, `step_03_filtering`, `step_04_statistics`.

Finally, run:

```bash
cd workshop && cargo run
```

You'll see the CLI demo that reads hardcoded CSV data, prints groupby results,
a filter count, and summary stats.

## 9. Complete Code Reference

```rust
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct FruitRecord {
    pub fruit: String,
    pub year: u32,
    pub price: f64,
}

pub fn read_fruits(bytes: &[u8]) -> Result<Vec<FruitRecord>, String> {
    let mut reader = csv::Reader::from_reader(bytes);
    let mut records = Vec::new();
    for result in reader.deserialize() {
        match result {
            Ok(record) => records.push(record),
            Err(e) => return Err(format!("CSV parse error: {}", e)),
        }
    }
    Ok(records)
}

pub fn mean_price_per_fruit(records: &[FruitRecord]) -> Vec<(String, f64)> {
    use std::collections::HashMap;
    let mut sums: HashMap<&str, (f64, usize)> = HashMap::new();
    for r in records {
        let entry = sums.entry(&r.fruit).or_insert((0.0, 0));
        entry.0 += r.price;
        entry.1 += 1;
    }
    let mut result: Vec<_> = sums
        .into_iter()
        .map(|(fruit, (sum, count))| (fruit.to_string(), sum / count as f64))
        .collect();
    result.sort_by(|a, b| a.0.cmp(&b.0));
    result
}

pub fn mean_price_per_year(records: &[FruitRecord]) -> Vec<(u32, f64)> {
    use std::collections::HashMap;
    let mut sums: HashMap<u32, (f64, usize)> = HashMap::new();
    for r in records {
        let entry = sums.entry(r.year).or_insert((0.0, 0));
        entry.0 += r.price;
        entry.1 += 1;
    }
    let mut result: Vec<_> = sums
        .into_iter()
        .map(|(year, (sum, count))| (year, sum / count as f64))
        .collect();
    result.sort_by_key(|&(year, _)| year);
    result
}

pub fn filter_by_price(records: &[FruitRecord], threshold: f64) -> Vec<FruitRecord> {
    records
        .iter()
        .filter(|r| r.price > threshold)
        .cloned()
        .collect()
}

pub fn write_fruits(records: &[FruitRecord]) -> Result<String, String> {
    let mut wtr = csv::Writer::from_writer(Vec::new());
    for r in records {
        wtr.serialize(r).map_err(|e| format!("CSV write error: {}", e))?;
    }
    wtr.flush().map_err(|e| format!("CSV flush error: {}", e))?;
    String::from_utf8(
        wtr.into_inner().map_err(|e| format!("CSV inner error: {}", e))?
    ).map_err(|e| format!("UTF-8 error: {}", e))
}

pub fn summary_stats(records: &[FruitRecord]) -> (f64, f64, f64, usize) {
    let count = records.len();
    if count == 0 {
        return (0.0, 0.0, 0.0, 0);
    }
    let mut prices: Vec<f64> = records.iter().map(|r| r.price).collect();
    prices.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let min = prices[0];
    let max = prices[prices.len() - 1];
    let sum: f64 = prices.iter().sum();
    let mean = sum / count as f64;
    (min, max, mean, count)
}
```

## 10. Summary

| Concept | Python Equivalent | Rust Implementation |
|---|---|---|
| CSV reading | `pd.read_csv()` | `csv::Reader` + `serde::Deserialize` |
| GroupBy mean | `df.groupby("x")["y"].mean()` | `HashMap` entry API + sum/count |
| Filtering | `df[df["price"] > x]` | `.iter().filter().cloned().collect()` |
| Statistics | `df["price"].describe()` | Sort, min, max, sum, count |
| CSV writing | `df.to_csv()` | `csv::Writer` + `serde::Serialize` |

**Exercises:**

1. (Easy) Add a `max_price_per_fruit()` function that returns the highest price
   for each fruit.
2. (Medium) Add a `filter_by_year()` function that filters records by a given
   year. Write unit tests for it.
3. (Hard) Implement a `sort_by_price()` function that returns records sorted by
   price ascending. Benchmark it against `records.sort_by()`.
