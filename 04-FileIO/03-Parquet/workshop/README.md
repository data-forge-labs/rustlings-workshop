# Workshop: Parquet

**Goal**: Implement all functions in `src/lib.rs` to pass all 12 tests.

## Functions to Implement

### Step 1 — `Record` struct (basic iteration)

#### `filter_by_threshold`
- **Signature**: `pub fn filter_by_threshold(records: &[Record], threshold: f64) -> Vec<&Record>`
- **Task**: Return references to records whose `value` field is >= `threshold`.
- **Tests**: test_filter_by_threshold

#### `total_value`
- **Signature**: `pub fn total_value(records: &[Record]) -> f64`
- **Task**: Compute the sum of `value * count` for all records.
- **Tests**: test_total_value, test_total_value_empty

#### `record_summary`
- **Signature**: `pub fn record_summary(record: &Record) -> String`
- **Task**: Return a formatted string containing the record's name and value.
- **Tests**: test_record_summary

### Step 2 — Schema and batch construction

#### `sales_schema`
- **Signature**: `pub fn sales_schema() -> SchemaRef`
- **Task**: Return a `SchemaRef` (Arc<Schema>) with three fields: `product: Utf8`, `amount: Float64`, `units: Int64`.

#### `sales_batch`
- **Signature**: `pub fn sales_batch(rows: &[(String, f64, i64)]) -> RecordBatch`
- **Task**: Build a `RecordBatch` from `(product, amount, units)` tuples using `sales_schema()`.

### Step 3 — Parquet round-trip

#### `write_parquet_file`
- **Signature**: `pub fn write_parquet_file(path: &str, batch: &RecordBatch) -> Result<(), Box<dyn std::error::Error>>`
- **Task**: Write a `RecordBatch` to disk as a Parquet file using `ArrowWriter`.
- **Hint**: `parquet::arrow::ArrowWriter::try_new(file, batch.schema(), WriterProperties::default())`

#### `read_parquet_file`
- **Signature**: `pub fn read_parquet_file(path: &str) -> Result<RecordBatch, Box<dyn std::error::Error>>`
- **Task**: Read a Parquet file back into a single `RecordBatch` using `ParquetRecordBatchReaderBuilder`.

### Step 4 — Parquet statistics

#### `parquet_min_value`
- **Signature**: `pub fn parquet_min_value(path: &str, column: &str) -> Result<Option<f64>, Box<dyn std::error::Error>>`
- **Task**: Use `SerializedFileReader` to read the row group's column chunk statistics and return the min.
- **Hint**: `row_group.columns()[i].statistics()` returns `Option<Statistics>`.

### Step 5 — Projection pushdown

#### `read_with_projection`
- **Signature**: `pub fn read_with_projection(path: &str, columns: &[&str]) -> Result<RecordBatch, Box<dyn std::error::Error>>`
- **Task**: Read only the named columns. Hint: `ParquetRecordBatchReaderBuilder::with_projection`.

### Step 6 — Schema evolution

#### `merge_schemas`
- **Signature**: `pub fn merge_schemas(a: &Schema, b: &Schema) -> Schema`
- **Task**: Union the fields of `a` and `b`, deduplicating by name.
- **Hint**: Use a `HashSet<String>` to track seen field names.

## Structs

### `Record`
- Fields: `name: String`, `value: f64`, `count: u32`

## Test Modules

| Module | Tests | What It Tests |
|--------|-------|---------------|
| step_01_records | 4 | Total value, filtering, and record summary |
| step_02_parquet_schema | 2 | Sales schema field count and types |
| step_03_parquet_roundtrip | 2 | Write→read preserves rows and values |
| step_04_parquet_statistics | 2 | Parquet min via column statistics |
| step_05_parquet_projection | 1 | Column projection on read |
| step_06_schema_evolution | 2 | Schema merge unions and dedupes fields |

## How to Run Tests
```bash
cargo test
```

## Exercises

* **Easy** – modify the existing function to handle an extra edge case.
* **Medium** – extend the project with a new helper function that re‑uses the core logic.

