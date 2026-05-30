/// A generic matrix that can display as HTML in Jupyter
pub struct Matrix<T> {
    pub values: Vec<T>,
    pub row_size: usize,
}

impl<T: std::fmt::Debug> Matrix<T> {
    pub fn new(values: Vec<T>, row_size: usize) -> Self {
        if row_size == 0 {
            panic!("row_size must be greater than 0");
        }
        if values.len() % row_size != 0 {
            panic!("values length must be divisible by row_size");
        }
        Matrix { values, row_size }
    }

    pub fn num_rows(&self) -> usize {
        self.values.len() / self.row_size
    }

    pub fn num_cols(&self) -> usize {
        self.row_size
    }

    pub fn row(&self, index: usize) -> &[T] {
        if index >= self.num_rows() {
            panic!("row index out of bounds");
        }
        let start = index * self.row_size;
        &self.values[start..start + self.row_size]
    }

    pub fn get(&self, row: usize, col: usize) -> Option<&T> {
        if row >= self.num_rows() || col >= self.num_cols() {
            return None;
        }
        Some(&self.values[row * self.row_size + col])
    }

    pub fn to_html(&self) -> String {
        let mut html = String::from("<table>\n");
        for r in 0..self.num_rows() {
            html.push_str("  <tr>\n");
            for c in 0..self.num_cols() {
                if let Some(val) = self.get(r, c) {
                    html.push_str(&format!("    <td>{:?}</td>\n", val));
                }
            }
            html.push_str("  </tr>\n");
        }
        html.push_str("</table>");
        html
    }
}

/// A simple DataFrame-like structure
pub struct SimpleDataFrame {
    pub columns: Vec<String>,
    pub data: Vec<Vec<String>>,
}

impl SimpleDataFrame {
    pub fn new(columns: Vec<String>, data: Vec<Vec<String>>) -> Self {
        let num_cols = columns.len();
        for (i, row) in data.iter().enumerate() {
            if row.len() != num_cols {
                panic!(
                    "row {} has {} columns, expected {}",
                    i,
                    row.len(),
                    num_cols
                );
            }
        }
        SimpleDataFrame { columns, data }
    }

    pub fn num_rows(&self) -> usize {
        self.data.len()
    }

    pub fn num_cols(&self) -> usize {
        self.columns.len()
    }

    pub fn to_html(&self) -> String {
        let mut html = String::from("<table>\n  <tr>\n");
        for col in &self.columns {
            html.push_str(&format!("    <th>{}</th>\n", col));
        }
        html.push_str("  </tr>\n");
        for row in &self.data {
            html.push_str("  <tr>\n");
            for val in row {
                html.push_str(&format!("    <td>{}</td>\n", val));
            }
            html.push_str("  </tr>\n");
        }
        html.push_str("</table>");
        html
    }
}

/// Generate a range of numbers (like Python's range)
pub fn range_f64(start: f64, end: f64, step: f64) -> Vec<f64> {
    if step == 0.0 {
        panic!("step must be non-zero");
    }
    let mut result = Vec::new();
    let mut current = start;
    if step > 0.0 {
        while current < end {
            result.push(current);
            current += step;
        }
    } else {
        while current > end {
            result.push(current);
            current += step;
        }
    }
    result
}

/// List crates worth exploring with evcxr_jupyter
pub fn list_interactive_crates() -> Vec<&'static str> {
    vec![
        "plotters",
        "itertools",
        "serde",
        "polars",
        "regex",
        "rayon",
        "ndarray",
    ]
}

/// Use cases for Rust in Jupyter notebooks
pub fn rust_notebook_use_cases() -> Vec<&'static str> {
    vec![
        "Educational tool for teaching Rust",
        "Data processing and analysis",
        "Algorithm development and prototyping",
        "Interactive documentation",
        "Research and experimentation",
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    mod step_01_matrix {
        use super::*;

        #[test]
        fn test_new_valid() {
            let m = Matrix::new(vec![1, 2, 3, 4], 2);
            assert_eq!(m.num_rows(), 2);
            assert_eq!(m.num_cols(), 2);
        }

        #[test]
        fn test_new_invalid_row_size() {
            let result = std::panic::catch_unwind(|| {
                Matrix::new(vec![1, 2, 3], 2);
            });
            assert!(result.is_err());
        }

        #[test]
        fn test_new_single_row() {
            let m = Matrix::new(vec![1, 2, 3], 3);
            assert_eq!(m.num_rows(), 1);
            assert_eq!(m.num_cols(), 3);
        }

        #[test]
        fn test_new_single_column() {
            let m = Matrix::new(vec![1, 2, 3], 1);
            assert_eq!(m.num_rows(), 3);
            assert_eq!(m.num_cols(), 1);
        }

        #[test]
        fn test_num_rows() {
            let m = Matrix::new(vec![1, 2, 3, 4, 5, 6], 3);
            assert_eq!(m.num_rows(), 2);
        }

        #[test]
        fn test_num_cols() {
            let m = Matrix::new(vec![1, 2, 3, 4], 2);
            assert_eq!(m.num_cols(), 2);
        }

        #[test]
        fn test_get_valid() {
            let m = Matrix::new(vec![1, 2, 3, 4], 2);
            assert_eq!(m.get(0, 0), Some(&1));
            assert_eq!(m.get(0, 1), Some(&2));
            assert_eq!(m.get(1, 0), Some(&3));
            assert_eq!(m.get(1, 1), Some(&4));
        }

        #[test]
        fn test_get_out_of_bounds() {
            let m = Matrix::new(vec![1, 2], 2);
            assert_eq!(m.get(0, 2), None);
            assert_eq!(m.get(1, 0), None);
        }

        #[test]
        fn test_row_valid() {
            let m = Matrix::new(vec![1, 2, 3, 4], 2);
            assert_eq!(m.row(0), &[1, 2]);
            assert_eq!(m.row(1), &[3, 4]);
        }

        #[test]
        #[should_panic(expected = "row index out of bounds")]
        fn test_row_out_of_bounds() {
            let m = Matrix::new(vec![1, 2], 2);
            m.row(1);
        }
    }

    mod step_02_dataframe {
        use super::*;

        #[test]
        fn test_new_valid() {
            let df = SimpleDataFrame::new(
                vec!["name".to_string(), "age".to_string()],
                vec![
                    vec!["Alice".to_string(), "30".to_string()],
                    vec!["Bob".to_string(), "25".to_string()],
                ],
            );
            assert_eq!(df.num_rows(), 2);
            assert_eq!(df.num_cols(), 2);
        }

        #[test]
        fn test_new_empty_columns() {
            let df = SimpleDataFrame::new(vec![], vec![]);
            assert_eq!(df.num_rows(), 0);
            assert_eq!(df.num_cols(), 0);
        }

        #[test]
        fn test_new_mismatched_row_lengths() {
            let result = std::panic::catch_unwind(|| {
                SimpleDataFrame::new(
                    vec!["a".to_string(), "b".to_string()],
                    vec![vec!["only_one".to_string()]],
                );
            });
            assert!(result.is_err());
        }
    }

    mod step_03_html_display {
        use super::*;

        #[test]
        fn test_matrix_to_html_contains_table_tags() {
            let m = Matrix::new(vec![1, 2, 3, 4], 2);
            let html = m.to_html();
            assert!(html.contains("<table>"));
            assert!(html.contains("</table>"));
            assert!(html.contains("<tr>"));
            assert!(html.contains("<td>"));
        }

        #[test]
        fn test_matrix_to_html_contains_data() {
            let m = Matrix::new(vec![1, 2], 2);
            let html = m.to_html();
            assert!(html.contains("1"));
            assert!(html.contains("2"));
        }

        #[test]
        fn test_dataframe_to_html_contains_table_tags() {
            let df = SimpleDataFrame::new(
                vec!["name".to_string()],
                vec![vec!["Alice".to_string()]],
            );
            let html = df.to_html();
            assert!(html.contains("<table>"));
            assert!(html.contains("</table>"));
            assert!(html.contains("<th>"));
            assert!(html.contains("<td>"));
        }

        #[test]
        fn test_range_f64_basic() {
            let r = range_f64(0.0, 3.0, 1.0);
            assert_eq!(r, vec![0.0, 1.0, 2.0]);
        }

        #[test]
        fn test_range_f64_negative_step() {
            let r = range_f64(3.0, 0.0, -1.0);
            assert_eq!(r, vec![3.0, 2.0, 1.0]);
        }

        #[test]
        fn test_range_f64_empty() {
            let r = range_f64(0.0, 0.0, 1.0);
            assert_eq!(r, Vec::<f64>::new());
        }
    }

    mod step_04_concepts {
        use super::*;

        #[test]
        fn test_list_interactive_crates() {
            let crates = list_interactive_crates();
            assert!(!crates.is_empty());
            assert!(crates.contains(&"serde"));
            assert!(crates.contains(&"rayon"));
        }

        #[test]
        fn test_rust_notebook_use_cases() {
            let cases = rust_notebook_use_cases();
            assert!(!cases.is_empty());
            assert!(cases.iter().any(|c| c.contains("Educational")));
        }
    }
}
