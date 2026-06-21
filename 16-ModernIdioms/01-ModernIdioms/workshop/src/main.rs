use modern_idioms::*;

fn main() {
    println!("=== Modern Rust Idioms Workshop ===");
    println!();

    // LazyLock demonstration
    println!("1. LazyLock for global state:");
    println!("   Config loaded: {}", get_config().max_retries);
    println!();

    // array_windows demonstration
    println!("2. array_windows for sliding windows:");
    let data = vec![1, 2, 3, 4, 5, 6, 7, 8];
    let sums = sliding_sum(&data, 3);
    println!("   Sliding sums (window=3): {:?}", sums);
    println!();

    // if let chains demonstration
    println!("3. if let chains for complex matching:");
    let result = process_message(Some(42), Some("hello"));
    println!("   Result: {:?}", result);
    println!();

    // cfg_select! demonstration
    println!("4. cfg_select! for platform-specific code:");
    let platform_info = get_platform_info();
    println!("   Platform: {}", platform_info);
    println!();

    // assert_matches! demonstration
    println!("5. assert_matches! for pattern assertions:");
    let value = parse_response("OK:42");
    println!("   Parsed: {:?}", value);
}