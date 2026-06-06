use duckdb::Connection;
use ducklake_catalog::{
    catalog_url, compute_stats, create_orders_table, in_memory_url, merge_upsert_statement,
    select_at_snapshot, should_compact, snapshot_id_for, target_file_size_bytes, LakeOrder,
    TableMetadata, ATTACH_DUCKLAKE,
};
use std::time::Duration;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::try_from_default_env()
            .unwrap_or_else(|_| "info".into()))
        .init();

    let path = std::env::var("DUCKLAKE_PATH").unwrap_or_else(|_| in_memory_url());
    tracing::info!("opening duckdb catalog: {path}");

    // Open a DuckDB connection. For DuckLake we would normally
    // execute ATTACH; for the in-memory demo we skip the extension
    // and exercise the SQL builders in `lib.rs`.
    let conn = Connection::open(
        path.strip_prefix("duckdb://").unwrap_or("demo.duckdb")
    )?;
    conn.execute_batch("PRAGMA threads=4")?;

    // ---- 1. Create a local table (no DuckLake extension in demo) ----
    conn.execute_batch(&create_orders_table().replace("lake.orders", "orders"))?;
    tracing::info!("ensured orders table");

    // ---- 2. Insert a few rows ----
    let now = chrono::Utc::now();
    let snap = snapshot_id_for(now);
    for i in 0..5 {
        let row = LakeOrder {
            id: uuid::Uuid::new_v4(),
            customer_id: uuid::Uuid::new_v4(),
            product_id: uuid::Uuid::new_v4(),
            quantity: (i as u32) + 1,
            unit_price: 9.99,
            total_price: 9.99 * ((i as f64) + 1.0),
            status: "paid".into(),
            created_at: now,
            snapshot_id: snap,
        };
        conn.execute(
            "INSERT INTO orders VALUES (?, ?, ?, ?, ?, ?, ?, ?)",
            duckdb::params![
                row.id, row.customer_id, row.product_id,
                row.quantity, row.unit_price, row.total_price,
                row.status, row.created_at
            ],
        )?;
    }
    tracing::info!("inserted 5 rows");

    // ---- 3. Time-travel SELECT (DuckLake syntax; demo prints it) ----
    let tt = select_at_snapshot(snap);
    tracing::info!(sql = %tt, "time-travel query");

    // ---- 4. Merge statement preview ----
    let m = merge_upsert_statement();
    tracing::info!("merge statement: {} chars", m.len());

    // ---- 5. Compaction decision ----
    let meta = TableMetadata::new("orders", "lake", "data/orders");
    let compact = should_compact(&meta, 5);
    tracing::info!(compact, target_bytes = target_file_size_bytes(), "compaction plan");

    // ---- 6. Aggregate stats (over the rows we just inserted) ----
    let mut stmt = conn.prepare("SELECT id, customer_id, product_id, quantity, unit_price, total_price, status, created_at FROM orders")?;
    let rows = stmt.query_map([], |r| {
        Ok(LakeOrder {
            id: r.get(0)?,
            customer_id: r.get(1)?,
            product_id: r.get(2)?,
            quantity: r.get(3)?,
            unit_price: r.get(4)?,
            total_price: r.get(5)?,
            status: r.get(6)?,
            created_at: r.get(7)?,
            snapshot_id: snap,
        })
    })?;
    let mut collected: Vec<LakeOrder> = Vec::new();
    for r in rows { collected.push(r?); }
    let stats = compute_stats(&collected);
    tracing::info!(rows = stats.row_count, revenue = stats.total_revenue,
        customers = stats.distinct_customers, "stats");

    // ---- 7. Print the canonical ATTACH statement (so users see it) ----
    tracing::info!(attach = ATTACH_DUCKLAKE, "DuckLake attach example");

    tokio::time::sleep(Duration::from_millis(10)).await;
    Ok(())
}

// Suppress unused import warnings for the helpers we may swap in
#[allow(dead_code)]
fn _suppress(_u: String) { let _ = catalog_url("x"); }
