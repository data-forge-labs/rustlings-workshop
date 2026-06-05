fn main() {
    println!("Rust Testing — Python pytest equivalent");

    println!("add(2, 3) = {}", rust_testing::add(2, 3));
    println!("divide(10, 2) = {:?}", rust_testing::divide(10.0, 2.0));
    println!("find_item in [1,2,3] for 2: {:?}", rust_testing::find_item(&[1, 2, 3], &2));
    println!("fibonacci(10) = {}", rust_testing::fibonacci(10));
    println!("validate_email('a@b.com'): {}", rust_testing::validate_email("a@b.com"));

    for (rust, py) in rust_testing::test_types() {
        println!("{}  ->  {}", rust, py);
    }
}
