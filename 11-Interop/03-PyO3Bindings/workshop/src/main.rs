use pyo3_bindings_workshop::{double_values, hello_from_rust, moving_average, normalize, sum_values};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let data = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0];

    println!("Original:    {:?}", data);
    println!("Doubled:     {:?}", double_values(&data));
    println!("Sum:         {}", sum_values(&data));
    println!("Normalized:  {:?}", normalize(&data));
    println!("Moving avg3: {:?}", moving_average(&data, 3));
    println!("Greeting:    {}", hello_from_rust("data engineer"));

    Ok(())
}
