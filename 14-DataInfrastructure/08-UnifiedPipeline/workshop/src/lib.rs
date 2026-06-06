//! Unified pipeline: a single orchestrator that consumes CDC events
//! and fans them out to multiple sinks (Kafka, ClickHouse, DuckLake).
//!
//! Pure-data module — actual I/O lives in `main.rs`.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use uuid::Uuid;

// =========================================================================
// Step 1 — Pipeline config
// =========================================================================

/// Static pipeline configuration. Parsed once at startup.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PipelineConfig {
    pub name: String,
    pub source: String,
    pub sinks: Vec<String>,
    pub max_batch: usize,
    pub max_wait_ms: u64,
    pub checkpoint_every: u64,
}

impl PipelineConfig {
    pub fn new(name: impl Into<String>, source: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            source: source.into(),
            sinks: vec!["kafka".into(), "clickhouse".into(), "ducklake".into()],
            max_batch: 1000,
            max_wait_ms: 1_000,
            checkpoint_every: 100,
        }
    }

    /// Add a sink by name. Returns `&mut self` for chaining.
    pub fn with_sink(mut self, sink: impl Into<String>) -> Self {
        self.sinks.push(sink.into());
        self
    }
}

// =========================================================================
// Step 2 — Event payload (canonical)
// =========================================================================

/// Canonical event that flows through the pipeline. Built from
/// CDC events (Project 07) or directly from outbox rows
/// (Project 02) or stream entries (Project 01/05).
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PipelineEvent {
    pub id: Uuid,
    pub source: String,
    pub event_type: String,
    pub aggregate_id: Uuid,
    pub payload: serde_json::Value,
    pub created_at: DateTime<Utc>,
}

impl PipelineEvent {
    pub fn new(
        source: impl Into<String>,
        event_type: impl Into<String>,
        aggregate_id: Uuid,
        payload: serde_json::Value,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            source: source.into(),
            event_type: event_type.into(),
            aggregate_id,
            payload,
            created_at: Utc::now(),
        }
    }
}

// =========================================================================
// Step 3 — Sink outcomes
// =========================================================================

/// Per-event outcome from one sink. We collect outcomes so we can
/// checkpoint, retry, or dead-letter per sink.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SinkOutcome {
    pub sink: String,
    pub success: bool,
    pub error: Option<String>,
    pub duration_ms: u64,
}

impl SinkOutcome {
    pub fn ok(sink: impl Into<String>, duration_ms: u64) -> Self {
        Self { sink: sink.into(), success: true, error: None, duration_ms }
    }
    pub fn err(sink: impl Into<String>, error: impl Into<String>, duration_ms: u64) -> Self {
        Self { sink: sink.into(), success: false, error: Some(error.into()), duration_ms }
    }
}

// =========================================================================
// Step 4 — Fan-out policy
// =========================================================================

/// Decide which sinks should receive an event. The default policy
/// sends everything to all configured sinks.
pub fn fanout_targets(event: &PipelineEvent, config: &PipelineConfig) -> Vec<String> {
    config.sinks.clone()
}

// =========================================================================
// Step 5 — Aggregation / windowing
// =========================================================================

/// Per-event-type running counters. Useful for live dashboards
/// (Project 04 read-side materialization).
#[derive(Debug, Default, Clone, PartialEq)]
pub struct WindowCounters {
    pub by_type: BTreeMap<String, u64>,
    pub total: u64,
    pub errors: u64,
}

impl WindowCounters {
    pub fn record(&mut self, _event: &PipelineEvent) {
        *self.by_type.entry(event.event_type.clone()).or_insert(0) += 1;
        self.total += 1;
    }
    pub fn record_error(&mut self) {
        self.errors += 1;
    }
    /// Count of distinct event types.
    pub fn distinct_types(&self) -> usize {
        self.by_type.len()
    }
}

// =========================================================================
// Step 6 — Retry policy for sinks
// =========================================================================

/// Compute a backoff for a failed sink. Same shape as Project 04's
/// `ClickHouseRetry::next_backoff` but inlined so this project
/// has no dependency on Project 04.
pub fn sink_backoff_ms(attempt: u32, base_ms: u64, cap_ms: u64) -> Option<u64> {
    if attempt >= 5 { return None; }
    let exp = 1u64 << attempt.min(20);
    let raw = base_ms.saturating_mul(exp);
    Some(raw.min(cap_ms))
}

// =========================================================================
// Step 7 — Dead-letter queue
// =========================================================================

/// A dead-lettered event — the pipeline gave up on it.
#[derive(Debug, Clone, PartialEq)]
pub struct DeadLetter {
    pub event: PipelineEvent,
    pub sink: String,
    pub last_error: String,
    pub attempts: u32,
    pub created_at: DateTime<Utc>,
}

impl DeadLetter {
    pub fn new(event: PipelineEvent, sink: impl Into<String>, last_error: impl Into<String>, attempts: u32) -> Self {
        Self { event, sink: sink.into(), last_error: last_error.into(), attempts, created_at: Utc::now() }
    }
}

// =========================================================================
// Step 8 — Pipeline stats
// =========================================================================

/// Aggregate stats for a single pipeline run. Reset at startup.
#[derive(Debug, Default, Clone, PartialEq)]
pub struct PipelineStats {
    pub events_in: u64,
    pub events_fanned_out: u64,
    pub sink_failures: u64,
    pub dead_letters: u64,
    pub uptime_ms: u64,
}

impl PipelineStats {
    pub fn success_rate(&self) -> f64 {
        let total = self.events_fanned_out + self.sink_failures;
        if total == 0 {
            1.0
        } else {
            1.0 - (self.sink_failures as f64 / total as f64)
        }
    }
}

// =========================================================================
// Tests
// =========================================================================
#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    // ----- Step 1 -----
    mod step_01_config {
        use super::*;
        /// `PipelineConfig::new` defaults to 3 sinks (kafka,
        /// clickhouse, ducklake), batch=1000, wait=1000ms.
        #[test]
        fn new_has_three_sinks() {
            let c = PipelineConfig::new("p1", "dataeng.orders");
            assert_eq!(c.sinks.len(), 3);
            assert!(c.sinks.contains(&"kafka".to_string()));
            assert!(c.max_batch == 1000);
        }
        /// `with_sink` adds a sink.
        #[test]
        fn with_sink_appends() {
            let c = PipelineConfig::new("p1", "dataeng.orders").with_sink("redis");
            assert_eq!(c.sinks.len(), 4);
            assert!(c.sinks.contains(&"redis".to_string()));
        }
    }

    // ----- Step 2 -----
    mod step_02_event {
        use super::*;
        /// `PipelineEvent::new` populates id and timestamp.
        #[test]
        fn new_stamps_id_and_timestamp() {
            let before = Utc::now();
            let e = PipelineEvent::new("dataeng.orders", "order.created", Uuid::new_v4(), json!({}));
            assert_eq!(e.id.get_version_num(), 4);
            assert!(e.created_at >= before);
        }
        /// `PipelineEvent` round-trips through JSON.
        #[test]
        fn round_trips() {
            let e = PipelineEvent::new("s", "t", Uuid::new_v4(), json!({"a": 1}));
            let s = serde_json::to_string(&e).unwrap();
            let back: PipelineEvent = serde_json::from_str(&s).unwrap();
            assert_eq!(e, back);
        }
    }

    // ----- Step 3 -----
    mod step_03_outcome {
        use super::*;
        /// `SinkOutcome::ok` has `success=true` and no error.
        #[test]
        fn ok_has_no_error() {
            let o = SinkOutcome::ok("kafka", 5);
            assert!(o.success);
            assert!(o.error.is_none());
            assert_eq!(o.duration_ms, 5);
        }
        /// `SinkOutcome::err` has `success=false` and a message.
        #[test]
        fn err_has_message() {
            let o = SinkOutcome::err("clickhouse", "timeout", 100);
            assert!(!o.success);
            assert_eq!(o.error.as_deref(), Some("timeout"));
        }
    }

    // ----- Step 4 -----
    mod step_04_fanout {
        use super::*;
        /// Default policy fans out to every configured sink.
        #[test]
        fn default_fanout_to_all() {
            let c = PipelineConfig::new("p1", "s");
            let e = PipelineEvent::new("s", "t", Uuid::new_v4(), json!({}));
            let targets = fanout_targets(&e, &c);
            assert_eq!(targets, c.sinks);
        }
    }

    // ----- Step 5 -----
    mod step_05_counters {
        use super::*;
        /// `record` increments per-type and total.
        #[test]
        fn record_increments() {
            let mut c = WindowCounters::default();
            c.record(&PipelineEvent::new("s", "order.created", Uuid::new_v4(), json!({})));
            c.record(&PipelineEvent::new("s", "order.created", Uuid::new_v4(), json!({})));
            c.record(&PipelineEvent::new("s", "order.paid",    Uuid::new_v4(), json!({})));
            assert_eq!(c.total, 3);
            assert_eq!(c.by_type.get("order.created"), Some(&2));
            assert_eq!(c.by_type.get("order.paid"), Some(&1));
            assert_eq!(c.distinct_types(), 2);
        }
        /// `record_error` increments the error counter.
        #[test]
        fn record_error_increments() {
            let mut c = WindowCounters::default();
            c.record_error();
            c.record_error();
            assert_eq!(c.errors, 2);
        }
    }

    // ----- Step 6 -----
    mod step_06_retry {
        use super::*;
        /// `sink_backoff_ms` returns `None` after 5 attempts.
        #[test]
        fn returns_none_at_max() {
            assert!(sink_backoff_ms(0, 100, 10_000).is_some());
            assert!(sink_backoff_ms(4, 100, 10_000).is_some());
            assert!(sink_backoff_ms(5, 100, 10_000).is_none());
        }
        /// Backoff doubles up to the cap.
        #[test]
        fn backoff_doubles() {
            assert_eq!(sink_backoff_ms(0, 100, 10_000), Some(100));
            assert_eq!(sink_backoff_ms(1, 100, 10_000), Some(200));
            assert_eq!(sink_backoff_ms(2, 100, 10_000), Some(400));
            assert_eq!(sink_backoff_ms(10, 100, 10_000), Some(10_000));
        }
    }

    // ----- Step 7 -----
    mod step_07_dlq {
        use super::*;
        /// `DeadLetter::new` populates all fields.
        #[test]
        fn new_populates() {
            let e = PipelineEvent::new("s", "t", Uuid::new_v4(), json!({}));
            let d = DeadLetter::new(e.clone(), "kafka", "broker down", 5);
            assert_eq!(d.event, e);
            assert_eq!(d.sink, "kafka");
            assert_eq!(d.attempts, 5);
        }
    }

    // ----- Step 8 -----
    mod step_08_stats {
        use super::*;
        /// `success_rate` is 1.0 when there are no events.
        #[test]
        fn success_rate_default_is_one() {
            assert!((PipelineStats::default().success_rate() - 1.0).abs() < 1e-9);
        }
        /// `success_rate` falls as `sink_failures` rise.
        #[test]
        fn success_rate_falls_with_failures() {
            let s = PipelineStats { events_fanned_out: 80, sink_failures: 20, ..Default::default() };
            assert!((s.success_rate() - 0.8).abs() < 1e-9);
        }
    }
}
