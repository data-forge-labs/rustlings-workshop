use duckdb::{params, Connection, Result};

pub fn open_in_memory() -> Result<Connection> {
    todo!()
}

pub fn create_products_table(conn: &Connection) -> Result<()> {
    todo!()
}

pub fn insert_product(conn: &Connection, id: i32, name: &str, region: &str) -> Result<()> {
    todo!()
}

pub fn count_products(conn: &Connection) -> Result<i64> {
    todo!()
}

pub fn products_in_region(conn: &Connection, region: &str) -> Result<Vec<(i32, String)>> {
    todo!()
}

pub fn regions_with_count(conn: &Connection) -> Result<Vec<(String, i64)>> {
    todo!()
}

pub fn import_csv_from_file(conn: &Connection, table: &str, path: &str) -> Result<usize> {
    todo!()
}

pub fn run_sql(conn: &Connection, sql: &str) -> Result<Vec<Vec<String>>> {
    todo!()
}

pub fn prepared_count(conn: &Connection, region: &str) -> Result<i64> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    mod step_01_connection {
        use super::*;

        #[test]
        fn test_open_in_memory() {
            let conn = open_in_memory().unwrap();
            let _ = conn.execute("SELECT 1", []);
        }
    }

    mod step_02_table {
        use super::*;

        #[test]
        fn test_create_products_table_and_insert() {
            let conn = open_in_memory().unwrap();
            create_products_table(&conn).unwrap();
            insert_product(&conn, 1, "Apple", "North").unwrap();
            insert_product(&conn, 2, "Bread", "South").unwrap();
            assert_eq!(count_products(&conn).unwrap(), 2);
        }
    }

    mod step_03_queries {
        use super::*;

        fn seed() -> Connection {
            let conn = open_in_memory().unwrap();
            create_products_table(&conn).unwrap();
            insert_product(&conn, 1, "Apple", "North").unwrap();
            insert_product(&conn, 2, "Bread", "South").unwrap();
            insert_product(&conn, 3, "Milk", "North").unwrap();
            insert_product(&conn, 4, "Cheese", "South").unwrap();
            conn
        }

        #[test]
        fn test_products_in_region() {
            let conn = seed();
            let north = products_in_region(&conn, "North").unwrap();
            assert_eq!(north.len(), 2);
            assert!(north.iter().any(|(id, name)| *id == 1 && name == "Apple"));
            assert!(north.iter().any(|(id, name)| *id == 3 && name == "Milk"));
        }

        #[test]
        fn test_regions_with_count() {
            let conn = seed();
            let regions = regions_with_count(&conn).unwrap();
            let north_count = regions.iter().find(|(r, _)| r == "North").map(|(_, c)| *c);
            assert_eq!(north_count, Some(2));
        }
    }

    mod step_04_csv_import {
        use super::*;

        #[test]
        fn test_import_csv_from_file() {
            let conn = open_in_memory().unwrap();
            let n = import_csv_from_file(&conn, "products", "data/products.csv").unwrap();
            assert_eq!(n, 6);
            assert_eq!(count_products_with_table(&conn, "products").unwrap(), 6);
        }

        fn count_products_with_table(conn: &Connection, table: &str) -> Result<i64> {
            let sql = format!("SELECT COUNT(*) FROM {}", table);
            let mut stmt = conn.prepare(&sql)?;
            let count: i64 = stmt.query_row([], |row| row.get(0))?;
            Ok(count)
        }
    }

    mod step_05_prepared {
        use super::*;

        #[test]
        fn test_prepared_count_uses_parameter() {
            let conn = open_in_memory().unwrap();
            create_products_table(&conn).unwrap();
            insert_product(&conn, 1, "Apple", "North").unwrap();
            insert_product(&conn, 2, "Bread", "South").unwrap();
            insert_product(&conn, 3, "Milk", "North").unwrap();
            assert_eq!(prepared_count(&conn, "North").unwrap(), 2);
            assert_eq!(prepared_count(&conn, "South").unwrap(), 1);
        }
    }

    mod step_06_ad_hoc_sql {
        use super::*;

        #[test]
        fn test_run_sql_aggregates() {
            let conn = open_in_memory().unwrap();
            create_products_table(&conn).unwrap();
            insert_product(&conn, 1, "Apple", "North").unwrap();
            insert_product(&conn, 2, "Bread", "South").unwrap();
            insert_product(&conn, 3, "Milk", "North").unwrap();
            let rows = run_sql(&conn, "SELECT region, COUNT(*) FROM products GROUP BY region ORDER BY region").unwrap();
            assert_eq!(rows.len(), 2);
            assert_eq!(rows[0][0], "North");
            assert_eq!(rows[0][1], "2");
        }
    }
}
