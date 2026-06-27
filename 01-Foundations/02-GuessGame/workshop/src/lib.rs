// ============================================================
// 02-GuessGame — Library
// ============================================================
// Replace each `todo!()` with your implementation as you
// progress through the README tutorial.
// Run `cargo test` to watch your pass count grow.
// ============================================================

#![allow(unused_variables)]
#![allow(unused_imports)]

/// The outcome of comparing a guess to the secret number.
#[derive(Debug, PartialEq)]
pub enum GuessOutcome {
    Correct,
    TooHigh,
    TooLow,
}

/// Compare a guess to the secret and report whether it's correct,
/// too high, or too low.
/// README §3: Custom `enum`
pub fn check_guess(secret: u32, guess: u32) -> GuessOutcome {
    if secret == guess {
        GuessOutcome::Correct
    } else if guess > secret {
        GuessOutcome::TooHigh
    } else {
        GuessOutcome::TooLow
    }
}

/// Convert a hint outcome to the message shown to the player.
/// README §3: Custom `enum`
pub fn hint_for(outcome: GuessOutcome) -> &'static str {
    match outcome {
        GuessOutcome::Correct => "Correct!",
        GuessOutcome::TooHigh => "Too high!",
        GuessOutcome::TooLow => "Too low!",
    }
}

/// Parse the player's input into a number.
/// Returns Ok(number) for valid input, Err(message) for invalid input.
/// README §5 / §7: String vs &str, Result<T, E>, .parse()
pub fn parse_guess(input: &str) -> Result<u32, String> {
    let trimmed = input.trim();
    trimmed
        .parse::<u32>()
        .map_err(|e| format!("Invalid input: {}", e))
}

/// Play one round: parse the input, then compare to the secret.
/// Returns the outcome or an error message if the input was bad.
/// README §6 / §8: ? operator
pub fn play_round(secret: u32, input: &str) -> Result<GuessOutcome, String> {
    let guess = parse_guess(input)?;
    Ok(check_guess(secret, guess))
}

/// Generate a secret number in the inclusive range [min, max].
/// Uses the `rand` crate.
/// README §4: External crates
pub fn generate_secret(min: u32, max: u32) -> u32 {
    use rand::RngExt;
    rand::rng().random_range(min..=max)
}

// ============================================================
// Tests
// ============================================================

#[cfg(test)]
mod tests {
    use super::*;

    // ---- Step 1: check_guess (README §3) ----

    mod step_01_check {
        use super::super::*;

        #[test]
        fn test_correct() {
            assert_eq!(check_guess(42, 42), GuessOutcome::Correct);
        }

        #[test]
        fn test_too_high() {
            assert_eq!(check_guess(42, 50), GuessOutcome::TooHigh);
        }

        #[test]
        fn test_too_low() {
            assert_eq!(check_guess(42, 30), GuessOutcome::TooLow);
        }

        #[test]
        fn test_boundary_high() {
            assert_eq!(check_guess(99, 100), GuessOutcome::TooHigh);
        }

        #[test]
        fn test_boundary_low() {
            assert_eq!(check_guess(10, 0), GuessOutcome::TooLow);
        }
    }

    // ---- Step 2: hint_for (README §3) ----

    mod step_02_hint {
        use super::super::*;

        #[test]
        fn test_hint_correct() {
            assert_eq!(hint_for(GuessOutcome::Correct), "Correct!");
        }

        #[test]
        fn test_hint_too_high() {
            assert_eq!(hint_for(GuessOutcome::TooHigh), "Too high!");
        }

        #[test]
        fn test_hint_too_low() {
            assert_eq!(hint_for(GuessOutcome::TooLow), "Too low!");
        }
    }

    // ---- Step 3: parse_guess (README §5 / §7) ----

    mod step_03_parse {
        use super::super::*;

        #[test]
        fn test_parse_valid() {
            assert_eq!(parse_guess("42"), Ok(42));
        }

        #[test]
        fn test_parse_with_whitespace() {
            // .trim() handles the trailing newline from read_line
            assert_eq!(parse_guess("  42 \n"), Ok(42));
        }

        #[test]
        fn test_parse_zero() {
            assert_eq!(parse_guess("0"), Ok(0));
        }

        #[test]
        fn test_parse_negative_out_of_range() {
            // u32 can't be negative — .parse() will fail
            assert!(parse_guess("-1").is_err());
        }

        #[test]
        fn test_parse_non_numeric() {
            assert!(parse_guess("hello").is_err());
        }

        #[test]
        fn test_parse_empty_string() {
            assert!(parse_guess("").is_err());
        }
    }

    // ---- Step 4: play_round (README §6 / §8) ----

    mod step_04_play_round {
        use super::super::*;

        #[test]
        fn test_play_round_correct() {
            assert_eq!(
                play_round(42, "42"),
                Ok(GuessOutcome::Correct)
            );
        }

        #[test]
        fn test_play_round_too_low() {
            assert_eq!(
                play_round(42, "30"),
                Ok(GuessOutcome::TooLow)
            );
        }

        #[test]
        fn test_play_round_too_high() {
            assert_eq!(
                play_round(42, "60"),
                Ok(GuessOutcome::TooHigh)
            );
        }

        #[test]
        fn test_play_round_bad_input() {
            assert!(play_round(42, "abc").is_err());
        }
    }

    // ---- Step 5: generate_secret (README §4) ----

    mod step_05_secret {
        use super::super::*;

        #[test]
        fn test_secret_in_range_small() {
            for _ in 0..20 {
                let s = generate_secret(10, 99);
                assert!((10..=99).contains(&s), "secret {} not in 10..=99", s);
            }
        }

        #[test]
        fn test_secret_in_range_wide() {
            for _ in 0..20 {
                let s = generate_secret(1, 1000);
                assert!((1..=1000).contains(&s), "secret {} not in 1..=1000", s);
            }
        }
    }
}
