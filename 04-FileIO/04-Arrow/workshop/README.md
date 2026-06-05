# Workshop: Apache Arrow

**Goal**: Implement all 20 functions in `src/lib.rs` to pass all 20 tests.

## Functions to Implement

### `step_01_arrays` (3 tests)
- `build_int32_array(values: Vec<i32>) -> Int32Array` — `Int32Array::from(values)`
- `build_string_array(values: Vec<&str>) -> StringArray` — `StringArray::from(values)`
- `build_float64_array(values: Vec<f64>) -> Float64Array` — `Float64Array::from(values)`

### `step_02_schema` (2 tests)
- `build_schema() -> Schema` — 3-field schema: `id: Int32` (non-null), `name: Utf8` (nullable), `age: Int32` (nullable)
- `nullable_field(name: &str, dt: DataType) -> Field` — `Field::new(name, dt, true)`

### `step_03_builders` (3 tests)
- `build_int32_with_builder(values: Vec<i32>) -> Int32Array` — use `Int32Builder`
- `build_string_with_builder(values: Vec<&str>) -> StringArray` — use `StringBuilder`
- `build_mixed_batch(names: Vec<&str>, ages: Vec<i32>) -> RecordBatch` — combine both builders into one batch

### `step_04_batch` (4 tests)
- `build_sample_batch() -> RecordBatch` — 5-row `people` batch: `id=[1..5]`, `name=[Alice, Bob, Carol, Dave, Eve]`, `age=[30, 25, 35, 28, 42]`
- `batch_num_rows(batch: &RecordBatch) -> usize`
- `batch_column_name(batch: &RecordBatch, idx: usize) -> String`
- `batch_schema_string(batch: &RecordBatch) -> String`

### `step_05_csv` (2 tests)
- `csv_bytes_to_batch(csv: &[u8]) -> Result<RecordBatch, Box<dyn std::error::Error>>` — use `arrow::csv::ReaderBuilder`
- `csv_with_nullable_schema(csv: &[u8]) -> Result<RecordBatch, Box<dyn std::error::Error>>` — same but with all-nullable schema

### `step_06_ipc` (2 tests)
- `write_ipc_to_bytes(batch: &RecordBatch) -> Result<Vec<u8>, Box<dyn std::error::Error>>` — use `arrow::ipc::writer::StreamWriter`
- `read_ipc_from_bytes(bytes: &[u8]) -> Result<RecordBatch, Box<dyn std::error::Error>>` — use `arrow::ipc::reader::StreamReader`

### `step_07_advanced` (4 tests)
- `sum_int32_column(batch: &RecordBatch, col_name: &str) -> Option<i64>` — hint: `arrow::compute::sum`
- `filter_batch_by_value(batch: &RecordBatch, col_name: &str, threshold: i32) -> RecordBatch` — hint: `arrow::compute::filter_record_batch`
- `slice_batch(batch: &RecordBatch, offset: usize, length: usize) -> RecordBatch` — hint: `RecordBatch::slice`
- `cast_int32_to_float64(batch: &RecordBatch, col_name: &str) -> RecordBatch` — hint: `arrow::compute::cast`

## Test Modules

| Module | Tests | What it tests |
|--------|-------|---------------|
| `step_01_arrays` | 3 | Primitive `Int32Array`, `StringArray`, `Float64Array` |
| `step_02_schema` | 2 | `Schema` field count + names, `Field` nullability |
| `step_03_builders` | 3 | `Int32Builder`, `StringBuilder`, multi-column batch |
| `step_04_batch` | 4 | `RecordBatch` shape, schema, named columns |
| `step_05_csv` | 2 | CSV bytes → `RecordBatch` (typed + nullable schemas) |
| `step_06_ipc` | 2 | IPC write + read roundtrip preserves shape |
| `step_07_advanced` | 4 | `sum`, `filter`, `slice`, `cast` over an `Int32` column |

## How to Run Tests

```bash
cargo test
```

All 20 tests should fail with `todo!()` until you implement each function.
