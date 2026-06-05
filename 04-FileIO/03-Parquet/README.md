# Project 55: Apache Parquet Columnar Format — Arrow Integration — Python to Rust Workshop

> **Test-driven approach**: This project includes a Cargo project with progressive unit tests. Each function in `workshop/src/lib.rs` starts as a `todo!()` stub. As you follow each section, replace `todo!()` with real code and run `cd workshop && cargo test` to watch the pass count grow. Your goal: **all 4 tests pass**.

## Why Use Parquet's Columnar Storage?

**Python pain:** A 100-column CSV at 10 GB forces you to read 10 GB even when you only need 2 columns (200 MB). `pd.read_parquet("data.parquet", columns=[...])` hides the column-projection win behind a one-liner, and there's no compile-time check that your struct matches the schema.

**Rust fix:** The `parquet` and `arrow` crates give explicit control over column projection, row groups, and predicate pushdown. You work with typed `Record` structs and zero-cost iterators:

```rust
let filtered: Vec<&Record> = records.iter().filter(|r| r.value > 100.0).collect();
let total:    f64        = records.iter().map(|r| r.value * r.count as f64).sum();
```

When connected to actual Parquet files, the `parquet` crate reads only the requested column chunks — no wasted I/O.

## At a Glance

| # | Concept | Rust | Python | Why it matters |
|---|---------|------|--------|----------------|
| 1 | Columnar vs Row-Oriented | Parquet file format | `pd.read_parquet` | Querying 2 of 100 columns reads only those 2 — no I/O waste |
| 2 | Data Struct | `struct Record` | `dataclass` | Typed schema, compile-time checked |
| 3 | String Formatting | `format!` macro | f-strings | Formatted strings, compiled to efficient code |
| 4 | Iterator Filter | `.filter(\|r\| ...)` | list comprehension `if` | Lazy, chainable filtering |
| 5 | Iterator Map + Sum | `.map().sum()` | `sum(f(x) for x in xs)` | Zero-copy aggregation |
| 6 | Reference Collections | `Vec<&Record>` | list of references | Borrow data, no copies |
| 7 | Numeric Conversion | `x as f64` | implicit `int → float` | Surface conversion decisions explicitly |
| 8 | Arrow Format | `arrow::RecordBatch` | `pyarrow.Table` | In-memory columnar format, cache + SIMD friendly |
| 9 | Parquet Features | column chunks, row groups, statistics | `pyarrow.parquet` | Compression + predicate pushdown |

---

## Table of Contents

1. [Introduction](#1-introduction)
2. [Prerequisites](#2-prerequisites)
3. [Concept: Row-Oriented vs Columnar Storage](#3-concept-row-oriented-vs-columnar-storage)
4. [Concept: The Apache Parquet Format](#4-concept-the-apache-parquet-format)
5. [Concept: Apache Arrow In-Memory Format](#5-concept-apache-arrow-in-memory-format)
6. [Concept: Rust `Record` Struct and Data Transformation](#6-concept-rust-record-struct-and-data-transformation)
7. [Concept: Filtering and Aggregation with Iterators](#7-concept-filtering-and-aggregation-with-iterators)
8. [Putting It All Together](#8-putting-it-all-together)
9. [Complete Code Reference](#9-complete-code-reference)
10. [Summary](#10-summary)

## 1. Introduction

Parquet is the dominant columnar storage format in modern data engineering. It powers data lakes on AWS, Azure, GCP, and is the native format for Apache Spark, DuckDB, and many OLAP engines. Understanding Parquet is essential for any data engineer.

**Python to Rust**: In Python, you read Parquet with `pandas.read_parquet()` or `pyarrow.parquet.read_table()`. These calls hide a complex columnar storage format behind a simple API. In Rust, the `parquet` and `arrow` crates give you the same power, but you interact with it through type-safe `Record` structs, iterators, and schema-aware APIs.

**Data-engineering motivation**: Parquet is 10-100x faster for analytical queries than CSV or JSON because of column pruning, compression, and predicate pushdown. Every data engineer building pipelines at scale should understand how Parquet works under the hood.

## 2. Prerequisites

- Completed [Project 54: CSVWriter](../02-CSVWriter/README.md) -- comfortable with structs, serde, and data transformation.
- Familiarity with `Vec`, iterators, and closures from [Section 3: Collections](../../03-Collections/README.md).

## 3. Concept: Row-Oriented vs Columnar Storage

### Explanation

CSV and JSON are **row-oriented** formats. All fields of a row are stored together:

```
Row 1: [Alice, 25, Engineer]
Row 2: [Bob,   30, Manager ]
Row 3: [Carol, 35, Analyst ]
```

To read only the `age` column, you must parse every row, skip the other fields, and extract the second value. The CPU wastes time parsing fields you don't need.

**Parquet is columnar**. All values of a column are stored together:

```
name:   [Alice, Bob,   Carol]
age:    [25,    30,    35   ]
role:   [Engineer, Manager, Analyst]
```

To read only `age`, the reader jumps directly to the `age` column chunk and reads only those bytes. This is called **column projection**.

### ASCII Diagram

```
Row-oriented (CSV):
  ┌───────┬─────┬──────────┐
  │ Alice │ 25  │ Engineer │  ← read all, keep 2nd field
  ├───────┼─────┼──────────┤
  │ Bob   │ 30  │ Manager  │
  ├───────┼─────┼──────────┤
  │ Carol │ 35  │ Analyst  │
  └───────┴─────┴──────────┘

Columnar (Parquet):
  name:   Alice  │ Bob  │ Carol     → only read this
  age:    25     │ 30   │ 35        chunk if querying age
  role:   Eng.   │ Mgr. │ Analyst
```

### Applying to Our Project

The `Record` struct in this project represents a single logical row. When you filter or aggregate, you work with a `Vec<Record>`. In Parquet on disk, these fields would be stored in separate column chunks for efficient access.

## 4. Concept: The Apache Parquet Format

### Explanation

Parquet is not just columnar -- it has several features that make it the standard for data lakes:

| Feature | What It Does | Python Equivalent |
|---------|-------------|-------------------|
| **Column chunks** | Each column is stored as a contiguous chunk per row group | `pyarrow.parquet.read_column` |
| **Encoding** | Dictionary, run-length, delta encoding for compression | Automatic in `pyarrow` |
| **Compression** | Snappy, Zstd, Gzip per column | `parquet.write_table(compression='snappy')` |
| **Row groups** | Horizontal partitioning within a file | `row_group_size` in pyarrow |
| **Statistics** | Min/max/null count per column chunk (enables predicate pushdown) | Automatic statistics |
| **Schema** | Nested, repeated, optional fields | `pyarrow.schema` |
| **Predicate pushdown** | Skip entire row groups based on stats | `filters=` in `read_parquet` |

### Python vs Rust: Reading Parquet

**In Python**:
```python
import pandas as pd
df = pd.read_parquet("data.parquet", columns=["name", "value"])
# Only "name" and "value" columns are loaded from disk
```

**In Rust** (using the `parquet` crate):
```rust
use parquet::file::reader::FileReader;
use parquet::file::reader::SerializedFileReader;

let file = std::fs::File::open("data.parquet")?;
let reader = SerializedFileReader::new(file)?;
let metadata = reader.metadata();
// Schema: Vec<ColumnDescriptor> with name, type, encoding
// Row groups: iterable, each with column chunks containing compressed data
```

### Why This Matters

When you read only 2 of 100 columns from a CSV file, you still read all 100 columns worth of bytes. With Parquet, you read exactly the bytes for the 2 columns. This is why Parquet can be 50x faster for analytical workloads.

## 5. Concept: Apache Arrow In-Memory Format

### Explanation

Apache Arrow is the in-memory columnar format that Parquet files are often read into. While Parquet is the on-disk format (optimized for compression and I/O), Arrow is the in-memory format (optimized for CPU cache and SIMD operations).

**In Python**:
```python
import pyarrow as pa
table = pa.table({
    "name": ["Alice", "Bob"],
    "value": [10.0, 20.0],
})
# Arrow Columnar format: name and value are separate arrays
```

**In Rust**:
```rust
use arrow::array::{Float64Array, StringArray};
use arrow::record_batch::RecordBatch;

let names = StringArray::from(vec!["Alice", "Bob"]);
let values = Float64Array::from(vec![10.0, 20.0]);
let batch = RecordBatch::try_new(
    schema.clone(),
    vec![Arc::new(names), Arc::new(values)],
)?;
```

The key difference from a `Vec<Record>` (row-oriented):

| Aspect | `Vec<Record>` (Row-oriented) | Arrow `RecordBatch` (Columnar) |
|--------|------------------------------|-------------------------------|
| Memory layout | Structs spread across heap | Contiguous arrays per column |
| Cache efficiency | Poor (fields interleaved) | Excellent (sequential access) |
| SIMD-friendly | No | Yes |
| Zero-copy slicing | No | Yes (by column) |

### Applying to Our Project

Our `Record` struct is row-oriented (like a CSV row or Python tuple). In a real Parquet pipeline, you would convert `Vec<Record>` into an Arrow `RecordBatch` before writing to Parquet, and convert `RecordBatch` back to `Vec<Record>` after reading.

## 6. Concept: Rust `Record` Struct and Data Transformation

### Explanation

The `Record` struct represents a single data point in our Parquet-like pipeline:

```rust
pub struct Record {
    pub name: String,
    pub value: f64,
    pub count: u32,
}
```

**In Python**, this is equivalent to a namedtuple or dataclass:
```python
from dataclasses import dataclass

@dataclass
class Record:
    name: str
    value: float
    count: int
```

The `record_summary` function formats a `Record` into a human-readable string:

```rust
pub fn record_summary(record: &Record) -> String {
    format!(
        "Record: {} | value={:.2}, count={}",
        record.name, record.value, record.count
    )
}
```

This uses Rust's `format!` macro -- equivalent to Python's f-string:
```python
f"Record: {record.name} | value={record.value:.2}, count={record.count}"
```

### Applying to Our Project

Implement `record_summary`:
```rust
pub fn record_summary(record: &Record) -> String {
    format!(
        "Record: {} | value={:.2}, count={}",
        record.name, record.value, record.count
    )
}
```

## 7. Concept: Filtering and Aggregation with Iterators

### Explanation

Two of the most common data operations are **filtering** (select rows matching a condition) and **aggregation** (compute a summary value).

**`filter_by_threshold`** returns references to records whose `value` exceeds a threshold:

**In Python**:
```python
def filter_by_threshold(records, threshold):
    return [r for r in records if r.value > threshold]
```

**In Rust**:
```rust
pub fn filter_by_threshold(records: &[Record], threshold: f64) -> Vec<&Record> {
    records.iter().filter(|r| r.value > threshold).collect()
}
```

Key points:
- `.iter()` returns references (`&Record`), so the return type is `Vec<&Record>` -- we borrow the original records, avoiding copies.
- `.filter(|r| ...)` takes a closure. This is equivalent to Python's `if` in a list comprehension.
- `.collect()` builds the `Vec` from the iterator.

**`total_value`** computes the sum of `value * count` across all records:

**In Python**:
```python
def total_value(records):
    return sum(r.value * r.count for r in records)
```

**In Rust**:
```rust
pub fn total_value(records: &[Record]) -> f64 {
    records.iter().map(|r| r.value * r.count as f64).sum()
}
```

Note: `r.count as f64` converts `u32` to `f64` explicitly. Rust does not do implicit numeric conversions (unlike Python, which auto-promotes `int` to `float`).

### The Iterator Pipeline

Rust's iterator pipeline is functionally equivalent to Python's generator expressions:

| Operation | Rust | Python |
|-----------|------|--------|
| Transform each element | `.map(\|x\| f(x))` | `(f(x) for x in ...)` |
| Keep matching elements | `.filter(\|x\| cond(x))` | `(x for x in ... if cond(x))` |
| Sum all values | `.sum()` | `sum(...)` |
| Collect into vector | `.collect::<Vec<_>>()` | `list(...)` |

### Applying to Our Project

```rust
pub fn filter_by_threshold(records: &[Record], threshold: f64) -> Vec<&Record> {
    records.iter().filter(|r| r.value > threshold).collect()
}

pub fn total_value(records: &[Record]) -> f64 {
    records.iter().map(|r| r.value * r.count as f64).sum()
}
```

## 8. Putting It All Together

The complete `workshop/src/lib.rs` for this project:

```rust
pub struct Record {
    pub name: String,
    pub value: f64,
    pub count: u32,
}

pub fn filter_by_threshold(records: &[Record], threshold: f64) -> Vec<&Record> {
    records.iter().filter(|r| r.value > threshold).collect()
}

pub fn total_value(records: &[Record]) -> f64 {
    records.iter().map(|r| r.value * r.count as f64).sum()
}

pub fn record_summary(record: &Record) -> String {
    format!(
        "Record: {} | value={:.2}, count={}",
        record.name, record.value, record.count
    )
}
```

After implementing all three functions, run:

```
cd workshop && cargo test
```

Expected output:
```
running 4 tests
test tests::step_01_records::test_total_value ... ok
test tests::step_01_records::test_filter_by_threshold ... ok
test tests::step_01_records::test_total_value_empty ... ok
test tests::step_01_records::test_record_summary ... ok

test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

### Connecting to Parquet (Extra)

In a real project, you would add these to `Cargo.toml`:

```toml
[dependencies]
parquet = "52.0"
arrow = "52.0"
```

Then read a Parquet file into `Vec<Record>`:

```rust
use parquet::file::reader::{FileReader, SerializedFileReader};
use std::fs::File;

fn read_parquet_records(path: &str) -> Result<Vec<Record>, Box<dyn std::error::Error>> {
    let file = File::open(path)?;
    let reader = SerializedFileReader::new(file)?;
    let mut records = Vec::new();

    for maybe_row in reader.get_row_iter(None)? {
        let row = maybe_row?;
        let name: String = row.get_string(0)?.to_string();
        let value: f64 = row.get_double(1)?;
        let count: u32 = row.get_int(2)? as u32;
        records.push(Record { name, value, count });
    }
    Ok(records)
}
```

This is the Rust equivalent of:
```python
import pandas as pd
df = pd.read_parquet("data.parquet")
records = [Record(row.name, row.value, row.count) for _, row in df.iterrows()]
```

## 9. Complete Code Reference

Project structure:

```
03-Parquet/
├── Cargo.toml         # No external dependencies required
├── src/
│   ├── lib.rs         # Record struct + filter/aggregate/summary + tests
│   └── main.rs        # CLI entry point
└── README.md          # This file
```

`Cargo.toml` is dependency-free for the core functions. Add `parquet` and `arrow` crates when you are ready to work with actual Parquet files.

## 10. Summary

| Concept | Rust Equivalent | Python Equivalent | Used In |
|---------|----------------|-------------------|---------|
| Columnar storage | Parquet column chunks | `pyarrow.parquet` | Format comparison |
| Row-oriented struct | `struct Record` | `dataclass` | Data model |
| String formatting | `format!` macro | f-strings | `record_summary` |
| Iterator filter | `.filter()` with closure | List comprehension `if` | `filter_by_threshold` |
| Iterator map + sum | `.map().sum()` | Generator + `sum()` | `total_value` |
| Reference collections | `Vec<&Record>` | List of references | `filter_by_threshold` |
| Numeric conversion | `as` keyword | Automatic promotion | `count as f64` |
| In-memory columnar | Arrow `RecordBatch` | `pyarrow.Table` | (Extra reading) |
| On-disk columnar | Parquet `SerializedFileReader` | `pd.read_parquet` | (Extra reading) |

**Exercises**:

1. **Easy**: Write a function `average_value(records: &[Record]) -> f64` that returns the mean of the `value` field across all records. Handle the empty case with `0.0`.
2. **Medium**: Write `filter_by_name(records: &[Record], prefix: &str) -> Vec<&Record>` that returns records whose `name` starts with the given prefix. Use `str::starts_with`.
3. **Hard**: Add the `parquet` crate to `Cargo.toml` and write a function `write_records_to_parquet(records: &[Record], path: &str) -> Result<(), Box<dyn std::error::Error>>` that writes the records to a Parquet file. Use `arrow::record_batch::RecordBatch` and `parquet::arrow::ArrowWriter`.
