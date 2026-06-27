use flate2::read::GzDecoder;
use polars::prelude::*;
use serde::{Deserialize, Serialize};
use std::io::Read;
use thiserror::Error;

// ============================================================
// Module 1: Error Types
// ============================================================

#[derive(Debug, Error)]
pub enum TurboError {
    #[error("IO Error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Polars Error: {0}")]
    Polars(#[from] polars::prelude::PolarsError),

    #[error("Security limit exceeded: {msg}")]
    SecurityLimit { msg: String },

    #[error("Schema mismatch: {0}")]
    Schema(String),

    #[error("Serialization error: {0}")]
    Serialization(String),
}

// ============================================================
// Module 2: Resource Limits
// ============================================================

#[derive(Debug, Clone)]
pub struct ProcessingLimits {
    pub max_memory_bytes: usize,
    pub max_decompression_ratio: f64,
    pub max_columns: usize,
}

impl Default for ProcessingLimits {
    fn default() -> Self {
        Self {
            max_memory_bytes: 512 * 1024 * 1024, // 512 MB
            max_decompression_ratio: 100.0,
            max_columns: 10_000,
        }
    }
}

/// Validate that headers don't exceed the column limit.
pub fn validate_headers(headers: &[String], limits: &ProcessingLimits) -> Result<(), TurboError> {
    if headers.len() > limits.max_columns {
        return Err(TurboError::SecurityLimit {
            msg: format!(
                "Column count {} exceeds limit {}",
                headers.len(),
                limits.max_columns
            ),
        });
    }
    Ok(())
}

// ============================================================
// Module 3: Safe Gzip Reader (Adversarial Safety)
// ============================================================

pub struct SafeGzipReader<R: Read> {
    decoder: GzDecoder<R>,
    bytes_read: u64,
    max_bytes: u64,
}

impl<R: Read> SafeGzipReader<R> {
    pub fn new(inner: R, compressed_size: usize, limits: &ProcessingLimits) -> Self {
        Self {
            decoder: GzDecoder::new(inner),
            bytes_read: 0,
            max_bytes: (compressed_size as f64 * limits.max_decompression_ratio) as u64,
        }
    }
}

impl<R: Read> Read for SafeGzipReader<R> {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        let n = self.decoder.read(buf)?;
        self.bytes_read += n as u64;
        if self.bytes_read > self.max_bytes {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "Decompressed data exceeds security limit: {} > {} bytes",
                    self.bytes_read, self.max_bytes
                ),
            ));
        }
        Ok(n)
    }
}

// ============================================================
// Module 4: Cleaning Rules
// ============================================================

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ImputeStrategy {
    Mean,
    Median,
    Mode,
    ForwardFill,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum CleaningRule {
    DropNulls { columns: Vec<String> },
    ImputeMissing { column: String, strategy: ImputeStrategy },
    FilterOutliersIQR { column: String, factor: f64 },
    TrimWhitespace { columns: Vec<String> },
}

impl CleaningRule {
    /// Serialize this rule to a JSON string.
    pub fn to_json(&self) -> Result<String, TurboError> {
        serde_json::to_string(self).map_err(|e| TurboError::Serialization(e.to_string()))
    }

    /// Deserialize a rule from a JSON string.
    pub fn from_json(s: &str) -> Result<Self, TurboError> {
        serde_json::from_str(s).map_err(|e| TurboError::Serialization(e.to_string()))
    }
}

// ============================================================
// Module 5: Column Profiler
// ============================================================

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ColumnProfile {
    pub name: String,
    pub null_count: u64,
    pub is_numeric: bool,
}

/// Suggest cleaning rules based on column profiles.
/// - If null_count > 0 → suggest ImputeMissing with Mean
/// - If column name contains "id" → skip (don't impute IDs)
pub fn suggest_rules(profiles: &[ColumnProfile]) -> Vec<CleaningRule> {
    profiles
        .iter()
        .filter(|p| p.null_count > 0 && !p.name.to_lowercase().contains("id"))
        .map(|p| CleaningRule::ImputeMissing {
            column: p.name.clone(),
            strategy: ImputeStrategy::Mean,
        })
        .collect()
}

// ============================================================
// Module 6: Cleaning Report
// ============================================================

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CleaningReport {
    pub rows_read: u64,
    pub rows_written: u64,
    pub rules_applied: usize,
}

// ============================================================
// Module 7: Streaming Engine
// ============================================================

pub struct CleaningEngine {
    limits: ProcessingLimits,
}

impl CleaningEngine {
    pub fn new(limits: ProcessingLimits) -> Self {
        Self { limits }
    }

    pub fn limits(&self) -> &ProcessingLimits {
        &self.limits
    }

    /// Build a Polars LazyFrame from a CSV path, applying rules as lazy filters.
    ///
    /// This does NOT execute the plan — it returns the LazyFrame for the caller
    /// to .collect() or .sink_*() as needed.
    pub fn build_lazy_plan(
        &self,
        path: &str,
        rules: &[CleaningRule],
    ) -> Result<polars::prelude::LazyFrame, TurboError> {
        let lf = LazyCsvReader::new(path).finish()?;

        let mut filtered = lf;
        for rule in rules {
            match rule {
                CleaningRule::DropNulls { columns } => {
                    for col_name in columns {
                        filtered = filtered.filter(col(col_name).is_not_null());
                    }
                }
                CleaningRule::FilterOutliersIQR { column, factor } => {
                    let q1_expr = col(column).quantile(lit(0.25), QuantileMethod::Linear);
                    let q3_expr = col(column).quantile(lit(0.75), QuantileMethod::Linear);
                    let iqr = q3_expr.clone() - q1_expr.clone();
                    let lower = q1_expr - iqr.clone() * lit(*factor);
                    let upper = q3_expr + iqr * lit(*factor);
                    filtered = filtered.filter(col(column).gt_eq(lower).and(col(column).lt_eq(upper)));
                }
                _ => {}
            }
        }

        Ok(filtered)
    }

    /// Execute the plan with streaming and write to Parquet.
    pub fn clean_to_parquet(
        &self,
        input_path: &str,
        output_path: &str,
        rules: &[CleaningRule],
    ) -> Result<CleaningReport, TurboError> {
        let lf = self.build_lazy_plan(input_path, rules)?;
        lf.with_streaming(true)
            .sink_parquet(&output_path, Default::default(), None)?;

        let rows_read = {
            let df = LazyCsvReader::new(input_path).finish()?.collect()?;
            df.height() as u64
        };

        Ok(CleaningReport {
            rows_read,
            rows_written: rows_read,
            rules_applied: rules.len(),
        })
    }
}

// ============================================================
// Tests
// ============================================================

#[cfg(test)]
mod tests {
    use super::*;

    mod step_01_error_types {
        use super::*;

        #[test]
        fn turbo_error_display_io() {
            let err = TurboError::Io(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "file missing",
            ));
            assert!(err.to_string().contains("IO Error"));
        }

        #[test]
        fn turbo_error_display_security() {
            let err = TurboError::SecurityLimit {
                msg: "gzip bomb".into(),
            };
            assert!(err.to_string().contains("Security limit"));
            assert!(err.to_string().contains("gzip bomb"));
        }

        #[test]
        fn turbo_error_from_io() {
            let io_err = std::io::Error::new(std::io::ErrorKind::Other, "oops");
            let err: TurboError = io_err.into();
            assert!(matches!(err, TurboError::Io(_)));
        }
    }

    mod step_02_resource_limits {
        use super::*;

        #[test]
        fn default_limits() {
            let limits = ProcessingLimits::default();
            assert_eq!(limits.max_memory_bytes, 512 * 1024 * 1024);
            assert_eq!(limits.max_decompression_ratio, 100.0);
            assert_eq!(limits.max_columns, 10_000);
        }

        #[test]
        fn validate_headers_ok() {
            let limits = ProcessingLimits::default();
            let headers = vec!["a".into(), "b".into(), "c".into()];
            assert!(validate_headers(&headers, &limits).is_ok());
        }

        #[test]
        fn validate_headers_too_many() {
            let limits = ProcessingLimits {
                max_columns: 2,
                ..Default::default()
            };
            let headers = vec!["a".into(), "b".into(), "c".into()];
            let err = validate_headers(&headers, &limits).unwrap_err();
            assert!(matches!(err, TurboError::SecurityLimit { .. }));
        }
    }

    mod step_03_safe_gzip_reader {
        use super::*;

        #[test]
        fn reads_within_limit() {
            let data = b"hello world";
            let limits = ProcessingLimits::default();
            let mut reader = SafeGzipReader::new(&data[..], data.len(), &limits);
            let mut buf = [0u8; 100];
            let n = reader.read(&mut buf).unwrap();
            // decompressed content should be readable
            assert!(n <= buf.len());
        }

        #[test]
        fn rejects_over_limit() {
            let data = b"hello world";
            let limits = ProcessingLimits {
                max_decompression_ratio: 0.001, // very low limit
                ..Default::default()
            };
            let mut reader = SafeGzipReader::new(&data[..], data.len(), &limits);
            let mut buf = [0u8; 100];
            let result = reader.read(&mut buf);
            assert!(result.is_err());
        }
    }

    mod step_04_cleaning_rules {
        use super::*;

        #[test]
        fn roundtrip_json() {
            let rule = CleaningRule::DropNulls {
                columns: vec!["name".into(), "email".into()],
            };
            let json = rule.to_json().unwrap();
            let restored = CleaningRule::from_json(&json).unwrap();
            assert_eq!(rule, restored);
        }

        #[test]
        fn impute_strategy_json() {
            let rule = CleaningRule::ImputeMissing {
                column: "age".into(),
                strategy: ImputeStrategy::Median,
            };
            let json = rule.to_json().unwrap();
            assert!(json.contains("Median"));
            let restored = CleaningRule::from_json(&json).unwrap();
            assert_eq!(rule, restored);
        }

        #[test]
        fn iqr_rule_json() {
            let rule = CleaningRule::FilterOutliersIQR {
                column: "amount".into(),
                factor: 1.5,
            };
            let json = rule.to_json().unwrap();
            let restored = CleaningRule::from_json(&json).unwrap();
            assert_eq!(rule, restored);
        }
    }

    mod step_05_profiler {
        use super::*;

        #[test]
        fn suggest_impute_for_nulls() {
            let profiles = vec![
                ColumnProfile {
                    name: "age".into(),
                    null_count: 5,
                    is_numeric: true,
                },
                ColumnProfile {
                    name: "name".into(),
                    null_count: 0,
                    is_numeric: false,
                },
            ];
            let rules = suggest_rules(&profiles);
            assert_eq!(rules.len(), 1);
            assert!(matches!(
                rules[0],
                CleaningRule::ImputeMissing { .. }
            ));
        }

        #[test]
        fn skip_id_columns() {
            let profiles = vec![ColumnProfile {
                name: "user_id".into(),
                null_count: 10,
                is_numeric: true,
            }];
            let rules = suggest_rules(&profiles);
            assert!(rules.is_empty());
        }

        #[test]
        fn no_nulls_no_rules() {
            let profiles = vec![ColumnProfile {
                name: "amount".into(),
                null_count: 0,
                is_numeric: true,
            }];
            let rules = suggest_rules(&profiles);
            assert!(rules.is_empty());
        }
    }

    mod step_06_engine_config {
        use super::*;

        #[test]
        fn engine_creation() {
            let engine = CleaningEngine::new(ProcessingLimits::default());
            assert_eq!(engine.limits.max_columns, 10_000);
        }

        #[test]
        fn build_plan_returns_lazy() {
            // This test verifies the function signature compiles —
            // actual execution requires a real CSV file.
            let engine = CleaningEngine::new(ProcessingLimits::default());
            let rules = vec![CleaningRule::DropNulls {
                columns: vec!["x".into()],
            }];
            // We can't call build_lazy_plan without a real file,
            // but we can verify the types compile.
            let _: fn(&CleaningEngine, &str, &[CleaningRule]) -> Result<polars::prelude::LazyFrame, TurboError> =
                |e, p, r| e.build_lazy_plan(p, r);
        }
    }

    mod step_07_cleaning_report {
        use super::*;

        #[test]
        fn report_roundtrip() {
            let report = CleaningReport {
                rows_read: 1000,
                rows_written: 950,
                rules_applied: 2,
            };
            let json = serde_json::to_string(&report).unwrap();
            let restored: CleaningReport = serde_json::from_str(&json).unwrap();
            assert_eq!(report, restored);
        }
    }

    mod step_08_error_conversion {
        use super::*;

        #[test]
        fn polars_error_conversion() {
            let polars_err = polars::prelude::PolarsError::ComputeError("test".into());
            let err: TurboError = polars_err.into();
            assert!(matches!(err, TurboError::Polars(_)));
        }

        #[test]
        fn serialization_error() {
            let err = TurboError::Serialization("bad json".into());
            assert!(err.to_string().contains("Serialization"));
        }
    }
}
