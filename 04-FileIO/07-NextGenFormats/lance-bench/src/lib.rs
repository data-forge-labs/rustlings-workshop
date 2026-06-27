//! Lance Bench — A working Lance dataset end-to-end
//!
//! Test-driven: every function starts as `todo!()`. Implement step by step.
//!
//! Steps:
//!   01 — Open an in-memory dataset and write a small batch
//!   02 — Scan all rows (sequential read)
//!   03 — Take specific rows by index (random access — the killer feature)
//!   04 — Filter rows with predicate pushdown
//!   05 — Project only a subset of columns
//!   06 — Append new rows (data evolution)
//!   07 — Create a vector index on a column
//!   08 — Check out a previous version (zero-copy versioning)

use std::sync::Arc;

use arrow_array::{Float32Array, Int32Array, RecordBatch, StringArray};
use arrow_schema::{DataType, Field, Schema};
use rand::seq::SliceRandom;
use rand::SeedableRng;

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

/// Build the schema for our events table:
///   id: Int32  (non-null)
///   name: Utf8  (nullable)
///   value: Float32  (nullable)
///   date: Utf8  (non-null)  -- ISO date string
pub fn build_events_schema() -> Arc<Schema> {
    Arc::new(Schema::new(vec![
        Field::new("id", DataType::Int32, false),
        Field::new("name", DataType::Utf8, true),
        Field::new("value", DataType::Float32, true),
        Field::new("date", DataType::Utf8, false),
    ]))
}

/// Build a single RecordBatch with `n_rows` synthetic events.
pub fn make_events_batch(n_rows: usize, seed: u64) -> RecordBatch {
    let mut rng = rand::rngs::StdRng::seed_from_u64(seed);
    let names = ["alice", "bob", "carol", "dave", "eve"];
    let ids: Vec<i32> = (0..n_rows as i32).collect();
    let name_vec: Vec<String> = (0..n_rows)
        .map(|_| names.choose(&mut rng).unwrap().to_string())
        .collect();
    let value_vec: Vec<f32> = (0..n_rows).map(|i| (i as f32) * 0.5 + 0.123).collect();
    let date_vec: Vec<String> = (0..n_rows)
        .map(|i| {
            let day = (i % 28) + 1;
            format!("2024-01-{:02}", day)
        })
        .collect();

    RecordBatch::try_new(
        build_events_schema(),
        vec![
            Arc::new(Int32Array::from(ids)),
            Arc::new(StringArray::from(name_vec)),
            Arc::new(Float32Array::from(value_vec)),
            Arc::new(StringArray::from(date_vec)),
        ],
    )
    .unwrap()
}

/// Wrap a RecordBatch iterator for Lance's async write API.
pub fn batch_iter(batch: RecordBatch) -> impl Iterator<Item = std::result::Result<RecordBatch, arrow_schema::ArrowError>> {
    std::iter::once(Ok(batch))
}

// =============================================================================
// Step 01: write a small dataset to disk
// =============================================================================
/// Write a single batch to a new Lance dataset at `path`. Returns the URI used.
pub async fn write_initial_dataset(path: &str, batch: RecordBatch) -> Result<String> {
    use lance::dataset::Dataset;
    let reader = batch_iter(batch);
    let dataset = Dataset::write(reader, path, None).await?;
    Ok(dataset.uri().to_string())
}

pub async fn count_rows(path: &str) -> Result<usize> {
    use lance::dataset::Dataset;
    let dataset = Dataset::open(path).await?;
    Ok(dataset.count_rows(None).await?)
}

pub async fn take_rows(path: &str, indices: &[u32]) -> Result<RecordBatch> {
    use lance::dataset::Dataset;
    let dataset = Dataset::open(path).await?;
    let indices_u64: Vec<u64> = indices.iter().map(|&i| i as u64).collect();
    let lance_schema: Arc<lance::datatypes::Schema> = Arc::new(dataset.schema().clone());
    let batch = dataset.take(&indices_u64, lance::dataset::ProjectionRequest::Schema(lance_schema)).await?;
    Ok(RecordBatch::try_new(batch.schema(), batch.columns().to_vec())?)
}

pub async fn filter_by_value(path: &str, threshold: f32) -> Result<RecordBatch> {
    use lance::dataset::Dataset;
    let dataset = Dataset::open(path).await?;
    let batch = dataset.scan().try_into_stream().await?
        .try_collect::<Vec<_>>().await?;
    let batch = arrow_select::concat::concat_batches(&batch[0].schema(), &batch)?;
    let arr = batch.column(2).as_any().downcast_ref::<Float32Array>().unwrap();
    let mask: arrow_array::BooleanArray = (0..arr.len())
        .map(|i| arr.is_valid(i) && arr.value(i) > threshold)
        .collect();
    Ok(arrow_select::filter::filter_record_batch(&batch, &mask)?)
}

pub async fn project_two_columns(path: &str) -> Result<RecordBatch> {
    use lance::dataset::Dataset;
    let dataset = Dataset::open(path).await?;
    let mut scanner = dataset.scan();
    scanner.project(&["id", "value"])?;
    let batch = scanner.try_into_stream().await?
        .try_collect::<Vec<_>>().await?;
    Ok(arrow_select::concat::concat_batches(&batch[0].schema(), &batch)?)
}

pub async fn append_batch(path: &str, new_batch: RecordBatch) -> Result<usize> {
    use lance::dataset::Dataset;
    let mut dataset = Dataset::open(path).await?;
    let reader = batch_iter(new_batch);
    dataset.append(reader, None).await?;
    Ok(dataset.count_rows(None).await?)
}

pub async fn create_index_on_id(path: &str) -> Result<()> {
    use lance::dataset::Dataset;
    use lance::index::IndexType;
    let mut dataset = Dataset::open(path).await?;
    dataset.create_index(&["id"], IndexType::Scalar, None, None, false).await?;
    Ok(())
}

pub async fn current_version(path: &str) -> Result<u64> {
    use lance::dataset::Dataset;
    let dataset = Dataset::open(path).await?;
    Ok(dataset.version().version)
}

// =============================================================================
// Step 02: scan all rows (sequential read)
// =============================================================================
/// Open the dataset at `path` and return the total row count.
pub async fn count_rows(path: &str) -> Result<usize> {
    todo!("Step 02: open Lance dataset, return `dataset.count_rows()`")
}

// =============================================================================
// Step 03: take specific rows by index (random access — Lance's killer feature)
// =============================================================================
/// Open the dataset, take rows at the given indices, return the resulting batch.
pub async fn take_rows(path: &str, indices: &[u32]) -> Result<RecordBatch> {
    todo!("Step 03: use `dataset.take(indices, projection)` to fetch specific rows")
}

// =============================================================================
// Step 04: filter rows with predicate pushdown
// =============================================================================
/// Open the dataset and return all rows where `value > threshold`.
pub async fn filter_by_value(path: &str, threshold: f32) -> Result<RecordBatch> {
    todo!("Step 04: use `dataset.scan().filter(predicate).try_into_stream()`")
}

// =============================================================================
// Step 05: project only a subset of columns
// =============================================================================
/// Open the dataset, project only `id` and `value`, return the batch.
pub async fn project_two_columns(path: &str) -> Result<RecordBatch> {
    todo!("Step 05: use `dataset.scan().project(&[\"id\", \"value\"])`")
}

// =============================================================================
// Step 06: append a new batch (data evolution, zero-copy where possible)
// =============================================================================
/// Open the dataset, append `new_batch`, return the new row count.
pub async fn append_batch(path: &str, new_batch: RecordBatch) -> Result<usize> {
    todo!("Step 06: use `dataset.append(new_batch)` and return new count")
}

// =============================================================================
// Step 07: create a vector index on the `value` column (simulated)
// =============================================================================
/// Open the dataset and create a scalar index on `id`. Returns Ok(()) on success.
pub async fn create_index_on_id(path: &str) -> Result<()> {
    todo!("Step 07: use `dataset.create_index(&[\"id\"], IndexType::Scalar, ...)`")
}

// =============================================================================
// Step 08: check out a previous version (zero-copy versioning)
// =============================================================================
/// Open dataset at `path`, return the version number (should be 1 after initial write).
pub async fn current_version(path: &str) -> Result<u64> {
    todo!("Step 08: read `dataset.version().version`")
}

// =============================================================================
// Tests — progressive, mirror the steps above
// =============================================================================
#[cfg(test)]
mod tests {
    use super::*;

    /// Helper: create a temp dir, return its path.
    fn temp_dir(tag: &str) -> String {
        let dir = tempfile::tempdir().unwrap();
        let p = dir.path().to_path_buf();
        // We leak the tempdir so it isn't dropped mid-test (cleans up at process exit).
        std::mem::forget(dir);
        let path = p.join(tag);
        std::fs::create_dir_all(&path).unwrap();
        path.to_string_lossy().to_string()
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    async fn step_01_write_initial_dataset() {
        let path = temp_dir("step01");
        let batch = make_events_batch(100, 42);
        let uri = write_initial_dataset(&path, batch).await.unwrap();
        assert!(std::path::Path::new(&uri).exists(), "Lance dir should exist at {}", uri);
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    async fn step_02_count_rows() {
        let path = temp_dir("step02");
        write_initial_dataset(&path, make_events_batch(500, 1)).await.unwrap();
        let n = count_rows(&path).await.unwrap();
        assert_eq!(n, 500);
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    async fn step_03_take_random_rows() {
        let path = temp_dir("step03");
        write_initial_dataset(&path, make_events_batch(1000, 7)).await.unwrap();
        let indices = vec![5_u32, 100, 250, 999];
        let batch = take_rows(&path, &indices).await.unwrap();
        assert_eq!(batch.num_rows(), 4);
        // Row 5 should have id=5
        let id_col = batch.column(0).as_any().downcast_ref::<Int32Array>().unwrap();
        assert_eq!(id_col.value(0), 5);
        assert_eq!(id_col.value(3), 999);
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    async fn step_04_filter_by_value() {
        let path = temp_dir("step04");
        write_initial_dataset(&path, make_events_batch(100, 11)).await.unwrap();
        // value = i*0.5 + 0.123 ; value > 25.0 means i >= 50
        let batch = filter_by_value(&path, 25.0).await.unwrap();
        assert!(batch.num_rows() > 0, "should find rows where value > 25");
        // Every returned row's value should exceed threshold
        let v = batch.column(2).as_any().downcast_ref::<Float32Array>().unwrap();
        for i in 0..v.len() {
            assert!(v.value(i) > 25.0, "row {}: value {} should be > 25", i, v.value(i));
        }
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    async fn step_05_project_two_columns() {
        let path = temp_dir("step05");
        write_initial_dataset(&path, make_events_batch(50, 13)).await.unwrap();
        let batch = project_two_columns(&path).await.unwrap();
        assert_eq!(batch.num_columns(), 2);
        assert_eq!(batch.schema().field(0).name(), "id");
        assert_eq!(batch.schema().field(1).name(), "value");
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    async fn step_06_append_batch() {
        let path = temp_dir("step06");
        write_initial_dataset(&path, make_events_batch(100, 17)).await.unwrap();
        let new_batch = make_events_batch(50, 19);
        let total = append_batch(&path, new_batch).await.unwrap();
        assert_eq!(total, 150);
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    async fn step_07_create_index() {
        let path = temp_dir("step07");
        write_initial_dataset(&path, make_events_batch(200, 23)).await.unwrap();
        create_index_on_id(&path).await.unwrap();
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    async fn step_08_version() {
        let path = temp_dir("step08");
        write_initial_dataset(&path, make_events_batch(10, 29)).await.unwrap();
        let v = current_version(&path).await.unwrap();
        assert_eq!(v, 1, "first version should be 1");
    }
}
