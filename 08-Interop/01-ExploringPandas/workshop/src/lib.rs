use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct FruitRecord {
    pub fruit: String,
    pub year: u32,
    pub price: f64,
}

/// Read fruit records from CSV bytes (like pd.read_csv)
pub fn read_fruits(bytes: &[u8]) -> Result<Vec<FruitRecord>, String> {
    let mut reader = csv::Reader::from_reader(bytes);
    let mut records = Vec::new();
    for result in reader.deserialize() {
        match result {
            Ok(record) => records.push(record),
            Err(e) => return Err(format!("CSV parse error: {}", e)),
        }
    }
    Ok(records)
}

/// Calculate mean price per fruit (like df.groupby("fruit")["price"].mean())
pub fn mean_price_per_fruit(records: &[FruitRecord]) -> Vec<(String, f64)> {
    use std::collections::HashMap;
    let mut sums: HashMap<&str, (f64, usize)> = HashMap::new();
    for r in records {
        let entry = sums.entry(&r.fruit).or_insert((0.0, 0));
        entry.0 += r.price;
        entry.1 += 1;
    }
    let mut result: Vec<_> = sums
        .into_iter()
        .map(|(fruit, (sum, count))| (fruit.to_string(), sum / count as f64))
        .collect();
    result.sort_by(|a, b| a.0.cmp(&b.0));
    result
}

/// Calculate mean price per year
pub fn mean_price_per_year(records: &[FruitRecord]) -> Vec<(u32, f64)> {
    use std::collections::HashMap;
    let mut sums: HashMap<u32, (f64, usize)> = HashMap::new();
    for r in records {
        let entry = sums.entry(r.year).or_insert((0.0, 0));
        entry.0 += r.price;
        entry.1 += 1;
    }
    let mut result: Vec<_> = sums
        .into_iter()
        .map(|(year, (sum, count))| (year, sum / count as f64))
        .collect();
    result.sort_by_key(|&(year, _)| year);
    result
}

/// Filter records where price > threshold (like df[df["price"] > threshold])
pub fn filter_by_price(records: &[FruitRecord], threshold: f64) -> Vec<FruitRecord> {
    records
        .iter()
        .filter(|r| r.price > threshold)
        .cloned()
        .collect()
}

/// Write records to CSV string (like df.to_csv)
pub fn write_fruits(records: &[FruitRecord]) -> Result<String, String> {
    let mut wtr = csv::Writer::from_writer(Vec::new());
    for r in records {
        wtr.serialize(r).map_err(|e| format!("CSV write error: {}", e))?;
    }
    wtr.flush().map_err(|e| format!("CSV flush error: {}", e))?;
    String::from_utf8(wtr.into_inner().map_err(|e| format!("CSV inner error: {}", e))?)
        .map_err(|e| format!("UTF-8 error: {}", e))
}

/// Summary statistics (like df.describe()): (min, max, mean, count)
pub fn summary_stats(records: &[FruitRecord]) -> (f64, f64, f64, usize) {
    let count = records.len();
    if count == 0 {
        return (0.0, 0.0, 0.0, 0);
    }
    let mut prices: Vec<f64> = records.iter().map(|r| r.price).collect();
    prices.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let min = prices[0];
    let max = prices[prices.len() - 1];
    let sum: f64 = prices.iter().sum();
    let mean = sum / count as f64;
    (min, max, mean, count)
}

#[cfg(test)]
mod tests {
    mod step_01_csv_io {
        use crate::{read_fruits, write_fruits, FruitRecord};

        #[test]
        fn read_basic_csv() {
            let data = b"fruit,year,price\nApple,2020,1.20\nBanana,2020,0.80\n";
            let records = read_fruits(data).unwrap();
            assert_eq!(records.len(), 2);
            assert_eq!(records[0].fruit, "Apple");
            assert_eq!(records[0].year, 2020);
            assert!((records[0].price - 1.20).abs() < 1e-10);
        }

        #[test]
        fn read_empty_body() {
            let data = b"fruit,year,price\n";
            let records = read_fruits(data).unwrap();
            assert_eq!(records.len(), 0);
        }

        #[test]
        fn read_empty_input() {
            let records = read_fruits(b"");
            assert!(records.is_err());
        }

        #[test]
        fn write_and_roundtrip() {
            let records = vec![
                FruitRecord {
                    fruit: "Apple".into(),
                    year: 2020,
                    price: 1.5,
                },
                FruitRecord {
                    fruit: "Banana".into(),
                    year: 2021,
                    price: 0.9,
                },
            ];
            let csv_out = write_fruits(&records).unwrap();
            let parsed = read_fruits(csv_out.as_bytes()).unwrap();
            assert_eq!(parsed, records);
        }

        #[test]
        fn write_empty() {
            let csv_out = write_fruits(&[]).unwrap();
            assert_eq!(csv_out, "fruit,year,price\n");
        }
    }

    mod step_02_groupby {
        use crate::{mean_price_per_fruit, mean_price_per_year, FruitRecord};

        fn sample_records() -> Vec<FruitRecord> {
            vec![
                FruitRecord { fruit: "Apple".into(), year: 2020, price: 2.0 },
                FruitRecord { fruit: "Apple".into(), year: 2021, price: 3.0 },
                FruitRecord { fruit: "Banana".into(), year: 2020, price: 1.0 },
            ]
        }

        #[test]
        fn mean_price_single_fruit() {
            let records = vec![
                FruitRecord { fruit: "Apple".into(), year: 2020, price: 2.0 },
                FruitRecord { fruit: "Apple".into(), year: 2021, price: 4.0 },
            ];
            let result = mean_price_per_fruit(&records);
            assert_eq!(result.len(), 1);
            assert!((result[0].1 - 3.0).abs() < 1e-10);
        }

        #[test]
        fn mean_price_multiple_fruits() {
            let records = sample_records();
            let mut result = mean_price_per_fruit(&records);
            result.sort_by(|a, b| a.0.cmp(&b.0));
            assert_eq!(result.len(), 2);
            assert!((result[0].1 - 2.5).abs() < 1e-10); // Apple (2+3)/2
            assert!((result[1].1 - 1.0).abs() < 1e-10); // Banana 1
        }

        #[test]
        fn mean_price_empty() {
            let result = mean_price_per_fruit(&[]);
            assert!(result.is_empty());
        }

        #[test]
        fn mean_price_per_year_multiple() {
            let records = sample_records();
            let result = mean_price_per_year(&records);
            assert_eq!(result.len(), 2);
        }

        #[test]
        fn mean_price_per_year_single() {
            let records = vec![
                FruitRecord { fruit: "Apple".into(), year: 2020, price: 5.0 },
            ];
            let result = mean_price_per_year(&records);
            assert_eq!(result, vec![(2020, 5.0)]);
        }
    }

    mod step_03_filtering {
        use crate::{filter_by_price, FruitRecord};

        fn sample() -> Vec<FruitRecord> {
            vec![
                FruitRecord { fruit: "Apple".into(), year: 2020, price: 1.0 },
                FruitRecord { fruit: "Banana".into(), year: 2020, price: 2.0 },
                FruitRecord { fruit: "Cherry".into(), year: 2021, price: 3.0 },
            ]
        }

        #[test]
        fn some_match() {
            let result = filter_by_price(&sample(), 1.5);
            assert_eq!(result.len(), 2);
            assert_eq!(result[0].fruit, "Banana");
            assert_eq!(result[1].fruit, "Cherry");
        }

        #[test]
        fn none_match() {
            let result = filter_by_price(&sample(), 5.0);
            assert_eq!(result.len(), 0);
        }

        #[test]
        fn all_match() {
            let result = filter_by_price(&sample(), 0.5);
            assert_eq!(result.len(), 3);
        }

        #[test]
        fn empty_records() {
            let result = filter_by_price(&[], 1.0);
            assert!(result.is_empty());
        }
    }

    mod step_04_statistics {
        use crate::{summary_stats, FruitRecord};

        #[test]
        fn normal_stats() {
            let records = vec![
                FruitRecord { fruit: "Apple".into(), year: 2020, price: 1.0 },
                FruitRecord { fruit: "Banana".into(), year: 2020, price: 2.0 },
                FruitRecord { fruit: "Cherry".into(), year: 2021, price: 3.0 },
            ];
            let (min, max, mean, count) = summary_stats(&records);
            assert!((min - 1.0).abs() < 1e-10);
            assert!((max - 3.0).abs() < 1e-10);
            assert!((mean - 2.0).abs() < 1e-10);
            assert_eq!(count, 3);
        }

        #[test]
        fn single_record() {
            let records = vec![
                FruitRecord { fruit: "Apple".into(), year: 2020, price: 4.5 },
            ];
            let (min, max, mean, count) = summary_stats(&records);
            assert!((min - 4.5).abs() < 1e-10);
            assert!((max - 4.5).abs() < 1e-10);
            assert!((mean - 4.5).abs() < 1e-10);
            assert_eq!(count, 1);
        }

        #[test]
        fn empty_records() {
            let (min, max, mean, count) = summary_stats(&[]);
            assert!((min - 0.0).abs() < 1e-10);
            assert!((max - 0.0).abs() < 1e-10);
            assert!((mean - 0.0).abs() < 1e-10);
            assert_eq!(count, 0);
        }
    }
}
