use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::Duration;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct LogLine {
    pub timestamp: DateTime<Utc>,
    pub level: String,
    pub target: String,
    pub message: String,
    pub correlation_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SpanRecord {
    pub name: String,
    pub correlation_id: String,
    pub start: DateTime<Utc>,
    pub duration_ms: u64,
    pub attributes: std::collections::HashMap<String, String>,
}

pub fn new_correlation_id() -> String {
    Uuid::new_v4().to_string()
}

pub fn format_log_line(level: &str, target: &str, message: &str, correlation_id: Option<&str>) -> LogLine {
    LogLine {
        timestamp: Utc::now(),
        level: level.to_string(),
        target: target.to_string(),
        message: message.to_string(),
        correlation_id: correlation_id.map(|s| s.to_string()),
    }
}

pub fn build_span(
    name: &str,
    correlation_id: &str,
    attributes: std::collections::HashMap<String, String>,
    duration: Duration,
) -> SpanRecord {
    SpanRecord {
        name: name.to_string(),
        correlation_id: correlation_id.to_string(),
        start: Utc::now(),
        duration_ms: duration.as_millis() as u64,
        attributes,
    }
}

pub fn with_correlation<F: FnOnce(&str) -> R, R>(correlation_id: &str, f: F) -> R {
    f(correlation_id)
}

pub fn parse_log_level(s: &str) -> Result<u8, String> {
    match s.to_uppercase().as_str() {
        "TRACE" => Ok(10),
        "DEBUG" => Ok(15),
        "INFO" => Ok(20),
        "WARN" => Ok(30),
        "ERROR" => Ok(40),
        _ => Err(format!("unknown log level: {}", s)),
    }
}

pub fn span_duration_ms(d: Duration) -> u64 {
    d.as_millis() as u64
}

pub fn merge_attributes(
    a: std::collections::HashMap<String, String>,
    b: std::collections::HashMap<String, String>,
) -> std::collections::HashMap<String, String> {
    let mut merged = a;
    for (k, v) in b {
        merged.insert(k, v);
    }
    merged
}

pub fn otel_attribute(key: &str, value: &str) -> (String, String) {
    (key.to_string(), value.to_string())
}

pub struct PipelineMetrics {
    pub processed: AtomicU64,
    pub failed: AtomicU64,
    pub spans_emitted: AtomicU64,
}

impl PipelineMetrics {
    pub fn new() -> Self {
        Self {
            processed: AtomicU64::new(0),
            failed: AtomicU64::new(0),
            spans_emitted: AtomicU64::new(0),
        }
    }

    pub fn record_success(&self) {
        self.processed.fetch_add(1, Ordering::Relaxed);
    }

    pub fn record_failure(&self) {
        self.failed.fetch_add(1, Ordering::Relaxed);
    }

    pub fn record_span(&self) {
        self.spans_emitted.fetch_add(1, Ordering::Relaxed);
    }

    pub fn snapshot(&self) -> (u64, u64, u64) {
        (
            self.processed.load(Ordering::Relaxed),
            self.failed.load(Ordering::Relaxed),
            self.spans_emitted.load(Ordering::Relaxed),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    mod step_01_correlation {
        use super::*;

        #[test]
        fn test_correlation_id_is_uuid() {
            let id = new_correlation_id();
            assert!(Uuid::parse_str(&id).is_ok());
        }

        #[test]
        fn test_correlation_ids_unique() {
            let a = new_correlation_id();
            let b = new_correlation_id();
            assert_ne!(a, b);
        }
    }

    mod step_02_log_line {
        use super::*;

        #[test]
        fn test_format_log_line_basic() {
            let line = format_log_line("INFO", "auth", "user logged in", None);
            assert_eq!(line.level, "INFO");
            assert_eq!(line.target, "auth");
            assert_eq!(line.message, "user logged in");
            assert!(line.correlation_id.is_none());
        }

        #[test]
        fn test_format_log_line_with_correlation() {
            let line = format_log_line("WARN", "etl", "retry", Some("abc-123"));
            assert_eq!(line.correlation_id.as_deref(), Some("abc-123"));
        }
    }

    mod step_03_span {
        use super::*;

        #[test]
        fn test_build_span() {
            let mut attrs = HashMap::new();
            attrs.insert("pipeline".into(), "orders".into());
            let s = build_span("ingest", "cid-1", attrs, Duration::from_millis(123));
            assert_eq!(s.name, "ingest");
            assert_eq!(s.correlation_id, "cid-1");
            assert_eq!(s.duration_ms, 123);
            assert_eq!(s.attributes.get("pipeline").unwrap(), "orders");
        }
    }

    mod step_04_with_correlation {
        use super::*;

        #[test]
        fn test_with_correlation_passes_id() {
            let result = with_correlation("cid-1", |id| id.to_string());
            assert_eq!(result, "cid-1");
        }

        #[test]
        fn test_with_correlation_can_return_value() {
            let result = with_correlation("cid-1", |_id| 42);
            assert_eq!(result, 42);
        }
    }

    mod step_05_log_level {
        use super::*;

        #[test]
        fn test_parse_log_level_ok() {
            assert_eq!(parse_log_level("INFO").unwrap(), 20);
            assert_eq!(parse_log_level("ERROR").unwrap(), 40);
        }

        #[test]
        fn test_parse_log_level_unknown() {
            assert!(parse_log_level("LOUD").is_err());
        }
    }

    mod step_06_duration {
        use super::*;

        #[test]
        fn test_span_duration_ms() {
            assert_eq!(span_duration_ms(Duration::from_millis(250)), 250);
            assert_eq!(span_duration_ms(Duration::from_secs(2)), 2000);
        }
    }

    mod step_07_merge {
        use super::*;

        #[test]
        fn test_merge_attributes_disjoint() {
            let mut a = HashMap::new();
            a.insert("k1".into(), "v1".into());
            let mut b = HashMap::new();
            b.insert("k2".into(), "v2".into());
            let merged = merge_attributes(a, b);
            assert_eq!(merged.len(), 2);
        }

        #[test]
        fn test_merge_attributes_overlap() {
            let mut a = HashMap::new();
            a.insert("k".into(), "v1".into());
            let mut b = HashMap::new();
            b.insert("k".into(), "v2".into());
            let merged = merge_attributes(a, b);
            assert_eq!(merged.get("k").unwrap(), "v2");
        }
    }

    mod step_08_otel_attr {
        use super::*;

        #[test]
        fn test_otel_attribute() {
            assert_eq!(otel_attribute("http.method", "GET"), ("http.method".into(), "GET".into()));
        }
    }

    mod step_09_metrics {
        use super::*;

        #[test]
        fn test_pipeline_metrics_success() {
            let m = PipelineMetrics::new();
            m.record_success();
            m.record_success();
            assert_eq!(m.snapshot(), (2, 0, 0));
        }

        #[test]
        fn test_pipeline_metrics_failure() {
            let m = PipelineMetrics::new();
            m.record_failure();
            assert_eq!(m.snapshot(), (0, 1, 0));
        }

        #[test]
        fn test_pipeline_metrics_span() {
            let m = PipelineMetrics::new();
            m.record_span();
            m.record_span();
            m.record_span();
            assert_eq!(m.snapshot(), (0, 0, 3));
        }

        #[test]
        fn test_pipeline_metrics_mixed() {
            let m = PipelineMetrics::new();
            m.record_success();
            m.record_failure();
            m.record_span();
            assert_eq!(m.snapshot(), (1, 1, 1));
        }
    }
}
