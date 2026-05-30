fn main() {
    println!("Hello, data engineers!");

    // §5: Functions
    let c = 100.0;
    let f = intro::celsius_to_fahrenheit(c);
    println!("{c}°C = {f}°F");

    // §6: Variables (shadowing)
    let data = [1.0, 2.0, 3.0, 4.0, 5.0];
    let result = intro::mean(&data);
    println!("Mean: {result}");

    // §7: if as expression
    let status = if result > 2.0 { "above average" } else { "below average" };
    println!("Status: {status}");

    // §10: Guess game — simulate one round
    let won = intro::play_guess_game(42, 42);
    println!("Guess 42 vs secret 42 → won: {won}");
}
