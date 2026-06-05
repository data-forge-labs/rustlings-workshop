mod lib;

fn main() {
    println!("Exploring Pandas — Rust DataFrame Equivalents");
    let csv_data = b"fruit,year,price\nApple,2020,1.20\nBanana,2020,0.80\nApple,2021,1.30\n";
    match lib::read_fruits(csv_data) {
        Ok(records) => {
            println!("Read {} records", records.len());
            println!("Mean price per fruit: {:?}", lib::mean_price_per_fruit(&records));
            println!("Filtered (>1.0): {:?}", lib::filter_by_price(&records, 1.0).len());
            println!("Stats: {:?}", lib::summary_stats(&records));
        }
        Err(e) => eprintln!("Error: {}", e),
    }
}
