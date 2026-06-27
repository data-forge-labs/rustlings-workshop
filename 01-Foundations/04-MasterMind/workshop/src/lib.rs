// ============================================================
// 2-MasterMind — Library
// ============================================================
// Replace each `todo!()` with your implementation as you
// progress through the README / master_mind.md tutorial.
// Run `cargo test` to watch your pass count grow.
// ============================================================

#![allow(unused_variables)]
#![allow(unused_imports)]

pub const DEFAULT_ATTEMPTS: u32 = 20;
pub const HINT_POSITION_COST: u32 = 5;
pub const HINT_DIGIT_COST: u32 = 3;

/// Returns true if the given string consists of 4 unique digits.
/// README §5: Strings, iterators
pub fn has_unique_digits(s: &str) -> bool {
    if s.len() != 4 {
        return false;
    }
    let mut seen = [false; 10];
    for ch in s.chars() {
        match ch.to_digit(10) {
            Some(d) => {
                if seen[d as usize] {
                    return false;
                }
                seen[d as usize] = true;
            }
            None => return false,
        }
    }
    true
}

/// A secret 4-digit code with hint-tracking state.
/// README §7: Structs, Vec, impl
pub struct SecretCode {
    pub digits: Vec<u8>,
    pub revealed_positions: Vec<bool>,
    pub revealed_digits: Vec<bool>,
}

impl SecretCode {
    /// Creates a new random 4-digit code with no hints revealed.
    /// README §7: struct constructors
    pub fn new() -> Self {
        use rand::RngExt;
        let mut digits = Vec::new();
        let mut rng = rand::rng();
        while digits.len() < 4 {
            let d = rng.random_range(0..=9);
            if !digits.contains(&d) {
                digits.push(d);
            }
        }
        SecretCode {
            digits,
            revealed_positions: vec![false; 4],
            revealed_digits: vec![false; 10],
        }
    }

    /// Compares a guess (exactly 4 digits) with the secret.
    /// Returns (green, yellow, red) — exact matches, wrong-position, none.
    /// README §9: Iterators, zip, filter, count
    pub fn evaluate_guess(&self, guess: &str) -> (usize, usize, usize) {
        let guess_digits: Vec<u8> = guess
            .chars()
            .map(|c| c.to_digit(10).unwrap() as u8)
            .collect();

        let mut green = 0;
        let mut yellow = 0;
        let mut secret_matched = [false; 4];
        let mut guess_matched = [false; 4];

        for i in 0..4 {
            if self.digits[i] == guess_digits[i] {
                green += 1;
                secret_matched[i] = true;
                guess_matched[i] = true;
            }
        }

        for i in 0..4 {
            if guess_matched[i] {
                continue;
            }
            for j in 0..4 {
                if !secret_matched[j] && self.digits[j] == guess_digits[i] {
                    yellow += 1;
                    secret_matched[j] = true;
                    break;
                }
            }
        }

        let red = 4 - green - yellow;
        (green, yellow, red)
    }

    /// Returns true if at least one position hint remains.
    pub fn can_give_position_hint(&self) -> bool {
        self.revealed_positions.iter().any(|&r| !r)
    }

    /// Returns true if at least one digit hint remains.
    pub fn can_give_digit_hint(&self) -> bool {
        self.digits.iter().any(|&d| !self.revealed_digits[d as usize])
    }

    /// Reveals one unrevealed position. Returns Some((index, digit)) or None.
    /// README §8: Option, match
    pub fn give_position_hint(&mut self) -> Option<(usize, u8)> {
        for i in 0..4 {
            if !self.revealed_positions[i] {
                self.revealed_positions[i] = true;
                return Some((i, self.digits[i]));
            }
        }
        None
    }

    /// Reveals one unrevealed digit (without position). Returns Some(digit) or None.
    pub fn give_digit_hint(&mut self) -> Option<u8> {
        for &d in &self.digits.clone() {
            if !self.revealed_digits[d as usize] {
                self.revealed_digits[d as usize] = true;
                return Some(d);
            }
        }
        None
    }
}

/// The main game controller: manages attempts, input, hints, and game flow.
/// README §7: struct + impl patterns
pub struct MastermindGame {
    pub secret: SecretCode,
    pub attempts_left: u32,
    pub guess_count: u32,
}

impl MastermindGame {
    /// Creates a new game with a random secret.
    pub fn new(max_attempts: u32) -> Self {
        MastermindGame {
            secret: SecretCode::new(),
            attempts_left: max_attempts,
            guess_count: 0,
        }
    }

    /// Runs the main game loop.
    /// README §11: I/O, §5: loops, §9: branching
    pub fn play(&mut self) {
        use std::io::{self, Write};
        println!("Welcome to Mastermind!");
        println!("I've picked a 4-digit code with unique digits.");
        println!("You have {} attempts.", self.attempts_left);
        println!("Commands: 'hint' (position), 'digit' (digit), 'quit'");

        loop {
            print!("\nAttempts left: {} | Guesses so far: {} | > ", self.attempts_left, self.guess_count);
            io::stdout().flush().unwrap();

            let mut input = String::new();
            if io::stdin().read_line(&mut input).is_err() {
                println!("Error reading input.");
                continue;
            }
            let input = input.trim();

            match input {
                "quit" | "q" => {
                    println!("Thanks for playing!");
                    break;
                }
                "hint" => {
                    if self.attempts_left < HINT_POSITION_COST {
                        println!("Not enough attempts for a position hint (cost: {}).", HINT_POSITION_COST);
                        continue;
                    }
                    match self.secret.give_position_hint() {
                        Some((pos, digit)) => {
                            self.attempts_left -= HINT_POSITION_COST;
                            println!("Position {} is {}", pos, digit);
                        }
                        None => println!("No position hints left."),
                    }
                    continue;
                }
                "digit" => {
                    if self.attempts_left < HINT_DIGIT_COST {
                        println!("Not enough attempts for a digit hint (cost: {}).", HINT_DIGIT_COST);
                        continue;
                    }
                    match self.secret.give_digit_hint() {
                        Some(digit) => {
                            self.attempts_left -= HINT_DIGIT_COST;
                            println!("The code contains digit {}", digit);
                        }
                        None => println!("No digit hints left."),
                    }
                    continue;
                }
                _ => {}
            }

            if !has_unique_digits(input) {
                println!("Please enter exactly 4 unique digits.");
                continue;
            }

            self.attempts_left -= 1;
            self.guess_count += 1;

            let (green, yellow, red) = self.secret.evaluate_guess(input);
            println!("Result: {} green, {} yellow, {} red", green, yellow, red);

            if green == 4 {
                println!("Congratulations! You cracked the code in {} guesses!", self.guess_count);
                break;
            }

            if self.attempts_left == 0 {
                println!("Out of attempts! The code was: {:?}", self.secret.digits);
                break;
            }
        }
    }
}

// ============================================================
// Tests — organised by tutorial step
// ============================================================

#[cfg(test)]
mod tests {
    use super::*;

    // ---- Step 1: Unique-digit validation (README §5) ----

    mod step_01_validation {
        use super::super::*;

        #[test]
        fn test_has_unique_digits_valid() {
            assert!(has_unique_digits("1234"));
        }

        #[test]
        fn test_has_unique_digits_repeat() {
            assert!(!has_unique_digits("1123"));
        }

        #[test]
        fn test_has_unique_digits_all_same() {
            assert!(!has_unique_digits("1111"));
        }

        #[test]
        fn test_has_unique_digits_too_short() {
            assert!(!has_unique_digits("123"));
        }

        #[test]
        fn test_has_unique_digits_too_long() {
            assert!(!has_unique_digits("12345"));
        }

        #[test]
        fn test_has_unique_digits_non_digit() {
            assert!(!has_unique_digits("12a4"));
        }

        #[test]
        fn test_has_unique_digits_empty() {
            assert!(!has_unique_digits(""));
        }
    }

    // ---- Step 2: SecretCode creation and evaluation (README §7, §9) ----

    mod step_02_secret_code {
        use super::super::*;

        fn known_secret() -> SecretCode {
            SecretCode {
                digits: vec![1, 2, 3, 4],
                revealed_positions: vec![false; 4],
                revealed_digits: vec![false; 10],
            }
        }

        #[test]
        fn test_new_secret_has_four_digits() {
            let secret = SecretCode::new();
            assert_eq!(secret.digits.len(), 4);
        }

        #[test]
        fn test_new_secret_digits_unique() {
            let secret = SecretCode::new();
            let mut seen = [false; 10];
            for &d in &secret.digits {
                assert!(!seen[d as usize], "digit {} repeated", d);
                seen[d as usize] = true;
            }
        }

        #[test]
        fn test_new_secret_no_hints_revealed() {
            let secret = SecretCode::new();
            assert!(secret.revealed_positions.iter().all(|&r| !r));
            assert!(secret.revealed_digits.iter().all(|&r| !r));
        }

        #[test]
        fn test_evaluate_all_green() {
            let secret = known_secret();
            let (g, y, r) = secret.evaluate_guess("1234");
            assert_eq!(g, 4);
            assert_eq!(y, 0);
            assert_eq!(r, 0);
        }

        #[test]
        fn test_evaluate_all_yellow() {
            let secret = known_secret();
            let (g, y, r) = secret.evaluate_guess("4321");
            assert_eq!(g, 0);
            assert_eq!(y, 4);
            assert_eq!(r, 0);
        }

        #[test]
        fn test_evaluate_no_match() {
            let secret = known_secret();
            let (g, y, r) = secret.evaluate_guess("5678");
            assert_eq!(g, 0);
            assert_eq!(y, 0);
            assert_eq!(r, 4);
        }

        #[test]
        fn test_evaluate_mixed() {
            let secret = known_secret();
            let (g, y, r) = secret.evaluate_guess("1256");
            assert_eq!(g, 2); // 1,2 correct position
            assert_eq!(y, 0);
            assert_eq!(r, 2);
        }

        #[test]
        fn test_evaluate_repeat_in_guess() {
            let secret = known_secret();
            let (g, y, r) = secret.evaluate_guess("1156");
            // 1 is green, second 1 has no match (only one 1 in secret)
            assert_eq!(g, 1);
            assert_eq!(y, 0);
            assert_eq!(r, 3);
        }
    }

    // ---- Step 3: Hint system (README §8, §9) ----

    mod step_03_hints {
        use super::super::*;

        fn fresh_secret() -> SecretCode {
            SecretCode {
                digits: vec![1, 2, 3, 4],
                revealed_positions: vec![false; 4],
                revealed_digits: vec![false; 10],
            }
        }

        #[test]
        fn test_can_give_position_hint_initial() {
            let secret = fresh_secret();
            assert!(secret.can_give_position_hint());
        }

        #[test]
        fn test_can_give_digit_hint_initial() {
            let secret = fresh_secret();
            assert!(secret.can_give_digit_hint());
        }

        #[test]
        fn test_give_position_hint_returns_valid() {
            let mut secret = fresh_secret();
            let hint = secret.give_position_hint();
            assert!(hint.is_some());
            let (pos, digit) = hint.unwrap();
            assert!(pos < 4);
            assert_eq!(secret.digits[pos], digit);
        }

        #[test]
        fn test_give_position_hint_marks_revealed() {
            let mut secret = fresh_secret();
            let (pos, _) = secret.give_position_hint().unwrap();
            assert!(secret.revealed_positions[pos]);
        }

        #[test]
        fn test_give_position_hint_exhausts() {
            let mut secret = fresh_secret();
            for _ in 0..4 {
                secret.give_position_hint();
            }
            assert!(!secret.can_give_position_hint());
            assert!(secret.give_position_hint().is_none());
        }

        #[test]
        fn test_give_digit_hint_returns_valid() {
            let mut secret = fresh_secret();
            let hint = secret.give_digit_hint();
            assert!(hint.is_some());
            let digit = hint.unwrap();
            assert!(secret.digits.contains(&digit));
        }

        #[test]
        fn test_give_digit_hint_marks_revealed() {
            let mut secret = fresh_secret();
            let digit = secret.give_digit_hint().unwrap();
            assert!(secret.revealed_digits[digit as usize]);
        }

        #[test]
        fn test_give_digit_hint_exhausts() {
            let mut secret = fresh_secret();
            // At most 4 unique digits to reveal
            for _ in 0..4 {
                secret.give_digit_hint();
            }
            assert!(!secret.can_give_digit_hint());
            assert!(secret.give_digit_hint().is_none());
        }
    }

    // ---- Step 4: Game construction (README §7) ----

    mod step_04_game_setup {
        use super::super::*;

        #[test]
        fn test_game_new_has_secret() {
            let game = MastermindGame::new(10);
            assert_eq!(game.secret.digits.len(), 4);
        }

        #[test]
        fn test_game_new_attempts() {
            let game = MastermindGame::new(15);
            assert_eq!(game.attempts_left, 15);
        }

        #[test]
        fn test_game_new_guess_count_zero() {
            let game = MastermindGame::new(10);
            assert_eq!(game.guess_count, 0);
        }

        #[test]
        fn test_game_default_attempts() {
            let game = MastermindGame::new(DEFAULT_ATTEMPTS);
            assert_eq!(game.attempts_left, 20);
        }

        #[test]
        fn test_game_constants_are_sane() {
            assert!(HINT_POSITION_COST < DEFAULT_ATTEMPTS);
            assert!(HINT_DIGIT_COST < HINT_POSITION_COST);
        }
    }

    // ---- Step 5: Integration — deterministic game flow ----

    mod step_05_integration {
        use super::super::*;

        #[test]
        fn test_known_secret_evaluate_full() {
            let secret = SecretCode {
                digits: vec![7, 2, 9, 0],
                revealed_positions: vec![false; 4],
                revealed_digits: vec![false; 10],
            };
            // Exact match
            assert_eq!(secret.evaluate_guess("7290"), (4, 0, 0));
            // Reverse
            assert_eq!(secret.evaluate_guess("0927"), (0, 4, 0));
            // Partial overlap
            let (g, y, r) = secret.evaluate_guess("7254");
            assert_eq!(g, 1); // 7
            assert_eq!(y, 1); // 2
            assert_eq!(r, 2);
        }

        #[test]
        fn test_evaluate_with_duplicate_secret_value() {
            // Secret has unique digits by design, but test edge: guess uses a digit
            // that appears in secret only once, guessed twice
            let secret = SecretCode {
                digits: vec![5, 1, 3, 8],
                revealed_positions: vec![false; 4],
                revealed_digits: vec![false; 10],
            };
            let (g, y, r) = secret.evaluate_guess("5555");
            // Only one 5 in secret, so only one match
            assert_eq!(g, 1);
            assert_eq!(y, 0);
            assert_eq!(r, 3);
        }
    }
}
