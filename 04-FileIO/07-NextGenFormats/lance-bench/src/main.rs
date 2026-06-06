use lance_bench::{count_rows, make_events_batch, write_initial_dataset};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let path = "demo_dataset.lance";
    if std::path::Path::new(path).exists() {
        std::fs::remove_dir_all(path).ok();
    }
    std::fs::create_dir_all(path)?;

    let batch = make_events_batch(1000, 42);
    let uri = write_initial_dataset(path, batch).await?;
    println!("Wrote dataset to {}", uri);

    let n = count_rows(path).await?;
    println!("Row count: {}", n);

    Ok(())
}
