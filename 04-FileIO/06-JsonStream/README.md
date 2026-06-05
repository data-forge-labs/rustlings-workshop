# 🦀 JSON Streaming — Python to Rust Workshop

*Subtitle: Parse, query, and stream large JSON / NDJSON files with `serde_json`.*

> **Test-driven approach**: This project includes a Cargo project with progressive
> unit tests. Each function in `src/lib.rs` starts as a `todo!()` stub. As you
> follow each section, replace `todo!()` with real code and run `cargo test` to
> watch the pass count grow. Your goal: **all 14 tests pass**.

---

## Why NDJSON for Data Streams?

**Python pain:** `json.load(open("users.json"))` loads the **entire file** into memory. A 10 GB log file becomes 50 GB of `dict` objects in RAM, and you OOM. The fix in Python is to read line-by-line with `for line in f:`, but then you have to call `json.loads(line)` on every line — slow and unsafe (no schema).

**Rust fix:** `serde_json::from_str` parses one line at a time, with a typed `User` struct — every line is validated at parse time. `BufReader::lines()` streams the file, so memory stays constant regardless of file size. The whole file can be gigabytes; your process holds one line at a time plus a `Vec` of parsed structs (or you can process them one at a time and discard).

## At a Glance

| # | Concept | Rust | Python | Why it matters |
|---|---------|------|--------|----------------|
| 1 | Typed parse | `serde_json::from_str::<User>` | `json.loads(line)` | Compile-time schema check |
| 2 | Typed serialize | `serde_json::to_string` | `json.dumps` | Round-trip back to JSON |
| 3 | Untyped walk | `serde_json::Value` | `dict` | Explore unknown JSON |
| 4 | Pretty print | `serde_json::to_string_pretty` | `json.dumps(indent=2)` | Human-readable output |
| 5 | Nested access | walk `Value` by path | `d["a"]["b"]["c"]` | Type-safe accessor |
| 6 | NDJSON streaming | `BufReader::lines` + per-line parse | `for line in f: json.loads(line)` | O(1) memory regardless of file size |
| 7 | NDJSON writing | `BufWriter` + per-line serialize | `for r in rows: f.write(...)` | Same streaming model on write |
| 8 | Error per line | `Result<User, serde_json::Error>` | try/except | Skip bad lines, continue |

---

## Table of Contents
1. [Introduction](#1-introduction)
2. [Prerequisites](#2-prerequisites)
3. [Concept: `serde_json` and Typed Parsing](#3-concept-serde_json-and-typed-parsing)
4. [Concept: Walking Untyped `Value`](#4-concept-walking-untyped-value)
5. [Concept: NDJSON Streaming I/O](#5-concept-ndjson-streaming-io)
6. [Concept: JSON Merge for Overlays](#6-concept-json-merge-for-overlays)
7. [Putting It All Together](#7-putting-it-all-together)
8. [Complete Code Reference](#8-complete-code-reference)
9. [Summary](#9-summary)

## 1. Introduction

JSON comes in two flavors in data engineering: **single-object** (a config file, an API response) and **NDJSON** (newline-delimited, one JSON value per line — used by Elasticsearch, Datadog, Splunk, Kafka Connect sinks, BigQuery streaming inserts, and most log shippers).

This workshop covers both, plus untyped `Value` walking for when you don't know the schema in advance (a common case when consuming a third-party API).

**Python to Rust:** Python's `json.loads` and `json.dumps` work on strings. Rust's equivalents are `serde_json::from_str` and `serde_json::to_string`. The big difference: `serde_json` integrates with `serde` derives, so a single `#[derive(Serialize, Deserialize)]` lets you round-trip any struct.

## 2. Prerequisites

- Completed [04-FileIO/05-YAML](../05-YAML/README.md) — `serde` derive, custom enums.
- Familiarity with `BufReader`/`BufWriter` from [04-FileIO/01-CSVCookbook](../01-CSVCookbook/README.md).

## 3. Concept: `serde_json` and Typed Parsing

```rust
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct User {
    id: u32,
    name: String,
    age: u32,
}

let u: User = serde_json::from_str(r#"{"id":1,"name":"Alice","age":30}"#)?;
```

A type mismatch (`"age": "thirty"`) returns `Err(serde_json::Error)` with a line and column pointer. The error type implements `Display`, so `format!("{}", e)` gives a readable message.

**In Python**, `json.loads` returns a `dict`, and you discover type errors only when you read a field:

```python
u = json.loads(line)
u["age"] + 1  # TypeError: can only concatenate str (not "int") to str
```

The Rust version catches the type error at parse time, before the value is in memory.

## 4. Concept: Walking Untyped `Value`

When you don't know the schema, `serde_json::Value` is the equivalent of a Python `dict`:

```rust
let v: Value = serde_json::from_str(json)?;
println!("{}", v["name"]);           // display
let n: u32 = v["age"].as_u64().unwrap() as u32;  // extract with type check
```

`Value` is an enum:

```rust
enum Value {
    Null, Bool(bool), Number(Number),
    String(String), Array(Vec<Value>), Object(Map<String, Value>),
}
```

You walk an object with `as_object()` (returns `Option<&Map<String, Value>>`) or by indexing (`v["key"]` panics on missing keys for objects but is safe for arrays-of-arrays). For nested lookups, build a small walker like `get_nested_string`.

**In Python**, the same operation is:

```python
v = json.loads(json)
v["user"]["profile"]["name"]
```

The Python version raises `KeyError` on a missing key. Rust's index operator on `Value` also panics on missing keys — for safe access, use `.get(key)` and pattern-match on `Option`.

## 5. Concept: NDJSON Streaming I/O

NDJSON is **one JSON value per line, separated by `\n`**. It's the streaming JSON format: trivially splittable, line-buffered, no closing bracket to wait for. The whole log/sensor/event world uses it.

```rust
use std::io::{BufRead, BufReader, Write, BufWriter};
use std::fs::File;

let file = File::open("data/users.ndjson")?;
let reader = BufReader::new(file);
let mut users = Vec::new();
for line in reader.lines() {
    let line = line?;
    if line.trim().is_empty() { continue; }
    let u: User = serde_json::from_str(&line)?;
    users.push(u);
}
```

The `BufReader` keeps disk I/O batched (8 KB by default), and we hold only one line plus the `Vec` in memory. For files larger than RAM, replace the `Vec` with a streaming consumer and process each `User` as it arrives.

**In Python:**

```python
import json
with open("data/users.ndjson") as f:
    for line in f:
        if not line.strip(): continue
        u = json.loads(line)
        process(u)
```

Functionally identical. The Rust version has the advantage that the line is `&str` (no decode overhead) and the per-line `Result` lets you skip malformed lines without aborting.

## 6. Concept: JSON Merge for Overlays

Sometimes you have a base config and an override, and you want to layer them. For two `Value` objects, the overlay rule is: keys in `b` win, but keys only in `a` are kept.

```rust
pub fn merge_values(a: &Value, b: &Value) -> Value {
    if let (Some(a_obj), Some(b_obj)) = (a.as_object(), b.as_object()) {
        let mut merged = a_obj.clone();
        for (k, v) in b_obj {
            merged.insert(k.clone(), v.clone());
        }
        Value::Object(merged)
    } else {
        a.clone()
    }
}
```

**In Python:**

```python
merged = {**a, **b}
```

But Python silently overwrites non-dict values. The Rust version is explicit: it only merges if **both** are objects, otherwise it returns `a` unchanged. This avoids the trap of `{"a": 1}` + `{"a": [2, 3]}` silently becoming `{"a": [2, 3]}`.

## 7. Putting It All Together

The `lib.rs` is organized in five progressive steps:

1. **Step 1 (`step_01_basic_typed`)** — typed `User` parse and serialize.
2. **Step 2 (`step_02_value_walking`)** — `Value` parsing, pretty-print, nested access, key count.
3. **Step 3 (`step_03_merge`)** — object overlay.
4. **Step 4 (`step_04_ndjson_streaming`)** — read/write NDJSON files, filter by age.
5. **Step 5 (`step_05_file_pretty_write`)** — pretty-write a `Value` to disk.

`main.rs` reads the sample NDJSON, filters, serializes one user, parses nested JSON, and pretty-writes.

## 8. Complete Code Reference

See [`workshop/src/lib.rs`](workshop/src/lib.rs) and [`workshop/src/main.rs`](workshop/src/main.rs). The sample NDJSON is at [`workshop/data/users.ndjson`](workshop/data/users.ndjson).

## 9. Summary

| Concept | Used In |
|---------|---------|
| `serde_json::from_str` | `parse_user`, `parse_value` |
| `serde_json::to_string` | `serialize_user` |
| `serde_json::to_string_pretty` | `pretty_print`, `write_pretty_json_file` |
| `Value` walking | `get_nested_string`, `count_keys` |
| `BufReader::lines` | `read_ndjson_users` |
| `BufWriter<File>` | `write_ndjson_users` |
| Object merge | `merge_values` |
| Iterator filter | `filter_users_by_age` |

## Further Reading

- [serde_json docs](https://docs.rs/serde_json/) — performance tips, `RawValue` for zero-copy
- [simd-json](https://docs.rs/simd-json/) — 4x faster parsing, but requires `&mut [u8]`
- [ndjson crate](https://docs.rs/ndjson/) — streaming NDJSON reader built on `serde_json`
- [Streaming JSON in Rust](https://blog.logrocket.com/parsing-json-rust/) — overview of approaches

## Exercises

1. **Easy**: Add `sum_age(users: &[User]) -> u32` and 1 test.
2. **Medium**: Add `write_compact_json_file(path: &str, users: &[User])` that writes a single JSON array (not NDJSON) and add a test that reads it back with `serde_json::from_reader`.
3. **Hard**: Add a function `count_ndjson_lines(path: &str) -> Result<u64, Error>` that streams the file and counts without parsing — to handle invalid JSON gracefully. Compare its speed to `read_ndjson_users` on a 1 GB test file.
