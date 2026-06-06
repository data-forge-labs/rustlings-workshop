//! DuckLake catalog: SQL builders, schema definitions, and merge logic
//! for an analytical lakehouse layer on top of DuckDB.
//!
//! DuckLake stores its catalog metadata in a SQL database (here: a
//! DuckDB file) and the actual data in Parquet files on disk or
//! object storage. This module exposes the *pure* SQL builders and
//! data shapes; `main.rs` opens a `duckdb::Connection` and runs the
//! SQL.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

// =========================================================================
// Step 1 — Connection configuration
// =========================================================================

/// Build a `duckdb:` URL for a file-backed catalog.
pub fn catalog_url(path: &str) -> String {
    format!("duckdb://{}", path)
}

/// Build a `duckdb:` URL for an in-memory catalog (used in tests).
pub fn in_memory_url() -> String {
    "duckdb://:memory:".into()
}

/// Build a S3/MinIO path for the data layer.
pub fn s3_data_path(bucket: &str, prefix: &str) -> String {
    format!("s3://{bucket}/{prefix}/")
}

// =========================================================================
// Step 2 — Domain types
// =========================================================================

/// One row in `lake.orders`.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct LakeOrder {
    pub id: Uuid,
    pub customer_id: Uuid,
    pub product_id: Uuid,
    pub quantity: u32,
    pub unit_price: f64,
    pub total_price: f64,
    pub status: String,
    pub created_at: DateTime<Utc>,
    /// DuckLake snapshot id this row was added in.
    pub snapshot_id: i64,
}

/// Catalog metadata for a single DuckLake table.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TableMetadata {
    pub table_name: String,
    pub schema_name: String,
    pub path: String,
    pub format: String,
    pub created_at: DateTime<Utc>,
    pub row_count: i64,
}

impl TableMetadata {
    pub fn new(table_name: impl Into<String>, schema_name: impl Into<String>, path: impl Into<String>) -> Self {
        Self {
            table_name: table_name.into(),
            schema_name: schema_name.into(),
            path: path.into(),
            format: "parquet".into(),
            created_at: Utc::now(),
            row_count: 0,
        }
    }
}

// =========================================================================
// Step 3 — SQL DDL
// =========================================================================

/// DuckLake attaches a catalog to a SQL backend and a data backend.
/// SQL backend: this DuckDB file. Data backend: local `data/` dir.
pub const ATTACH_DUCKLAKE: &str = "
ATTACH 'ducklake:metadata.duckdb' AS lakehouse (DATA_PATH 'data/')
";

/// Build a CREATE TABLE statement for `lake.<table>` with the
/// canonical order schema.
pub fn create_orders_table() -> String {
    "
    CREATE TABLE IF NOT EXISTS lake.orders (
        id          UUID NOT NULL,
        customer_id UUID NOT NULL,
        product_id  UUID NOT NULL,
        quantity    UINTEGER NOT NULL,
        unit_price  DOUBLE NOT NULL,
        total_price DOUBLE NOT NULL,
        status      VARCHAR NOT NULL,
        created_at  TIMESTAMP NOT NULL
    )
    ".to_string()
}

/// Build a SELECT statement that joins the latest snapshot view.
pub const SELECT_FROM_ORDERS: &str = "SELECT * FROM lake.orders";

// =========================================================================
// Step 4 — Snapshot helpers
// =========================================================================

/// Compute a deterministic snapshot id from a timestamp. Real
/// DuckLake generates these internally; we expose the same shape
/// so tests can assert against it.
pub fn snapshot_id_for(ts: DateTime<Utc>) -> i64 {
    ts.timestamp_millis()
}

/// Format a snapshot id for display (e.g. `"snap:1700000000000"`).
pub fn format_snapshot(id: i64) -> String {
    format!("snap:{id}")
}

// =========================================================================
// Step 5 — Time-travel queries
// =========================================================================

/// Build a SELECT statement that reads the table as of a specific
/// snapshot. DuckLake's time-travel syntax: `... AT (VERSION => N)`.
pub fn select_at_snapshot(snapshot: i64) -> String {
    format!("{SELECT_FROM_ORDERS} AT (VERSION => {snapshot})")
}

// =========================================================================
// Step 6 — Merge (upsert) helpers
// =========================================================================

/// Build a `MERGE INTO` statement that upserts by `id`.
pub fn merge_upsert_statement() -> String {
    "
    MERGE INTO lake.orders AS target
    USING (SELECT * FROM staging.orders_incoming) AS source
    ON target.id = source.id
    WHEN MATCHED THEN UPDATE SET
        customer_id = source.customer_id,
        product_id  = source.product_id,
        quantity    = source.quantity,
        unit_price  = source.unit_price,
        total_price = source.total_price,
        status      = source.status,
        created_at  = source.created_at
    WHEN NOT MATCHED THEN INSERT
        (id, customer_id, product_id, quantity, unit_price, total_price, status, created_at)
        VALUES
        (source.id, source.customer_id, source.product_id, source.quantity, source.unit_price, source.total_price, source.status, source.created_at)
    ".to_string()
}

// =========================================================================
// Step 7 — Compaction helpers
// =========================================================================

/// Decide if a table should be compacted based on the number of
/// small Parquet files. DuckLake benefits from compacting
/// ~100 MB files (target).
pub fn should_compact(metadata: &TableMetadata, small_file_count: i64) -> bool {
    small_file_count >= 10 && metadata.row_count > 100_000
}

/// Compute a target file size in bytes (default 128 MB).
pub fn target_file_size_bytes() -> i64 {
    128 * 1024 * 1024
}

// =========================================================================
// Step 8 — Cost-based aggregation
// =========================================================================

/// Aggregate count + revenue from a slice of lake orders.
#[derive(Debug, Clone, PartialEq)]
pub struct LakeStats {
    pub row_count: i64,
    pub total_revenue: f64,
    pub distinct_customers: i64,
}

/// Compute stats from in-memory rows. Useful for testing without
/// opening a real catalog.
pub fn compute_stats(rows: &[LakeOrder]) -> LakeStats {
    use std::collections::HashSet;
    let customers: HashSet<Uuid> = rows.iter().map(|r| r.customer_id).collect();
    LakeStats {
        row_count: rows.len() as i64,
        total_revenue: rows.iter().map(|r| r.total_price).sum(),
        distinct_customers: customers.len() as i64,
    }
}

// =========================================================================
// Tests
// =========================================================================
#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone;

    fn make_row(ts: i64, total: f64) -> LakeOrder {
        LakeOrder {
            id: Uuid::new_v4(),
            customer_id: Uuid::new_v4(),
            product_id: Uuid::new_v4(),
            quantity: 1,
            unit_price: total,
            total_price: total,
            status: "paid".into(),
            created_at: Utc.timestamp_opt(ts, 0).unwrap(),
            snapshot_id: snapshot_id_for(Utc.timestamp_opt(ts, 0).unwrap()),
        }
    }

    // ----- Step 1 -----
    mod step_01_connection {
        use super::*;
        /// `catalog_url` formats as `duckdb://<path>`.
        #[test]
        fn catalog_url_format() {
            assert_eq!(catalog_url("metadata.duckdb"), "duckdb://metadata.duckdb");
        }
        /// `in_memory_url` is `duckdb://:memory:`.
        #[test]
        fn in_memory_url() {
            assert_eq!(in_memory_url(), "duckdb://:memory:");
        }
        /// `s3_data_path` joins bucket and prefix.
        #[test]
        fn s3_data_path_joins() {
            assert_eq!(s3_data_path("lake", "orders"), "s3://lake/orders/");
        }
    }

    // ----- Step 2 -----
    mod step_02_domain {
        use super::*;
        /// `LakeOrder` round-trips through JSON.
        #[test]
        fn lake_order_round_trips() {
            let r = make_row(1_700_000_000, 99.5);
            let s = serde_json::to_string(&r).unwrap();
            let back: LakeOrder = serde_json::from_str(&s).unwrap();
            assert_eq!(r, back);
        }
        /// `TableMetadata::new` uses sensible defaults.
        #[test]
        fn table_metadata_defaults() {
            let m = TableMetadata::new("orders", "lake", "data/orders");
            assert_eq!(m.format, "parquet");
            assert_eq!(m.row_count, 0);
        }
    }

    // ----- Step 3 -----
    mod step_03_ddl {
        use super::*;
        /// `ATTACH_DUCKLAKE` references `ducklake:` and a `DATA_PATH`.
        #[test]
        fn attach_includes_data_path() {
            assert!(ATTACH_DUCKLAKE.contains("ducklake:"));
            assert!(ATTACH_DUCKLAKE.contains("DATA_PATH"));
        }
        /// `create_orders_table` mentions every column of the struct.
        #[test]
        fn create_orders_has_all_columns() {
            let sql = create_orders_table();
            for col in ["id","customer_id","product_id","quantity",
                        "unit_price","total_price","status","created_at"] {
                assert!(sql.contains(col), "missing: {col}");
            }
        }
    }

    // ----- Step 4 -----
    mod step_04_snapshot {
        use super::*;
        /// `snapshot_id_for` returns a millisecond timestamp.
        #[test]
        fn snapshot_id_is_millis() {
            let ts = Utc.timestamp_opt(1_700_000_000, 0).unwrap();
            assert_eq!(snapshot_id_for(ts), 1_700_000_000_000);
        }
        /// `format_snapshot` prefixes with `snap:`.
        #[test]
        fn format_snapshot_prefixes() {
            assert_eq!(format_snapshot(42), "snap:42");
        }
    }

    // ----- Step 5 -----
    mod step_05_time_travel {
        use super::*;
        /// `select_at_snapshot` uses DuckLake's `AT (VERSION => N)`.
        #[test]
        fn select_uses_at_version_clause() {
            let s = select_at_snapshot(99);
            assert!(s.contains("AT (VERSION => 99)"));
        }
    }

    // ----- Step 6 -----
    mod step_06_merge {
        use super::*;
        /// `merge_upsert_statement` has `WHEN MATCHED` and `WHEN NOT MATCHED`.
        #[test]
        fn merge_has_both_branches() {
            let s = merge_upsert_statement();
            assert!(s.contains("WHEN MATCHED THEN UPDATE"));
            assert!(s.contains("WHEN NOT MATCHED THEN INSERT"));
        }
    }

    // ----- Step 7 -----
    mod step_07_compaction {
        use super::*;
        /// `should_compact` returns `true` when many small files and
        /// many rows; `false` otherwise.
        #[test]
        fn should_compact_triggers_on_threshold() {
            let mut m = TableMetadata::new("orders", "lake", "data/orders");
            m.row_count = 1_000_000;
            assert!(should_compact(&m, 50));
            assert!(!should_compact(&m, 1));
            m.row_count = 100;
            assert!(!should_compact(&m, 50));
        }
        /// `target_file_size_bytes` is 128 MB.
        #[test]
        fn target_is_128_mb() {
            assert_eq!(target_file_size_bytes(), 128 * 1024 * 1024);
        }
    }

    // ----- Step 8 -----
    mod step_08_stats {
        use super::*;
        /// `compute_stats` sums revenue, counts distinct customers.
        #[test]
        fn stats_aggregate_correctly() {
            let c1 = Uuid::new_v4();
            let c2 = Uuid::new_v4();
            let mut rows = vec![
                make_row(1_700_000_000, 50.0),
                make_row(1_700_000_001, 75.0),
            ];
            rows[0].customer_id = c1;
            rows[1].customer_id = c1;
            let mut r3 = make_row(1_700_000_002, 25.0);
            r3.customer_id = c2;
            rows.push(r3);
            let s = compute_stats(&rows);
            assert_eq!(s.row_count, 3);
            assert!((s.total_revenue - 150.0).abs() < 1e-9);
            assert_eq!(s.distinct_customers, 2);
        }
    }
}
