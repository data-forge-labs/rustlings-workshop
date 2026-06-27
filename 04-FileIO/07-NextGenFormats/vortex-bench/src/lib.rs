//! Vortex Bench — A working Vortex file with cascading compression
//!
//! Test-driven: every function starts as `todo!()`. Implement step by step.
//!
//! Steps:
//!   01 — Build a Vortex array from Arrow and write to disk
//!   02 — Open the file and read it back into a Vortex array
//!   03 — Convert Vortex → Arrow (zero-copy)
//!   04 — Inspect the file structure (footer, layout, segments)
//!   05 — Apply a compression strategy (cascading compression)
//!   06 — Project a single column from the layout
//!   07 — Compute sum/filter on encoded data
//!   08 — Save & restore with custom session

use std::sync::Arc;

use arrow_array::{Float32Array, Int32Array, RecordBatch, StringArray};
use arrow_schema::{DataType, Field, Schema};
use rand::seq::SliceRandom;
use rand::SeedableRng;

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub fn build_events_schema() -> Arc<Schema> {
    Arc::new(Schema::new(vec![
        Field::new("id", DataType::Int32, false),
        Field::new("name", DataType::Utf8, true),
        Field::new("value", DataType::Float32, true),
        Field::new("date", DataType::Utf8, false),
    ]))
}

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

// =============================================================================
// Step 01: build a Vortex array from Arrow and write to disk
// =============================================================================
/// Convert an Arrow RecordBatch into a Vortex `ArrayRef` (StructArray) and
/// write it to a Vortex file at `path`. Returns the file size in bytes.
pub async fn write_vortex_file(path: &str, batch: RecordBatch) -> Result<u64> {
    use vortex::array::arrow::FromArrowArray;
    use vortex::file::VortexWriteOptions;
    use vortex::session::VortexSession;
    use vortex::VortexSessionDefault;

    let session = VortexSession::default();
    let vp_array = vortex::array::ArrayRef::from_arrow(batch.clone(), false)?;
    let file = tokio::fs::File::create(path).await?;
    VortexWriteOptions::new(session)
        .write(file, vp_array.to_array_stream())
        .await?;
    Ok(std::fs::metadata(path)?.len())
}

pub async fn read_vortex_row_count(path: &str) -> Result<usize> {
    use vortex::file::OpenOptionsSessionExt;
    use vortex::session::VortexSession;
    use vortex::VortexSessionDefault;

    let session = VortexSession::default();
    let vf = session.open_options().open_path(path).await?;
    Ok(vf.scan()?.into_array_stream()?.dtype().len())
}

pub async fn vortex_to_arrow(path: &str) -> Result<RecordBatch> {
    use arrow_array::StructArray;
    use arrow_schema::Schema;
    use vortex::array::arrow::IntoArrowArray;
    use vortex::file::OpenOptionsSessionExt;
    use vortex::session::VortexSession;
    use vortex::VortexSessionDefault;
    use futures::StreamExt;

    let session = VortexSession::default();
    let vf = session.open_options().open_path(path).await?;
    let stream = vf.scan()?.into_array_stream()?;
    tokio::pin!(stream);
    if let Some(chunk) = stream.next().await {
        #[allow(deprecated)]
        let arrow_arr = chunk?.into_arrow_preferred()?;
        let struct_arr = arrow_arr.as_any().downcast_ref::<StructArray>()
            .ok_or("expected StructArray")?;
        let schema = Schema::new(struct_arr.fields().to_vec());
        Ok(RecordBatch::try_new(Arc::new(schema), struct_arr.columns().to_vec())?)
    } else {
        Err("empty stream".into())
    }
}

pub fn inspect_vortex_structure(path: &str) -> Result<String> {
    let metadata = std::fs::metadata(path)?;
    let file_size = metadata.len();
    let bytes = std::fs::read(path)?;
    let segment_count = bytes.windows(4).filter(|w| w == b"VTX\0").count().max(1);
    Ok(format!("Vortex file: {} bytes, ~{} segments", file_size, segment_count))
}

pub async fn write_with_cascading_compression(path: &str, batch: RecordBatch) -> Result<u64> {
    use vortex::array::arrow::FromArrowArray;
    use vortex::compressor::CompactCompressor;
    use vortex::file::VortexWriteOptions;
    use vortex::session::VortexSession;
    use vortex::VortexSessionDefault;

    let session = VortexSession::default();
    let vp_array = vortex::array::ArrayRef::from_arrow(batch.clone(), false)?;
    let compressed = CompactCompressor::new(&session).compress(&vp_array)?;
    let file = tokio::fs::File::create(path).await?;
    VortexWriteOptions::new(session)
        .write(file, compressed.to_array_stream())
        .await?;
    Ok(std::fs::metadata(path)?.len())
}

pub async fn project_value_column(path: &str) -> Result<Vec<f32>> {
    use vortex::file::OpenOptionsSessionExt;
    use vortex::session::VortexSession;
    use vortex::VortexSessionDefault;
    use futures::StreamExt;

    let session = VortexSession::default();
    let vf = session.open_options().open_path(path).await?;
    let stream = vf.scan()?.project(&["value"])?.into_array_stream()?;
    tokio::pin!(stream);
    let mut all_values = Vec::new();
    while let Some(chunk) = stream.next().await {
        let chunk = chunk?;
        use arrow_array::cast::AsArray;
        #[allow(deprecated)]
        let arrow_arr = chunk.into_arrow_preferred()?;
        let struct_arr = arrow_arr.as_any().downcast_ref::<arrow_array::StructArray>().unwrap();
        let float_arr = struct_arr.column_by_name("value").unwrap().as_primitive::<arrow_array::types::Float32Type>();
        for i in 0..float_arr.len() {
            all_values.push(float_arr.value(i));
        }
    }
    Ok(all_values)
}

pub async fn sum_id_column(path: &str) -> Result<i64> {
    use vortex::file::OpenOptionsSessionExt;
    use vortex::session::VortexSession;
    use vortex::VortexSessionDefault;
    use futures::StreamExt;

    let session = VortexSession::default();
    let vf = session.open_options().open_path(path).await?;
    let stream = vf.scan()?.project(&["id"])?.into_array_stream()?;
    tokio::pin!(stream);
    let mut sum: i64 = 0;
    while let Some(chunk) = stream.next().await {
        let chunk = chunk?;
        use arrow_array::cast::AsArray;
        #[allow(deprecated)]
        let arrow_arr = chunk.into_arrow_preferred()?;
        let struct_arr = arrow_arr.as_any().downcast_ref::<arrow_array::StructArray>().unwrap();
        let int_arr = struct_arr.column_by_name("id").unwrap().as_primitive::<arrow_array::types::Int32Type>();
        for i in 0..int_arr.len() {
            sum += int_arr.value(i) as i64;
        }
    }
    Ok(sum)
}

pub async fn list_field_names(path: &str) -> Result<Vec<String>> {
    use vortex::file::OpenOptionsSessionExt;
    use vortex::session::VortexSession;
    use vortex::VortexSessionDefault;

    let session = VortexSession::default();
    let vf = session.open_options().open_path(path).await?;
    let dtype = vf.scan()?.into_array_stream()?.dtype();
    match dtype {
        vortex::dtype::DType::Struct(fields, _) => {
            Ok(fields.iter().map(|(name, _)| name.to_string()).collect())
        }
        _ => Err("expected struct dtype".into()),
    }
}

// =============================================================================
// Step 02: open a Vortex file and read back the array
// =============================================================================
/// Open the Vortex file at `path` and return the total row count.
pub async fn read_vortex_row_count(path: &str) -> Result<usize> {
    todo!("Step 02: open VTX file, return layout.row_count()")
}

// =============================================================================
// Step 03: convert Vortex → Arrow (zero-copy)
// =============================================================================
/// Read the Vortex file and convert the first chunk to an Arrow RecordBatch.
pub async fn vortex_to_arrow(path: &str) -> Result<RecordBatch> {
    todo!("Step 03: read Vortex, call `.into_arrow()` on the array")
}

// =============================================================================
// Step 04: inspect the file structure
// =============================================================================
/// Read the file, return a human-readable summary of the on-disk structure.
pub fn inspect_vortex_structure(path: &str) -> Result<String> {
    todo!("Step 04: read footer, return string with file size + segment count")
}

// =============================================================================
// Step 05: compression strategy
// =============================================================================
/// Write `batch` to `path` using `vortex::compressor::CompactCompressor` (cascading).
pub async fn write_with_cascading_compression(path: &str, batch: RecordBatch) -> Result<u64> {
    todo!("Step 05: use CompactCompressor for sample-aware encoding selection")
}

// =============================================================================
// Step 06: project a single column
// =============================================================================
/// Read the Vortex file, return the `value` column as a Vec<f32>.
pub async fn project_value_column(path: &str) -> Result<Vec<f32>> {
    todo!("Step 06: use scan builder with projection=['value']")
}

// =============================================================================
// Step 07: compute sum on encoded data
// =============================================================================
/// Read the Vortex file, sum the `id` column, return the result.
pub async fn sum_id_column(path: &str) -> Result<i64> {
    todo!("Step 07: scan + execute sum(id)")
}

// =============================================================================
// Step 08: list fields in the file
// =============================================================================
/// Open the Vortex file, return the field names in the schema.
pub async fn list_field_names(path: &str) -> Result<Vec<String>> {
    todo!("Step 08: read dtype, return field names")
}

// =============================================================================
// Tests
// =============================================================================
#[cfg(test)]
mod tests {
    use super::*;

    fn temp_dir(tag: &str) -> String {
        let dir = tempfile::tempdir().unwrap();
        let p = dir.path().to_path_buf();
        std::mem::forget(dir);
        let path = p.join(tag);
        std::fs::create_dir_all(&path).unwrap();
        path.to_string_lossy().to_string()
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    async fn step_01_write_vortex() {
        let dir = temp_dir("step01");
        let path = format!("{}/data.vortex", dir);
        let batch = make_events_batch(100, 42);
        let size = write_vortex_file(&path, batch).await.unwrap();
        assert!(size > 0, "file should be non-empty");
        assert!(std::path::Path::new(&path).exists());
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    async fn step_02_read_row_count() {
        let dir = temp_dir("step02");
        let path = format!("{}/data.vortex", dir);
        write_vortex_file(&path, make_events_batch(500, 1)).await.unwrap();
        let n = read_vortex_row_count(&path).await.unwrap();
        assert_eq!(n, 500);
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    async fn step_03_vortex_to_arrow() {
        let dir = temp_dir("step03");
        let path = format!("{}/data.vortex", dir);
        write_vortex_file(&path, make_events_batch(50, 7)).await.unwrap();
        let batch = vortex_to_arrow(&path).await.unwrap();
        assert_eq!(batch.num_rows(), 50);
        assert_eq!(batch.num_columns(), 4);
    }

    #[test]
    fn step_04_inspect_structure() {
        let dir = temp_dir("step04");
        let path = format!("{}/data.vortex", dir);
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async {
            write_vortex_file(&path, make_events_batch(100, 9)).await.unwrap();
        });
        let summary = inspect_vortex_structure(&path).unwrap();
        assert!(!summary.is_empty());
        eprintln!("Vortex structure: {}", summary);
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    async fn step_05_cascading_compression() {
        let dir = temp_dir("step05");
        let path = format!("{}/data.vortex", dir);
        let size = write_with_cascading_compression(&path, make_events_batch(200, 11))
            .await
            .unwrap();
        assert!(size > 0);
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    async fn step_06_project_column() {
        let dir = temp_dir("step06");
        let path = format!("{}/data.vortex", dir);
        write_vortex_file(&path, make_events_batch(50, 13)).await.unwrap();
        let values = project_value_column(&path).await.unwrap();
        assert_eq!(values.len(), 50);
        // First value should be ~0.123
        assert!((values[0] - 0.123).abs() < 0.01);
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    async fn step_07_sum() {
        let dir = temp_dir("step07");
        let path = format!("{}/data.vortex", dir);
        write_vortex_file(&path, make_events_batch(100, 17)).await.unwrap();
        let s = sum_id_column(&path).await.unwrap();
        // sum 0..100 = 4950
        assert_eq!(s, 4950);
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    async fn step_08_field_names() {
        let dir = temp_dir("step08");
        let path = format!("{}/data.vortex", dir);
        write_vortex_file(&path, make_events_batch(20, 19)).await.unwrap();
        let names = list_field_names(&path).await.unwrap();
        assert_eq!(names, vec!["id", "name", "value", "date"]);
    }
}
