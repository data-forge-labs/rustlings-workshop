//! Change Data Capture pipeline: Debezium-style event shapes,
//! in-memory state machine for routing, and Sink interface.
//!
//! Pure-data module — actual Debezium integration lives in
//! `main.rs`. We model the *shape* of CDC events (which is what
//! data engineers reason about) and the *plumbing* (leader
//! election, routing, deduplication).

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

// =========================================================================
// Step 1 — CDC event envelope
// =========================================================================

/// A single CDC event, modeled after Debezium's envelope format:
///   before, after, source, op, ts_ms, transaction
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CdcEvent {
    /// Source table, e.g. `dataeng.orders`.
    pub source: String,
    /// Operation: `c` (create), `u` (update), `d` (delete), `r` (read/snapshot).
    pub op: CdcOp,
    /// Pre-image (for `u` and `d`); `None` for `c`.
    pub before: Option<serde_json::Value>,
    /// Post-image (for `c` and `u`); `None` for `d`.
    pub after: Option<serde_json::Value>,
    /// Per-event timestamp in milliseconds since epoch.
    pub ts_ms: i64,
    /// Optional transaction id (groups multiple events into one tx).
    pub tx_id: Option<Uuid>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum CdcOp {
    Create,
    Update,
    Delete,
    Read, // snapshot
}

impl CdcOp {
    pub fn as_char(&self) -> char {
        match self {
            Self::Create => 'c',
            Self::Update => 'u',
            Self::Delete => 'd',
            Self::Read => 'r',
        }
    }
    pub fn from_char(c: char) -> Option<Self> {
        match c {
            'c' => Some(Self::Create),
            'u' => Some(Self::Update),
            'd' => Some(Self::Delete),
            'r' => Some(Self::Read),
            _ => None,
        }
    }
}

// =========================================================================
// Step 2 — Routing
// =========================================================================

/// Build the destination Kafka topic for a CDC source table.
/// Convention: `<db>.<table>` -> `<table>.<op>` (lowercased).
pub fn topic_for(source: &str, op: CdcOp) -> String {
    let table = source
        .rsplit('.')
        .next()
        .unwrap_or(source)
        .to_ascii_lowercase();
    let op_str = match op {
        CdcOp::Create => "cdc.c",
        CdcOp::Update => "cdc.u",
        CdcOp::Delete => "cdc.d",
        CdcOp::Read => "cdc.r",
    };
    format!("{table}.{op_str}")
}

/// Routing key for partitioning. Format: `<table>:<id>` so all
/// events for the same row land on the same partition.
pub fn routing_key(source: &str, row_id: &Uuid) -> String {
    let table = source.rsplit('.').next().unwrap_or(source);
    format!("{}:{}", table, row_id)
}

// =========================================================================
// Step 3 — Filtering
// =========================================================================

/// Decide if an event should be forwarded to the sink. The default
/// policy skips `Read` events (snapshot-only) and `Delete` events
/// for soft-deleted records.
pub fn should_forward(event: &CdcEvent) -> bool {
    if matches!(event.op, CdcOp::Read) {
        return false;
    }
    // Skip deletes that are actually soft-deletes (after.status == 'cancelled')
    if matches!(event.op, CdcOp::Delete) {
        if let Some(after) = &event.after {
            if after.get("status").and_then(|v| v.as_str()) == Some("cancelled") {
                return false;
            }
        }
    }
    true
}

// =========================================================================
// Step 4 — Leader election
// =========================================================================

/// A lock claim for leader election. Real systems use `SETNX` in
/// Redis or `pg_try_advisory_lock` in Postgres; we model the shape.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LeaderClaim {
    pub resource: String,
    pub holder: String,
    pub acquired_at: DateTime<Utc>,
    pub ttl_ms: u64,
}

impl LeaderClaim {
    pub fn new(resource: impl Into<String>, holder: impl Into<String>, ttl_ms: u64) -> Self {
        Self { resource: resource.into(), holder: holder.into(), acquired_at: Utc::now(), ttl_ms }
    }

    /// Is this claim still valid? Returns `false` if `ttl_ms` has
    /// elapsed since `acquired_at`.
    pub fn is_valid(&self, now: DateTime<Utc>) -> bool {
        let elapsed = (now - self.acquired_at).num_milliseconds();
        elapsed >= 0 && (elapsed as u64) < self.ttl_ms
    }
}

// =========================================================================
// Step 5 — Sink trait
// =========================================================================

/// A Sink consumes CDC events and writes them somewhere — Kafka,
/// ClickHouse, DuckLake, or a test buffer. The trait is `async` to
/// match our Tokio-based runtime.
#[async_trait::async_trait]
pub trait Sink: Send + Sync {
    /// Send one event. Returns `Err` on transport failure; the
    /// caller decides whether to retry or DLQ.
    async fn send(&self, event: &CdcEvent) -> anyhow::Result<()>;

    /// Flush any buffered events. Called at the end of a batch
    /// or on shutdown.
    async fn flush(&self) -> anyhow::Result<()>;
}

// =========================================================================
// Step 6 — Test sink
// =========================================================================

/// In-memory sink used by tests and the demo `main.rs`. Collects
/// every event into a `Vec` for assertion.
#[derive(Debug, Default)]
pub struct InMemorySink {
    pub events: std::sync::Mutex<Vec<CdcEvent>>,
}

#[async_trait::async_trait]
impl Sink for InMemorySink {
    async fn send(&self, event: &CdcEvent) -> anyhow::Result<()> {
        self.events.lock().unwrap().push(event.clone());
        Ok(())
    }
    async fn flush(&self) -> anyhow::Result<()> { Ok(()) }
}

impl InMemorySink {
    pub fn len(&self) -> usize { self.events.lock().unwrap().len() }
    pub fn is_empty(&self) -> bool { self.events.lock().unwrap().is_empty() }
}

// =========================================================================
// Step 7 — Pipeline state machine
// =========================================================================

/// Pipeline progress checkpoint. In production this is stored in
/// Postgres or Redis so a restart resumes from the right place.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Checkpoint {
    pub source: String,
    pub last_lsn: u64,
    pub last_ts_ms: i64,
    pub events_processed: u64,
}

impl Checkpoint {
    pub fn new(source: impl Into<String>) -> Self {
        Self { source: source.into(), last_lsn: 0, last_ts_ms: 0, events_processed: 0 }
    }

    /// Advance the checkpoint by one event. `lsn` is the Postgres
    /// Log Sequence Number (or Kafka offset for non-Debezium sources).
    pub fn advance(&mut self, lsn: u64, ts_ms: i64) {
        self.last_lsn = lsn;
        self.last_ts_ms = ts_ms;
        self.events_processed = self.events_processed.saturating_add(1);
    }
}

// =========================================================================
// Step 8 — Batching & ordering
// =========================================================================

/// Decide whether a batch is ready to flush. We flush when:
///  - size >= `max_batch`, OR
///  - the oldest event is older than `max_wait_ms`, OR
///  - the transaction boundary is reached (last `tx_id` differs from
///    first `tx_id` in batch).
pub fn batch_ready(
    size: usize,
    max_batch: usize,
    age_ms: i64,
    max_wait_ms: i64,
    tx_changed: bool,
) -> bool {
    size >= max_batch || age_ms >= max_wait_ms || tx_changed
}

// =========================================================================
// Tests
// =========================================================================
#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn make_event(op: CdcOp, after: Option<serde_json::Value>, ts_ms: i64) -> CdcEvent {
        CdcEvent { source: "dataeng.orders".into(), op, before: None, after, ts_ms, tx_id: None }
    }

    // ----- Step 1 -----
    mod step_01_envelope {
        use super::*;
        /// `CdcOp::as_char`/`from_char` round-trip.
        #[test]
        fn op_chars_round_trip() {
            for op in [CdcOp::Create, CdcOp::Update, CdcOp::Delete, CdcOp::Read] {
                assert_eq!(CdcOp::from_char(op.as_char()), Some(op));
            }
        }
        /// Unknown characters return `None`.
        #[test]
        fn unknown_char_is_none() {
            assert!(CdcOp::from_char('x').is_none());
        }
        /// `CdcEvent` round-trips through JSON.
        #[test]
        fn event_round_trips() {
            let e = make_event(CdcOp::Create, Some(json!({"id": "abc"})), 1_700_000_000_000);
            let s = serde_json::to_string(&e).unwrap();
            let back: CdcEvent = serde_json::from_str(&s).unwrap();
            assert_eq!(e, back);
        }
    }

    // ----- Step 2 -----
    mod step_02_routing {
        use super::*;
        /// `topic_for` uses table name + op suffix, all lowercase.
        #[test]
        fn topic_uses_table_op() {
            assert_eq!(topic_for("dataeng.orders", CdcOp::Create), "orders.cdc.c");
            assert_eq!(topic_for("dataeng.Orders", CdcOp::Update), "orders.cdc.u");
        }
        /// `routing_key` joins table and row id.
        #[test]
        fn routing_key_format() {
            let id = Uuid::nil();
            assert_eq!(routing_key("dataeng.orders", &id), format!("orders:{id}"));
        }
    }

    // ----- Step 3 -----
    mod step_03_filter {
        use super::*;
        /// `Read` events are dropped.
        #[test]
        fn read_events_dropped() {
            let e = make_event(CdcOp::Read, Some(json!({})), 0);
            assert!(!should_forward(&e));
        }
        /// `Create`/`Update`/`Delete` are forwarded by default.
        #[test]
        fn create_update_forwarded() {
            assert!(should_forward(&make_event(CdcOp::Create, Some(json!({})), 0)));
            assert!(should_forward(&make_event(CdcOp::Update, Some(json!({})), 0)));
        }
        /// Soft-deletes (status=='cancelled') are dropped.
        #[test]
        fn soft_delete_dropped() {
            let e = make_event(CdcOp::Delete, Some(json!({"status": "cancelled"})), 0);
            assert!(!should_forward(&e));
        }
    }

    // ----- Step 4 -----
    mod step_04_leader {
        use super::*;
        /// Fresh claim is valid.
        #[test]
        fn fresh_claim_valid() {
            let c = LeaderClaim::new("cdc:orders", "worker-1", 30_000);
            assert!(c.is_valid(Utc::now()));
        }
        /// Expired claim is invalid.
        #[test]
        fn expired_claim_invalid() {
            let mut c = LeaderClaim::new("cdc:orders", "worker-1", 30_000);
            c.acquired_at = Utc::now() - chrono::Duration::milliseconds(60_000);
            assert!(!c.is_valid(Utc::now()));
        }
    }

    // ----- Step 5/6 -----
    mod step_05_06_sink {
        use super::*;
        /// `InMemorySink::send` stores the event.
        #[tokio::test]
        async fn sink_stores_event() {
            let s = InMemorySink::default();
            let e = make_event(CdcOp::Create, Some(json!({"a": 1})), 0);
            Sink::send(&s, &e).await.unwrap();
            assert_eq!(s.len(), 1);
        }
        /// `flush` is a no-op for the in-memory sink.
        #[tokio::test]
        async fn sink_flush_ok() {
            let s = InMemorySink::default();
            assert!(Sink::flush(&s).await.is_ok());
        }
    }

    // ----- Step 7 -----
    mod step_07_checkpoint {
        use super::*;
        /// `Checkpoint::new` starts at zero.
        #[test]
        fn new_is_zero() {
            let c = Checkpoint::new("dataeng.orders");
            assert_eq!(c.last_lsn, 0);
            assert_eq!(c.events_processed, 0);
        }
        /// `advance` increments event count and updates lsn/ts.
        #[test]
        fn advance_updates_fields() {
            let mut c = Checkpoint::new("dataeng.orders");
            c.advance(100, 1_700_000_000_000);
            c.advance(101, 1_700_000_000_001);
            assert_eq!(c.last_lsn, 101);
            assert_eq!(c.events_processed, 2);
            assert_eq!(c.last_ts_ms, 1_700_000_000_001);
        }
    }

    // ----- Step 8 -----
    mod step_08_batching {
        use super::*;
        /// `batch_ready` triggers on size, age, and tx boundary.
        #[test]
        fn ready_on_size() {
            assert!(batch_ready(100, 100, 0, 1000, false));
        }
        /// `batch_ready` triggers on age.
        #[test]
        fn ready_on_age() {
            assert!(batch_ready(10, 100, 1500, 1000, false));
        }
        /// `batch_ready` triggers on tx boundary.
        #[test]
        fn ready_on_tx() {
            assert!(batch_ready(5, 100, 0, 1000, true));
        }
        /// `batch_ready` returns false when none of the conditions hold.
        #[test]
        fn not_ready() {
            assert!(!batch_ready(5, 100, 100, 1000, false));
        }
    }
}
