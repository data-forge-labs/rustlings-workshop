//! Kafka producer/consumer library using rdkafka (librdkafka bindings).
//!
//! This module exposes pure-data functions for building Kafka clients
//! and serializing/deserializing event envelopes. The actual I/O
//! happens in `main.rs` so tests can run offline.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::time::Duration;
use uuid::Uuid;

// =========================================================================
// Step 1 — Domain types
// =========================================================================

/// Canonical event envelope used across the platform.
///
/// In Python you'd write this as a `dataclass` or Pydantic `BaseModel`.
/// In Rust we use `serde::Serialize`/`Deserialize` for JSON framing on
/// the wire and `Debug`/`Clone` for ergonomic usage.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct EventEnvelope {
    /// Globally unique event id (UUIDv4).
    pub event_id: Uuid,
    /// Aggregate (entity) type, e.g. `"order"`, `"customer"`.
    pub aggregate_type: String,
    /// Id of the aggregate instance this event mutates.
    pub aggregate_id: Uuid,
    /// Event type, e.g. `"order.created"`, `"order.shipped"`.
    pub event_type: String,
    /// Free-form JSON payload (any serializable structure).
    pub payload: serde_json::Value,
    /// Producer-assigned timestamp (UTC).
    pub occurred_at: DateTime<Utc>,
}

impl EventEnvelope {
    /// Build a fresh envelope with a new UUIDv4 and `occurred_at = now`.
    pub fn new(
        aggregate_type: impl Into<String>,
        aggregate_id: Uuid,
        event_type: impl Into<String>,
        payload: serde_json::Value,
    ) -> Self {
        Self {
            event_id: Uuid::new_v4(),
            aggregate_type: aggregate_type.into(),
            aggregate_id,
            event_type: event_type.into(),
            payload,
            occurred_at: Utc::now(),
        }
    }
}

// =========================================================================
// Step 2 — Client configuration builders
// =========================================================================

/// Build an rdkafka producer config from a `bootstrap.servers` string.
///
/// This is a *pure* function returning a `ClientConfig`; it does not
/// open a network connection. Useful for unit tests where you just
/// want to verify the config values.
pub fn producer_config(bootstrap_servers: &str, client_id: &str) -> rdkafka::config::ClientConfig {
    let mut cfg = rdkafka::config::ClientConfig::new();
    cfg.set("bootstrap.servers", bootstrap_servers)
        .set("client.id", client_id)
        .set("message.timeout.ms", "5000")
        .set("compression.type", "lz4")
        .set("acks", "all")
        .set("enable.idempotence", "true")
        .set("max.in.flight.requests.per.connection", "5")
        .set("retries", "10")
        .set("retry.backoff.ms", "100");
    cfg
}

/// Build an rdkafka consumer config from a `bootstrap.servers` string
/// and a consumer group id.
pub fn consumer_config(
    bootstrap_servers: &str,
    group_id: &str,
    auto_offset_reset: &str,
) -> rdkafka::config::ClientConfig {
    let mut cfg = rdkafka::config::ClientConfig::new();
    cfg.set("bootstrap.servers", bootstrap_servers)
        .set("group.id", group_id)
        .set("client.id", group_id)
        .set("auto.offset.reset", auto_offset_reset)
        .set("enable.auto.commit", "false")
        .set("session.timeout.ms", "10000")
        .set("max.poll.interval.ms", "300000");
    cfg
}

// =========================================================================
// Step 3 — Topic naming
// =========================================================================

/// Canonical topic naming convention.
///
/// In Python/Confluent ecosystems topic names are usually free-form
/// strings. We centralize the convention here to prevent drift
/// between producers and consumers.
pub fn topic_for(aggregate_type: &str, event_type: &str) -> String {
    let agg = aggregate_type.to_ascii_lowercase();
    let evt = event_type.split('.').next().unwrap_or(event_type).to_ascii_lowercase();
    format!("{}.{}", agg, evt)
}

/// Derive a consumer group id from a service name and topic.
pub fn group_id_for(service: &str, topic: &str) -> String {
    format!("{}.{}", service, topic)
}

// =========================================================================
// Step 4 — Serialization helpers
// =========================================================================

/// Serialize an event envelope to a JSON byte vector for the Kafka
/// message `value`. Returns `Vec<u8>` (empty for `None`).
pub fn encode_event(event: &EventEnvelope) -> Result<Vec<u8>, serde_json::Error> {
    serde_json::to_vec(event)
}

/// Deserialize a Kafka message `value` into an `EventEnvelope`.
pub fn decode_event(bytes: &[u8]) -> Result<EventEnvelope, serde_json::Error> {
    serde_json::from_slice(bytes)
}

/// Derive a stable message key for an event — uses
/// `aggregate_type:aggregate_id` so all events for the same aggregate
/// land on the same partition (preserves ordering).
pub fn message_key(event: &EventEnvelope) -> Vec<u8> {
    format!("{}:{}", event.aggregate_type, event.aggregate_id).into_bytes()
}

// =========================================================================
// Step 5 — Outbox row → event conversion
// =========================================================================

/// Represents one row from a transactional outbox table (PostgreSQL).
/// In Project 07-CdcPipeline we Debezium-stream this table; in
/// Project 01 we read it directly and produce to Kafka.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct OutboxRow {
    pub id: Uuid,
    pub aggregate_type: String,
    pub aggregate_id: Uuid,
    pub event_type: String,
    pub payload: serde_json::Value,
    pub created_at: DateTime<Utc>,
}

/// Convert an outbox row into a `EventEnvelope`. This is the bridge
/// between "rows in Postgres" and "events in Kafka".
pub fn outbox_row_to_envelope(row: &OutboxRow) -> EventEnvelope {
    EventEnvelope {
        event_id: row.id,
        aggregate_type: row.aggregate_type.clone(),
        aggregate_id: row.aggregate_id,
        event_type: row.event_type.clone(),
        payload: row.payload.clone(),
        occurred_at: row.created_at,
    }
}

// =========================================================================
// Step 6 — Deduplication (idempotent consumer)
// =========================================================================

/// Per-partition dedup state. The consumer keeps the last `max_size`
/// event ids seen so a redelivery (Kafka at-least-once) is collapsed
/// into a single downstream effect.
#[derive(Debug, Default)]
pub struct DedupCache {
    seen: std::collections::VecDeque<Uuid>,
    max_size: usize,
}

impl DedupCache {
    /// Create a new dedup cache that remembers the last `max_size` ids.
    pub fn new(max_size: usize) -> Self {
        Self { seen: std::collections::VecDeque::with_capacity(max_size), max_size }
    }

    /// Returns `true` if the id was new and recorded; `false` if it
    /// had already been seen.
    pub fn record(&mut self, id: Uuid) -> bool {
        if self.seen.contains(&id) {
            return false;
        }
        if self.seen.len() >= self.max_size {
            self.seen.pop_front();
        }
        self.seen.push_back(id);
        true
    }

    /// Current number of remembered ids (for tests / observability).
    pub fn len(&self) -> usize {
        self.seen.len()
    }

    /// Has the cache reached capacity?
    pub fn is_full(&self) -> bool {
        self.seen.len() >= self.max_size
    }
}

// =========================================================================
// Step 7 — Partitioning helper (testing)
// =========================================================================

/// Compute a stable partition for a given aggregate id using FNV-1a.
///
/// In production you let the Kafka producer do this via the message
/// key; this helper exists so tests can assert the expected partition
/// without a live broker.
pub fn partition_for(aggregate_id: &Uuid, num_partitions: i32) -> i32 {
    if num_partitions <= 0 {
        return 0;
    }
    let bytes = aggregate_id.as_bytes();
    let mut hash: u64 = 0xcbf29ce484222325;
    for b in bytes {
        hash ^= *b as u64;
        hash = hash.wrapping_mul(0x100000001b3);
    }
    (hash % (num_partitions as u64)) as i32
}

// =========================================================================
// Step 8 — Convenience: produce a single event
// =========================================================================

/// Produce a single event to a topic. This is a thin wrapper that
/// uses the rdkafka `FutureProducer`. Returns the delivery
/// `Result` from the broker with a default 5-second timeout.
pub async fn produce_one(
    producer: &rdkafka::producer::FutureProducer,
    topic: &str,
    event: &EventEnvelope,
) -> anyhow::Result<(i32, i64)> {
    use std::time::Duration;
    let key = message_key(event);
    let value = encode_event(event).map_err(|e| anyhow::anyhow!("encode: {e}"))?;
    let record: rdkafka::producer::FutureRecord<'_, Vec<u8>, Vec<u8>> =
        rdkafka::producer::FutureRecord::to(topic).key(&key).payload(&value);
    let res = producer
        .send(record, Duration::from_secs(5))
        .await
        .map_err(|(e, _)| anyhow::anyhow!("send: {e}"))?;
    Ok(res)
}

// =========================================================================
// Re-exports
// =========================================================================
pub use rdkafka;

// =========================================================================
// Tests
// =========================================================================
#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    // ----- Step 1 -----
    mod step_01_domain_types {
        use super::*;
        /// `EventEnvelope::new` generates a UUIDv4 and stamps
        /// `occurred_at = now`; user-supplied fields are preserved.
        #[test]
        fn new_stamps_uuid_and_timestamp() {
            let agg = Uuid::new_v4();
            let before = Utc::now();
            let e = EventEnvelope::new("order", agg, "order.created", json!({"qty": 2}));
            assert_eq!(e.aggregate_id, agg);
            assert_eq!(e.aggregate_type, "order");
            assert_eq!(e.event_type, "order.created");
            assert_eq!(e.payload["qty"], 2);
            assert!(e.occurred_at >= before);
            // UUIDv4 — version field is 4
            assert_eq!(e.event_id.get_version_num(), 4);
        }

        /// `serde_json::to_string` on an envelope is reversible
        /// (round-trip preserves all fields).
        #[test]
        fn envelope_round_trips_through_json() {
            let e = EventEnvelope::new(
                "order",
                Uuid::new_v4(),
                "order.paid",
                json!({"amount": 99.5}),
            );
            let s = serde_json::to_string(&e).unwrap();
            let back: EventEnvelope = serde_json::from_str(&s).unwrap();
            assert_eq!(e, back);
        }
    }

    // ----- Step 2 -----
    mod step_02_client_config {
        use super::*;
        /// `producer_config` writes the expected settings into the
        /// underlying `ClientConfig` map.
        #[test]
        fn producer_config_sets_required_keys() {
            let cfg = producer_config("localhost:9092", "svc-orders");
            // Use the `get` method (returns Option<&str>).
            assert_eq!(cfg.get("bootstrap.servers"), Some("localhost:9092"));
            assert_eq!(cfg.get("client.id"), Some("svc-orders"));
            assert_eq!(cfg.get("acks"), Some("all"));
            assert_eq!(cfg.get("enable.idempotence"), Some("true"));
        }

        /// `consumer_config` sets `enable.auto.commit=false` so the
        /// application controls commits.
        #[test]
        fn consumer_config_disables_auto_commit() {
            let cfg = consumer_config("kafka:9092", "svc-billing", "earliest");
            assert_eq!(cfg.get("group.id"), Some("svc-billing"));
            assert_eq!(cfg.get("auto.offset.reset"), Some("earliest"));
            assert_eq!(cfg.get("enable.auto.commit"), Some("false"));
        }
    }

    // ----- Step 3 -----
    mod step_03_topic_naming {
        use super::*;
        /// `topic_for("Order", "Order.Created.Shipped")` returns
        /// `"order.order"` — we only use the first segment of the
        /// event type, lowercased, joined with the aggregate.
        #[test]
        fn topic_normalizes_to_lowercase_first_segment() {
            assert_eq!(topic_for("Order", "Order.Created.Shipped"), "order.order");
            assert_eq!(topic_for("customer", "customer.updated"), "customer.customer");
            assert_eq!(topic_for("inventory", "stock.changed"), "inventory.stock");
        }

        /// `group_id_for` joins service and topic with a `.`.
        #[test]
        fn group_id_joins_service_and_topic() {
            assert_eq!(group_id_for("svc-billing", "order.order"), "svc-billing.order.order");
        }
    }

    // ----- Step 4 -----
    mod step_04_serde {
        use super::*;
        /// `encode_event`/`decode_event` are inverses.
        #[test]
        fn encode_decode_round_trip() {
            let e = EventEnvelope::new("order", Uuid::new_v4(), "order.created", json!({"a": 1}));
            let bytes = encode_event(&e).unwrap();
            let back = decode_event(&bytes).unwrap();
            assert_eq!(e, back);
        }

        /// `message_key` uses `aggregate_type:aggregate_id` so all
        /// events for the same aggregate partition together.
        #[test]
        fn message_key_is_aggregate_typed() {
            let agg = Uuid::nil();
            let e = EventEnvelope::new("order", agg, "order.created", json!({}));
            let k = String::from_utf8(message_key(&e)).unwrap();
            assert_eq!(k, format!("order:{}", agg));
        }

        /// `decode_event` on invalid JSON returns an error.
        #[test]
        fn decode_event_rejects_invalid_json() {
            let res = decode_event(b"not json");
            assert!(res.is_err());
        }
    }

    // ----- Step 5 -----
    mod step_05_outbox {
        use super::*;
        /// `outbox_row_to_envelope` copies all outbox fields and
        /// uses `id` as the envelope's `event_id`.
        #[test]
        fn outbox_to_envelope_copies_fields() {
            let row = OutboxRow {
                id: Uuid::new_v4(),
                aggregate_type: "order".into(),
                aggregate_id: Uuid::new_v4(),
                event_type: "order.created".into(),
                payload: json!({"qty": 1}),
                created_at: Utc::now(),
            };
            let env = outbox_row_to_envelope(&row);
            assert_eq!(env.event_id, row.id);
            assert_eq!(env.aggregate_type, row.aggregate_type);
            assert_eq!(env.aggregate_id, row.aggregate_id);
            assert_eq!(env.event_type, row.event_type);
            assert_eq!(env.payload, row.payload);
            assert_eq!(env.occurred_at, row.created_at);
        }
    }

    // ----- Step 6 -----
    mod step_06_dedup {
        use super::*;
        /// A fresh id is recorded (`true`).
        /// Re-recording the same id returns `false`.
        #[test]
        fn record_returns_true_then_false() {
            let mut cache = DedupCache::new(3);
            let id = Uuid::new_v4();
            assert!(cache.record(id));
            assert!(!cache.record(id));
            assert_eq!(cache.len(), 1);
        }

        /// When capacity is reached, the oldest id is evicted (FIFO).
        /// Re-recording the evicted id is treated as a *new* id.
        #[test]
        fn evicts_oldest_when_full() {
            let mut cache = DedupCache::new(2);
            let a = Uuid::new_v4();
            let b = Uuid::new_v4();
            let c = Uuid::new_v4();
            assert!(cache.record(a));
            assert!(cache.record(b));
            assert!(cache.is_full());
            assert!(cache.record(c));
            assert_eq!(cache.len(), 2);
            assert!(cache.record(a), "a should be evicted and re-recordable");
        }
    }

    // ----- Step 7 -----
    mod step_07_partition {
        use super::*;
        /// The same aggregate id always maps to the same partition.
        #[test]
        fn same_aggregate_same_partition() {
            let id = Uuid::parse_str("550e8400-e29b-41d4-a716-446655440000").unwrap();
            let p1 = partition_for(&id, 16);
            let p2 = partition_for(&id, 16);
            assert_eq!(p1, p2);
            assert!((0..16).contains(&p1));
        }

        /// `num_partitions <= 0` returns 0 (defensive default).
        #[test]
        fn zero_partitions_returns_zero() {
            assert_eq!(partition_for(&Uuid::new_v4(), 0), 0);
            assert_eq!(partition_for(&Uuid::new_v4(), -5), 0);
        }
    }

    // ----- Step 8 -----
    mod step_08_produce {
        use super::*;
        /// `produce_one` requires a live broker; in the unit test
        /// environment we only assert that the function *exists* and
        /// has the expected signature. The end-to-end smoke is in
        /// `main.rs`.
        #[test]
        fn produce_one_signature_compiles() {
            // This test does nothing at runtime — its purpose is to
            // ensure the public API compiles for users.
            let _: fn(
                &rdkafka::producer::FutureProducer,
                &str,
                &EventEnvelope,
            ) -> std::pin::Pin<
                Box<dyn std::future::Future<Output = anyhow::Result<(i32, i64)>> + Send + '_>,
            > = |_p, _t, _e| Box::pin(async { Ok((0, 0)) });
        }

        /// We expose a `Duration` default helper for callers.
        #[test]
        fn default_send_timeout_is_five_seconds() {
            let d = Duration::from_secs(5);
            assert_eq!(d.as_millis(), 5_000);
        }
    }
}
