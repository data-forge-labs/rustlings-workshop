use vortex_bench::{make_events_batch, read_vortex_row_count, write_vortex_file};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let path = "demo_dataset.vortex";
    if std::path::Path::new(path).exists() {
        std::fs::remove_file(path).ok();
    }

    let batch = make_events_batch(1000, 42);
    let size = write_vortex_file(path, batch).await?;
    println!("Wrote Vortex file {} ({} bytes)", path, size);

    let n = read_vortex_row_count(path).await?;
    println!("Row count: {}", n);

    Ok(())
}
