use polars_workshop::{
    filter_expensive, load_sales_csv, revenue_per_product, total_revenue, total_units,
    write_parquet,
};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let df = load_sales_csv("data/sales.csv")?;
    println!("Loaded {} rows × {} columns", df.height(), df.width());

    println!("Total units sold: {}", total_units(&df)?);
    println!("Total revenue: ${:.2}", total_revenue(&df)?);

    let expensive = filter_expensive(&df, 5.0)?;
    println!("Items costing $5+: {}", expensive.height());

    let by_product = revenue_per_product(&df)?;
    println!("Products by revenue:\n{}", by_product);

    let tmp = std::env::temp_dir().join("sales.parquet");
    write_parquet(&df, tmp.to_str().unwrap())?;
    println!("Wrote Parquet to {}", tmp.display());

    Ok(())
}
