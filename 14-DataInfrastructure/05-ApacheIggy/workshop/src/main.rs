// Note: The actual Iggy SDK call shape varies by version; the demo below
// is a representative end-to-end flow that you would adapt to your
// `iggy` crate version. The pure logic in `lib.rs` is what we test.

use apache_iggy::{
    consumer_parallelism, decode, default_partition_count, encode, partition_for,
    stream_for, topic_for, IggyDedup, IggyMessage, OffsetCursor,
};
use serde_json::json;
use uuid::Uuid;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::try_from_default_env()
            .unwrap_or_else(|_| "info".into()))
        .init();

    let stream_name = stream_for("orders");
    let topic_name  = topic_for("order", "order.created");
    let num_partitions = default_partition_count();
    let parallelism = consumer_parallelism(num_partitions);
    tracing::info!(stream = %stream_name, topic = %topic_name,
        partitions = num_partitions, consumers = parallelism,
        "iggy plan");

    // ---- 1. Build a message and pick a partition ----
    let agg = Uuid::new_v4();
    let key = format!("order:{}", agg);
    let partition = partition_for(&key, num_partitions);
    let msg = IggyMessage::new(&stream_name, &topic_name, partition, json!({
        "order_id": agg,
        "customer_id": Uuid::new_v4(),
        "items": [{"sku": "RUST-101", "qty": 1, "unit_price": 49.99}],
        "total": 49.99,
    })).with_key(&key)
      .with_header("trace-id", "demo-1");
    let bytes = encode(&msg)?;
    tracing::info!(id = %msg.id, key = %key, partition, bytes = bytes.len(),
        "encoded message");

    // ---- 2. Decode round-trip ----
    let back = decode(&bytes)?;
    assert_eq!(msg, back);
    tracing::info!("round-trip ok");

    // ---- 3. Offset cursor + dedup ----
    let mut cursor = OffsetCursor::new(&stream_name, &topic_name, partition);
    cursor.advance(1);
    let mut dedup = IggyDedup::new(1024);
    let _ = dedup.record(msg.id);
    let _ = dedup.record(msg.id); // second is a duplicate
    tracing::info!(offset = cursor.offset, dedup_size = dedup.len(), "consumer state");

    Ok(())
}
