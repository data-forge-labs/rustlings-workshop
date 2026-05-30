use std::io::{BufRead, BufReader, Read};

pub fn count_lines_reader<R: Read>(reader: R) -> usize {
    todo!()
}

pub fn read_file_simulated(path: &str, content: &str) -> Result<String, String> {
    todo!()
}

pub fn safe_divide(a: f64, b: f64) -> Result<f64, String> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    mod step_01_buffered_io {
        #[test]
        fn test_count_lines() {
            let data = b"line1\nline2\nline3\n";
            let count = count_lines_reader(&data[..]);
            assert_eq!(count, 3);
        }

        #[test]
        fn test_count_lines_empty() {
            let data = b"";
            let count = count_lines_reader(&data[..]);
            assert_eq!(count, 0);
        }

        #[test]
        fn test_count_lines_trailing_newline() {
            let data = b"a\nb\n";
            let count = count_lines_reader(&data[..]);
            assert_eq!(count, 2);
        }
    }

    mod step_02_error_handling {
        #[test]
        fn test_safe_divide_normal() {
            assert_eq!(safe_divide(10.0, 2.0), Ok(5.0));
        }

        #[test]
        fn test_safe_divide_by_zero() {
            assert!(safe_divide(10.0, 0.0).is_err());
        }

        #[test]
        fn test_read_file() {
            let result = read_file_simulated("test.txt", "hello world");
            assert_eq!(result, Ok("hello world".to_string()));
        }
    }
}
