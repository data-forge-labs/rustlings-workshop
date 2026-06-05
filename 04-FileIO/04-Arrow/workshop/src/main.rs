use arrow::array::{Float64Array, Int32Array, StringArray};
use arrow::datatypes::{DataType, Field, Schema};
use arrow::record_batch::RecordBatch;
use std::sync::Arc;

fn main() {
    println!("Apache Arrow demo — building a RecordBatch\n");

    // 1. Build three primitive Arrow arrays.
    let ids = Int32Array::from(vec![1, 2, 3, 4, 5]);
    let names = StringArray::from(vec!["Alice", "Bob", "Carol", "Dave", "Eve"]);
    let ages = Int32Array::from(vec![30, 25, 35, 28, 42]);

    // 2. Define a schema (id is non-null; name and age are nullable).
    let schema = Schema::new(vec![
        Field::new("id", DataType::Int32, false),
        Field::new("name", DataType::Utf8, true),
        Field::new("age", DataType::Int32, true),
    ]);

    // 3. Build the RecordBatch — note the Arc-wrapped columns and schema.
    let batch = RecordBatch::try_new(
        Arc::new(schema),
        vec![Arc::new(ids), Arc::new(names), Arc::new(ages)],
    )
    .expect("RecordBatch should be valid");

    // 4. Print shape and schema.
    println!("Shape: {} rows × {} columns", batch.num_rows(), batch.num_columns());
    println!("Schema:\n{:#?}\n", batch.schema());

    // 5. Cast the age column to Float64 and compute a sum.
    let age_f64 = arrow::compute::cast(batch.column_by_name("age").unwrap(), &DataType::Float64)
        .expect("cast should succeed");
    let age_f64_ref = age_f64.as_any().downcast_ref::<Float64Array>().unwrap();
    let total_age: f64 = (0..age_f64_ref.len())
        .map(|i| age_f64_ref.value(i))
        .sum();
    println!("Total age (as f64): {}", total_age);

    println!("\nRun `cargo test` to see all 20 progressive tests.");
}
