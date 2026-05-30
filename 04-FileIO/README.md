# Section 4: File I/O — CSV & Parquet at Scale

*Python's pandas reads CSVs. Rust's csv and parquet crates do it faster, with less memory.*

## Prerequisites

- Completed [Section 3: Collections](../03-Collections/README.md)
- Understand `Result` and error handling

## Projects in This Section

| # | Project | Concepts | Format |
|---|---------|----------|--------|
| 53 | **CSVCookbook** — read, write, transform CSV | `csv` crate, deserialization, record iteration, error handling | Project |
| 54 | **CSVWriter** — programmatic CSV writing | `csv::Writer`, custom delimiters, `serde` (`Deserialize`/`Serialize`) | Project |
| 55 | **Parquet** — Apache Parquet columnar format | Parquet format, columnar storage, Arrow integration | Project |
| 56 | **DataManagementLessonReflection** — I/O reflection | File I/O, serialization, columnar vs row-oriented | Reflection |

## Learning Path

1. Start with **53-CSVCookbook** for basic CSV reading
2. Move to **54-CSVWriter** for writing CSV data
3. Explore **55-Parquet** for columnar storage
4. Reflect with **56-DataManagementLessonReflection**

## Key Concepts Covered

| Rust Concept | Python Equivalent | Why It Matters |
|---|---|---|
| `csv` crate | `csv` module | Fast CSV parsing |
| `serde` | N/A (Pydantic) | Type-safe deserialization |
| `Deserialize` / `Serialize` | `json.loads` / `json.dumps` | Data format conversion |
| Parquet + Arrow | `pyarrow` / `pandas` | Columnar storage |
| `BufReader` / `BufWriter` | File buffering | Efficient I/O |
| `std::fs::File` | `open()` | File operations |
