pub fn parse_csv_line(line: &str) -> Vec<String> {
    line.split(',').map(|s| s.to_string()).collect()
}

pub fn format_csv_line(fields: &[&str]) -> String {
    fields.join(",")
}

pub fn count_lines(csv_content: &str) -> usize {
    csv_content.lines().count()
}

pub fn column_values(csv_content: &str, col_index: usize) -> Vec<String> {
    csv_content
        .lines()
        .filter_map(|line| line.split(',').nth(col_index).map(|s| s.to_string()))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    mod step_01_parse {
        #[test]
        fn test_parse_simple_line() {
            let fields = parse_csv_line("a,b,c");
            assert_eq!(fields, vec!["a", "b", "c"]);
        }

        #[test]
        fn test_parse_empty_line() {
            let fields: Vec<String> = parse_csv_line("");
            assert!(fields.is_empty() || fields == vec![""]);
        }

        #[test]
        fn test_parse_single_field() {
            let fields = parse_csv_line("hello");
            assert_eq!(fields, vec!["hello"]);
        }
    }

    mod step_02_format {
        #[test]
        fn test_format_csv_line() {
            let line = format_csv_line(&["a", "b", "c"]);
            assert_eq!(line, "a,b,c");
        }

        #[test]
        fn test_format_single_field() {
            let line = format_csv_line(&["only"]);
            assert_eq!(line, "only");
        }
    }

    mod step_03_content {
        #[test]
        fn test_count_lines() {
            let csv = "a\nb\nc";
            assert_eq!(count_lines(csv), 3);
        }

        #[test]
        fn test_column_values() {
            let csv = "x,y,z\n1,2,3\n4,5,6";
            let col = column_values(csv, 1);
            assert_eq!(col, vec!["y", "2", "5"]);
        }
    }
}
