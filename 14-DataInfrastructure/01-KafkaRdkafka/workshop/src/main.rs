use kafka_rdkafka::rdkafka::config::ClientConfig;
use kafka_rdkafka::rdkafka::consumer::{Consumer, StreamConsumer};
use kafka_rdkafka::rdkafka::message::Message;
use kafka_rdkafka::rdkafka::producer::FutureProducer;
use kafka_rdkafka::{
    consumer_config, decode_event, encode_event, group_id_for, message_key, outbox_row_to_envelope,
    produce_one, producer_config, topic_for, DedupCache, EventEnvelope, OutboxRow,
};
use std::time::Duration;
use uuid::Uuid;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::try_from_default_env()
            .unwrap_or_else(|_| "info".into()))
        .init();

    let brokers = std::env::var("KAFKA_BROKERS").unwrap_or_else(|_| "kafka:9092".into());

    // ---- Producer demo ----
    let producer: FutureProducer = producer_config(&brokers, "demo-producer")
        .create()?;
    tracing::info!("connected producer to {brokers}");

    let agg_id = Uuid::new_v4();
    let event = EventEnvelope::new("order", agg_id, "order.created", serde_json::json!({
        "customer_id": Uuid::new_v4(),
        "items": [{"sku": "RUST-101", "qty": 1, "unit_price": 49.99}],
        "total": 49.99,
    }));
    let topic = topic_for(&event.aggregate_type, &event.event_type);
    let (part, off) = produce_one(&producer, &topic, &event).await?;
    tracing::info!(topic, partition = part, offset = off, "produced order.created");

    // ---- Outbox row -> envelope demo ----
    let row = OutboxRow {
        id: Uuid::new_v4(),
        aggregate_type: "order".into(),
        aggregate_id: agg_id,
        event_type: "order.paid".into(),
        payload: serde_json::json!({"amount": 49.99}),
        created_at: chrono::Utc::now(),
    };
    let env = outbox_row_to_envelope(&row);
    let (part, off) = produce_one(&producer, &topic_for(&env.aggregate_type, &env.event_type), &env).await?;
    tracing::info!(topic, partition = part, offset = off, "produced order.paid via outbox");

    // ---- Consumer demo ----
    let consumer: StreamConsumer = consumer_config(&brokers, &group_id_for("demo-consumer", &topic), "earliest")
        .create()?;
    consumer.subscribe(&[&topic])?;
    tracing::info!("subscribed to {topic}");

    let mut cache = DedupCache::new(1024);
    let mut received = 0usize;
    let deadline = tokio::time::Instant::now() + Duration::from_secs(8);
    while tokio::time::Instant::now() < deadline && received < 2 {
        let msg = tokio::time::timeout(Duration::from_secs(2), consumer.recv()).await;
        match msg {
            Ok(Ok(m)) => {
                let payload = m.payload().unwrap_or_default();
                if let Ok(env) = decode_event(payload) {
                    if cache.record(env.event_id) {
                        tracing::info!(partition = m.partition(), offset = m.offset(),
                            event_id = %env.event_id, event_type = %env.event_type,
                            "consumed (new)");
                    } else {
                        tracing::info!(event_id = %env.event_id, "consumed (dup, skipped)");
                    }
                    received += 1;
                }
                consumer.commit_message(&m, rdkafka::consumer::CommitMode::Async)?;
            }
            _ => {}
        }
    }
    tracing::info!(received, "demo complete");
    Ok(())
}

// Keep ClientConfig import live (used by type inference in main above)
#[allow(dead_code)]
fn _type_anchor(_c: ClientConfig) {}
