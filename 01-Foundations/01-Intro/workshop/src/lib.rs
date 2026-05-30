// ============================================================
// 0-Intro — Library
// ============================================================
// Replace each `todo!()` with your implementation as you
// progress through the README tutorial.
// Run `cargo test` to watch your pass count grow.
// ============================================================

#![allow(unused_variables)]
#![allow(unused_imports)]

/// Converts Celsius to Fahrenheit.
/// README §5: Functions
pub fn celsius_to_fahrenheit(c: f64) -> f64 {
    todo!()
}

/// Converts Fahrenheit to Celsius.
/// README §5: Exercise
pub fn fahrenheit_to_celsius(f: f64) -> f64 {
    todo!()
}

/// Doubles an integer (demonstrates return value without `return` keyword).
/// README §5: The last expression is returned
pub fn double(x: i32) -> i32 {
    todo!()
}

/// Calculate the mean of a slice of f64 values.
/// README §8: Putting It All Together
pub fn mean(values: &[f64]) -> f64 {
    todo!()
}

/// Classifies a temperature as "cold", "mild", or "hot".
/// Demonstrates `if` as an expression.
/// README §7: if/else as expression
pub fn classify_temp(temp: i32) -> &'static str {
    todo!()
}

/// Plays a round of the Guess the Number game.
/// Returns true if the player wins within the given attempts.
/// README §10: Exercise
pub fn play_guess_game(secret: u32, guess: u32) -> bool {
    todo!()
}

// ============================================================
// Tests
// ============================================================

#[cfg(test)]
mod tests {
    use super::*;

    // ---- Step 1: Functions and return values (README §5) ----

    mod step_01_functions {
        use super::super::*;

        #[test]
        fn test_celsius_to_fahrenheit_freezing() {
            assert!((celsius_to_fahrenheit(0.0) - 32.0).abs() < 1e-10);
        }

        #[test]
        fn test_celsius_to_fahrenheit_boiling() {
            assert!((celsius_to_fahrenheit(100.0) - 212.0).abs() < 1e-10);
        }

        #[test]
        fn test_celsius_to_fahrenheit_negative() {
            assert!((celsius_to_fahrenheit(-40.0) - (-40.0)).abs() < 1e-10);
        }

        #[test]
        fn test_fahrenheit_to_celsius_freezing() {
            assert!((fahrenheit_to_celsius(32.0) - 0.0).abs() < 1e-10);
        }

        #[test]
        fn test_fahrenheit_to_celsius_boiling() {
            assert!((fahrenheit_to_celsius(212.0) - 100.0).abs() < 1e-10);
        }

        #[test]
        fn test_double_positive() {
            assert_eq!(double(5), 10);
        }

        #[test]
        fn test_double_negative() {
            assert_eq!(double(-3), -6);
        }

        #[test]
        fn test_double_zero() {
            assert_eq!(double(0), 0);
        }
    }

    // ---- Step 2: Variables and mutability (README §6) ----

    mod step_02_variables {
        use super::super::*;

        #[test]
        fn test_mean_basic() {
            let data = [1.0, 2.0, 3.0, 4.0, 5.0];
            assert!((mean(&data) - 3.0).abs() < 1e-10);
        }

        #[test]
        fn test_mean_single() {
            assert!((mean(&[42.0]) - 42.0).abs() < 1e-10);
        }

        #[test]
        fn test_mean_zero_values() {
            let data = [0.0];
            assert!((mean(&data) - 0.0).abs() < 1e-10);
        }

        #[test]
        fn test_mean_empty_returns_nan() {
            let data: [f64; 0] = [];
            assert!(mean(&data).is_nan());
        }
    }

    // ---- Step 3: if/else as expression (README §7) ----

    mod step_03_control_flow {
        use super::super::*;

        #[test]
        fn test_classify_hot() {
            assert_eq!(classify_temp(35), "hot");
        }

        #[test]
        fn test_classify_threshold_hot() {
            assert_eq!(classify_temp(30), "hot");
        }

        #[test]
        fn test_classify_mild() {
            assert_eq!(classify_temp(20), "mild");
        }

        #[test]
        fn test_classify_cold() {
            assert_eq!(classify_temp(5), "cold");
        }

        #[test]
        fn test_classify_threshold_cold() {
            assert_eq!(classify_temp(10), "mild"); // 10 is not < 10
        }
    }

    // ---- Step 4: Guess the Number (README §10) ----

    mod step_04_guess_game {
        use super::super::*;

        #[test]
        fn test_correct_guess_wins() {
            assert!(play_guess_game(42, 42));
        }

        #[test]
        fn test_incorrect_guess_loses() {
            assert!(!play_guess_game(42, 43));
        }

        #[test]
        fn test_guess_zero() {
            assert!(play_guess_game(0, 0));
        }

        #[test]
        fn test_guess_boundary() {
            assert!(play_guess_game(99, 99));
        }
    }
}
