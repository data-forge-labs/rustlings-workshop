/// Generate sample data for charting
pub fn generate_sample_data() -> Vec<f64> {
    vec![3.0, 7.0, 2.0, 9.0, 5.0, 6.0, 8.0, 1.0, 4.0]
}

/// Format data as ASCII bar chart (text-based, no external crate)
pub fn ascii_bar_chart(data: &[f64], labels: &[&str]) -> Vec<String> {
    if data.is_empty() || labels.is_empty() {
        return Vec::new();
    }
    let max = data.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
    if max == 0.0 {
        return labels.iter().map(|l| format!("{:<10} |", l)).collect();
    }
    let bar_width = 40.0;
    labels
        .iter()
        .zip(data.iter())
        .map(|(label, &value)| {
            let bar_len = ((value / max) * bar_width).round() as usize;
            let bar = "█".repeat(bar_len);
            format!("{:<10} | {} ({})", label, bar, value)
        })
        .collect()
}

/// Find min, max, mean of a data slice
pub fn data_summary(data: &[f64]) -> (f64, f64, f64) {
    if data.is_empty() {
        return (f64::NAN, f64::NAN, f64::NAN);
    }
    let min = data.iter().cloned().fold(f64::INFINITY, f64::min);
    let max = data.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
    let mean = data.iter().sum::<f64>() / data.len() as f64;
    (min, max, mean)
}

/// Normalize data to 0..100 range
pub fn normalize_data(data: &[f64]) -> Vec<f64> {
    if data.is_empty() {
        return Vec::new();
    }
    let (min, max, _) = data_summary(data);
    if (max - min).abs() < f64::EPSILON {
        return data.iter().map(|_| 50.0).collect();
    }
    data.iter()
        .map(|&v| ((v - min) / (max - min)) * 100.0)
        .collect()
}

/// Create a labeled data series from names and values
pub fn create_series<'a>(names: &[&'a str], values: &[f64]) -> Vec<(&'a str, f64)> {
    assert_eq!(
        names.len(),
        values.len(),
        "names and values must have the same length"
    );
    names.iter().zip(values.iter()).map(|(&n, &v)| (n, v)).collect()
}

#[cfg(test)]
mod tests {
    mod step_01_data_basics {
        use crate::{data_summary, generate_sample_data};

        #[test]
        fn test_generate_sample_data_non_empty() {
            let data = generate_sample_data();
            assert!(!data.is_empty());
        }

        #[test]
        fn test_generate_sample_data_type() {
            let data = generate_sample_data();
            for &v in &data {
                assert!(v.is_finite());
            }
        }

        #[test]
        fn test_data_summary_normal() {
            let data = vec![1.0, 2.0, 3.0, 4.0, 5.0];
            let (min, max, mean) = data_summary(&data);
            assert!((min - 1.0).abs() < f64::EPSILON);
            assert!((max - 5.0).abs() < f64::EPSILON);
            assert!((mean - 3.0).abs() < f64::EPSILON);
        }

        #[test]
        fn test_data_summary_empty_returns_nan() {
            let data: Vec<f64> = vec![];
            let (min, max, mean) = data_summary(&data);
            assert!(min.is_nan());
            assert!(max.is_nan());
            assert!(mean.is_nan());
        }

        #[test]
        fn test_data_summary_single_element() {
            let data = vec![42.0];
            let (min, max, mean) = data_summary(&data);
            assert!((min - 42.0).abs() < f64::EPSILON);
            assert!((max - 42.0).abs() < f64::EPSILON);
            assert!((mean - 42.0).abs() < f64::EPSILON);
        }

        #[test]
        fn test_data_summary_negative_values() {
            let data = vec![-5.0, -1.0, -3.0];
            let (min, max, mean) = data_summary(&data);
            assert!((min - (-5.0)).abs() < f64::EPSILON);
            assert!((max - (-1.0)).abs() < f64::EPSILON);
            assert!((mean - (-3.0)).abs() < f64::EPSILON);
        }
    }

    mod step_02_visualization {
        use crate::{ascii_bar_chart, normalize_data};

        #[test]
        fn test_ascii_bar_chart_basic() {
            let data = vec![5.0, 10.0, 2.0];
            let labels = vec!["A", "B", "C"];
            let chart = ascii_bar_chart(&data, &labels);
            assert_eq!(chart.len(), 3);
            assert!(chart[0].contains("A"));
            assert!(chart[1].contains("B"));
            assert!(chart[2].contains("C"));
        }

        #[test]
        fn test_ascii_bar_chart_empty() {
            let data: Vec<f64> = vec![];
            let labels: Vec<&str> = vec![];
            let chart = ascii_bar_chart(&data, &labels);
            assert!(chart.is_empty());
        }

        #[test]
        fn test_ascii_bar_chart_single_bar() {
            let data = vec![42.0];
            let labels = vec!["X"];
            let chart = ascii_bar_chart(&data, &labels);
            assert_eq!(chart.len(), 1);
            assert!(chart[0].contains("X"));
            assert!(chart[0].contains("42"));
        }

        #[test]
        fn test_ascii_bar_chart_contains_value() {
            let data = vec![7.5];
            let labels = vec!["test"];
            let chart = ascii_bar_chart(&data, &labels);
            assert!(chart[0].contains("7.5"));
        }

        #[test]
        fn test_normalize_data_typical() {
            let data = vec![0.0, 50.0, 100.0];
            let norm = normalize_data(&data);
            assert!((norm[0] - 0.0).abs() < f64::EPSILON);
            assert!((norm[1] - 50.0).abs() < f64::EPSILON);
            assert!((norm[2] - 100.0).abs() < f64::EPSILON);
        }

        #[test]
        fn test_normalize_data_empty() {
            let data: Vec<f64> = vec![];
            let norm = normalize_data(&data);
            assert!(norm.is_empty());
        }

        #[test]
        fn test_normalize_data_constant() {
            let data = vec![5.0, 5.0, 5.0];
            let norm = normalize_data(&data);
            for &v in &norm {
                assert!((v - 50.0).abs() < f64::EPSILON);
            }
        }
    }

    mod step_03_series {
        use crate::create_series;

        #[test]
        fn test_create_series_matching_lengths() {
            let names = vec!["a", "b", "c"];
            let values = vec![1.0, 2.0, 3.0];
            let series = create_series(&names, &values);
            assert_eq!(series.len(), 3);
            assert_eq!(series[0], ("a", 1.0));
            assert_eq!(series[1], ("b", 2.0));
            assert_eq!(series[2], ("c", 3.0));
        }

        #[test]
        fn test_create_series_empty() {
            let names: Vec<&str> = vec![];
            let values: Vec<f64> = vec![];
            let series = create_series(&names, &values);
            assert!(series.is_empty());
        }

        #[test]
        #[should_panic(expected = "must have the same length")]
        fn test_create_series_mismatched_lengths() {
            let names = vec!["a", "b"];
            let values = vec![1.0];
            create_series(&names, &values);
        }

        #[test]
        fn test_create_series_single_pair() {
            let names = vec!["only"];
            let values = vec![99.9];
            let series = create_series(&names, &values);
            assert_eq!(series.len(), 1);
            assert_eq!(series[0], ("only", 99.9));
        }
    }
}
