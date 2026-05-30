use parquet::{filter_by_threshold, total_value, Record};

fn main() {
    let records = vec![
        Record { name: "A".into(), value: 10.0, count: 5 },
        Record { name: "B".into(), value: 20.0, count: 3 },
        Record { name: "C".into(), value: 5.0, count: 10 },
    ];
    println!("Total value: {}", total_value(&records));
    let high = filter_by_threshold(&records, 15.0);
    println!("Records above threshold: {}", high.len());
}
