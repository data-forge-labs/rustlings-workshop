// ============================================================
// 02-GuessGame — main.rs (the runnable game)
// ============================================================
// Demonstrates how the lib.rs functions are composed into a
// real interactive program. Run with `cargo run`.
// ============================================================

use std::io;
use std::io::Write; // for `flush()` — see README §4

use guess_game::{check_guess, generate_secret, hint_for, parse_guess};

const MIN: u32 = 1;
const MAX: u32 = 100;
const ATTEMPTS: u32 = 7;

fn main() {
    let secret = generate_secret(MIN, MAX);

    println!("I'm thinking of a number between {MIN} and {MAX}.");
    println!("You have {ATTEMPTS} attempts. Good luck!");

    for attempt in 1..=ATTEMPTS {
        print!("Attempt {attempt}/{ATTEMPTS} > ");
        // Prompt is sometimes buffered without flush; this forces it to print
        // before we read input. Covered in README §4.
        io::stdout().flush().expect("failed to flush stdout");

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("failed to read line");

        match parse_guess(&input) {
            Err(msg) => {
                println!("  ⚠ {msg}");
                println!("  (attempt not counted — try again)");
                continue;
            }
            Ok(guess) => {
                let outcome = check_guess(secret, guess);
                println!("  {}", hint_for(outcome));
                if outcome == guess_game::GuessOutcome::Correct {
                    println!("You win! The secret was {secret}.");
                    return;
                }
            }
        }
    }

    println!("Out of attempts! The secret was {secret}.");
}
