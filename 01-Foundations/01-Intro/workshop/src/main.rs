fn main() {
    println!("Hello, data engineers!");

    // §5: Functions
    let c = 100.0;
    let f = intro::celsius_to_fahrenheit(c);
    println!("{c}°C = {f}°F");

    // §7: if/else as expression
    let label = intro::classify_temp(35);
    println!("35°C is {label}");

    // §8: Loops over a fixed array
    let readings = [10, -3, 25, 0, 7];
    let n = intro::count_positive(readings);
    println!("Positive readings: {n}");

    // §9: Tuples
    let row = (1, 5.0, true);
    let status = intro::categorize_row(row);
    println!("Row status: {status}");

    // §10: Arrays
    let max = intro::max_of_five([3, 1, 4, 1, 5]);
    println!("Max: {max}");

    // §11: Putting it all together
    let (n, l) = intro::hot_readings_summary([22, 28, 31, 35, 30]);
    println!("Hot readings: {n} → {l}");
}
