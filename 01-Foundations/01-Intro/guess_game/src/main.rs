use std::io;

fn main() {
    let secret: u32 = rand::random_range(10..=99);  // rand 0.10+
    let attempts = 5;

    println!("Guess a 2-digits number");
    for i in 1..=attempts {
        println!("Guess {}/{}:", i, attempts);
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("error reading line");
        let guess: u32 = input.trim().parse().expect("Please enter a valid number");

        if guess == secret {
            println!("You Win!");
            return;
        } else if guess > secret {
            println!("Too High!");
        } else {
            println!("Too Low");
        }
    }
    println!("Game Over!!!");
}
