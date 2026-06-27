use polars::prelude::*;

pub fn load_sales_csv(path: &str) -> Result<DataFrame, PolarsError> {
    CsvReadOptions::default()
        .try_into_reader_with_file_path(Some(path.into()))?
        .finish()
}

pub fn total_units(sales: &DataFrame) -> Result<i64, PolarsError> {
    sales.column("units")?.sum()
}

pub fn total_revenue(sales: &DataFrame) -> Result<f64, PolarsError> {
    let units = sales.column("units")?.f64()?;
    let amount = sales.column("amount")?.f64()?;
    let revenue: f64 = units.into_iter().zip(amount.into_iter())
        .map(|(u, a)| u.unwrap_or(0.0) * a.unwrap_or(0.0))
        .sum();
    Ok(revenue)
}

pub fn filter_expensive(sales: &DataFrame, min_amount: f64) -> Result<DataFrame, PolarsError> {
    let mask = sales.column("amount")?.f64()?.gt(min_amount);
    sales.filter(&mask)
}

pub fn revenue_per_product(sales: &DataFrame) -> Result<DataFrame, PolarsError> {
    let units = sales.column("units")?.cast(&DataType::Float64)?;
    let amount = sales.column("amount")?.f64()?;
    let revenue: Series = units.f64()?.into_iter().zip(amount.into_iter())
        .map(|(u, a)| u.unwrap_or(0.0) * a.unwrap_or(0.0))
        .collect();
    let df = DataFrame::new(vec![
        sales.column("name")?.clone(),
        revenue.into(),
    ])?;
    df.group_by(["name"])?.agg(&[(&"revenue", &["sum"])]?.into())
}

pub fn high_revenue_products(sales: &DataFrame, min_revenue: f64) -> Result<DataFrame, PolarsError> {
    let result = revenue_per_product(sales)?;
    let mask = result.column("revenue_sum")?.f64()?.gt(min_revenue);
    result.filter(&mask)
}

pub fn write_parquet(df: &DataFrame, path: &str) -> Result<(), PolarsError> {
    let file = std::fs::File::create(path)?;
    ParquetWriter::new(file).finish(df)?;
    Ok(())
}

pub fn read_parquet(path: &str) -> Result<DataFrame, PolarsError> {
    ParquetReader::new(std::fs::File::open(path)?).finish()
}

pub fn lazy_filter_expensive(min_amount: f64) -> Result<DataFrame, PolarsError> {
    LazyFrame::scan_parquet("data/sales.parquet", Default::default())?
        .filter(col("amount").gt(lit(min_amount)))
        .collect()
}

pub fn lazy_group_by_total() -> Result<DataFrame, PolarsError> {
    LazyFrame::scan_parquet("data/sales.parquet", Default::default())?
        .group_by(["name"])
        .agg([col("units").sum().alias("total_units")])
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    mod step_01_load {
        use super::*;

        #[test]
        fn test_load_sales_csv() {
            let df = load_sales_csv("data/sales.csv").unwrap();
            assert_eq!(df.height(), 10);
            assert_eq!(df.width(), 4);
        }

        #[test]
        fn test_load_sales_csv_columns() {
            let df = load_sales_csv("data/sales.csv").unwrap();
            let names: Vec<String> = df.get_column_names().into_iter().map(|s| s.to_string()).collect();
            assert_eq!(names, vec!["id", "name", "amount", "units"]);
        }
    }

    mod step_02_aggregations {
        use super::*;

        #[test]
        fn test_total_units() {
            let df = load_sales_csv("data/sales.csv").unwrap();
            assert_eq!(total_units(&df).unwrap(), 100 + 50 + 25 + 10 + 200 + 40 + 80 + 30 + 60 + 150);
        }

        #[test]
        fn test_total_revenue() {
            let df = load_sales_csv("data/sales.csv").unwrap();
            let revenue = total_revenue(&df).unwrap();
            let expected = 1.50 * 100.0 + 2.25 * 50.0 + 3.99 * 25.0
                + 5.49 * 10.0 + 2.99 * 200.0 + 4.49 * 40.0
                + 1.99 * 80.0 + 8.99 * 30.0 + 3.49 * 60.0 + 2.19 * 150.0;
            assert!((revenue - expected).abs() < 1e-6);
        }
    }

    mod step_03_filter_select {
        use super::*;

        #[test]
        fn test_filter_expensive_keeps_only_high_amount() {
            let df = load_sales_csv("data/sales.csv").unwrap();
            let filtered = filter_expensive(&df, 5.0).unwrap();
            assert_eq!(filtered.height(), 2);
            let names: Vec<String> = filtered
                .column("name")
                .unwrap()
                .str()
                .unwrap()
                .into_iter()
                .map(|s| s.unwrap().to_string())
                .collect();
            assert!(names.contains(&"Cheese".to_string()));
            assert!(names.contains(&"Coffee".to_string()));
        }
    }

    mod step_04_group_by {
        use super::*;

        #[test]
        fn test_revenue_per_product() {
            let df = load_sales_csv("data/sales.csv").unwrap();
            let result = revenue_per_product(&df).unwrap();
            assert!(result.height() > 0);
            let names: Vec<String> = result.get_column_names().into_iter().map(|s| s.to_string()).collect();
            assert!(names.contains(&"name".to_string()));
            assert!(names.contains(&"revenue".to_string()));
        }

        #[test]
        fn test_high_revenue_products_threshold() {
            let df = load_sales_csv("data/sales.csv").unwrap();
            let result = high_revenue_products(&df, 100.0).unwrap();
            assert!(result.height() >= 1);
        }
    }

    mod step_05_parquet {
        use super::*;
        use std::fs;

        #[test]
        fn test_parquet_roundtrip() {
            let df = load_sales_csv("data/sales.csv").unwrap();
            let tmp = std::env::temp_dir().join("polars_test_roundtrip.parquet");
            write_parquet(&df, tmp.to_str().unwrap()).unwrap();
            let back = read_parquet(tmp.to_str().unwrap()).unwrap();
            assert_eq!(back.height(), 10);
            assert_eq!(back.width(), 4);
            let _ = fs::remove_file(&tmp);
        }
    }

    mod step_06_lazy {
        use super::*;

        #[test]
        fn test_lazy_filter_expensive() {
            let df = lazy_filter_expensive(5.0).unwrap();
            assert!(df.height() >= 1);
        }

        #[test]
        fn test_lazy_group_by_total() {
            let df = lazy_group_by_total().unwrap();
            assert!(df.height() > 0);
        }
    }
}
