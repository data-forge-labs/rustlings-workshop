//! Apache Iggy message stream client (Rust-native alternative to Kafka).
//!
//! Pure-data module — all I/O lives in `main.rs`.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

// =========================================================================
// Step 1 — Connection configuration
// =========================================================================

/// Build a TCP connection URL for Apache Iggy.
pub fn tcp_url(host: &str, port: u16) -> String {
    format!("{host}:{port}")
}

/// Build an HTTP connection URL for Apache Iggy.
pub fn http_url(host: &str, port: u16) -> String {
    format!("http://{host}:{port}")
}

// =========================================================================
// Step 2 — Domain types
// =========================================================================

/// Canonical message envelope for Iggy.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct IggyMessage {
    pub id: Uuid,
    pub stream: String,
    pub topic: String,
    pub partition: u32,
    pub key: Option<String>,
    pub value: serde_json::Value,
    pub headers: std::collections::HashMap<String, String>,
    pub created_at: DateTime<Utc>,
}

impl IggyMessage {
    /// Build a fresh message with a new UUIDv4 and `created_at = now`.
    pub fn new(
        stream: impl Into<String>,
        topic: impl Into<String>,
        partition: u32,
        value: serde_json::Value,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            stream: stream.into(),
            topic: topic.into(),
            partition,
            key: None,
            value,
            headers: std::collections::HashMap::new(),
            created_at: Utc::now(),
        }
    }

    /// Attach a routing key.
    pub fn with_key(mut self, key: impl Into<String>) -> Self {
        self.key = Some(key.into());
        self
    }

    /// Attach a header.
    pub fn with_header(mut self, k: impl Into<String>, v: impl Into<String>) -> Self {
        self.headers.insert(k.into(), v.into());
        self
    }
}

// =========================================================================
// Step 3 — Stream / topic / partition naming
// =========================================================================

/// Canonical stream name. Iggy is multi-tenant: each service has its
/// own stream; topics are sub-channels within the stream.
pub fn stream_for(domain: &str) -> String {
    format!("stream:{}", domain.to_ascii_lowercase())
}

/// Topic name within a stream. Convention: `aggregate.event`.
pub fn topic_for(aggregate_type: &str, event_type: &str) -> String {
    let agg = aggregate_type.to_ascii_lowercase();
    let evt = event_type.split('.').next().unwrap_or(event_type).to_ascii_lowercase();
    format!("{}.{}", agg, evt)
}

/// Default partition count. Iggy is a thread-per-core system, so
/// partition count should match the number of consumers in a group
/// — typically `num_cpus::get()` rounded up.
pub fn default_partition_count() -> u32 {
    std::thread::available_parallelism()
        .map(|n| n.get() as u32)
        .unwrap_or(4)
        .max(1)
}

// =========================================================================
// Step 4 — Partition selection
// =========================================================================

/// Compute a stable partition for a given key. We use FNV-1a to
/// match the Kafka project so keys land on the *same* partition
/// regardless of broker.
pub fn partition_for(key: &str, num_partitions: u32) -> u32 {
    if num_partitions == 0 {
        return 0;
    }
    let mut hash: u64 = 0xcbf29ce484222325;
    for b in key.as_bytes() {
        hash ^= *b as u64;
        hash = hash.wrapping_mul(0x100000001b3);
    }
    (hash % (num_partitions as u64)) as u32
}

// =========================================================================
// Step 5 — Serialization
// =========================================================================

/// Serialize a message to JSON bytes for the wire.
pub fn encode(msg: &IggyMessage) -> Result<Vec<u8>, serde_json::Error> {
    serde_json::to_vec(msg)
}

/// Deserialize JSON bytes to a message.
pub fn decode(bytes: &[u8]) -> Result<IggyMessage, serde_json::Error> {
    serde_json::from_slice(bytes)
}

// =========================================================================
// Step 6 — Consumer state
// =========================================================================

/// Per-consumer offset tracker. Used by the demo to poll-and-commit
/// without a group coordinator (Iggy is single-broker in our setup).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OffsetCursor {
    pub stream: String,
    pub topic: String,
    pub partition: u32,
    pub offset: u64,
}

impl OffsetCursor {
    pub fn new(stream: impl Into<String>, topic: impl Into<String>, partition: u32) -> Self {
        Self { stream: stream.into(), topic: topic.into(), partition, offset: 0 }
    }

    /// Advance by `n` (after `n` successful polls).
    pub fn advance(&mut self, n: u64) {
        self.offset = self.offset.saturating_add(n);
    }

    /// Replay from offset 0 (or any explicit `from`).
    pub fn rewind_to(&mut self, from: u64) {
        self.offset = from;
    }
}

// =========================================================================
// Step 7 — Dedup cache (in-memory)
// =========================================================================

/// In-memory dedup cache. Identical semantics to Project 01's
/// `DedupCache`; we re-export it under the Iggy namespace so the
/// demo can be self-contained.
#[derive(Debug, Default)]
pub struct IggyDedup {
    seen: std::collections::VecDeque<Uuid>,
    max_size: usize,
}

impl IggyDedup {
    pub fn new(max_size: usize) -> Self {
        Self { seen: std::collections::VecDeque::with_capacity(max_size), max_size }
    }

    /// Returns `true` if the id was new and recorded.
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

    pub fn len(&self) -> usize { self.seen.len() }
    pub fn is_empty(&self) -> bool { self.seen.is_empty() }
}

// =========================================================================
// Step 8 — Thread-per-core scaling
// =========================================================================

/// Compute the consumer parallelism (number of consumers) for a
/// given partition count. Heuristic: 1 consumer per partition is
/// ideal, but Iggy's thread-per-core model means we cap at the
/// number of physical cores.
pub fn consumer_parallelism(num_partitions: u32) -> u32 {
    let cores = std::thread::available_parallelism()
        .map(|n| n.get() as u32)
        .unwrap_or(4);
    num_partitions.min(cores).max(1)
}

// =========================================================================
// Tests
// =========================================================================
#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    // ----- Step 1 -----
    mod step_01_connection {
        use super::*;
        /// `tcp_url` formats as `host:port`.
        #[test]
        fn tcp_url_format() {
            assert_eq!(tcp_url("iggy", 3000), "iggy:3000");
        }
        /// `http_url` formats as `http://host:port`.
        #[test]
        fn http_url_format() {
            assert_eq!(http_url("iggy", 8090), "http://iggy:8090");
        }
    }

    // ----- Step 2 -----
    mod step_02_domain {
        use super::*;
        /// `IggyMessage::new` populates id and timestamp.
        #[test]
        fn new_stamps_id_and_timestamp() {
            let before = Utc::now();
            let m = IggyMessage::new("stream:orders", "order.order", 0, json!({"qty": 1}));
            assert_eq!(m.id.get_version_num(), 4);
            assert!(m.created_at >= before);
            assert_eq!(m.stream, "stream:orders");
            assert_eq!(m.topic, "order.order");
        }
        /// `with_key` and `with_header` chain.
        #[test]
        fn builders_chain() {
            let m = IggyMessage::new("s", "t", 0, json!({}))
                .with_key("k")
                .with_header("trace-id", "abc");
            assert_eq!(m.key.as_deref(), Some("k"));
            assert_eq!(m.headers.get("trace-id").map(String::as_str), Some("abc"));
        }
    }

    // ----- Step 3 -----
    mod step_03_naming {
        use super::*;
        /// `stream_for` prefixes with `stream:` and lowercases.
        #[test]
        fn stream_lowercases() {
            assert_eq!(stream_for("Orders"), "stream:orders");
        }
        /// `topic_for` joins aggregate and event-type first segment.
        #[test]
        fn topic_joins_segments() {
            assert_eq!(topic_for("Order", "Order.Created.Shipped"), "order.order");
        }
        /// `default_partition_count` is at least 1.
        #[test]
        fn partition_count_at_least_one() {
            assert!(default_partition_count() >= 1);
        }
    }

    // ----- Step 4 -----
    mod step_04_partition {
        use super::*;
        /// Same key always maps to the same partition.
        #[test]
        fn same_key_same_partition() {
            let p1 = partition_for("order:abc", 8);
            let p2 = partition_for("order:abc", 8);
            assert_eq!(p1, p2);
            assert!(p1 < 8);
        }
        /// Zero partitions returns 0.
        #[test]
        fn zero_partitions_zero() {
            assert_eq!(partition_for("k", 0), 0);
        }
    }

    // ----- Step 5 -----
    mod step_05_serde {
        use super::*;
        /// `encode`/`decode` are inverses.
        #[test]
        fn round_trip() {
            let m = IggyMessage::new("s", "t", 0, json!({"a": 1}));
            let v = encode(&m).unwrap();
            let back = decode(&v).unwrap();
            assert_eq!(m, back);
        }
    }

    // ----- Step 6 -----
    mod step_06_cursor {
        use super::*;
        /// `OffsetCursor::new` starts at offset 0.
        #[test]
        fn new_starts_at_zero() {
            let c = OffsetCursor::new("s", "t", 0);
            assert_eq!(c.offset, 0);
            assert_eq!(c.partition, 0);
        }
        /// `advance` increments by `n`.
        #[test]
        fn advance_increments() {
            let mut c = OffsetCursor::new("s", "t", 0);
            c.advance(5);
            c.advance(3);
            assert_eq!(c.offset, 8);
        }
        /// `rewind_to` resets the offset.
        #[test]
        fn rewind_resets() {
            let mut c = OffsetCursor::new("s", "t", 0);
            c.advance(100);
            c.rewind_to(50);
            assert_eq!(c.offset, 50);
        }
    }

    // ----- Step 7 -----
    mod step_07_dedup {
        use super::*;
        /// `record` returns `true` once per id, then `false`.
        #[test]
        fn record_returns_true_then_false() {
            let mut d = IggyDedup::new(3);
            let id = Uuid::new_v4();
            assert!(d.record(id));
            assert!(!d.record(id));
            assert_eq!(d.len(), 1);
        }
    }

    // ----- Step 8 -----
    mod step_08_parallelism {
        use super::*;
        /// `consumer_parallelism` returns at least 1.
        #[test]
        fn at_least_one() {
            assert!(consumer_parallelism(0) >= 1);
            assert!(consumer_parallelism(100) >= 1);
        }
        /// `consumer_parallelism` caps at the number of cores.
        #[test]
        fn caps_at_cores() {
            let cores = std::thread::available_parallelism()
                .map(|n| n.get() as u32)
                .unwrap_or(4);
            assert!(consumer_parallelism(1000) <= cores);
        }
    }
}
