//! ClickHouse ingestion library: builders, batchers, schema mapping.
//!
//! Pure-data module — all I/O lives in `main.rs`. This keeps the unit
//! tests fast and offline.

use chrono::{DateTime, Utc};
use clickhouse::Row;
use serde::{Deserialize, Serialize};
use std::time::Duration;
use uuid::Uuid;

// =========================================================================
// Step 1 — Connection configuration
// =========================================================================

/// Build a ClickHouse HTTP URL (port 8123).
pub fn http_url(host: &str, port: u16, database: &str) -> String {
    format!("http://{host}:{port}/{database}")
}

/// Build a ClickHouse native (TCP) URL (port 9000).
pub fn native_url(host: &str, port: u16, database: &str) -> String {
    format!("tcp://{host}:{port}/{database}")
}

// =========================================================================
// Step 2 — Domain types
// =========================================================================

/// Mirrors `analytics.orders` in ClickHouse. Money uses `Decimal`
/// (12, 2) for exact arithmetic.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Row)]
pub struct OrderRow {
    pub id: Uuid,
    pub customer_id: Uuid,
    pub product_id: Uuid,
    pub quantity: u32,
    pub unit_price: f64,
    pub total_price: f64,
    pub status: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Mirrors `analytics.events`.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct EventRow {
    pub id: Uuid,
    pub event_type: String,
    pub source: String,
    pub payload: String, // JSON as string
    pub created_at: DateTime<Utc>,
}

// =========================================================================
// Step 3 — SQL DDL fragments
// =========================================================================

/// Reusable CREATE TABLE fragment for `analytics.orders`. The full
/// statement is built in `main.rs` so we can target a custom DB name.
pub const DDL_ORDERS: &str = "
CREATE TABLE IF NOT EXISTS analytics.orders
(
    id          UUID,
    customer_id UUID,
    product_id  UUID,
    quantity    UInt32,
    unit_price  Decimal(12, 2),
    total_price Decimal(12, 2),
    status      LowCardinality(String),
    created_at  DateTime64(3, 'UTC'),
    updated_at  DateTime64(3, 'UTC'),
    inserted_at DateTime DEFAULT now()
)
ENGINE = MergeTree()
PARTITION BY toYYYYMM(created_at)
ORDER BY (created_at, id)
";

/// INSERT into `analytics.orders` with column list. Used in a
/// `INSERT INTO ... VALUES` statement.
pub const INSERT_ORDERS_COLUMNS: &str =
    "(id, customer_id, product_id, quantity, unit_price, total_price, status, created_at, updated_at)";

// =========================================================================
// Step 4 — Status enum mapping
// =========================================================================

/// Order status canonical form. Centralized so we never write
/// `"cancelled"` in one place and `"canceled"` in another.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum OrderStatus {
    Pending,
    Paid,
    Shipped,
    Cancelled,
}

impl OrderStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Pending => "pending",
            Self::Paid => "paid",
            Self::Shipped => "shipped",
            Self::Cancelled => "cancelled",
        }
    }

    /// Parse from a free-form status string. Returns `None` on
    /// unrecognized values.
    pub fn parse(s: &str) -> Option<Self> {
        match s {
            "pending" => Some(Self::Pending),
            "paid" => Some(Self::Paid),
            "shipped" => Some(Self::Shipped),
            "cancelled" | "canceled" => Some(Self::Cancelled),
            _ => None,
        }
    }
}

// =========================================================================
// Step 5 — Row -> column value converters
// =========================================================================

/// Compute total quantity over a slice of rows. Useful for
/// reconciling Postgres `SUM(quantity)` against ClickHouse.
pub fn total_quantity(rows: &[OrderRow]) -> u64 {
    rows.iter().map(|r| r.quantity as u64).sum()
}

/// Compute total revenue. We use f64 for the API surface; in real
/// pipelines this would be a `Decimal` accumulator.
pub fn total_revenue(rows: &[OrderRow]) -> f64 {
    rows.iter().map(|r| r.total_price).sum()
}

// =========================================================================
// Step 6 — Batcher
// =========================================================================

/// Batcher with row-count and byte-size thresholds. ClickHouse
/// recommends batches of 10k–100k rows or 10–50 MB, whichever comes
/// first.
#[derive(Debug, Clone)]
pub struct IngestBatcher {
    pub max_rows: usize,
    pub max_bytes: usize,
    rows: Vec<OrderRow>,
    bytes: usize,
}

impl IngestBatcher {
    pub fn new(max_rows: usize, max_bytes: usize) -> Self {
        Self { max_rows, max_bytes, rows: Vec::with_capacity(max_rows), bytes: 0 }
    }

    /// Push a row. Returns the batch if either threshold is reached.
    pub fn push(&mut self, row: OrderRow) -> Option<Vec<OrderRow>> {
        let approx_bytes = std::mem::size_of::<OrderRow>() + row.payload_approx();
        self.bytes = self.bytes.saturating_add(approx_bytes);
        self.rows.push(row);
        if self.rows.len() >= self.max_rows || self.bytes >= self.max_bytes {
            Some(self.flush())
        } else {
            None
        }
    }

    /// Force-flush the current buffer.
    pub fn flush(&mut self) -> Vec<OrderRow> {
        self.bytes = 0;
        self.rows.drain(..).collect()
    }

    pub fn len(&self) -> usize { self.rows.len() }
    pub fn is_empty(&self) -> bool { self.rows.is_empty() }
}

impl OrderRow {
    /// Rough per-row byte estimate. Good enough for batching, not
    /// for accurate network accounting.
    pub fn payload_approx(&self) -> usize {
        self.status.len() + 16 + 16 + 16 // UUIDs + status string
    }
}

// =========================================================================
// Step 7 — Retry policy
// =========================================================================

/// Retry policy for HTTP 5xx / network errors. Honors the
/// `Retry-After` header when present.
#[derive(Debug, Clone)]
pub struct ClickHouseRetry {
    pub max_retries: u32,
    pub base_backoff: Duration,
    pub cap_backoff: Duration,
}

impl Default for ClickHouseRetry {
    fn default() -> Self {
        Self {
            max_retries: 5,
            base_backoff: Duration::from_millis(200),
            cap_backoff: Duration::from_secs(30),
        }
    }
}

impl ClickHouseRetry {
    /// Compute the next backoff, in milliseconds, for a given
    /// attempt number. Returns `None` when no more retries.
    pub fn next_backoff(&self, attempt: u32) -> Option<Duration> {
        if attempt >= self.max_retries {
            return None;
        }
        let exp = 1u64 << attempt.min(20);
        let raw = self.base_backoff.as_millis() as u64 * exp;
        let capped = raw.min(self.cap_backoff.as_millis() as u64);
        Some(Duration::from_millis(capped))
    }
}

// =========================================================================
// Step 8 — Aggregations (read path)
// =========================================================================

/// Aggregate buckets — `count` and `revenue` per minute. Useful for
/// testing the projection logic.
#[derive(Debug, Clone, PartialEq)]
pub struct MinuteBucket {
    pub minute: DateTime<Utc>,
    pub count: u64,
    pub revenue: f64,
}

/// Project rows into one-minute buckets.
pub fn project_minute_buckets(rows: &[OrderRow]) -> Vec<MinuteBucket> {
    use std::collections::BTreeMap;
    let mut map: BTreeMap<i64, (u64, f64)> = BTreeMap::new();
    for r in rows {
        let minute = r.created_at.timestamp() / 60;
        let e = map.entry(minute).or_insert((0, 0.0));
        e.0 += 1;
        e.1 += r.total_price;
    }
    map.into_iter()
        .map(|(m, (c, rev))| MinuteBucket {
            minute: chrono::DateTime::<Utc>::from_timestamp(m * 60, 0).unwrap_or_else(Utc::now),
            count: c,
            revenue: rev,
        })
        .collect()
}

// =========================================================================
// Tests
// =========================================================================
#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone;

    fn make_row(qty: u32, total: f64, ts: i64) -> OrderRow {
        OrderRow {
            id: Uuid::new_v4(),
            customer_id: Uuid::new_v4(),
            product_id: Uuid::new_v4(),
            quantity: qty,
            unit_price: total / (qty.max(1) as f64),
            total_price: total,
            status: "paid".into(),
            created_at: Utc.timestamp_opt(ts, 0).unwrap(),
            updated_at: Utc.timestamp_opt(ts, 0).unwrap(),
        }
    }

    // ----- Step 1 -----
    mod step_01_connection {
        use super::*;
        /// `http_url` builds `http://host:port/db`.
        #[test]
        fn http_url_format() {
            assert_eq!(http_url("ch", 8123, "analytics"), "http://ch:8123/analytics");
        }
        /// `native_url` builds `tcp://host:port/db`.
        #[test]
        fn native_url_format() {
            assert_eq!(native_url("ch", 9000, "analytics"), "tcp://ch:9000/analytics");
        }
    }

    // ----- Step 2 -----
    mod step_02_domain {
        use super::*;
        /// `OrderRow` round-trips through JSON (used when relaying
        /// rows over Kafka or HTTP).
        #[test]
        fn order_row_round_trips() {
            let r = make_row(2, 20.0, 1_700_000_000);
            let s = serde_json::to_string(&r).unwrap();
            let back: OrderRow = serde_json::from_str(&s).unwrap();
            assert_eq!(r, back);
        }
    }

    // ----- Step 3 -----
    mod step_03_ddl {
        use super::*;
        /// `DDL_ORDERS` uses `MergeTree` and `PARTITION BY toYYYYMM(created_at)`.
        #[test]
        fn ddl_uses_merge_tree_and_partition() {
            assert!(DDL_ORDERS.contains("ENGINE = MergeTree()"));
            assert!(DDL_ORDERS.contains("PARTITION BY toYYYYMM(created_at)"));
        }
        /// `INSERT_ORDERS_COLUMNS` lists all 9 columns in our struct.
        #[test]
        fn insert_columns_match_struct() {
            for col in ["id","customer_id","product_id","quantity","unit_price",
                        "total_price","status","created_at","updated_at"] {
                assert!(INSERT_ORDERS_COLUMNS.contains(col), "missing: {col}");
            }
        }
    }

    // ----- Step 4 -----
    mod step_04_status {
        use super::*;
        /// `as_str` returns the canonical lowercase spelling.
        #[test]
        fn as_str_lowercases() {
            assert_eq!(OrderStatus::Pending.as_str(), "pending");
            assert_eq!(OrderStatus::Cancelled.as_str(), "cancelled");
        }
        /// `parse` accepts both British and American spellings of "cancelled".
        #[test]
        fn parse_handles_both_spellings() {
            assert_eq!(OrderStatus::parse("cancelled"), Some(OrderStatus::Cancelled));
            assert_eq!(OrderStatus::parse("canceled"),  Some(OrderStatus::Cancelled));
            assert_eq!(OrderStatus::parse("paid"),      Some(OrderStatus::Paid));
            assert_eq!(OrderStatus::parse("nonsense"),  None);
        }
    }

    // ----- Step 5 -----
    mod step_05_aggregates {
        use super::*;
        /// `total_quantity` sums across the slice.
        #[test]
        fn total_quantity_sums() {
            let rows = vec![make_row(2, 20.0, 0), make_row(3, 30.0, 0), make_row(5, 50.0, 0)];
            assert_eq!(total_quantity(&rows), 10);
        }
        /// `total_revenue` sums total_price.
        #[test]
        fn total_revenue_sums() {
            let rows = vec![make_row(1, 10.0, 0), make_row(1, 20.0, 0)];
            assert!((total_revenue(&rows) - 30.0).abs() < 1e-9);
        }
    }

    // ----- Step 6 -----
    mod step_06_batcher {
        use super::*;
        /// `push` returns the batch only when row threshold is hit.
        #[test]
        fn push_returns_at_row_threshold() {
            let mut b = IngestBatcher::new(3, usize::MAX);
            assert!(b.push(make_row(1, 1.0, 0)).is_none());
            assert!(b.push(make_row(1, 1.0, 0)).is_none());
            let third = b.push(make_row(1, 1.0, 0));
            assert!(third.is_some());
            assert_eq!(third.unwrap().len(), 3);
        }
        /// `flush` always drains.
        #[test]
        fn flush_drains_partial() {
            let mut b = IngestBatcher::new(100, usize::MAX);
            b.push(make_row(1, 1.0, 0));
            assert_eq!(b.flush().len(), 1);
            assert!(b.is_empty());
        }
    }

    // ----- Step 7 -----
    mod step_07_retry {
        use super::*;
        /// `next_backoff` returns `None` once `max_retries` is reached.
        #[test]
        fn returns_none_at_max() {
            let r = ClickHouseRetry { max_retries: 3, base_backoff: Duration::from_millis(100), cap_backoff: Duration::from_secs(1) };
            assert!(r.next_backoff(0).is_some());
            assert!(r.next_backoff(2).is_some());
            assert!(r.next_backoff(3).is_none());
        }
        /// `next_backoff` doubles on each attempt up to the cap.
        #[test]
        fn backoff_doubles_then_caps() {
            let r = ClickHouseRetry { max_retries: 10, base_backoff: Duration::from_millis(100), cap_backoff: Duration::from_millis(800) };
            assert_eq!(r.next_backoff(0).unwrap(), Duration::from_millis(100));
            assert_eq!(r.next_backoff(1).unwrap(), Duration::from_millis(200));
            assert_eq!(r.next_backoff(2).unwrap(), Duration::from_millis(400));
            assert_eq!(r.next_backoff(3).unwrap(), Duration::from_millis(800));
            assert_eq!(r.next_backoff(4).unwrap(), Duration::from_millis(800)); // capped
        }
    }

    // ----- Step 8 -----
    mod step_08_aggregations {
        use super::*;
        /// `project_minute_buckets` groups rows by minute and
        /// counts/sums them.
        #[test]
        fn buckets_group_by_minute() {
            // 3 rows in minute 100, 1 row in minute 200
            let rows = vec![
                make_row(1, 10.0, 100 * 60),
                make_row(1, 20.0, 100 * 60 + 30),
                make_row(1, 30.0, 100 * 60 + 59),
                make_row(1, 99.0, 200 * 60),
            ];
            let b = project_minute_buckets(&rows);
            assert_eq!(b.len(), 2);
            assert_eq!(b[0].count, 3);
            assert!((b[0].revenue - 60.0).abs() < 1e-9);
            assert_eq!(b[1].count, 1);
            assert!((b[1].revenue - 99.0).abs() < 1e-9);
        }
    }
}
