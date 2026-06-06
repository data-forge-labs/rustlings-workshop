use postgres_sqlx::{
    connection_url, kafka_event_type, new_outbox_row, validate_create_order, CreateOrderInput,
    INSERT_ORDER_COLUMNS, MARK_OUTBOX_PROCESSED, SELECT_ORDERS, SELECT_UNPROCESSED_OUTBOX,
};
use sqlx::postgres::PgPoolOptions;
use sqlx::Row;
use uuid::Uuid;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::try_from_default_env()
            .unwrap_or_else(|_| "info".into()))
        .init();

    let url = std::env::var("DATABASE_URL").unwrap_or_else(|_|
        connection_url("dataeng", "dataeng", "localhost", 5432, "dataeng"));
    let url = postgres_sqlx::with_application_name(&url, "demo-pg");

    let pool = PgPoolOptions::new()
        .max_connections(8)
        .acquire_timeout(std::time::Duration::from_secs(5))
        .connect(&url).await?;
    tracing::info!("connected to postgres");

    // ---- 1. Insert a fresh order (uses SELECT_ORDERS, INSERT_ORDER_COLUMNS) ----
    let input = CreateOrderInput {
        customer_id: Uuid::new_v4(),
        product_id: Uuid::new_v4(),
        quantity: 3,
        unit_price: 12.50,
    };
    validate_create_order(&input).map_err(|e| anyhow::anyhow!(e))?;
    let total = postgres_sqlx::total_price(&input);

    let mut tx = pool.begin().await?;
    let order_id: Uuid = sqlx::query_scalar(
        &format!(
            "INSERT INTO dataeng.orders ({INSERT_ORDER_COLUMNS})
             VALUES ($1, $2, $3, $4, 'pending') RETURNING id"
        ),
    )
    .bind(input.customer_id)
    .bind(input.product_id)
    .bind(input.quantity)
    .bind(bigdecimal::BigDecimal::try_from(input.unit_price).unwrap_or_default())
    .fetch_one(&mut *tx).await?;

    // ---- 2. Insert outbox row in the same transaction ----
    let row = new_outbox_row("order", order_id, "created", serde_json::json!({
        "customer_id": input.customer_id,
        "product_id": input.product_id,
        "quantity": input.quantity,
        "unit_price": input.unit_price,
        "total": total,
    }));
    sqlx::query(
        "INSERT INTO dataeng.outbox
            (id, aggregate_type, aggregate_id, event_type, payload, created_at, retry_count)
         VALUES ($1, $2, $3, $4, $5, $6, 0)",
    )
    .bind(row.id)
    .bind(&row.aggregate_type)
    .bind(row.aggregate_id)
    .bind(&row.event_type)
    .bind(&row.payload)
    .bind(row.created_at)
    .execute(&mut *tx).await?;
    tx.commit().await?;
    tracing::info!(order_id = %order_id, outbox_id = %row.id, "wrote order + outbox");

    // ---- 3. Read unprocessed outbox rows ----
    let q = format!("{SELECT_ORDERS} FROM dataeng.orders ORDER BY created_at DESC LIMIT 1");
    let _ = q; // preview the order
    let mut unprocessed = sqlx::query(SELECT_UNPROCESSED_OUTBOX).fetch_all(&pool).await?;
    tracing::info!(count = unprocessed.len(), "unprocessed outbox rows");

    // ---- 4. Mark the one we just wrote as processed (this would happen
    //         AFTER the Kafka producer confirmed in a real pipeline) ----
    if let Some(r) = unprocessed.pop() {
        let id: Uuid = r.try_get("id")?;
        sqlx::query(MARK_OUTBOX_PROCESSED).bind(id).execute(&pool).await?;
        tracing::info!(kafka_event = %kafka_event_type(&new_outbox_row("order", id, "created", serde_json::json!({})),
            ), "marked outbox processed");
    }

    Ok(())
}
