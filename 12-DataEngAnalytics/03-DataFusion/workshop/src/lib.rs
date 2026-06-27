use datafusion::arrow::array::{Array, Float64Array, Int64Array, StringArray};
use datafusion::arrow::record_batch::RecordBatch;
use datafusion::error::Result;
use datafusion::prelude::*;

pub async fn create_context() -> Result<SessionContext> {
    Ok(SessionContext::new())
}

pub async fn register_csv(ctx: &SessionContext, table: &str, path: &str) -> Result<()> {
    ctx.register_csv(table, path, CsvReadOptions::default()).await?;
    Ok(())
}

pub async fn count_rows(ctx: &SessionContext, table: &str) -> Result<i64> {
    let df = ctx.sql(&format!("SELECT COUNT(*) AS n FROM {}", table)).await?;
    let batches = df.collect().await?;
    let batch = &batches[0];
    let n_col = batch.column(0).as_any().downcast_ref::<Int64Array>().unwrap();
    Ok(n_col.value(0))
}

pub async fn total_amount(ctx: &SessionContext, table: &str) -> Result<f64> {
    let df = ctx.sql(&format!("SELECT SUM(amount) AS total FROM {}", table)).await?;
    let batches = df.collect().await?;
    let batch = &batches[0];
    let col = batch.column(0).as_any().downcast_ref::<Float64Array>().unwrap();
    Ok(col.value(0))
}

pub async fn rows_above_amount(ctx: &SessionContext, table: &str, threshold: f64) -> Result<usize> {
    let df = ctx.sql(&format!("SELECT COUNT(*) AS n FROM {} WHERE amount > {}", table, threshold)).await?;
    let batches = df.collect().await?;
    let batch = &batches[0];
    let n_col = batch.column(0).as_any().downcast_ref::<Int64Array>().unwrap();
    Ok(n_col.value(0) as usize)
}

pub async fn names_above_amount(ctx: &SessionContext, table: &str, threshold: f64) -> Result<Vec<String>> {
    let df = ctx.sql(&format!("SELECT name FROM {} WHERE amount > {} ORDER BY name", table, threshold)).await?;
    let batches = df.collect().await?;
    let mut names = Vec::new();
    for batch in &batches {
        let col = batch.column(0).as_any().downcast_ref::<StringArray>().unwrap();
        for i in 0..col.len() {
            names.push(col.value(i).to_string());
        }
    }
    Ok(names)
}

pub async fn run_sql(ctx: &SessionContext, sql: &str) -> Result<Vec<RecordBatch>> {
    let df = ctx.sql(sql).await?;
    Ok(df.collect().await?)
}

pub async fn write_parquet(ctx: &SessionContext, table: &str, path: &str) -> Result<()> {
    let df = ctx.sql(&format!("SELECT * FROM {}", table)).await?;
    df.write_parquet(path, None).await?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    mod step_01_context {
        use super::*;

        #[tokio::test]
        async fn test_create_context() {
            let ctx = create_context().await.unwrap();
            let batches = ctx
                .sql("SELECT 1 AS one")
                .await
                .unwrap()
                .collect()
                .await
                .unwrap();
            assert_eq!(batches.len(), 1);
        }
    }

    mod step_02_csv {
        use super::*;

        #[tokio::test]
        async fn test_register_csv() {
            let ctx = create_context().await.unwrap();
            register_csv(&ctx, "orders", "data/orders.csv").await.unwrap();
            assert_eq!(count_rows(&ctx, "orders").await.unwrap(), 6);
        }
    }

    mod step_03_aggregations {
        use super::*;

        async fn seed() -> SessionContext {
            let ctx = create_context().await.unwrap();
            register_csv(&ctx, "orders", "data/orders.csv").await.unwrap();
            ctx
        }

        #[tokio::test]
        async fn test_total_amount() {
            let ctx = seed().await;
            let total = total_amount(&ctx, "orders").await.unwrap();
            let expected = 1.50 + 2.25 + 3.99 + 5.49 + 2.99 + 4.49;
            assert!((total - expected).abs() < 1e-6);
        }

        #[tokio::test]
        async fn test_rows_above_amount() {
            let ctx = seed().await;
            let n = rows_above_amount(&ctx, "orders", 3.0).await.unwrap();
            assert_eq!(n, 3);
        }

        #[tokio::test]
        async fn test_names_above_amount() {
            let ctx = seed().await;
            let names = names_above_amount(&ctx, "orders", 3.0).await.unwrap();
            assert!(names.contains(&"Milk".to_string()));
            assert!(names.contains(&"Cheese".to_string()));
            assert!(names.contains(&"Butter".to_string()));
        }
    }

    mod step_04_sql {
        use super::*;

        #[tokio::test]
        async fn test_run_sql_with_aggregation() {
            let ctx = create_context().await.unwrap();
            register_csv(&ctx, "orders", "data/orders.csv").await.unwrap();
            let batches = run_sql(&ctx, "SELECT COUNT(*) AS n, AVG(amount) AS avg_amt FROM orders").await.unwrap();
            assert!(!batches.is_empty());
            let batch = &batches[0];
            let n_col = batch.column(0).as_any().downcast_ref::<Int64Array>().unwrap();
            assert_eq!(n_col.value(0), 6);
        }
    }

    mod step_05_parquet {
        use super::*;
        use std::fs;

        #[tokio::test]
        async fn test_write_parquet() {
            let ctx = create_context().await.unwrap();
            register_csv(&ctx, "orders", "data/orders.csv").await.unwrap();
            let tmp = std::env::temp_dir().join("datafusion_orders_test.parquet");
            write_parquet(&ctx, "orders", tmp.to_str().unwrap()).await.unwrap();
            assert!(tmp.exists());
            let _ = fs::remove_file(&tmp);
        }
    }
}
