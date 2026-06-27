use std::time::Duration;

pub fn format_bytes(n: u64) -> String {
    if n == 0 {
        return "0 B".to_string();
    }
    const UNITS: &[&str] = &["B", "KiB", "MiB", "GiB", "TiB"];
    let mut value = n as f64;
    let mut unit_idx = 0;
    while value >= 1024.0 && unit_idx < UNITS.len() - 1 {
        value /= 1024.0;
        unit_idx += 1;
    }
    if unit_idx == 0 {
        format!("{} B", n)
    } else {
        format!("{:.2} {}", value, UNITS[unit_idx])
    }
}

pub fn format_duration(d: Duration) -> String {
    let total_secs = d.as_secs();
    let millis = d.subsec_millis();
    if total_secs == 0 {
        format!("{}ms", millis)
    } else {
        let mins = total_secs / 60;
        let secs = total_secs % 60;
        if mins > 0 {
            format!("{}m{}s", mins, secs)
        } else if millis > 0 {
            format!("{}.{:02}s", total_secs, millis / 10)
        } else {
            format!("{}.{:02}s", total_secs, 0)
        }
    }
}

pub fn format_sql_select(table: &str, columns: &[&str]) -> String {
    format!("SELECT {} FROM {}", columns.join(", "), table)
}

pub fn format_log_line(level: &str, target: &str, message: &str) -> String {
    format!("[{}] {}: {}", level, target, message)
}

pub fn format_path(parts: &[&str]) -> String {
    parts.join("/")
}

pub fn format_table(headers: &[&str], rows: &[Vec<String>]) -> String {
    let col_widths: Vec<usize> = headers
        .iter()
        .enumerate()
        .map(|(i, h)| {
            rows.iter()
                .map(|r| r.get(i).map_or(0, |c| c.len()))
                .chain(std::iter::once(h.len()))
                .max()
                .unwrap_or(0)
        })
        .collect();

    let header_line: Vec<String> = headers
        .iter()
        .enumerate()
        .map(|(i, h)| format!("{:<width$}", h, width = col_widths[i]))
        .collect();
    let sep_line: Vec<String> = col_widths.iter().map(|w| "-".repeat(*w)).collect();

    let mut lines = vec![header_line.join(" | "), sep_line.join("+")];
    for row in rows {
        let cells: Vec<String> = row
            .iter()
            .enumerate()
            .map(|(i, c)| format!("{:<width$}", c, width = col_widths[i]))
            .collect();
        lines.push(cells.join(" | "));
    }
    lines.join("\n")
}

pub fn format_currency_cents(cents: u64) -> String {
    format!("${}.{:02}", cents / 100, cents % 100)
}

pub fn format_error_chain(top: &str, sources: &[&str]) -> String {
    let mut parts = vec![top.to_string()];
    for source in sources {
        parts.push(format!("caused by: {}", source));
    }
    parts.join(": ")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_bytes() {
        insta::assert_snapshot!(format_bytes(0), @"0 B");
        insta::assert_snapshot!(format_bytes(512), @"512 B");
        insta::assert_snapshot!(format_bytes(1024), @"1.00 KiB");
        insta::assert_snapshot!(format_bytes(1_048_576), @"1.00 MiB");
        insta::assert_snapshot!(format_bytes(1_073_741_824), @"1.00 GiB");
    }

    #[test]
    fn test_format_duration() {
        insta::assert_snapshot!(format_duration(Duration::from_millis(0)), @"0ms");
        insta::assert_snapshot!(format_duration(Duration::from_millis(250)), @"250ms");
        insta::assert_snapshot!(format_duration(Duration::from_secs(1)), @"1.00s");
        insta::assert_snapshot!(format_duration(Duration::from_secs(90)), @"1m30s");
    }

    #[test]
    fn test_format_sql_select() {
        insta::assert_snapshot!(
            format_sql_select("users", &["id", "name", "email"]),
            @"SELECT id, name, email FROM users"
        );
        insta::assert_snapshot!(
            format_sql_select("orders", &["*"]),
            @"SELECT * FROM orders"
        );
    }

    #[test]
    fn test_format_log_line() {
        insta::assert_snapshot!(
            format_log_line("INFO", "auth", "user logged in"),
            @r#"[INFO] auth: user logged in"#
        );
        insta::assert_snapshot!(
            format_log_line("ERROR", "etl", "batch failed"),
            @r#"[ERROR] etl: batch failed"#
        );
    }

    #[test]
    fn test_format_path() {
        insta::assert_snapshot!(
            format_path(&["data", "raw", "users.csv"]),
            @"data/raw/users.csv"
        );
        insta::assert_snapshot!(
            format_path(&["", "home", "alice", "file.txt"]),
            @"/home/alice/file.txt"
        );
    }

    #[test]
    fn test_format_table() {
        let rows = vec![
            vec!["1".into(), "Alice".into(), "30".into()],
            vec!["2".into(), "Bob".into(), "25".into()],
        ];
        insta::assert_snapshot!(
            format_table(&["id", "name", "age"], &rows),
            @"id | name  | age
---+-------+-----
1  | Alice | 30
2  | Bob   | 25"
        );
    }

    #[test]
    fn test_format_currency() {
        insta::assert_snapshot!(format_currency_cents(0), @"$0.00");
        insta::assert_snapshot!(format_currency_cents(99), @"$0.99");
        insta::assert_snapshot!(format_currency_cents(100), @"$1.00");
        insta::assert_snapshot!(format_currency_cents(12_345), @"$123.45");
    }

    #[test]
    fn test_format_error_chain() {
        insta::assert_snapshot!(
            format_error_chain("connection refused", &[]),
            @"connection refused"
        );
        insta::assert_snapshot!(
            format_error_chain("query failed", &["timeout", "network down"]),
            @r#"query failed: caused by: timeout: caused by: network down"#
        );
    }
}
