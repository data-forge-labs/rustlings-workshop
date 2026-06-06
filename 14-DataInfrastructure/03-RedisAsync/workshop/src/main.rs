use redis::aio::ConnectionManager;
use redis::AsyncCommands;
use redis_async::{
    connection_url, order_key, stream_name,
    entry_to_fields, CacheItem, CacheKind, CacheStats, ConsumerGroup, IdempotencyClaim,
    StreamEntry, ttl_for,
};
use std::time::Duration;
use uuid::Uuid;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::try_from_default_env()
            .unwrap_or_else(|_| "info".into()))
        .init();

    let url = std::env::var("REDIS_URL")
        .unwrap_or_else(|_| connection_url("localhost", 6379, 0, None));
    let client = redis::Client::open(url.as_str())?;
    let mut conn: ConnectionManager = ConnectionManager::new(client).await?;
    tracing::info!("connected to redis: {url}");

    // ---- 1. Set + Get cache item ----
    let order_id = Uuid::new_v4();
    let key = order_key(&order_id);
    let item = CacheItem {
        key: key.clone(),
        value: serde_json::json!({"customer_id": Uuid::new_v4(), "total": 99.5, "status": "paid"}),
        ttl_ms: Some(ttl_for(CacheKind::HotOrder).as_millis() as u64),
    };
    let payload = serde_json::to_string(&item.value)?;
    let _: () = conn.set_ex(&key, payload, ttl_for(CacheKind::HotOrder).as_secs()).await?;
    let cached: Option<String> = conn.get(&key).await?;
    tracing::info!(key = %key, hit = cached.is_some(), "cached order");

    // ---- 2. Push to stream ----
    let stream = stream_name("orders");
    let entry = StreamEntry {
        id: Uuid::new_v4(),
        event_type: "order.created".into(),
        payload: serde_json::json!({"order_id": order_id}),
        created_at_ms: chrono::Utc::now().timestamp_millis(),
    };
    let fields = entry_to_fields(&entry);
    let id: String = conn.xadd(&stream, "*", &fields).await?;
    tracing::info!(stream = %stream, id = %id, "xadd");

    // ---- 3. Idempotency claim (SETNX) ----
    let claim = IdempotencyClaim::for_event(&entry.id, 60_000);
    let setnx: bool = conn.set_nx(&claim.key, "1").await?;
    let second: bool = conn.set_nx(&claim.key, "1").await?;
    let _: i64 = conn.pexpire(&claim.key, claim.ttl_ms as i64).await?;
    tracing::info!(first = setnx, second = second, "setnx result");

    // ---- 4. Consumer group demo (in-memory loop, no I/O) ----
    let g = ConsumerGroup::new(&stream, "demo-group", "c-1")
        .with_block_ms(100)
        .with_batch(10);
    tracing::info!(stream = %g.stream, group = %g.group, consumer = %g.consumer,
        block_ms = g.block_ms, batch = g.batch, "consumer group");

    // ---- 5. Stats summary ----
    let mut stats = CacheStats::default();
    if cached.is_some() { stats.record_hit(); } else { stats.record_miss(); }
    tracing::info!(hits = stats.hits, misses = stats.misses,
        ratio = stats.hit_ratio(), "cache stats");

    tokio::time::sleep(Duration::from_millis(50)).await;
    Ok(())
}
