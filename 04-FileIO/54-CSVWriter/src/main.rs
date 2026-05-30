use csv_writer::{Product, DISCOUNT, apply_discount};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    use csv;
    let mut rdr = csv::Reader::from_path("data/products.csv")?;
    let mut wtr = csv::Writer::from_path("data/discounted_products.csv")?;

    let mut savings = 0.0;
    for result in rdr.deserialize::<Product>() {
        let record = result?;
        let discounted = apply_discount(&record);
        wtr.serialize(discounted)?;
        savings += record.price * DISCOUNT;
    }
    wtr.flush()?;
    println!("Savings: ${:.2}", savings);
    Ok(())
}
