pub fn double_values(values: &[f64]) -> Vec<f64> {
    values.iter().map(|v| v * 2.0).collect()
}

pub fn sum_values(values: &[f64]) -> f64 {
    values.iter().sum()
}

pub fn normalize(values: &[f64]) -> Vec<f64> {
    if values.is_empty() {
        return vec![];
    }
    let min = values.iter().cloned().fold(f64::INFINITY, f64::min);
    let max = values.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
    let range = max - min;
    if range == 0.0 {
        return values.iter().map(|_| 0.0).collect();
    }
    values.iter().map(|v| (v - min) / range).collect()
}

pub fn moving_average(values: &[f64], window: usize) -> Vec<f64> {
    if window == 0 || values.len() < window {
        return vec![];
    }
    values
        .windows(window)
        .map(|w| w.iter().sum::<f64>() / window as f64)
        .collect()
}

pub fn count_above_threshold(values: &[f64], threshold: f64) -> usize {
    values.iter().filter(|&&v| v > threshold).count()
}

pub fn reverse_in_place(values: &mut [f64]) {
    values.reverse();
}

pub fn hello_from_rust(name: &str) -> String {
    format!("Hello, {} from Rust!", name)
}

pub fn parse_csv_line(line: &str) -> Vec<f64> {
    line.split(',').filter_map(|s| s.trim().parse().ok()).collect()
}

#[cfg(feature = "python")]
#[pyo3::pymodule]
fn pyo3_bindings_workshop(m: &pyo3::Bound<'_, pyo3::types::PyModule>) -> pyo3::PyResult<()> {
    use pyo3::types::PyModuleMethods;
    use pyo3::wrap_pyfunction;

    #[pyo3::pyfunction]
    fn double_values_py(values: Vec<f64>) -> Vec<f64> {
        double_values(&values)
    }

    #[pyo3::pyfunction]
    fn sum_values_py(values: Vec<f64>) -> f64 {
        sum_values(&values)
    }

    #[pyo3::pyfunction]
    fn normalize_py(values: Vec<f64>) -> Vec<f64> {
        normalize(&values)
    }

    #[pyo3::pyfunction]
    fn moving_average_py(values: Vec<f64>, window: usize) -> Vec<f64> {
        moving_average(&values, window)
    }

    #[pyo3::pyfunction]
    fn count_above_threshold_py(values: Vec<f64>, threshold: f64) -> usize {
        count_above_threshold(&values, threshold)
    }

    #[pyo3::pyfunction]
    fn hello_from_rust_py(name: &str) -> String {
        hello_from_rust(name)
    }

    m.add_function(wrap_pyfunction!(double_values_py, m)?)?;
    m.add_function(wrap_pyfunction!(sum_values_py, m)?)?;
    m.add_function(wrap_pyfunction!(normalize_py, m)?)?;
    m.add_function(wrap_pyfunction!(moving_average_py, m)?)?;
    m.add_function(wrap_pyfunction!(count_above_threshold_py, m)?)?;
    m.add_function(wrap_pyfunction!(hello_from_rust_py, m)?)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn approx_eq(a: f64, b: f64) -> bool {
        (a - b).abs() < 1e-9
    }

    mod step_01_basic {
        use super::*;

        #[test]
        fn test_double_values() {
            let result = double_values(&[1.0, 2.0, 3.0]);
            assert_eq!(result, vec![2.0, 4.0, 6.0]);
        }

        #[test]
        fn test_double_empty() {
            let result = double_values(&[]);
            assert_eq!(result, Vec::<f64>::new());
        }

        #[test]
        fn test_sum_values() {
            assert!(approx_eq(sum_values(&[1.0, 2.0, 3.0, 4.0]), 10.0));
        }

        #[test]
        fn test_sum_values_negative() {
            assert!(approx_eq(sum_values(&[-1.0, 1.0, -2.0, 2.0]), 0.0));
        }
    }

    mod step_02_transforms {
        use super::*;

        #[test]
        fn test_normalize_to_unit_range() {
            let result = normalize(&[1.0, 2.0, 3.0, 4.0, 5.0]);
            assert!(approx_eq(result[0], 0.0));
            assert!(approx_eq(result[4], 1.0));
        }

        #[test]
        fn test_normalize_constant_is_zero() {
            let result = normalize(&[3.0, 3.0, 3.0]);
            for v in &result {
                assert!(approx_eq(*v, 0.0));
            }
        }
    }

    mod step_03_windowed {
        use super::*;

        #[test]
        fn test_moving_average_window_2() {
            let result = moving_average(&[1.0, 3.0, 5.0, 7.0], 2);
            assert_eq!(result.len(), 3);
            assert!(approx_eq(result[0], 2.0));
            assert!(approx_eq(result[1], 4.0));
            assert!(approx_eq(result[2], 6.0));
        }

        #[test]
        fn test_moving_average_window_3() {
            let result = moving_average(&[1.0, 2.0, 3.0, 4.0, 5.0], 3);
            assert_eq!(result.len(), 3);
            assert!(approx_eq(result[0], 2.0));
            assert!(approx_eq(result[1], 3.0));
            assert!(approx_eq(result[2], 4.0));
        }
    }

    mod step_04_counting {
        use super::*;

        #[test]
        fn test_count_above_threshold() {
            assert_eq!(count_above_threshold(&[1.0, 5.0, 10.0, 15.0], 8.0), 2);
        }

        #[test]
        fn test_count_above_threshold_none() {
            assert_eq!(count_above_threshold(&[1.0, 2.0, 3.0], 100.0), 0);
        }
    }

    mod step_05_misc {
        use super::*;

        #[test]
        fn test_hello_from_rust() {
            assert_eq!(hello_from_rust("Alice"), "Hello, Alice from Rust!");
        }

        #[test]
        fn test_reverse_in_place() {
            let mut v = vec![1.0, 2.0, 3.0, 4.0];
            reverse_in_place(&mut v);
            assert_eq!(v, vec![4.0, 3.0, 2.0, 1.0]);
        }

        #[test]
        fn test_parse_csv_line() {
            let result = parse_csv_line("1.5,2.5,3.5");
            assert_eq!(result, vec![1.5, 2.5, 3.5]);
        }
    }
}
