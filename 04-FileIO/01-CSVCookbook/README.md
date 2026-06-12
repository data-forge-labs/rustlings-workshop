# Project 53: Read, Write, and Transform CSV Files — Python to Rust Workshop

> **Test-driven approach**: This project includes a Cargo project with progressive unit tests. Each function in `workshop/src/lib.rs` starts as a `todo!()` stub. As you follow each section, replace `todo!()` with real code and run `cd workshop && cargo test` to watch the pass count grow. Your goal: **all 7 tests pass**.

## Why Stream CSVs in Rust?

---

## Why Stream CSVs in Rust?

**Python pain:** `pandas.read_csv("5gb.csv")` reads the entire file into memory, guesses types, and stores Python objects for every cell — peak memory is roughly 4× the file size, and a malformed row only surfaces hours into a batch job. There is no compile-time check that the columns match the schema.

**Rust fix:** The `csv` crate processes rows **lazily** — one row at a time, ~1 KB working set, types enforced at compile time via `serde`:

```rust
use csv::ReaderBuilder;
let mut rdr = ReaderBuilder::new().from_path("transactions_2024.csv")?;
for result in rdr.deserialize() {
    let record: Transaction = result?;   // type-checked at compile time
    process(&record);                     // process, discard, move on
}
```

## At a Glance

| # | Concept | Rust | Python | Why it matters |
|---|---------|------|--------|----------------|
| 1 | CSV data model | comma-separated fields, optional header | `csv` module | Understanding CSV structure |
| 2 | String splitting | `.split(pat)` | `str.split(pat)` | Break CSV line into fields |
| 3 | String joining | `.join(sep)` | `str.join(sep)` | Rebuild CSV line from fields |
| 4 | Line iteration | `.lines()` | `.splitlines()` | Process file line by line |
| 5 | String trimming | `.trim()` | `.strip()` | Clean whitespace from fields |
| 6 | Manual CSV parsing | `Vec<String>` from split+trim | N/A (usually a lib) | Build your own parser to understand the format |
| 7 | `csv::Reader` | `csv` crate | `csv.reader` | Production-grade parsing with quoting/escaping |
| 8 | Deserialization | `rdr.deserialize::<T>()` | N/A (dicts) | Auto-map rows to typed structs |
| 9 | Per-row errors | `Result<T, E>` per row | `try/except` around loop | One bad row doesn't crash the pipeline |
| 10 | Buffered reading | `BufReader` | File buffering | Efficient sequential reads |

---

## Table of Contents

1. [Introduction](#1-introduction)
2. [Prerequisites](#2-prerequisites)
3. [Concept: CSV Data Model](#3-concept-csv-data-model)
4. [Concept: Rust String Manipulation](#4-concept-rust-string-manipulation)
5. [Concept: Parsing CSV Lines](#5-concept-parsing-csv-lines)
6. [Concept: Formatting CSV Lines](#6-concept-formatting-csv-lines)
7. [Concept: Multi-Line Content Processing](#7-concept-multi-line-content-processing)
8. [Concept: The `csv` Crate](#8-concept-the-csv-crate)
9. [Putting It All Together](#9-putting-it-all-together)
10. [Complete Code Reference](#10-complete-code-reference)
11. [Summary](#11-summary)

## 1. Introduction

CSV (Comma-Separated Values) is the most common data exchange format in data engineering. Almost every pipeline starts with a CSV ingest or ends with a CSV export. In this workshop, you will build a CSV cookbook: functions to parse, format, count lines, and extract columns from CSV data.

**Python to Rust**: In Python, you reach for `csv.reader`, `csv.writer`, or `pandas.read_csv()`. In Rust, you can use the `csv` crate for production work, but here we will first implement CSV operations manually using Rust's string methods. This gives you a deep feel for how CSV parsing works under the hood and builds fluency with Rust's `str`/`String` API.

**Data-engineering motivation**: Every ETL pipeline that ingests CSVs needs to preview, validate, and transform raw data. The functions you write here are the building blocks of a robust CSV ingestion system.

## 2. Prerequisites

- Completed [Section 3: Collections](../../../03-Collections/README.md) -- comfortable with `Vec<String>`, iterators, and string slicing.
- Basic familiarity with Rust's `&str` vs `String` distinction.

## 3. Concept: CSV Data Model

### Explanation

A CSV file is a text file where each line represents a record, and fields within a line are separated by a delimiter (usually a comma). The first line is often a header row.

```
Name,Price,Quantity
Widget,10.99,100
Gadget,24.99,50
```

**In Python**, you parse CSV like this:
```python
import csv
with open("data.csv") as f:
    reader = csv.reader(f)
    for row in reader:
        print(row)  # ["Name", "Price", "Quantity"], then ["Widget", "10.99", "100"]
```

**In Rust**, we will parse each line by splitting on commas. The `csv` crate handles quoting and escaping automatically (just like Python's `csv` module), but for this workshop we will use string splitting to understand the fundamentals.

### ASCII Diagram: CSV Layout

```
Line 1:  "Name"  ,  "Price"  ,  "Quantity"
          field_0   field_1     field_2
Line 2:  "Widget" ,  "10.99"  ,  "100"
Line 3:  "Gadget" ,  "24.99"  ,  "50"
```

Each line becomes a `Vec<String>`.

## 4. Concept: Rust String Manipulation

### Explanation

Rust has two string types: `&str` (a string slice -- a view into a string) and `String` (an owned, heap-allocated string). Python has only `str` (immutable, heap-allocated).

**Common string methods in Rust**:

| Method | What it does | Python equivalent |
|--------|-------------|-------------------|
| `s.split(pat)` | Returns iterator over substrings split by pattern | `s.split(sep)` |
| `s.trim()` | Removes leading/trailing whitespace | `s.strip()` |
| `fields.join(sep)` | Joins string slices with separator | `sep.join(list_of_strings)` |
| `s.lines()` | Returns iterator over lines | `s.splitlines()` |
| `s.to_string()` | Converts `&str` to `String` | `str(s)` |

### Example

```rust
let line = "a,b,c";
let fields: Vec<&str> = line.split(',').collect();
println!("{:?}", fields); // ["a", "b", "c"]

let joined = fields.join(",");
println!("{}", joined); // "a,b,c"
```

### Applying to Our Project

Every function in this project manipulates CSV content as strings. You will use:

- `split(',')` to parse a line into fields
- `join(",")` to format fields back into a line
- `lines()` to iterate over multi-line content
- `nth()` or indexed access to extract a specific field

## 5. Concept: Parsing CSV Lines

### Explanation

Parsing a CSV line means splitting it by the delimiter (`,`) and returning individual fields as a vector of strings. The simplest approach uses `str::split`.

**In Python**:
```python
line = "a,b,c"
fields = line.split(",")  # ["a", "b", "c"]
```

**In Rust**:
```rust
fn parse_csv_line(line: &str) -> Vec<String> {
    line.split(',')
        .map(|s| s.to_string())
        .collect()
}
```

Key points:
- `split` returns an iterator, so we `.collect()` into a `Vec`.
- We `.map(|s| s.to_string())` to convert each `&str` slice into an owned `String`.

### Edge Cases

- **Empty line**: `split` on an empty string returns one empty string `[""]`.
- **Single field**: No comma means one field (the whole string).
- **Trailing comma**: `"a,"` produces `["a", ""]`.

### Applying to Our Project

Implement `parse_csv_line` in `workshop/src/lib.rs`:

```rust
pub fn parse_csv_line(line: &str) -> Vec<String> {
    line.split(',')
        .map(|s| s.to_string())
        .collect()
}
```

**Run tests**: `cd workshop && cargo test` -- the first three tests (`step_01_parse`) should pass.

## 6. Concept: Formatting CSV Lines

### Explanation

Formatting is the inverse of parsing: take a slice of string fields and join them with commas.

**In Python**:
```python
fields = ["a", "b", "c"]
line = ",".join(fields)  # "a,b,c"
```

**In Rust**:
```rust
fn format_csv_line(fields: &[&str]) -> String {
    fields.join(",")
}
```

The `join` method is available on slices of string-like types. It inserts the separator between each element and returns an owned `String`.

### Applying to Our Project

```rust
pub fn format_csv_line(fields: &[&str]) -> String {
    fields.join(",")
}
```

**Run tests**: Tests `test_format_csv_line` and `test_format_single_field` should pass.

## 7. Concept: Multi-Line Content Processing

### Explanation

Real CSV data has multiple lines. You need to count lines and extract columns across all lines.

**Counting lines** uses `str::lines()`:

**In Python**:
```python
content = "a\nb\nc"
line_count = len(content.splitlines())  # 3
```

**In Rust**:
```rust
pub fn count_lines(csv_content: &str) -> usize {
    csv_content.lines().count()
}
```

**Extracting a column** means: for each line, split to fields, then pick the field at `col_index`:

**In Python**:
```python
content = "x,y,z\n1,2,3\n4,5,6"
col_index = 1
values = [line.split(",")[col_index] for line in content.splitlines()]
# ["y", "2", "5"]
```

**In Rust**:
```rust
pub fn column_values(csv_content: &str, col_index: usize) -> Vec<String> {
    csv_content
        .lines()
        .map(|line| {
            line.split(',')
                .nth(col_index)
                .unwrap_or("")
                .to_string()
        })
        .collect()
}
```

Note: `nth(col_index)` returns `Option<&str>` because the line might not have that many fields. `unwrap_or("")` handles that gracefully, just as Python's `list[i]` would raise `IndexError`.

### Applying to Our Project

```rust
pub fn count_lines(csv_content: &str) -> usize {
    csv_content.lines().count()
}

pub fn column_values(csv_content: &str, col_index: usize) -> Vec<String> {
    csv_content
        .lines()
        .map(|line| {
            line.split(',')
                .nth(col_index)
                .unwrap_or("")
                .to_string()
        })
        .collect()
}
```

**Run tests**: Tests `test_count_lines` and `test_column_values` should pass -- **all 7 tests green**.

## 8. Concept: The `csv` Crate

### Explanation

The `csv` crate is Rust's equivalent of Python's `csv` module. It handles all edge cases that our manual splitting does not:

- **Quoted fields**: `"hello, world"` is one field, not two
- **Escaped quotes**: `""hello""` is the field `"hello"`
- **Custom delimiters**: tab (`\t`), pipe (`|`), etc.
- **Record deserialization**: Map rows directly into Rust structs (like `csv.DictReader`)

**In Python**:
```python
import csv
with open("data.csv") as f:
    reader = csv.DictReader(f)
    for row in reader:
        print(row["Name"], row["Price"])  # dict with header keys
```

**In Rust** (with `csv` + `serde`):
```rust
use csv::ReaderBuilder;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Record {
    name: String,
    price: f64,
}

let mut reader = ReaderBuilder::new()
    .from_path("data.csv")?;
for result in reader.deserialize() {
    let record: Record = result?;
    println!("{} {}", record.name, record.price);
}
```

### Error Handling Comparison

| Python | Rust |
|--------|------|
| `try/except csv.Error` | `Result<Record, csv::Error>` |
| `csv.QUOTE_ALL` | `ReaderBuilder::quoting()` |
| `lineno` on reader | `reader.position().line()` |

The `csv` crate uses `Result` for all operations. Each row read can fail (malformed CSV), so you must handle errors with `?` or `match`. This is Rust's way of making sure you never silently swallow malformed data.

### Applying to Your Data Engineering Work

In production, always use the `csv` crate instead of manual splitting. Add it to `Cargo.toml`:

```toml
[dependencies]
csv = "1.1"
```

Then replace your manual parsing with `csv::Reader` for real-world workloads. Your manual functions in this project serve as a learning exercise -- they show you exactly what the `csv` crate does internally.

## 9. Putting It All Together

The complete `workshop/src/lib.rs` for this project:

```rust
pub fn parse_csv_line(line: &str) -> Vec<String> {
    line.split(',')
        .map(|s| s.to_string())
        .collect()
}

pub fn format_csv_line(fields: &[&str]) -> String {
    fields.join(",")
}

pub fn count_lines(csv_content: &str) -> usize {
    csv_content.lines().count()
}

pub fn column_values(csv_content: &str, col_index: usize) -> Vec<String> {
    csv_content
        .lines()
        .map(|line| {
            line.split(',')
                .nth(col_index)
                .unwrap_or("")
                .to_string()
        })
        .collect()
}
```

After implementing all four functions, run:

```
cd workshop && cargo test
```

You should see:
```
running 7 tests
test tests::step_01_parse::test_parse_simple_line ... ok
test tests::step_01_parse::test_parse_empty_line ... ok
test tests::step_01_parse::test_parse_single_field ... ok
test tests::step_02_format::test_format_csv_line ... ok
test tests::step_02_format::test_format_single_field ... ok
test tests::step_03_content::test_count_lines ... ok
test tests::step_03_content::test_column_values ... ok

test result: ok. 7 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

## 10. Complete Code Reference

The complete project structure:

```
01-CSVCookbook/
├── Cargo.toml
├── src/
│   ├── lib.rs       # All public functions + tests
│   └── main.rs      # CLI entry point (optional)
└── README.md         # This file
```

The `Cargo.toml` has no external dependencies -- we use only Rust's standard library. If you want to use the `csv` crate in a real project, add `csv = "1.1"` to `[dependencies]`.

## 11. Summary

| Concept | Rust Equivalent | Python Equivalent | Used In |
|---------|----------------|-------------------|---------|
| String splitting | `str::split()` | `str.split()` | `parse_csv_line` |
| String joining | `slice::join()` | `str.join()` | `format_csv_line` |
| Line iteration | `str::lines()` | `str.splitlines()` | `count_lines`, `column_values` |
| Optional indexing | `Iterator::nth()` | `list[i]` | `column_values` |
| Professional CSV parsing | `csv` crate | `csv` module | Production replacement |
| Type-safe deserialization | `csv::Reader::deserialize` + `serde` | `csv.DictReader` | Production replacement |

**Exercises**:

1. **Easy**: Modify `column_values` to skip the header line (first line) when extracting values.
2. **Medium**: Write a `has_header(content: &str) -> bool` function that checks if the first line contains only alphabetic field names.
3. **Hard**: Add the `csv` crate to `Cargo.toml` and write a new function `read_csv_file(path: &str) -> Result<Vec<Vec<String>>, csv::Error>` that reads a real CSV file from disk.

## Exercises

* **Easy** – modify the existing function to handle an extra edge case.
* **Medium** – extend the project with a new helper function that re‑uses the core logic.

