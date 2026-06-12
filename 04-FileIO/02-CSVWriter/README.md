# Project 54: Programmatic CSV Writing with Custom Delimiters and Serde — Python to Rust Workshop

> **Test-driven approach**: This project includes a Cargo project with progressive unit tests. Each function in `workshop/src/lib.rs` starts as a `todo!()` stub. As you follow each section, replace `todo!()` with real code and run `cd workshop && cargo test` to watch the pass count grow. Your goal: **all 4 tests pass**.

## Why Serialize Structs Directly to CSV?

Ownership note: In Rust, values like `String` and `Vec` live on the heap, while primitive values (e.g., `i32`, `bool`) live on the stack. Ownership rules govern when heap data is cleaned up.


---
- **Field Renaming (`#[serde(rename_all = "PascalCase")]`)**: Transforms Rust's `snake_case` field names to `PascalCase` (or other cases) for CSV headers. Python's `DictWriter` lets you pass any `fieldnames` list — Rust enforces consistency via attributes.
- **Custom Delimiters (`WriterBuilder::delimiter()`)**: Change the separator from comma to tab, pipe, or semicolon. Same as Python's `delimiter=` kwarg but configured via the builder pattern.
- **Builder Pattern (`WriterBuilder`)**: A fluent API for configuring writers before creation. Python uses kwargs; Rust uses chained method calls with compile-time safety.
- **Flushing (`Writer::flush()`)**: Forces buffered data to disk. Python's `with` block auto-flushes on exit; Rust requires explicit `flush()` or relies on `Drop`.
- **In-Memory Writing (`Writer::from_writer(vec![])`)**: Write to a `Vec<u8>` buffer instead of a file — great for tests. Python equivalent: `io.StringIO`.
- **Error Handling (`?`)**: Every `csv::Writer` operation returns `Result`. The `?` operator propagates errors, similar to Python's `try/except` but zero-overhead at runtime.

---

## Table of Contents

1. [Introduction](#1-introduction)
2. [Prerequisites](#2-prerequisites)
3. [Concept: The `csv` Crate's Writer](#3-concept-the-csv-crates-writer)
4. [Concept: Serde Serialization](#4-concept-serde-serialization)
5. [Concept: Custom Delimiters](#5-concept-custom-delimiters)
6. [Concept: Struct Transformation (Discount Logic)](#6-concept-struct-transformation-discount-logic)
7. [Putting It All Together](#7-putting-it-all-together)
8. [Complete Code Reference](#8-complete-code-reference)
9. [Summary](#9-summary)

## 1. Introduction

Writing CSV files programmatically is a core skill in data engineering. You produce CSVs for exports, reports, and downstream consumers. In this project, you will learn to use Rust's `csv::Writer` with `serde::Serialize` to write type-safe CSV data. As a bonus, the project applies a discount calculation to each product -- a realistic ETL transformation.

**Python to Rust**: Python's `csv.writer` and `csv.DictWriter` let you write rows as lists or dicts. Rust's `csv::Writer` works the same way, but with serde you also get compile-time field name validation. If you misspell a field name, the program won't compile -- Python would only fail at runtime.

**Data-engineering motivation**: Imagine a pipeline that reads product prices from a database, applies a 10% discount, and writes the result to a CSV file for a partner. This project simulates exactly that workflow: transform a `Product` struct (discount logic in `apply_discount`, `total_savings`) and then (in a real app) write it with `csv::Writer`.

## 2. Prerequisites

- Completed [Project 53: CSVCookbook](../../01-CSVCookbook/README.md) -- understanding of CSV structure and the `csv` crate basics.
- Familiarity with `struct` and `impl` from [Section 2: Ownership](../../../../02-Ownership/README.md).

## 3. Concept: The `csv` Crate's Writer

### Explanation

`csv::Writer` is Rust's equivalent of Python's `csv.writer`. It writes records to a writer (file, buffer, stdout) with proper quoting and escaping.

**In Python**:
```python
import csv
with open("output.csv", "w", newline="") as f:
    writer = csv.writer(f)
    writer.writerow(["Name", "Price"])
    writer.writerow(["Widget", 10.99])
```

**In Rust**:
```rust
use csv::Writer;
use std::fs::File;

let mut wtr = Writer::from_path("output.csv")?;
wtr.write_record(&["Name", "Price"])?;
wtr.write_record(&["Widget", "10.99"])?;
wtr.flush()?;
```

Key differences:
- `write_record` takes a slice of string-like values (`&[&str]` or `&[String]`).
- Every operation returns `Result` -- you must handle I/O errors with `?`.
- `flush()` ensures all buffered data is written to disk (like `f.close()` or `with` block exit in Python).

### Writing Structs with Serde

The real power comes from `csv::Writer::serialize`, which takes any type that implements `serde::Serialize` and writes it as a CSV row automatically.

**In Rust**:
```rust
#[derive(serde::Serialize)]
struct Product {
    name: String,
    price: f64,
}

let mut wtr = Writer::from_path("output.csv")?;
wtr.write_record(&["Name", "Price"])?;  // manual header
wtr.serialize(&Product { name: "Widget".into(), price: 10.99 })?;
wtr.flush()?;
```

This is equivalent to Python's `csv.DictWriter` writing a row from a dict:
```python
writer = csv.DictWriter(f, fieldnames=["Name", "Price"])
writer.writeheader()
writer.writerow({"Name": "Widget", "Price": 10.99})
```

### Applying to Our Project

The `Product` struct in `workshop/src/lib.rs` already derives `Serialize`, and uses `#[serde(rename_all = "PascalCase")]` so Rust's `name` and `price` fields become `Name` and `Price` in the CSV header. This mirrors how Python's `DictWriter` uses keys as column headers.

## 4. Concept: Serde Serialization

### Explanation

Serde is Rust's most popular serialization framework. It converts Rust data structures (structs, enums) to/from various formats (CSV, JSON, TOML, YAML, etc. -- over 40 formats).

**In Python**, serialization is dynamic:
```python
import json
data = {"name": "Widget", "price": 10.99}
json.dumps(data)  # '{"name": "Widget", "price": 10.99}'
```

**In Rust**, serde uses trait-based, compile-time serialization:
```rust
use serde::Serialize;

#[derive(Serialize)]
struct Product {
    name: String,
    price: f64,
}

let p = Product { name: "Widget".into(), price: 10.99 };
// Pass to any format writer: csv, json, toml, etc.
```

Key concepts:
- **`#[derive(Serialize)]`**: Auto-generates the code to convert this struct to any format. Equivalent to Python's `__dict__` but type-safe.
- **`#[serde(rename_all = "PascalCase")]`**: Renames fields automatically: `name` -> `Name`, `price` -> `Price`. Other options: `"snake_case"`, `"camelCase"`, `"SCREAMING_SNAKE_CASE"`.
- **Zero-cost abstraction**: The serialization code is generated at compile time -- no reflection, no runtime overhead.

### Applying to Our Project

The `Product` struct:
```rust
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct Product {
    pub name: String,
    pub price: f64,
}
```

When serialized to CSV, this produces:
```
Name,Price
Widget,100.0
```

`Deserialize` is also derived so you could read products from CSV too -- a common pattern in ETL (read CSV, transform, write CSV).

## 5. Concept: Custom Delimiters

### Explanation

CSV files sometimes use delimiters other than commas: tab-separated (TSV), pipe-separated (`|`), semicolon-separated (common in European locales where comma is the decimal separator).

**In Python**:
```python
writer = csv.writer(f, delimiter="|")
writer.writerow(["a", "b", "c"])  # a|b|c
```

**In Rust**:
```rust
let mut wtr = WriterBuilder::new()
    .delimiter(b'|')
    .from_writer(vec![]);

wtr.write_record(&["a", "b", "c"])?;
let data = String::from_utf8(wtr.into_inner()?)?;
// data == "a|b|c\n"
```

Key points:
- The delimiter is a byte (`b'|'`), not a string. This avoids Unicode edge cases.
- `WriterBuilder` uses the builder pattern (method chaining) to configure the writer.
- You can write to an in-memory `Vec<u8>` using `from_writer` for testing.

### Applying to Our Project

In a real pipeline, you might write tab-separated output:
```rust
use csv::WriterBuilder;

let mut wtr = WriterBuilder::new()
    .delimiter(b'\t')
    .from_path("products.tsv")?;
```

## 6. Concept: Struct Transformation (Discount Logic)

### Explanation

The business logic in this project transforms a `Product` by applying a 10% discount. This is pure data transformation -- no I/O -- which makes it easy to test.

**In Python**:
```python
class Product:
    def __init__(self, name: str, price: float):
        self.name = name
        self.price = price

DISCOUNT = 0.1

def apply_discount(product: Product) -> Product:
    return Product(product.name, product.price * (1 - DISCOUNT))

products = [Product("A", 100.0), Product("B", 200.0)]
discounted = [apply_discount(p) for p in products]
savings = sum(p.price * DISCOUNT for p in products)  # 30.0
```

**In Rust**:
```rust
pub const DISCOUNT: f64 = 0.1;

pub fn apply_discount(product: &Product) -> Product {
    Product {
        name: product.name.clone(),
        price: product.price * (1.0 - DISCOUNT),
    }
}

pub fn total_savings(products: &[Product]) -> f64 {
    products.iter().map(|p| p.price * DISCOUNT).sum()
}
```

Key Rust notes:
- `const` vs Python's module-level constant: same concept, but Rust's `const` is inlined at compile time.
- `name: product.name.clone()` -- we must clone the string because the new `Product` needs ownership. Unlike Python, where everything is a reference, Rust requires explicit cloning.
- `.iter().map(...).sum()` -- the iterator pipeline. Equivalent to Python's `sum(p.price * DISCOUNT for p in products)`.

### Applying to Our Project

These are the exact implementations you'll write in `workshop/src/lib.rs`:

```rust
pub const DISCOUNT: f64 = 0.1;

pub fn apply_discount(product: &Product) -> Product {
    Product {
        name: product.name.clone(),
        price: product.price * (1.0 - DISCOUNT),
    }
}

pub fn total_savings(products: &[Product]) -> f64 {
    products.iter().map(|p| p.price * DISCOUNT).sum()
}
```

## 7. Putting It All Together

The complete `workshop/src/lib.rs` for this project:

```rust
use serde::{Deserialize, Serialize};

pub const DISCOUNT: f64 = 0.1;

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct Product {
    pub name: String,
    pub price: f64,
}

pub fn apply_discount(product: &Product) -> Product {
    Product {
        name: product.name.clone(),
        price: product.price * (1.0 - DISCOUNT),
    }
}

pub fn total_savings(products: &[Product]) -> f64 {
    products.iter().map(|p| p.price * DISCOUNT).sum()
}
```

After implementing, run:

```
cd workshop && cargo test
```

Expected output:
```
running 4 tests
test tests::step_01_discount::test_apply_discount ... ok
test tests::step_01_discount::test_apply_discount_zero ... ok
test tests::step_01_discount::test_total_savings ... ok
test tests::step_01_discount::test_total_savings_empty ... ok

test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

### Writing to a CSV File (Extra)

Once the functions pass, you can tie it all together in `main.rs`:

```rust
use csv::Writer;
use csv_writer::{apply_discount, Product};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let products = vec![
        Product { name: "Widget".into(), price: 100.0 },
        Product { name: "Gadget".into(), price: 200.0 },
    ];

    let mut wtr = Writer::from_path("discounted_products.csv")?;
    for p in &products {
        let discounted = apply_discount(p);
        wtr.serialize(&discounted)?;
    }
    wtr.flush()?;
    println!("Written to discounted_products.csv");
    Ok(())
}
```

This produces:
```
Name,Price
Widget,90
Gadget,180
```

## 8. Complete Code Reference

Project structure:

```
02-CSVWriter/
├── Cargo.toml         # csv = "1.1.6", serde = "1.0.136" with "derive"
├── src/
│   ├── lib.rs         # Product struct + discount functions + tests
│   └── main.rs        # CLI entry point
└── README.md          # This file
```

`Cargo.toml` dependencies:
```toml
[dependencies]
csv = "1.1.6"
serde = { version = "1.0.136", features = ["derive"] }
```

## 9. Summary

| Concept | Rust Equivalent | Python Equivalent | Used In |
|---------|----------------|-------------------|---------|
| CSV writing | `csv::Writer` | `csv.writer` | Output generation |
| Struct serialization | `serde::Serialize` | `dataclass` + manual dict | `wtr.serialize()` |
| Field renaming | `#[serde(rename_all = "...")]` | `csv.DictWriter` fieldnames | CSV header control |
| Custom delimiters | `WriterBuilder::delimiter()` | `csv.writer(delimiter=...)` | TSV/pipe output |
| Builder pattern | `WriterBuilder::new()` | N/A (kwargs pattern) | Writer configuration |
| Constant values | `const` | Module-level constant | `DISCOUNT` |
| Iterator sum | `.iter().map().sum()` | `sum(...)` | `total_savings` |

**Exercises**:

1. **Easy**: Change `DISCOUNT` to 0.15 and update `total_savings` to calculate savings from the discounted prices (original - discounted).
2. **Medium**: Add a new struct `DiscountedProduct` with fields `name`, `original_price`, and `discounted_price`. Write a function `to_discounted(product: &Product) -> DiscountedProduct`.
3. **Hard**: Write a complete command-line pipeline: use `csv::Reader` with `Deserialize` to read products from a CSV file, apply discounts, and write the output to a new CSV file.

## Exercises

* **Easy** – modify the existing function to handle an extra edge case.
* **Medium** – extend the project with a new helper function that re‑uses the core logic.

