use basic_calculator::*;

fn main() {
    println!("=== Basic Calculator ===\n");

    // Basic operations
    println!("10 + 5 = {}", add(10, 5));
    println!("10 - 5 = {}", subtract(10, 5));
    println!("10 * 5 = {}", multiply(10, 5));
    println!("10 / 5 = {}", divide(10, 5));

    // Safe factorial
    println!("\n--- Safe Factorial ---");
    for i in 0..=20 {
        let fact = factorial_safe(i);
        if fact == 0 && i > 0 {
            println!("{}! = OVERFLOW (returned 0)", i);
        } else {
            println!("{}! = {}", i, fact);
        }
    }

    // Demonstrate type ranges
    println!("\n--- Type Information ---");
    demonstrate_types();

    // Integer vs float division
    println!("\n--- Integer vs Float Division ---");
    println!("5 / 2 = {}", 5 / 2);
    println!("5.0 / 2.0 = {}", 5.0 / 2.0);

    // Type casting
    println!("\n--- Type Casting ---");
    let a: u8 = 200;
    let b: u32 = a as u32;
    println!("u8 {} cast to u32: {}", a, b);

    // Temperature categorization
    println!("\n--- Temperature Check ---");
    let temps = [-5, 15, 35];
    for temp in temps {
        let category = if temp > 30 {
            "hot"
        } else if temp < 10 {
            "cold"
        } else {
            "mild"
        };
        println!("{}°C: {}", temp, category);
    }
}

fn demonstrate_types() {
    println!("usize = {} bits", std::mem::size_of::<usize>() * 8);
    println!("u8:     {} to {}", u8::MIN, u8::MAX);
    println!("u32:    {} to {}", u32::MIN, u32::MAX);
    println!("i32:    {} to {}", i32::MIN, i32::MAX);
    println!("u64:    {} to {}", u64::MIN, u64::MAX);

    let data = [10, 20, 30, 40, 50];
    let index: usize = 2;
    println!("data[{}] = {}", index, data[index]);
}
