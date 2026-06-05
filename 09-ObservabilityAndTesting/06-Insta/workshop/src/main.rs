use insta_workshop::{
    format_bytes, format_currency_cents, format_duration, format_error_chain, format_log_line,
    format_path, format_sql_select, format_table,
};
use std::error::Error;
use std::time::Duration;

fn main() -> Result<(), Box<dyn Error>> {
    println!("{}", format_bytes(1_572_864));
    println!("{}", format_duration(Duration::from_secs(125)));
    println!("{}", format_sql_select("users", &["id", "name"]));
    println!("{}", format_log_line("INFO", "etl", "batch done"));
    println!("{}", format_path(&["", "data", "raw", "x.parquet"]));
    println!(
        "{}",
        format_table(
            &["id", "name"],
            &vec![
                vec!["1".into(), "Alice".into()],
                vec!["2".into(), "Bob".into()],
            ]
        )
    );
    println!("{}", format_currency_cents(12_345));
    println!(
        "{}",
        format_error_chain("query failed", &["timeout", "network down"])
    );
    Ok(())
}
