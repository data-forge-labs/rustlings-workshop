use parquet_workshop::{read_parquet_file, sales_batch, total_value, write_parquet_file};
use std::error::Error;
use tempfile::NamedTempFile;

fn main() -> Result<(), Box<dyn Error>> {
    let rows = vec![
        ("apple".to_string(), 1.50, 100),
        ("bread".to_string(), 2.25, 50),
        ("milk".to_string(), 3.99, 25),
    ];

    let batch = sales_batch(&rows);
    println!(
        "Built RecordBatch: {} rows × {} columns",
        batch.num_rows(),
        batch.num_columns()
    );

    let tmp = NamedTempFile::new()?;
    write_parquet_file(tmp.path().to_str().unwrap(), &batch)?;
    println!("Wrote Parquet to {}", tmp.path().display());

    let read_back = read_parquet_file(tmp.path().to_str().unwrap())?;
    println!(
        "Read back: {} rows × {} columns",
        read_back.num_rows(),
        read_back.num_columns()
    );

    let records: Vec<parquet_workshop::Record> = rows
        .into_iter()
        .map(|(name, value, count)| parquet_workshop::Record {
            name,
            value,
            count: count as u32,
        })
        .collect();
    println!("Total value: {}", total_value(&records));

    Ok(())
}
