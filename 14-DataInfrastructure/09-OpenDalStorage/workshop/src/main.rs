use anyhow::Result;
use open_dal_storage::*;
use tracing::info;

/// MinIO / S3 configuration (matches docker-compose defaults).
const MINIO_ENDPOINT: &str = "http://localhost:9000";
const MINIO_BUCKET: &str = "dataeng";
const MINIO_ACCESS_KEY: &str = "minioadmin";
const MINIO_SECRET_KEY: &str = "minioadmin";

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize structured logging
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "info".parse().unwrap()),
        )
        .init();

    info!("=== OpenDAL Storage — Unified Storage Abstraction ===");

    // ---------------------------------------------------------------
    // 1. Local FS operator — read source data from disk
    // ---------------------------------------------------------------
    let local_dir = std::env::temp_dir().join("open_dal_demo");
    std::fs::create_dir_all(&local_dir)?;
    std::fs::write(local_dir.join("orders.csv"), b"id,product,qty\n1,laptop,2\n2,phone,5\n3,tablet,1")?;
    std::fs::write(local_dir.join("inventory.csv"), b"product,stock\nlaptop,50\nphone,200\ntablet,75")?;
    info!(path = %local_dir.display(), "Created local source files");

    let local_op = operator_fs(local_dir.to_str().unwrap())?;

    // ---------------------------------------------------------------
    // 2. MinIO operator — write to S3-compatible object storage
    // ---------------------------------------------------------------
    let minio_op = operator_s3(MINIO_ENDPOINT, MINIO_BUCKET, MINIO_ACCESS_KEY, MINIO_SECRET_KEY)?;

    // ---------------------------------------------------------------
    // 3. Pipeline: Local FS → MinIO
    // ---------------------------------------------------------------
    info!("Starting Local FS → MinIO pipeline...");
    let count = pipeline_copy(&local_op, "", &minio_op, "raw/").await?;
    info!(files_copied = count, "Pipeline complete");

    // ---------------------------------------------------------------
    // 4. Verify: list and read back from MinIO
    // ---------------------------------------------------------------
    let entries = list_dir(&minio_op, "raw/").await?;
    info!(entries = ?entries, "Files in MinIO after pipeline");

    for entry in &entries {
        let data = read_file(&minio_op, entry).await?;
        let preview = String::from_utf8_lossy(&data);
        let first_line = preview.lines().next().unwrap_or("(empty)");
        info!(path = entry, first_line, "Preview");
    }

    // ---------------------------------------------------------------
    // 5. Stats
    // ---------------------------------------------------------------
    for entry in &entries {
        let (len, is_file, is_dir, etag) = stat_file(&minio_op, entry).await?;
        info!(path = entry, bytes = len, is_file, is_dir, etag, "Metadata");
    }

    // Cleanup
    std::fs::remove_dir_all(&local_dir).ok();
    info!("Demo complete.");

    Ok(())
}
