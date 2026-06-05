use datafusion_workshop::{
    count_rows, create_context, names_above_amount, register_csv, run_sql, total_amount,
    write_parquet,
};
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let ctx = create_context().await?;
    register_csv(&ctx, "orders", "data/orders.csv").await?;

    println!("Loaded {} rows", count_rows(&ctx, "orders").await?);
    println!("Total amount: ${:.2}", total_amount(&ctx, "orders").await?);

    let expensive = names_above_amount(&ctx, "orders", 3.0).await?;
    println!("Items > $3.00: {:?}", expensive);

    let batches = run_sql(
        &ctx,
        "SELECT name, amount FROM orders ORDER BY amount DESC LIMIT 3",
    )
    .await?;
    for batch in &batches {
        println!("{:?}", batch);
    }

    let tmp = std::env::temp_dir().join("orders.parquet");
    write_parquet(&ctx, "orders", tmp.to_str().unwrap()).await?;
    println!("Wrote Parquet to {}", tmp.display());

    Ok(())
}
