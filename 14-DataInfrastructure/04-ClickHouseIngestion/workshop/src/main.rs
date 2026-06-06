use chrono::Utc;
use clickhouse::Client;
use clickhouse_ingestion::{
    http_url, project_minute_buckets, total_quantity, total_revenue, ClickHouseRetry, DDL_ORDERS,
    IngestBatcher, OrderRow, OrderStatus, INSERT_ORDERS_COLUMNS,
};
use std::time::Duration;
use uuid::Uuid;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::try_from_default_env()
            .unwrap_or_else(|_| "info".into()))
        .init();

    let host = std::env::var("CLICKHOUSE_HOST").unwrap_or_else(|_| "localhost".into());
    let http = std::env::var("CLICKHOUSE_URL")
        .unwrap_or_else(|_| http_url(&host, 8123, "analytics"));
    let user = std::env::var("CLICKHOUSE_USER").unwrap_or_else(|_| "dataeng".into());
    let password = std::env::var("CLICKHOUSE_PASSWORD").unwrap_or_else(|_| "dataeng".into());

    let client = Client::default()
        .with_url(&http)
        .with_user(&user)
        .with_password(&password)
        .with_database("analytics");
    tracing::info!("connecting to {http}");

    // ---- 1. Create table (idempotent) ----
    client.query(DDL_ORDERS).execute().await?;
    tracing::info!("ensured analytics.orders exists");

    // ---- 2. Build a batch ----
    let mut batcher = IngestBatcher::new(100, 1_000_000);
    let now = Utc::now();
    for i in 0..50 {
        let row = OrderRow {
            id: Uuid::new_v4(),
            customer_id: Uuid::new_v4(),
            product_id: Uuid::new_v4(),
            quantity: (i as u32) + 1,
            unit_price: 9.99,
            total_price: 9.99 * ((i as f64) + 1.0),
            status: OrderStatus::Paid.as_str().into(),
            created_at: now,
            updated_at: now,
        };
        if let Some(drained) = batcher.push(row) {
            insert_batch(&client, &drained).await?;
        }
    }
    if !batcher.is_empty() {
        insert_batch(&client, &batcher.flush()).await?;
    }
    let total_q = total_quantity(&{
        // Recompute from a fresh view to demonstrate the API; in
        // real code we would have stored the rows we just inserted.
        let mut v = Vec::new();
        for i in 0..50u32 {
            v.push(OrderRow {
                id: Uuid::nil(), customer_id: Uuid::nil(), product_id: Uuid::nil(),
                quantity: i + 1, unit_price: 9.99,
                total_price: 9.99 * ((i as f64) + 1.0),
                status: "paid".into(),
                created_at: now, updated_at: now,
            });
        }
        v
    });
    tracing::info!(total_qty = total_q, "ingested 50 rows");

    // ---- 3. Read-back aggregation ----
    let rows: Vec<OrderRow> = client
        .query("SELECT ?fields FROM analytics.orders WHERE created_at > now() - INTERVAL 1 MINUTE")
        .fetch_all()
        .await?;
    let buckets = project_minute_buckets(&rows);
    let rev = total_revenue(&rows);
    tracing::info!(readback = rows.len(), revenue = rev, buckets = buckets.len(), "read-back");

    Ok(())
}

async fn insert_batch(client: &Client, rows: &[OrderRow]) -> anyhow::Result<()> {
    let retry = ClickHouseRetry::default();
    for attempt in 0..retry.max_retries {
        let mut insert = client.insert(&format!("analytics.orders {INSERT_ORDERS_COLUMNS}"))?;
        for r in rows {
            insert.write(r).await?;
        }
        match insert.end().await {
            Ok(_) => return Ok(()),
            Err(e) => {
                if let Some(wait) = retry.next_backoff(attempt) {
                    tracing::warn!(attempt, error = %e, "insert failed, retrying in {wait:?}");
                    tokio::time::sleep(wait).await;
                } else {
                    return Err(e.into());
                }
            }
        }
    }
    Ok(())
}
