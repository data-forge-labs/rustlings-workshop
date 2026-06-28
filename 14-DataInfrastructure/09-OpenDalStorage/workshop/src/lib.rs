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
    let op = Operator::new(services::Fs::default().root(root))?
        .finish();
    Ok(op)
}

/// Create an in-memory operator (great for tests — no disk I/O).
pub fn operator_memory() -> Result<Operator> {
    let op = Operator::new(services::Memory::default())?
        .finish();
    Ok(op)
}

/// Create a MinIO / S3-compatible operator.
///
/// # Arguments
/// * `endpoint` — e.g. `"http://localhost:9000"`
/// * `bucket` — e.g. `"my-bucket"`
/// * `access_key` — MinIO root user (e.g. `"minioadmin"`)
/// * `secret_key` — MinIO root password (e.g. `"minioadmin"`)
pub fn operator_s3(endpoint: &str, bucket: &str, access_key: &str, secret_key: &str) -> Result<Operator> {
    let op = Operator::new(
        services::S3::default()
            .bucket(bucket)
            .region("us-east-1")
            .endpoint(endpoint)
            .access_key_id(access_key)
            .secret_access_key(secret_key)
            .allow_anonymous(),
    )?
    .finish();
    Ok(op)
}

// =========================================================================
// Step 2 — Basic CRUD operations
// =========================================================================

/// Write `data` to `path` using the given operator.
///
/// Returns the number of bytes written (from metadata content_length).
pub async fn write_file(op: &Operator, path: &str, data: Vec<u8>) -> Result<u64> {
    let meta = op.write(path, data).await?;
    Ok(meta.content_length())
}

/// Read the entire file at `path` into a byte vector.
pub async fn read_file(op: &Operator, path: &str) -> Result<Vec<u8>> {
    let data = op.read(path).await?;
    Ok(data.to_vec())
}

// =========================================================================
// Step 3 — List and stat
// =========================================================================

/// List all entry paths under `prefix` (non-recursive).
///
/// Returns a sorted list of paths (just the path strings).
pub async fn list_dir(op: &Operator, prefix: &str) -> Result<Vec<String>> {
    let entries = op.list(prefix).await?;
    let mut paths: Vec<String> = entries
        .iter()
        .filter(|e| !e.metadata().is_dir())
        .map(|e| e.path().to_string())
        .collect();
    paths.sort();
    Ok(paths)
}

/// Get metadata for a file: returns (content_length, is_file, is_dir, etag).
///
/// `etag` is `None` if the backend doesn't provide one.
pub async fn stat_file(op: &Operator, path: &str) -> Result<(u64, bool, bool, Option<String>)> {
    let meta = op.stat(path).await?;
    Ok((
        meta.content_length(),
        meta.is_file(),
        meta.is_dir(),
        meta.etag().map(|s| s.to_string()),
    ))
}

// =========================================================================
// Step 4 — Copy, rename, delete
// =========================================================================

/// Copy a file from `src` to `dst` within the same operator.
pub async fn copy_file(op: &Operator, src: &str, dst: &str) -> Result<()> {
    // Try native copy first; fall back to read+write for backends that don't support it
    match op.copy(src, dst).await {
        Ok(_) => Ok(()),
        Err(_) => {
            let data = op.read(src).await?;
            op.write(dst, data).await?;
            Ok(())
        }
    }
}

/// Delete a file. Succeeds silently if the file does not exist.
pub async fn delete_file(op: &Operator, path: &str) -> Result<()> {
    op.delete(path).await?;
    Ok(())
}

// =========================================================================
// Step 5 — Composable layers
// =========================================================================

/// Wrap an operator with a retry layer (3 retries, 1s interval).
///
/// This is critical for production pipelines hitting remote storage
/// (S3, GCS) where transient failures are common.
pub fn with_retry_layer(op: Operator) -> Operator {
    op.layer(RetryLayer::default())
}

/// Wrap an operator with a logging layer.
///
/// Every read/write/list/delete operation will emit a log event.
pub fn with_logging_layer(op: Operator) -> Operator {
    op.layer(LoggingLayer::default())
}

/// Wrap an operator with a metrics layer.
///
/// Tracks operation counts, latencies, and bytes transferred.
/// In production, pair with a Prometheus exporter to monitor pipeline health.
pub fn with_metrics_layer(op: Operator) -> Operator {
    op.layer(MetricsLayer::default())
}

/// Build a fully layered operator: retry → logging → metrics.
///
/// Layer order matters: outer layers wrap inner ones.
/// The call chain is: metrics → logging → retry → service.
pub fn layered_operator(op: Operator) -> Operator {
    op.layer(RetryLayer::default())
        .layer(LoggingLayer::default())
        .layer(MetricsLayer::default())
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
    let entries = src_op.list(src_prefix).await?;
    let mut count = 0;
    for entry in &entries {
        // Skip directory entries (some backends include them in list results)
        if entry.metadata().is_dir() {
            continue;
        }
        let src_path = entry.path();
        let data = src_op.read(src_path).await?;
        let dst_path = format!("{}{}", dst_prefix, &src_path[src_prefix.len()..]);
        dst_op.write(&dst_path, data).await?;
        count += 1;
    }
    Ok(count)
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
