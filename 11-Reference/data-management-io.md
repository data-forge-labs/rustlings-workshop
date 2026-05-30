# Data Management & I/O — Reference

## File I/O Basics

### BufReader / BufWriter

```rust
use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Write};

// Reading line by line
let file = File::open("input.csv").unwrap();
let reader = BufReader::new(file);
for line in reader.lines() {
    let line = line.unwrap();
    // process line
}

// Writing
let file = File::create("output.csv").unwrap();
let mut writer = BufWriter::new(file);
writeln!(writer, "col1,col2,col3").unwrap();
```

- `BufReader` — buffered read (reduces syscalls)
- `BufWriter` — buffered write
- Always prefer over raw `File::read` / `write` for text processing

### Reading Entire Files

```rust
let contents = std::fs::read_to_string("file.txt").unwrap();
let bytes = std::fs::read("file.bin").unwrap();
```

## CSV with `csv` Crate

```rust
use csv::{Reader, Writer};

// Reading with headers
let mut rdr = Reader::from_path("data.csv").unwrap();
for result in rdr.deserialize() {
    let record: MyRow = result.unwrap();
}

// Writing with headers
let mut wtr = Writer::from_path("out.csv").unwrap();
wtr.write_record(&["name", "value"]).unwrap();
wtr.serialize(("alice", 42)).unwrap();
```

```toml
# Cargo.toml
[dependencies]
csv = "1.3"
serde = { version = "1", features = ["derive"] }
```

## Serde — Serialization Framework

```rust
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Record {
    name: String,
    value: f64,
    #[serde(rename = "timestamp")]
    ts: String,
}
```

- `#[serde(rename = "...")]` — map field names
- `#[serde(skip_serializing_if = "Option::is_none")]` — skip None fields
- `#[serde(default)]` — use Default for missing fields during deserialization
- Works with: JSON, CSV, TOML, YAML, Parquet, bincode, msgpack

## Parquet / Arrow

```toml
[dependencies]
parquet = "53"
arrow = "53"
```

```rust
use parquet::file::reader::FileReader;
use parquet::file::reader::SerializedFileReader;

let file = std::fs::File::open("data.parquet").unwrap();
let reader = SerializedFileReader::new(file).unwrap();
let metadata = reader.metadata();
let num_rows = metadata.file_metadata().num_rows();
```

Columnar format advantages:
- Compression (run-length, dictionary)
- Predicate pushdown (read only needed columns)
- Schema evolution
- **Industry standard for data lakes** (Iceberg, Delta Lake, LakeFS)

## Error Handling with `Result`

```rust
use std::io::{self, BufRead};
use csv::Error as CsvError;

fn process_csv(path: &str) -> Result<u64, Box<dyn std::error::Error>> {
    let mut rdr = csv::Reader::from_path(path)?;  // ? propagates error
    let mut count = 0;
    for result in rdr.records() {
        let _record = result?;
        count += 1;
    }
    Ok(count)
}
```

- `Result<T, E>` is the Rust equivalent of Python `try/except`
- `?` operator == implicit `return Err(e.into())`
- `Box<dyn Error>` for heterogeneous error types
- `thiserror` for custom error enums, `anyhow` for application-level errors

## Data Pipeline Best Practices

1. **Use `BufReader`/`BufWriter`** for sequential file processing.
2. **Deserialize via Serde** — avoid manual string splitting.
3. **Use `?` for propagation**, `context()` (from `anyhow`) for rich errors.
4. **Stream large files** — don't `read_to_string` GB-sized CSVs.
5. **Use `chunks` or `windows`** on `Vec<u8>` for batch processing.
6. **Leverage iterators** — compose `.map()`, `.filter()`, `.take()` for pipeline steps.
7. **Prefer Parquet** over CSV for production — typed, compressed, columnar.

## Python → Rust I/O

| Python | Rust |
|--------|------|
| `open(f)` / `for line in f` | `BufReader::new(File::open(p)?)` |
| `csv.DictReader` | `csv::Reader::from_path(p)` + `deserialize` |
| `pd.read_csv()` | `csv::Reader` + manual mapping |
| `json.dumps` / `json.loads` | `serde_json::to_string` / `from_str` |
| `pickle` | `bincode` or `rmp-serde` (MessagePack) |
| `pyarrow.parquet` | `parquet` crate |
| try/except | `Result<T, E>` + `?` |
