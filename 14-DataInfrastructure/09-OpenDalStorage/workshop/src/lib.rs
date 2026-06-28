//! OpenDAL storage: unified storage abstraction for data pipelines.
//!
//! OpenDAL provides one API to access 50+ storage backends (local FS,
//! S3, GCS, Azure Blob, etc.). This module exposes helper functions
//! that wrap the OpenDAL `Operator` for common data-engineering tasks:
//! reading, writing, listing, copying, and cross-backend pipelines.

use anyhow::{Context, Result};
use opendal::{layers::*, services, Metadata, Operator};

// =========================================================================
// Step 1 — Operator builders
// =========================================================================

/// Create a local filesystem operator rooted at `root`.
///
/// The operator reads/writes files relative to `root`. For example,
/// if `root` is `"/tmp/data"`, writing to `"file.csv"` creates
/// `"/tmp/data/file.csv"`.
pub fn operator_fs(root: &str) -> Result<Operator> {
    todo!("Create and return a filesystem Operator rooted at `root`")
}

/// Create an in-memory operator (great for tests — no disk I/O).
pub fn operator_memory() -> Result<Operator> {
    todo!("Create and return an in-memory Operator")
}

/// Create a MinIO / S3-compatible operator.
///
/// # Arguments
/// * `endpoint` — e.g. `"http://localhost:9000"`
/// * `bucket` — e.g. `"my-bucket"`
/// * `access_key` — MinIO root user (e.g. `"minioadmin"`)
/// * `secret_key` — MinIO root password (e.g. `"minioadmin"`)
pub fn operator_s3(endpoint: &str, bucket: &str, access_key: &str, secret_key: &str) -> Result<Operator> {
    todo!("Create and return an S3 operator using services::S3 builder")
}

// =========================================================================
// Step 2 — Basic CRUD operations
// =========================================================================

/// Write `data` to `path` using the given operator.
///
/// Returns the number of bytes written (from metadata content_length).
pub async fn write_file(op: &Operator, path: &str, data: Vec<u8>) -> Result<u64> {
    todo!("Write data to path using op.write(), return metadata content_length")
}

/// Read the entire file at `path` into a byte vector.
pub async fn read_file(op: &Operator, path: &str) -> Result<Vec<u8>> {
    todo!("Read path using op.read(), convert Buffer to Vec<u8>")
}

// =========================================================================
// Step 3 — List and stat
// =========================================================================

/// List all entry paths under `prefix` (non-recursive).
///
/// Returns a sorted list of paths (just the path strings).
pub async fn list_dir(op: &Operator, prefix: &str) -> Result<Vec<String>> {
    todo!("Use op.list(prefix) and collect entry.path() into a sorted Vec")
}

/// Get metadata for a file: returns (content_length, is_file, is_dir, etag).
///
/// `etag` is `None` if the backend doesn't provide one.
pub async fn stat_file(op: &Operator, path: &str) -> Result<(u64, bool, bool, Option<String>)> {
    todo!("Use op.stat(path) to get metadata, return the four fields")
}

// =========================================================================
// Step 4 — Copy, rename, delete
// =========================================================================

/// Copy a file from `src` to `dst` within the same operator.
pub async fn copy_file(op: &Operator, src: &str, dst: &str) -> Result<()> {
    todo!("Use op.copy(src, dst)")
}

/// Delete a file. Succeeds silently if the file does not exist.
pub async fn delete_file(op: &Operator, path: &str) -> Result<()> {
    todo!("Use op.delete(path)")
}

// =========================================================================
// Step 5 — Composable layers
// =========================================================================

/// Wrap an operator with a retry layer (3 retries, 1s interval).
///
/// This is critical for production pipelines hitting remote storage
/// (S3, GCS) where transient failures are common.
pub fn with_retry_layer(op: Operator) -> Operator {
    todo!("Add RetryLayer with 3 retries and 1s interval")
}

/// Wrap an operator with a logging layer.
///
/// Every read/write/list/delete operation will emit a log event.
pub fn with_logging_layer(op: Operator) -> Operator {
    todo!("Add LoggingLayer to the operator")
}

/// Wrap an operator with a metrics layer.
///
/// Tracks operation counts, latencies, and bytes transferred.
/// In production, pair with a Prometheus exporter to monitor pipeline health.
pub fn with_metrics_layer(op: Operator) -> Operator {
    todo!("Add MetricsLayer to the operator")
}

/// Build a fully layered operator: retry → logging → metrics.
///
/// Layer order matters: outer layers wrap inner ones.
/// The call chain is: metrics → logging → retry → service.
pub fn layered_operator(op: Operator) -> Operator {
    todo!("Apply all three layers in the correct order")
}

// =========================================================================
// Step 6 — Cross-backend pipeline
// =========================================================================

/// Copy all files from a source operator/prefix to a destination operator/prefix.
///
/// This is the core of storage-agnostic pipelines: read from local FS,
/// write to S3 (or vice versa) with the same code.
///
/// Returns the number of files copied.
pub async fn pipeline_copy(
    src_op: &Operator,
    src_prefix: &str,
    dst_op: &Operator,
    dst_prefix: &str,
) -> Result<usize> {
    todo!(
        "List files in src_prefix, read each, write to dst_prefix, return count"
    )
}

// =========================================================================
// Tests
// =========================================================================

#[cfg(test)]
mod tests {
    use super::*;

    // -----------------------------------------------------------------
    // Step 1 — Operator builders
    // -----------------------------------------------------------------
    mod step_01_operator_builders {
        use super::*;

        #[tokio::test]
        async fn test_operator_memory_creation() {
            let op = operator_memory().unwrap();
            // Write and read back to verify the operator works
            op.write("test.txt", b"hello".to_vec()).await.unwrap();
            let data = op.read("test.txt").await.unwrap();
            assert_eq!(data.to_vec(), b"hello".to_vec());
        }

        #[tokio::test]
        async fn test_operator_fs_creation() {
            let dir = std::env::temp_dir().join("opendal_test_fs");
            std::fs::create_dir_all(&dir).unwrap();
            let op = operator_fs(dir.to_str().unwrap()).unwrap();
            op.write("probe.txt", b"ok".to_vec()).await.unwrap();
            let data = op.read("probe.txt").await.unwrap();
            assert_eq!(data.to_vec(), b"ok".to_vec());
            std::fs::remove_dir_all(&dir).ok();
        }
    }

    // -----------------------------------------------------------------
    // Step 2 — Basic write and read
    // -----------------------------------------------------------------
    mod step_02_write_read {
        use super::*;

        #[tokio::test]
        async fn test_write_returns_byte_count() {
            let op = operator_memory().unwrap();
            let n = write_file(&op, "data.csv", b"name,age\nAlice,30".to_vec()).await.unwrap();
            assert!(n > 0, "content_length should be > 0");
        }

        #[tokio::test]
        async fn test_read_returns_written_data() {
            let op = operator_memory().unwrap();
            write_file(&op, "data.csv", b"name,age\nBob,25".to_vec()).await.unwrap();
            let data = read_file(&op, "data.csv").await.unwrap();
            assert_eq!(data, b"name,age\nBob,25".to_vec());
        }

        #[tokio::test]
        async fn test_read_nonexistent_file() {
            let op = operator_memory().unwrap();
            let result = read_file(&op, "no_such_file.txt").await;
            assert!(result.is_err(), "reading a missing file should fail");
        }
    }

    // -----------------------------------------------------------------
    // Step 3 — List and stat
    // -----------------------------------------------------------------
    mod step_03_list_stat {
        use super::*;

        #[tokio::test]
        async fn test_list_empty_directory() {
            let op = operator_memory().unwrap();
            op.create_dir("empty/").await.unwrap();
            let entries = list_dir(&op, "empty/").await.unwrap();
            assert!(entries.is_empty(), "empty dir should have no entries");
        }

        #[tokio::test]
        async fn test_list_with_files() {
            let op = operator_memory().unwrap();
            op.write("dir/a.txt", b"1".to_vec()).await.unwrap();
            op.write("dir/b.txt", b"2".to_vec()).await.unwrap();
            op.write("dir/c.txt", b"3".to_vec()).await.unwrap();
            let mut entries = list_dir(&op, "dir/").await.unwrap();
            entries.sort();
            assert_eq!(entries, vec!["dir/a.txt", "dir/b.txt", "dir/c.txt"]);
        }

        #[tokio::test]
        async fn test_stat_file() {
            let op = operator_memory().unwrap();
            op.write("stat_me.txt", b"hello world".to_vec()).await.unwrap();
            let (len, is_file, is_dir, _etag) = stat_file(&op, "stat_me.txt").await.unwrap();
            assert_eq!(len, 11);
            assert!(is_file);
            assert!(!is_dir);
        }
    }

    // -----------------------------------------------------------------
    // Step 4 — Copy, rename, delete
    // -----------------------------------------------------------------
    mod step_04_copy_delete {
        use super::*;

        #[tokio::test]
        async fn test_copy_file() {
            let op = operator_memory().unwrap();
            op.write("src.txt", b"copy me".to_vec()).await.unwrap();
            copy_file(&op, "src.txt", "dst.txt").await.unwrap();
            let data = read_file(&op, "dst.txt").await.unwrap();
            assert_eq!(data, b"copy me".to_vec());
        }

        #[tokio::test]
        async fn test_delete_file() {
            let op = operator_memory().unwrap();
            op.write("to_delete.txt", b"bye".to_vec()).await.unwrap();
            delete_file(&op, "to_delete.txt").await.unwrap();
            assert!(!op.exists("to_delete.txt").await.unwrap());
        }

        #[tokio::test]
        async fn test_delete_nonexistent_is_ok() {
            let op = operator_memory().unwrap();
            // Should not error
            delete_file(&op, "ghost.txt").await.unwrap();
        }
    }

    // -----------------------------------------------------------------
    // Step 5 — Composable layers
    // -----------------------------------------------------------------
    mod step_05_layers {
        use super::*;

        #[tokio::test]
        async fn test_retry_layer_does_not_break_read_write() {
            let op = operator_memory().unwrap();
            let op = with_retry_layer(op);
            op.write("layered.txt", b"retry test".to_vec()).await.unwrap();
            let data = op.read("layered.txt").await.unwrap();
            assert_eq!(data.to_vec(), b"retry test".to_vec());
        }

        #[tokio::test]
        async fn test_logging_layer_does_not_break_read_write() {
            let op = operator_memory().unwrap();
            let op = with_logging_layer(op);
            op.write("logged.txt", b"log me".to_vec()).await.unwrap();
            let data = op.read("logged.txt").await.unwrap();
            assert_eq!(data.to_vec(), b"log me".to_vec());
        }

        #[tokio::test]
        async fn test_metrics_layer_does_not_break_read_write() {
            let op = operator_memory().unwrap();
            let op = with_metrics_layer(op);
            op.write("metrics.txt", b"count me".to_vec()).await.unwrap();
            let data = op.read("metrics.txt").await.unwrap();
            assert_eq!(data.to_vec(), b"count me".to_vec());
        }

        #[tokio::test]
        async fn test_all_layers_combined() {
            let op = operator_memory().unwrap();
            let op = layered_operator(op);
            op.write("full.txt", b"all layers".to_vec()).await.unwrap();
            let data = op.read("full.txt").await.unwrap();
            assert_eq!(data.to_vec(), b"all layers".to_vec());
        }
    }

    // -----------------------------------------------------------------
    // Step 6 — Cross-backend pipeline
    // -----------------------------------------------------------------
    mod step_06_pipeline {
        use super::*;

        #[tokio::test]
        async fn test_pipeline_memory_to_memory() {
            let src = operator_memory().unwrap();
            let dst = operator_memory().unwrap();

            // Populate source
            src.write("pipeline/a.csv", b"col1,col2".to_vec()).await.unwrap();
            src.write("pipeline/b.csv", b"1,2".to_vec()).await.unwrap();

            let count = pipeline_copy(&src, "pipeline/", &dst, "out/").await.unwrap();
            assert_eq!(count, 2);

            // Verify destination
            let a = dst.read("out/a.csv").await.unwrap();
            assert_eq!(a.to_vec(), b"col1,col2".to_vec());
            let b = dst.read("out/b.csv").await.unwrap();
            assert_eq!(b.to_vec(), b"1,2".to_vec());
        }

        #[tokio::test]
        async fn test_pipeline_fs_to_memory() {
            let dir = std::env::temp_dir().join("opendal_pipeline_fs");
            std::fs::create_dir_all(&dir.join("src")).unwrap();
            std::fs::write(dir.join("src/data.csv"), b"fs,data".to_vec()).unwrap();

            let src = operator_fs(dir.to_str().unwrap()).unwrap();
            let dst = operator_memory().unwrap();

            let count = pipeline_copy(&src, "src/", &dst, "dst/").await.unwrap();
            assert_eq!(count, 1);

            let data = dst.read("dst/data.csv").await.unwrap();
            assert_eq!(data.to_vec(), b"fs,data".to_vec());

            std::fs::remove_dir_all(&dir).ok();
        }
    }
}
