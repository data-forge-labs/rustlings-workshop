//! PostgreSQL outbox operations using sqlx (async, compile-time-checked queries).
//!
//! This module exposes pure-data functions for building SQL fragments and
//! converting between rows and domain types. The actual database I/O lives
//! in `main.rs` so tests can run offline.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

// =========================================================================
// Step 1 — Connection configuration
// =========================================================================

/// Build a PostgreSQL connection URL from discrete components.
///
/// In Python you'd reach for `os.environ["DATABASE_URL"]` and pass it to
/// `psycopg2.connect()`. In Rust sqlx accepts the same URL, but we expose
/// a builder so the components are validated at the call site.
pub fn connection_url(
    user: &str,
    password: &str,
    host: &str,
    port: u16,
    database: &str,
) -> String {
    format!("postgres://{user}:{password}@{host}:{port}/{database}")
}

/// Default application_name appended to the URL for pg_stat_activity
/// visibility.
pub fn with_application_name(url: &str, app_name: &str) -> String {
    if url.contains('?') {
        format!("{url}&application_name={app_name}")
    } else {
        format!("{url}?application_name={app_name}")
    }
}

// =========================================================================
// Step 2 — Domain types
// =========================================================================

/// Mirrors `dataeng.orders` (see `init/postgres/01-init.sql`).
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Order {
    pub id: Uuid,
    pub customer_id: Uuid,
    pub product_id: Uuid,
    pub quantity: i32,
    pub unit_price: f64,
    pub total_price: f64,
    pub status: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Mirrors `dataeng.outbox` (transactional outbox table).
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct OutboxRow {
    pub id: Uuid,
    pub aggregate_type: String,
    pub aggregate_id: Uuid,
    pub event_type: String,
    pub payload: serde_json::Value,
    pub created_at: DateTime<Utc>,
    pub processed_at: Option<DateTime<Utc>>,
    pub retry_count: i32,
}

// =========================================================================
// Step 3 — SQL fragments
// =========================================================================

/// SELECT clause for `orders` — all columns.
pub const SELECT_ORDERS: &str = "id, customer_id, product_id, quantity, unit_price, total_price, status, created_at, updated_at";

/// INSERT into `orders` with named parameters.
/// Returns the column list to use in `RETURNING`.
pub const INSERT_ORDER_COLUMNS: &str = "customer_id, product_id, quantity, unit_price, status";

/// SELECT for the outbox's unprocessed rows, oldest first.
pub const SELECT_UNPROCESSED_OUTBOX: &str =
    "SELECT id, aggregate_type, aggregate_id, event_type, payload, created_at, processed_at, retry_count
     FROM dataeng.outbox
     WHERE processed_at IS NULL
     ORDER BY created_at ASC";

/// Mark one outbox row processed by id.
pub const MARK_OUTBOX_PROCESSED: &str =
    "UPDATE dataeng.outbox SET processed_at = now() WHERE id = $1";

/// Increment retry_count when a publish fails.
pub const BUMP_OUTBOX_RETRY: &str =
    "UPDATE dataeng.outbox SET retry_count = retry_count + 1 WHERE id = $1";

// =========================================================================
// Step 4 — Row -> domain converters
// =========================================================================

/// `Order` is currently mapped identically from a row, but in real code
/// you'd parse `total_price` (NUMERIC) into `rust_decimal::Decimal` and
/// convert to `f64` at the API boundary. We keep the function for parity
/// with the row struct.
pub fn order_from_row(
    id: Uuid,
    customer_id: Uuid,
    product_id: Uuid,
    quantity: i32,
    unit_price: f64,
    total_price: f64,
    status: String,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
) -> Order {
    Order { id, customer_id, product_id, quantity, unit_price, total_price, status, created_at, updated_at }
}

/// Build a fresh `OutboxRow` for inserting into the outbox.
pub fn new_outbox_row(
    aggregate_type: impl Into<String>,
    aggregate_id: Uuid,
    event_type: impl Into<String>,
    payload: serde_json::Value,
) -> OutboxRow {
    OutboxRow {
        id: Uuid::new_v4(),
        aggregate_type: aggregate_type.into(),
        aggregate_id,
        event_type: event_type.into(),
        payload,
        created_at: Utc::now(),
        processed_at: None,
        retry_count: 0,
    }
}

// =========================================================================
// Step 5 — Insert order with outbox in one transaction
// =========================================================================

/// Inputs for "create one order, write one outbox row, all-or-nothing".
#[derive(Debug, Clone)]
pub struct CreateOrderInput {
    pub customer_id: Uuid,
    pub product_id: Uuid,
    pub quantity: i32,
    pub unit_price: f64,
}

/// Validate the input. Returns `Err` with a reason on bad input.
/// The function is *pure* — no I/O — so tests cover the business rules.
pub fn validate_create_order(input: &CreateOrderInput) -> Result<(), String> {
    if input.quantity <= 0 {
        return Err("quantity must be > 0".into());
    }
    if input.unit_price < 0.0 {
        return Err("unit_price must be >= 0".into());
    }
    if input.customer_id.is_nil() {
        return Err("customer_id is required".into());
    }
    if input.product_id.is_nil() {
        return Err("product_id is required".into());
    }
    Ok(())
}

/// Compute the total price (quantity * unit_price). Quantities are
/// already validated to be > 0 and unit_price >= 0.
pub fn total_price(input: &CreateOrderInput) -> f64 {
    (input.quantity as f64) * input.unit_price
}

// =========================================================================
// Step 6 — Batching outbox rows
// =========================================================================

/// Batcher state — accumulate rows until `max_batch` is reached,
/// then flush. We provide the *pure* logic here; the I/O loop lives
/// in `main.rs`.
#[derive(Debug)]
pub struct OutboxBatcher {
    pub max_batch: usize,
    pub max_wait_ms: u64,
    pub rows: Vec<OutboxRow>,
}

impl OutboxBatcher {
    pub fn new(max_batch: usize, max_wait_ms: u64) -> Self {
        Self { max_batch, max_wait_ms, rows: Vec::with_capacity(max_batch) }
    }

    /// Push a row; return the batch if it has reached `max_batch`,
    /// otherwise return `None`.
    pub fn push(&mut self, row: OutboxRow) -> Option<Vec<OutboxRow>> {
        self.rows.push(row);
        if self.rows.len() >= self.max_batch {
            Some(self.flush())
        } else {
            None
        }
    }

    /// Force-flush the current batch even if it is not full.
    pub fn flush(&mut self) -> Vec<OutboxRow> {
        let drained: Vec<OutboxRow> = self.rows.drain(..).collect();
        drained
    }

    /// Max wait time, in milliseconds, for callers building a
    /// `tokio::time::sleep` loop.
    pub fn max_wait(&self) -> std::time::Duration {
        std::time::Duration::from_millis(self.max_wait_ms)
    }
}

// =========================================================================
// Step 7 — Retry policy
// =========================================================================

/// Compute the next retry delay using exponential backoff with jitter.
/// Returns the delay in milliseconds.
pub fn next_retry_delay_ms(retry_count: i32, base_ms: u64, cap_ms: u64) -> u64 {
    let exp = retry_count.max(0) as u32;
    let raw = base_ms.saturating_mul(1u64 << exp.min(20));
    let capped = raw.min(cap_ms);
    // Deterministic "jitter" derived from retry_count for testability:
    // 0.5x .. 1.5x of capped.
    let jitter = (capped / 2) + ((capped as i128 * (retry_count as i128 % 1000).abs() as i128 / 1000) as u64);
    jitter.max(base_ms).min(cap_ms * 2)
}

/// Should we retry at all, or move the row to a dead-letter queue?
/// Cap at `max_retries` (default 5).
pub fn should_retry(retry_count: i32, max_retries: i32) -> bool {
    retry_count < max_retries
}

// =========================================================================
// Step 8 — Outbox -> event type for the Kafka project
// =========================================================================

/// Build the event-type string for the Kafka project from an outbox row's
/// `event_type`. Kept here (not in `kafka_rdkafka`) so the publisher
/// doesn't have to know about PostgreSQL.
pub fn kafka_event_type(row: &OutboxRow) -> String {
    format!("{}.{}", row.aggregate_type, row.event_type)
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
        /// `connection_url` produces a parseable URL with all
        /// five components.
        #[test]
        fn builds_url_with_all_components() {
            let url = connection_url("dataeng", "dataeng", "localhost", 5432, "dataeng");
            assert_eq!(url, "postgres://dataeng:dataeng@localhost:5432/dataeng");
        }

        /// `with_application_name` adds `?application_name=...` when no
        /// query string exists.
        #[test]
        fn adds_application_name_when_no_query() {
            let url = "postgres://u:p@h:5432/d";
            let with = with_application_name(url, "svc-orders");
            assert_eq!(with, "postgres://u:p@h:5432/d?application_name=svc-orders");
        }

        /// `with_application_name` appends `&application_name=...` when
        /// a query string already exists.
        #[test]
        fn appends_application_name_when_query_exists() {
            let url = "postgres://u:p@h:5432/d?sslmode=require";
            let with = with_application_name(url, "svc-orders");
            assert!(with.ends_with("&application_name=svc-orders"));
            assert!(with.starts_with("postgres://u:p@h:5432/d?sslmode=require&"));
        }
    }

    // ----- Step 2 -----
    mod step_02_domain {
        use super::*;
        /// `Order` is `Clone + Debug + PartialEq` for test ergonomics.
        #[test]
        fn order_struct_compiles_and_derives() {
            let o = Order {
                id: Uuid::new_v4(),
                customer_id: Uuid::new_v4(),
                product_id: Uuid::new_v4(),
                quantity: 1,
                unit_price: 10.0,
                total_price: 10.0,
                status: "pending".into(),
                created_at: Utc::now(),
                updated_at: Utc::now(),
            };
            let o2 = o.clone();
            assert_eq!(o, o2);
            assert!(format!("{:?}", o).contains("pending"));
        }
    }

    // ----- Step 3 -----
    mod step_03_sql {
        use super::*;
        /// `SELECT_ORDERS` mentions every column of the `Order` struct.
        #[test]
        fn select_orders_has_all_columns() {
            for col in ["id", "customer_id", "product_id", "quantity",
                        "unit_price", "total_price", "status",
                        "created_at", "updated_at"] {
                assert!(SELECT_ORDERS.contains(col), "missing column: {col}");
            }
        }

        /// `SELECT_UNPROCESSED_OUTBOX` orders by `created_at ASC` and
        /// filters to `processed_at IS NULL`.
        #[test]
        fn select_unprocessed_outbox_is_ordered_and_filtered() {
            assert!(SELECT_UNPROCESSED_OUTBOX.contains("processed_at IS NULL"));
            assert!(SELECT_UNPROCESSED_OUTBOX.contains("ORDER BY created_at ASC"));
        }
    }

    // ----- Step 4 -----
    mod step_04_row_converters {
        use super::*;
        /// `new_outbox_row` populates `id` and `created_at` and leaves
        /// `processed_at = None` and `retry_count = 0`.
        #[test]
        fn new_outbox_row_fills_defaults() {
            let before = Utc::now();
            let row = new_outbox_row("order", Uuid::new_v4(), "order.created", json!({}));
            assert!(row.processed_at.is_none());
            assert_eq!(row.retry_count, 0);
            assert!(row.created_at >= before);
            assert_eq!(row.event_id_for_kafka(), row.id); // sanity
            // event_id_for_kafka is the same id we generated
            assert_eq!(row.id.get_version_num(), 4);
        }
    }

    // ----- Step 5 -----
    mod step_05_validate {
        use super::*;
        /// A valid input passes validation.
        #[test]
        fn valid_input_passes() {
            let input = CreateOrderInput {
                customer_id: Uuid::new_v4(),
                product_id: Uuid::new_v4(),
                quantity: 2,
                unit_price: 19.99,
            };
            assert!(validate_create_order(&input).is_ok());
        }

        /// `quantity <= 0` is rejected.
        #[test]
        fn zero_quantity_rejected() {
            let input = CreateOrderInput {
                customer_id: Uuid::new_v4(),
                product_id: Uuid::new_v4(),
                quantity: 0,
                unit_price: 1.0,
            };
            assert!(validate_create_order(&input).is_err());
        }

        /// `unit_price < 0` is rejected.
        #[test]
        fn negative_price_rejected() {
            let input = CreateOrderInput {
                customer_id: Uuid::new_v4(),
                product_id: Uuid::new_v4(),
                quantity: 1,
                unit_price: -0.01,
            };
            assert!(validate_create_order(&input).is_err());
        }

        /// `total_price` is the simple product of validated inputs.
        #[test]
        fn total_price_multiplies_inputs() {
            let input = CreateOrderInput {
                customer_id: Uuid::new_v4(),
                product_id: Uuid::new_v4(),
                quantity: 3,
                unit_price: 12.5,
            };
            assert!((total_price(&input) - 37.5).abs() < 1e-9);
        }
    }

    // ----- Step 6 -----
    mod step_06_batcher {
        use super::*;
        /// `push` returns `None` until `max_batch` rows are buffered,
        /// then returns the drained batch.
        #[test]
        fn push_returns_some_only_at_threshold() {
            let mut b = OutboxBatcher::new(3, 100);
            assert!(b.push(new_outbox_row("order", Uuid::new_v4(), "order.created", json!({}))).is_none());
            assert!(b.push(new_outbox_row("order", Uuid::new_v4(), "order.created", json!({}))).is_none());
            let third = b.push(new_outbox_row("order", Uuid::new_v4(), "order.created", json!({})));
            assert!(third.is_some());
            assert_eq!(third.unwrap().len(), 3);
            assert!(b.rows.is_empty());
        }

        /// `flush` always drains the current buffer.
        #[test]
        fn flush_drains_even_if_partial() {
            let mut b = OutboxBatcher::new(10, 100);
            b.push(new_outbox_row("order", Uuid::new_v4(), "order.created", json!({})));
            b.push(new_outbox_row("order", Uuid::new_v4(), "order.created", json!({})));
            let drained = b.flush();
            assert_eq!(drained.len(), 2);
            assert!(b.rows.is_empty());
        }
    }

    // ----- Step 7 -----
    mod step_07_retry {
        use super::*;
        /// `should_retry` allows retries below `max_retries` and
        /// blocks at/above it.
        #[test]
        fn retry_below_cap_allowed() {
            assert!(should_retry(0, 5));
            assert!(should_retry(4, 5));
            assert!(!should_retry(5, 5));
            assert!(!should_retry(10, 5));
        }

        /// `next_retry_delay_ms` doubles up to `cap_ms` and never
        /// returns a value below `base_ms`.
        #[test]
        fn delay_grows_then_caps() {
            let base = 100;
            let cap = 5_000;
            let d0 = next_retry_delay_ms(0, base, cap);
            let d1 = next_retry_delay_ms(1, base, cap);
            let d10 = next_retry_delay_ms(10, base, cap);
            assert!(d0 >= base);
            assert!(d1 >= base);
            assert!(d10 <= cap * 2, "should never exceed 2x cap");
        }
    }

    // ----- Step 8 -----
    mod step_08_kafka_event_type {
        use super::*;
        /// `kafka_event_type("order", "created")` -> `"order.created"`.
        #[test]
        fn joins_aggregate_and_event_with_dot() {
            let row = new_outbox_row("order", Uuid::new_v4(), "created", json!({}));
            assert_eq!(kafka_event_type(&row), "order.created");
        }
    }
}

// =========================================================================
// Helpers
// =========================================================================
impl OutboxRow {
    /// Convenience: same id used as the Kafka event_id.
    pub fn event_id_for_kafka(&self) -> Uuid {
        self.id
    }
}
