use mockall::predicate::*;
use mockall_workshop::{
    batch_etl, count_records, filter_rows, get_schema, is_healthy, run_etl,
    total_rows, validate_pipeline, MockDataSource,
};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let mut mock = MockDataSource::new();
    mock.expect_health().return_const(true);
    mock.expect_schema()
        .return_const(vec!["id".into(), "name".into()]);
    mock.expect_fetch()
        .with(eq("SELECT * FROM users"))
        .returning(|_| Ok(vec!["alice".into(), "bob".into(), "carol".into()]));
    mock.expect_fetch()
        .returning(|_| Ok(vec!["a".into(); 42]));
    mock.expect_record_count()
        .with(eq("users"))
        .returning(|_| Ok(3));
    mock.expect_record_count()
        .with(eq("orders"))
        .returning(|_| Ok(99));

    println!("healthy:   {}", is_healthy(&mock));
    println!("schema:    {:?}", get_schema(&mock));
    println!("etl count: {}", run_etl(&mock, "SELECT * FROM users")?);
    println!("rec count: {}", count_records(&mock, "SELECT *")?);
    println!("filter ap: {:?}", filter_rows(&mock, "SELECT *", "al")?);
    println!("batch:     {:?}", batch_etl(&mock, &["SELECT 1", "SELECT 2"])?);
    println!("validate:  {:?}", validate_pipeline(&mock));
    println!("total:     {}", total_rows(&mock, &["users", "orders"])?);

    Ok(())
}
