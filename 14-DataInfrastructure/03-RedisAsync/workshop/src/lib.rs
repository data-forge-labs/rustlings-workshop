//! Redis client library for caching + streams (read-side, write-side, dedup).
//!
//! This module exposes pure-data functions for building cache keys, value
//! frames, stream names, and consumer-group policies. The actual I/O lives
//! in `main.rs` so tests can run offline.

use serde::{Deserialize, Serialize};
use std::time::Duration;
use uuid::Uuid;

// =========================================================================
// Step 1 — Connection configuration
// =========================================================================

/// Build a Redis URL from discrete components.
pub fn connection_url(host: &str, port: u16, db: i64, password: Option<&str>) -> String {
    match password {
        Some(pw) => format!("redis://:{pw}@{host}:{port}/{db}"),
        None => format!("redis://{host}:{port}/{db}"),
    }
}

// =========================================================================
// Step 2 — Domain types
// =========================================================================

/// A cached item with optional expiry. `value` is JSON-serialized for
/// cross-language compatibility (Python/Node can read the same key).
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CacheItem {
    pub key: String,
    pub value: serde_json::Value,
    pub ttl_ms: Option<u64>,
}

/// Stream entry — wraps a payload with metadata for `XADD`/`XREAD`.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct StreamEntry {
    pub id: Uuid,
    pub event_type: String,
    pub payload: serde_json::Value,
    pub created_at_ms: i64,
}

// =========================================================================
// Step 3 — Cache key naming
// =========================================================================

/// Build a namespaced cache key. Replaces ad-hoc f-strings in Python
/// (`f"order:{order_id}"`) with a single function that the test suite
/// can verify.
pub fn order_key(order_id: &Uuid) -> String {
    format!("order:{order_id}")
}

/// Build a customer-scope cache key.
pub fn customer_orders_key(customer_id: &Uuid) -> String {
    format!("customer:{customer_id}:orders")
}

/// Build a leaderboard / counter key.
pub fn counter_key(metric: &str, window: &str) -> String {
    format!("counter:{metric}:{window}")
}

/// Distributed lock key (used by Project 07 CDC leader election).
pub fn lock_key(resource: &str) -> String {
    format!("lock:{resource}")
}

// =========================================================================
// Step 4 — Cache TTL strategies
// =========================================================================

/// Pick a TTL by data class. Mirrors Python `@lru_cache` decision trees.
pub fn ttl_for(kind: CacheKind) -> Duration {
    match kind {
        CacheKind::HotOrder => Duration::from_secs(60),
        CacheKind::CustomerList => Duration::from_secs(300),
        CacheKind::StaticRef => Duration::from_secs(3600),
        CacheKind::Session => Duration::from_secs(1800),
        CacheKind::Ephemeral => Duration::from_secs(5),
    }
}

/// Categorical TTL strategy. Centralized so adding a new category
/// forces a `match` update everywhere.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CacheKind {
    HotOrder,
    CustomerList,
    StaticRef,
    Session,
    Ephemeral,
}

// =========================================================================
// Step 5 — Stream naming + entry frames
// =========================================================================

/// Stream name for the canonical event stream.
pub fn stream_name(domain: &str) -> String {
    format!("stream:{domain}")
}

/// Convert a `StreamEntry` to the `Vec<(String, String)>` shape that
/// `redis::cmd("XADD")` expects.
pub fn entry_to_fields(entry: &StreamEntry) -> Vec<(String, String)> {
    vec![
        ("id".to_string(), entry.id.to_string()),
        ("event_type".to_string(), entry.event_type.clone()),
        ("payload".to_string(), entry.payload.to_string()),
        ("created_at_ms".to_string(), entry.created_at_ms.to_string()),
    ]
}

// =========================================================================
// Step 6 — Consumer group policy
// =========================================================================

/// Consumer group policy for `XREADGROUP` operations.
#[derive(Debug, Clone)]
pub struct ConsumerGroup {
    pub stream: String,
    pub group: String,
    pub consumer: String,
    pub block_ms: usize,
    pub batch: usize,
    pub start_id: String,
}

impl ConsumerGroup {
    pub fn new(
        stream: impl Into<String>,
        group: impl Into<String>,
        consumer: impl Into<String>,
    ) -> Self {
        Self {
            stream: stream.into(),
            group: group.into(),
            consumer: consumer.into(),
            block_ms: 5_000,
            batch: 64,
            start_id: ">".into(),
        }
    }

    pub fn with_block_ms(mut self, ms: usize) -> Self { self.block_ms = ms; self }
    pub fn with_batch(mut self, batch: usize) -> Self { self.batch = batch; self }
    pub fn with_start_id(mut self, id: impl Into<String>) -> Self { self.start_id = id.into(); self }
}

// =========================================================================
// Step 7 — Idempotency / dedup using SETNX
// =========================================================================

/// A simple `SET key value NX EX <ttl>` claim.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IdempotencyClaim {
    pub key: String,
    pub ttl_ms: u64,
}

impl IdempotencyClaim {
    pub fn for_event(event_id: &Uuid, ttl_ms: u64) -> Self {
        Self { key: format!("dedup:event:{event_id}"), ttl_ms }
    }

    /// If the claim is *new* the caller is the only writer for this id.
    /// If the claim already existed, another writer raced us.
    pub fn is_new(&self, setnx_result: bool) -> bool {
        setnx_result
    }
}

// =========================================================================
// Step 8 — Cache statistics
// =========================================================================

/// Hit/miss counters used by the demo. Pure data — the actual
/// `INCR` happens in `main.rs`.
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct CacheStats {
    pub hits: u64,
    pub misses: u64,
    pub sets: u64,
    pub deletes: u64,
    pub errors: u64,
}

impl CacheStats {
    /// Total operations.
    pub fn total(&self) -> u64 {
        self.hits + self.misses + self.sets + self.deletes + self.errors
    }

    /// Hit ratio in `[0.0, 1.0]`. Returns `0.0` if no lookups happened.
    pub fn hit_ratio(&self) -> f64 {
        let lookups = self.hits + self.misses;
        if lookups == 0 {
            0.0
        } else {
            self.hits as f64 / lookups as f64
        }
    }

    /// Record a hit and return the new totals.
    pub fn record_hit(&mut self) -> u64 {
        self.hits += 1;
        self.hits
    }
    /// Record a miss and return the new totals.
    pub fn record_miss(&mut self) -> u64 {
        self.misses += 1;
        self.misses
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
    mod step_01_connection {
        use super::*;
        /// `connection_url` produces a parseable Redis URL.
        #[test]
        fn builds_url_without_password() {
            assert_eq!(connection_url("redis", 6379, 0, None), "redis://redis:6379/0");
        }
        /// With a password, the URL embeds it as `:pw@host:port/db`.
        #[test]
        fn builds_url_with_password() {
            assert_eq!(
                connection_url("redis", 6379, 1, Some("sekret")),
                "redis://:sekret@redis:6379/1"
            );
        }
    }

    // ----- Step 2 -----
    mod step_02_domain {
        use super::*;
        /// `CacheItem` and `StreamEntry` round-trip through JSON.
        #[test]
        fn cache_item_round_trips() {
            let c = CacheItem {
                key: "order:abc".into(),
                value: json!({"qty": 2}),
                ttl_ms: Some(60_000),
            };
            let s = serde_json::to_string(&c).unwrap();
            let back: CacheItem = serde_json::from_str(&s).unwrap();
            assert_eq!(c, back);
        }
        /// `StreamEntry` is `Clone + Eq`.
        #[test]
        fn stream_entry_derives() {
            let e = StreamEntry {
                id: Uuid::new_v4(),
                event_type: "order.created".into(),
                payload: json!({}),
                created_at_ms: 1_700_000_000_000,
            };
            let e2 = e.clone();
            assert_eq!(e, e2);
        }
    }

    // ----- Step 3 -----
    mod step_03_keys {
        use super::*;
        /// `order_key` formats as `"order:<uuid>"`.
        #[test]
        fn order_key_uses_prefix() {
            let id = Uuid::nil();
            assert_eq!(order_key(&id), format!("order:{id}"));
        }
        /// `customer_orders_key` includes `customer:<uuid>:orders`.
        #[test]
        fn customer_key_includes_scope() {
            let id = Uuid::nil();
            assert_eq!(customer_orders_key(&id), format!("customer:{id}:orders"));
        }
        /// `counter_key` joins metric and window with a colon.
        #[test]
        fn counter_key_joins_metric_window() {
            assert_eq!(counter_key("orders", "1m"), "counter:orders:1m");
        }
        /// `lock_key` prefixes with `lock:`.
        #[test]
        fn lock_key_prefixes() {
            assert_eq!(lock_key("cdc-leader"), "lock:cdc-leader");
        }
    }

    // ----- Step 4 -----
    mod step_04_ttl {
        use super::*;
        /// Each `CacheKind` has the expected TTL band.
        #[test]
        fn ttl_bands_match_expectations() {
            assert_eq!(ttl_for(CacheKind::HotOrder), Duration::from_secs(60));
            assert_eq!(ttl_for(CacheKind::CustomerList), Duration::from_secs(300));
            assert_eq!(ttl_for(CacheKind::StaticRef), Duration::from_secs(3600));
            assert_eq!(ttl_for(CacheKind::Session), Duration::from_secs(1800));
            assert_eq!(ttl_for(CacheKind::Ephemeral), Duration::from_secs(5));
        }
    }

    // ----- Step 5 -----
    mod step_05_stream {
        use super::*;
        /// `stream_name` prefixes with `stream:`.
        #[test]
        fn stream_name_prefixes() {
            assert_eq!(stream_name("orders"), "stream:orders");
        }
        /// `entry_to_fields` produces a 4-tuple list with the expected
        /// keys in any order.
        #[test]
        fn entry_to_fields_has_four_entries() {
            let e = StreamEntry {
                id: Uuid::nil(),
                event_type: "order.created".into(),
                payload: json!({"qty": 1}),
                created_at_ms: 100,
            };
            let f = entry_to_fields(&e);
            assert_eq!(f.len(), 4);
            let names: Vec<&str> = f.iter().map(|(k, _)| k.as_str()).collect();
            assert!(names.contains(&"id"));
            assert!(names.contains(&"event_type"));
            assert!(names.contains(&"payload"));
            assert!(names.contains(&"created_at_ms"));
        }
    }

    // ----- Step 6 -----
    mod step_06_consumer_group {
        use super::*;
        /// `ConsumerGroup::new` uses sane defaults that can be
        /// overridden with the `with_*` builder methods.
        #[test]
        fn new_has_sane_defaults() {
            let g = ConsumerGroup::new("stream:orders", "svc-billing", "c-1");
            assert_eq!(g.stream, "stream:orders");
            assert_eq!(g.group, "svc-billing");
            assert_eq!(g.consumer, "c-1");
            assert_eq!(g.block_ms, 5_000);
            assert_eq!(g.batch, 64);
            assert_eq!(g.start_id, ">");
        }
        /// `with_*` builders are chainable.
        #[test]
        fn with_methods_chain() {
            let g = ConsumerGroup::new("s", "g", "c")
                .with_block_ms(100)
                .with_batch(10)
                .with_start_id("0");
            assert_eq!(g.block_ms, 100);
            assert_eq!(g.batch, 10);
            assert_eq!(g.start_id, "0");
        }
    }

    // ----- Step 7 -----
    mod step_07_idempotency {
        use super::*;
        /// `IdempotencyClaim::for_event` keys by event id with the
        /// `dedup:event:` prefix.
        #[test]
        fn claim_keys_by_event_id() {
            let id = Uuid::nil();
            let c = IdempotencyClaim::for_event(&id, 60_000);
            assert_eq!(c.key, format!("dedup:event:{id}"));
            assert_eq!(c.ttl_ms, 60_000);
        }
        /// `is_new` returns `true` when `SETNX` returns 1 (we got
        /// the claim) and `false` when it returns 0 (someone else
        /// claimed it).
        #[test]
        fn is_new_inverts_setnx() {
            let c = IdempotencyClaim::for_event(&Uuid::new_v4(), 60_000);
            assert!(c.is_new(true));
            assert!(!c.is_new(false));
        }
    }

    // ----- Step 8 -----
    mod step_08_stats {
        use super::*;
        /// `total` sums all five counters.
        #[test]
        fn total_sums_all_counters() {
            let s = CacheStats { hits: 1, misses: 2, sets: 3, deletes: 4, errors: 5 };
            assert_eq!(s.total(), 15);
        }
        /// `hit_ratio` returns `0.0` if no lookups happened, and
        /// `hits / (hits + misses)` otherwise.
        #[test]
        fn hit_ratio_zero_when_no_lookups() {
            assert_eq!(CacheStats::default().hit_ratio(), 0.0);
        }
        #[test]
        fn hit_ratio_computes_correctly() {
            let s = CacheStats { hits: 3, misses: 1, ..Default::default() };
            assert!((s.hit_ratio() - 0.75).abs() < 1e-9);
        }
        /// `record_hit` and `record_miss` return the new totals.
        #[test]
        fn record_returns_new_totals() {
            let mut s = CacheStats::default();
            assert_eq!(s.record_hit(), 1);
            assert_eq!(s.record_miss(), 1);
            assert_eq!(s.hits, 1);
            assert_eq!(s.misses, 1);
        }
    }
}
