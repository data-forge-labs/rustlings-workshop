pub struct Record {
    pub name: String,
    pub value: f64,
    pub count: u32,
}

pub fn filter_by_threshold(records: &[Record], threshold: f64) -> Vec<&Record> {
    todo!()
}

pub fn total_value(records: &[Record]) -> f64 {
    todo!()
}

pub fn record_summary(record: &Record) -> String {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    mod step_01_records {
        #[test]
        fn test_total_value() {
            let records = vec![
                Record { name: "a".into(), value: 10.0, count: 1 },
                Record { name: "b".into(), value: 20.0, count: 2 },
            ];
            assert!((total_value(&records) - 10.0 * 1.0 - 20.0 * 2.0).abs() < 1e-6);
        }

        #[test]
        fn test_filter_by_threshold() {
            let records = vec![
                Record { name: "low".into(), value: 5.0, count: 1 },
                Record { name: "high".into(), value: 15.0, count: 1 },
            ];
            let filtered = filter_by_threshold(&records, 10.0);
            assert_eq!(filtered.len(), 1);
            assert_eq!(filtered[0].name, "high");
        }

        #[test]
        fn test_total_value_empty() {
            assert!((total_value(&[]) - 0.0).abs() < 1e-6);
        }

        #[test]
        fn test_record_summary() {
            let r = Record { name: "test".into(), value: 42.0, count: 3 };
            let s = record_summary(&r);
            assert!(s.contains("test"));
            assert!(s.contains("42"));
        }
    }
}
