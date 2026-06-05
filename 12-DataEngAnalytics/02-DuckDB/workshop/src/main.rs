use duckdb_workshop::{
    count_products, create_products_table, import_csv_from_file, insert_product, open_in_memory,
    products_in_region, regions_with_count, run_sql,
};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let conn = open_in_memory()?;
    create_products_table(&conn)?;
    insert_product(&conn, 1, "Apple", "North")?;
    insert_product(&conn, 2, "Bread", "South")?;
    insert_product(&conn, 3, "Milk", "North")?;
    println!("Inserted {} products", count_products(&conn)?);

    let n = import_csv_from_file(&conn, "products_import", "data/products.csv")?;
    println!("Imported {} rows from CSV", n);

    let north = products_in_region(&conn, "North")?;
    println!("North region has {} products", north.len());

    let regions = regions_with_count(&conn)?;
    println!("Region counts: {:?}", regions);

    let rows = run_sql(&conn, "SELECT region, COUNT(*) AS n FROM products GROUP BY region")?;
    for row in rows {
        println!("{} -> {}", row[0], row[1]);
    }

    Ok(())
}
