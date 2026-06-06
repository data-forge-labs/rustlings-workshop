//! Warehouse Benchmark — realistic data engineering test
//!
//! Generates 1M synthetic e-commerce events, partitions by year=/month=/day=,
//! writes them with Parquet (baseline) and Lance, then benchmarks.

use std::path::{Path, PathBuf};
use std::sync::Arc;

use arrow_array::{
    Array, ArrayRef, BooleanArray, Float32Array, Int64Array, RecordBatch,
    StringArray, UInt32Array,
};
use arrow_schema::{DataType, Field, Schema};
use arrow_select::concat::concat;
use rand::seq::SliceRandom;
use rand::{Rng, SeedableRng};
use serde::{Deserialize, Serialize};

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

// =============================================================================
// Data generation
// =============================================================================

pub fn build_warehouse_schema() -> Arc<Schema> {
    Arc::new(Schema::new(vec![
        Field::new("id", DataType::UInt32, false),
        Field::new("user_id", DataType::UInt32, false),
        Field::new("event_type", DataType::Utf8, false),
        Field::new("amount", DataType::Float32, true),
        Field::new("country", DataType::Utf8, false),
        Field::new("timestamp", DataType::Int64, false),
    ]))
}

pub fn generate_events(n_rows: usize, seed: u64) -> RecordBatch {
    let mut rng = rand::rngs::StdRng::seed_from_u64(seed);
    let event_types = ["click", "view", "purchase", "cart"];
    let countries = ["US", "UK", "DE", "FR", "JP"];

    let ids: Vec<u32> = (0..n_rows as u32).collect();
    let user_ids: Vec<u32> = (0..n_rows).map(|_| rng.gen_range(1..100_000)).collect();
    let events: Vec<String> = (0..n_rows)
        .map(|_| event_types.choose(&mut rng).unwrap().to_string())
        .collect();
    let amounts: Vec<Option<f32>> = (0..n_rows)
        .map(|i| {
            if events[i] == "purchase" {
                Some(rng.gen_range(1.0..500.0))
            } else {
                None
            }
        })
        .collect();
    let country_vec: Vec<String> = (0..n_rows)
        .map(|_| countries.choose(&mut rng).unwrap().to_string())
        .collect();
    let timestamps: Vec<i64> = (0..n_rows)
        .map(|_| 1_704_067_200_000 + rng.gen_range(0..90 * 86_400_000))
        .collect();

    RecordBatch::try_new(
        build_warehouse_schema(),
        vec![
            Arc::new(UInt32Array::from(ids)) as ArrayRef,
            Arc::new(UInt32Array::from(user_ids)),
            Arc::new(StringArray::from(events)),
            Arc::new(Float32Array::from(amounts)),
            Arc::new(StringArray::from(country_vec)),
            Arc::new(Int64Array::from(timestamps)),
        ],
    )
    .unwrap()
}

// =============================================================================
// Parquet
// =============================================================================

pub fn write_parquet(path: &Path, batch: &RecordBatch) -> Result<u64> {
    use parquet::arrow::ArrowWriter;
    use parquet::file::properties::WriterProperties;

    let file = std::fs::File::create(path)?;
    let props = WriterProperties::builder()
        .set_compression(parquet::basic::Compression::SNAPPY)
        .build();
    let mut writer = ArrowWriter::try_new(file, batch.schema(), Some(props))?;
    writer.write(batch)?;
    writer.close()?;
    Ok(std::fs::metadata(path)?.len())
}

pub fn read_parquet(path: &Path) -> Result<RecordBatch> {
    use parquet::arrow::arrow_reader::ParquetRecordBatchReaderBuilder;

    let file = std::fs::File::open(path)?;
    let builder = ParquetRecordBatchReaderBuilder::try_new(file)?;
    let reader = builder.build()?;
    let batches: Vec<RecordBatch> = reader.collect::<std::result::Result<Vec<_>, _>>()?;
    concat_batches(batches)
}

pub fn read_parquet_columns(path: &Path, columns: &[String]) -> Result<RecordBatch> {
    use parquet::arrow::arrow_reader::ParquetRecordBatchReaderBuilder;

    let file = std::fs::File::open(path)?;
    let builder = ParquetRecordBatchReaderBuilder::try_new(file)?;
    let schema = builder.schema().clone();
    let parquet_schema = builder.parquet_schema().clone();
    let cols_idx: Vec<usize> = columns
        .iter()
        .map(|c| schema.index_of(c).unwrap())
        .collect();
    let mask = parquet::arrow::ProjectionMask::roots(&parquet_schema, cols_idx);
    let reader = builder.with_batch_size(8192).with_projection(mask).build()?;
    let batches: Vec<RecordBatch> = reader.collect::<std::result::Result<Vec<_>, _>>()?;
    concat_batches(batches)
}

fn concat_batches(batches: Vec<RecordBatch>) -> Result<RecordBatch> {
    if batches.is_empty() {
        return Err("empty batch list".into());
    }
    let schema = batches[0].schema();
    let n_cols = batches[0].num_columns();
    let mut all_cols: Vec<ArrayRef> = (0..n_cols)
        .map(|_| Arc::new(UInt32Array::from(Vec::<u32>::new())) as ArrayRef)
        .collect();
    for b in &batches {
        for i in 0..b.num_columns() {
            let combined = concat(&[all_cols[i].as_ref(), b.column(i).as_ref()]).unwrap();
            all_cols[i] = combined;
        }
    }
    Ok(RecordBatch::try_new(schema, all_cols)?)
}

// =============================================================================
// Lance — uses lance::deps::arrow_array (v58) for anything that crosses the
// Lance API boundary.
// =============================================================================

use lance::deps::arrow_array as lance_arr;
use lance::deps::arrow_schema as lance_sch;

/// Convert our external (v58) batch into Lance's bundled (v58) batch.
/// With arrow 58 workspace-wide, the types are the same and this is a no-op
/// clone, but we keep the indirection for clarity.
fn to_lance(batch: &RecordBatch) -> lance_arr::RecordBatch {
    lance_arr::RecordBatch::try_new(batch.schema(), batch.columns().to_vec()).unwrap()
}

fn from_lance(batch: lance_arr::RecordBatch) -> RecordBatch {
    RecordBatch::try_new(batch.schema(), batch.columns().to_vec()).unwrap()
}

/// One-shot batch reader. In arrow 58, `RecordBatchReader` is
/// `Iterator<Item = Result<RecordBatch, ArrowError>>` plus a `schema()` method.
struct OneShotReader {
    batch: Option<lance_arr::RecordBatch>,
    schema: Arc<lance_sch::Schema>,
}

impl OneShotReader {
    fn from_external(batch: &RecordBatch) -> Self {
        let lance_batch = to_lance(batch);
        let schema = lance_batch.schema();
        Self { batch: Some(lance_batch), schema }
    }
}

impl lance_arr::RecordBatchReader for OneShotReader {
    fn schema(&self) -> Arc<lance_sch::Schema> { self.schema.clone() }
}

impl Iterator for OneShotReader {
    type Item = std::result::Result<lance_arr::RecordBatch, lance::deps::arrow_schema::ArrowError>;
    fn next(&mut self) -> Option<Self::Item> {
        self.batch.take().map(Ok)
    }
}

pub async fn write_lance(path: &Path, batch: &RecordBatch) -> Result<u64> {
    use lance::dataset::Dataset;

    let reader = OneShotReader::from_external(batch);
    let _dataset = Dataset::write(reader, path.to_str().unwrap(), None).await?;
    Ok(dir_size(path))
}

pub async fn read_lance(path: &Path) -> Result<RecordBatch> {
    use lance::dataset::Dataset;

    let dataset = Dataset::open(path.to_str().unwrap()).await?;
    let scanner = dataset.scan();
    let mut stream = scanner.try_into_stream().await?;
    use futures::StreamExt;
    let mut batches: Vec<lance_arr::RecordBatch> = Vec::new();
    while let Some(b) = stream.next().await {
        batches.push(b?);
    }
    if batches.is_empty() {
        return Err("empty lance scan".into());
    }
    let external: Vec<RecordBatch> = batches.into_iter().map(from_lance).collect();
    concat_batches(external)
}

pub async fn read_lance_columns(path: &Path, columns: &[&str]) -> Result<RecordBatch> {
    use lance::dataset::Dataset;

    let dataset = Dataset::open(path.to_str().unwrap()).await?;
    let mut scanner = dataset.scan();
    scanner.project(columns).unwrap();
    let mut stream = scanner.try_into_stream().await?;
    use futures::StreamExt;
    let mut batches: Vec<lance_arr::RecordBatch> = Vec::new();
    while let Some(b) = stream.next().await {
        batches.push(b?);
    }
    if batches.is_empty() {
        return Err("empty lance column scan".into());
    }
    let external: Vec<RecordBatch> = batches.into_iter().map(from_lance).collect();
    concat_batches(external)
}

pub async fn take_lance(path: &Path, indices: &[u32]) -> Result<RecordBatch> {
    use lance::dataset::Dataset;

    let dataset = Dataset::open(path.to_str().unwrap()).await?;
    let indices_u64: Vec<u64> = indices.iter().map(|&i| i as u64).collect();
    let lance_schema: Arc<lance::datatypes::Schema> = Arc::new(dataset.schema().clone());
    let batch = dataset.take(
        &indices_u64,
        lance::dataset::ProjectionRequest::Schema(lance_schema),
    ).await?;
    Ok(from_lance(batch))
}

pub async fn filter_lance(path: &Path, event_type: &str) -> Result<RecordBatch> {
    use arrow_select::filter::filter_record_batch;

    let full = read_lance(path).await?;
    let arr = full.column(2).as_any().downcast_ref::<StringArray>().unwrap();
    let mask: BooleanArray = (0..arr.len())
        .map(|i| arr.is_valid(i) && arr.value(i) == event_type)
        .collect();
    Ok(filter_record_batch(&full, &mask)?)
}

pub async fn append_lance(path: &Path, new_batch: &RecordBatch) -> Result<usize> {
    use lance::dataset::Dataset;

    let mut dataset = Dataset::open(path.to_str().unwrap()).await?;
    let reader = OneShotReader::from_external(new_batch);
    dataset.append(reader, None).await?;
    let total = dataset.count_rows(None).await?;
    Ok(total)
}

pub async fn compact_lance(path: &Path) -> Result<()> {
    use lance::dataset::optimize::{compact_files, CompactionOptions};
    use lance::dataset::Dataset;

    let mut dataset = Dataset::open(path.to_str().unwrap()).await?;
    let _metrics = compact_files(&mut dataset, CompactionOptions::default(), None).await?;
    Ok(())
}

// =============================================================================
// Mock Nimble and F3 (simulated from published claims)
// =============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MockResult {
    pub throughput_rows_per_sec: f64,
    pub file_size_bytes: u64,
    pub latency_ms: f64,
}

pub fn mock_nimble_write(n_rows: usize) -> MockResult {
    MockResult {
        throughput_rows_per_sec: 250_000.0 * (n_rows as f64 / 1_000_000.0),
        file_size_bytes: estimate_parquet_size(n_rows) * 7 / 10,
        latency_ms: n_rows as f64 / 250_000.0 * 1000.0,
    }
}

pub fn mock_nimble_random_take(_n_rows: usize, n_indices: usize) -> MockResult {
    MockResult {
        throughput_rows_per_sec: 50_000.0 * (n_indices as f64),
        file_size_bytes: 0,
        latency_ms: (n_indices as f64) * 0.5,
    }
}

pub fn mock_nimble_scan(n_rows: usize) -> MockResult {
    MockResult {
        throughput_rows_per_sec: 800_000.0 * (n_rows as f64 / 1_000_000.0),
        file_size_bytes: 0,
        latency_ms: n_rows as f64 / 800_000.0 * 1000.0,
    }
}

pub fn mock_f3_write(n_rows: usize) -> MockResult {
    MockResult {
        throughput_rows_per_sec: 500_000.0 * (n_rows as f64 / 1_000_000.0),
        file_size_bytes: estimate_parquet_size(n_rows) + 15_360,
        latency_ms: n_rows as f64 / 500_000.0 * 1000.0,
    }
}

pub fn mock_f3_random_take(_n_rows: usize, n_indices: usize) -> MockResult {
    MockResult {
        throughput_rows_per_sec: 5_000.0 * (n_indices as f64),
        file_size_bytes: 0,
        latency_ms: (n_indices as f64) * 2.0,
    }
}

pub fn mock_f3_scan(n_rows: usize) -> MockResult {
    MockResult {
        throughput_rows_per_sec: 1_200_000.0 * (n_rows as f64 / 1_000_000.0),
        file_size_bytes: 0,
        latency_ms: n_rows as f64 / 1_200_000.0 * 1000.0,
    }
}

pub fn estimate_parquet_size(n_rows: usize) -> u64 {
    (n_rows as u64) * 6 * 8 / 2
}

// =============================================================================
// Report types
// =============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BenchResult {
    pub name: String,
    pub format: String,
    pub n_rows: usize,
    pub duration_ms: f64,
    pub rows_per_sec: f64,
    pub file_size_bytes: u64,
    pub notes: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FullBenchReport {
    pub timestamp: String,
    pub machine: String,
    pub total_rows: usize,
    pub results: Vec<BenchResult>,
}

pub fn dir_size(path: &Path) -> u64 {
    let mut total: u64 = 0;
    for entry in walkdir::WalkDir::new(path).into_iter().flatten() {
        if entry.file_type().is_file() {
            total += entry.metadata().map(|m| m.len()).unwrap_or(0);
        }
    }
    total
}

pub fn now_iso() -> String {
    chrono::Utc::now().to_rfc3339()
}

pub fn machine_info() -> String {
    format!(
        "{} cores, {}",
        std::thread::available_parallelism().map(|n| n.get()).unwrap_or(1),
        std::env::consts::OS,
    )
}

pub fn write_json_report(path: &Path, report: &FullBenchReport) -> Result<()> {
    let json = serde_json::to_string_pretty(report)?;
    std::fs::write(path, json)?;
    Ok(())
}

// =============================================================================
// Partition helpers
// =============================================================================

pub fn partition_path(base: &Path, year: u32, month: u32, day: u32) -> PathBuf {
    base.join(format!("year={year:04}"))
        .join(format!("month={month:02}"))
        .join(format!("day={day:02}"))
}

pub fn make_dirs(p: &Path) -> Result<()> {
    std::fs::create_dir_all(p)?;
    Ok(())
}
