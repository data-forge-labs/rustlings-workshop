//! # Reflection Questions:
//!
//! # What does the gen_counts() function in `decoder-ring/src/lib.rs` do?
//!
//! The `gen_counts()` function in `decoder-ring/src/lib.rs` constructs and returns
//! a `HashMap<char, f32>` that maps English letters to their corresponding
//! frequency percentages as used in the English language. This function manually
//! inserts a set of letters with their respective frequencies into the hash map,
//! covering the most commonly used letters which account for about 80% of all
//! letters in English texts. This frequency map can be utilized in various text
//! analysis tasks, such as deciphering encoded messages or performing
//! statistical analysis on English language data.
//!
//! # How does the guess_shift function determine the best shift for decryption?
//!
//! The `guess_shift` function in `decoder-ring/src/lib.rs` determines the best
//! shift for decryption by trying all possible shift values within a given
//! depth, decrypting the text with each shift, and then scoring the resulting
//! decryption based on a statistical analysis of the letter frequencies
//! compared to standard English language frequencies.
//!
//! It performs the following steps:
//! 1. Iterates over all possible shifts from 0 up to the specified depth.
//! 2. Decrypts the provided text with the current shift value.
//! 3. Performs statistical analysis on the decrypted text to determine how
//!    closely the letter frequencies match those of standard English.
//! 4. Calculates a score for the decrypted text, where a higher score
//!    indicates a closer match to English frequency norms.
//! 5. Keeps track of the shift with the highest score encountered so far.
//! 6. After all shifts have been tried, returns the shift that resulted in the
//!    highest score, along with the corresponding decrypted text and score.
//!
//! The shift that yields the decryption with the highest score is considered the
//! best guess for the actual shift used to encode the original text.
//!
//! # What role do the Args struct and clap::Parser play in `decoder-ring/src/main.rs`?
//!
//! In `decoder-ring/src/main.rs`, the `Args` struct defines the expected command
//! line arguments for the application. It uses annotations provided by the
//! `clap` crate to specify how command line arguments should be parsed and
//! what kind of values they should hold.
//!
//! The `clap::Parser` trait is implemented for the `Args` struct, enabling
//! automatic parsing of command line arguments based on the specifications in
//! `Args`. When the `parse` method of `clap::Parser` is called in `main`, it
//! processes the command line input provided by the user and populates the `Args`
//! struct with the parsed values.
//!
//! This allows the `main` function to easily access the command line arguments
//! (such as `message`, `stats`, and `guess`) and use them to control the
//! behavior of the program, such as deciding whether to perform a statistical
//! analysis or to guess the shift used in a Caesar cipher.
//!
//! Challenge Questions:
//! 
//! # How can you further optimize the scoring mechanism in guess_shift?
//!
//! The `guess_shift_parallel` version of the `guess_shift` function in
//! `decoder-ring/src/lib.rs` is optimized using the Rayon library to perform
//! decryption and analysis across multiple threads in parallel. It achieves this
//! by replacing the sequential iteration over possible shifts with Rayon's
//! `into_par_iter`, which divides the work of trying different shifts across
//! the available CPU cores. This parallel iteration allows for concurrent
//! decryption and scoring of text, significantly speeding up the process of
//! finding the best shift for decryption, especially when the number of shifts
//! (depth) is large.
//! 
//! To observe the performance difference between the `guess_shift` and
//! `guess_shift_parallel` functions, you can execute the provided benchmarks.
//! These are located in the `benches` directory, typically within a file named
//! `bench.rs`. By running these benchmarks, which utilize the Criterion
//! framework, you can measure and compare the execution time of both functions
//! under controlled conditions.
//!
//! To run the benchmarks, use the following command:
//!
//! ```sh
//! cargo bench
//! ```
//!
//! This command will compile the benchmark tests and then run them, outputting
//! the timing measurements for each function. By examining the results, you can
//! see the performance impact of the parallelization introduced in the
//! `guess_shift_parallel` function.
//! 
//! 

use std::collections::HashMap;

/// Build a map of English letter frequencies (percentages).
pub fn gen_counts() -> HashMap<char, f32> {
    let mut m = HashMap::new();
    m.insert('a', 8.17);
    m.insert('b', 1.49);
    m.insert('c', 2.78);
    m.insert('d', 4.25);
    m.insert('e', 12.70);
    m.insert('f', 2.23);
    m.insert('g', 2.02);
    m.insert('h', 6.09);
    m.insert('i', 6.97);
    m.insert('j', 0.15);
    m.insert('k', 0.77);
    m.insert('l', 4.03);
    m.insert('m', 2.41);
    m.insert('n', 6.75);
    m.insert('o', 7.51);
    m.insert('p', 1.93);
    m.insert('q', 0.10);
    m.insert('r', 5.99);
    m.insert('s', 6.33);
    m.insert('t', 9.06);
    m.insert('u', 2.76);
    m.insert('v', 0.98);
    m.insert('w', 2.36);
    m.insert('x', 0.15);
    m.insert('y', 1.97);
    m.insert('z', 0.07);
    m
}

/// Apply a Caesar shift back by `shift` positions.
pub fn decrypt(text: &str, shift: usize) -> String {
    text.chars()
        .map(|c| {
            if c.is_ascii_lowercase() {
                (b'a' + ((c as u8 - b'a' + 26 - shift as u8) % 26)) as char
            } else {
                c
            }
        })
        .collect()
}

/// Score how English-like a piece of text is using letter frequency comparison.
pub fn score_text(text: &str, freqs: &HashMap<char, f32>) -> f32 {
    let lower: String = text.chars().filter(|c| c.is_ascii_lowercase()).collect();
    if lower.is_empty() {
        return 0.0;
    }
    let len = lower.len() as f32;
    let mut observed = HashMap::new();
    for c in lower.chars() {
        *observed.entry(c).or_insert(0.0f32) += 1.0;
    }
    let mut score = 0.0f32;
    for (ch, count) in &observed {
        let observed_pct = (count / len) * 100.0;
        let expected_pct = freqs.get(ch).copied().unwrap_or(0.0);
        score += 1.0 - (observed_pct - expected_pct).abs() / expected_pct.max(1.0);
    }
    score
}

/// Try all shifts up to `depth` and return the best (shift, decrypted text, score).
pub fn guess_shift(text: &str, depth: usize) -> (usize, String, f32) {
    let freqs = gen_counts();
    let mut best_shift = 0;
    let mut best_text = String::new();
    let mut best_score = f32::NEG_INFINITY;
    for shift in 0..depth {
        let decrypted = decrypt(text, shift);
        let score = score_text(&decrypted, &freqs);
        if score > best_score {
            best_score = score;
            best_shift = shift;
            best_text = decrypted;
        }
    }
    (best_shift, best_text, best_score)
}

/// Parallel version of `guess_shift` using Rayon.
pub fn guess_shift_parallel(text: &str, depth: usize) -> (usize, String, f32) {
    use rayon::prelude::*;
    let freqs = gen_counts();
    (0..depth)
        .into_par_iter()
        .map(|shift| {
            let decrypted = decrypt(text, shift);
            let score = score_text(&decrypted, &freqs);
            (shift, decrypted, score)
        })
        .max_by(|a, b| a.2.partial_cmp(&b.2).unwrap())
        .unwrap()
}

#[cfg(test)]
mod tests {
    mod step_01_frequencies {
        use crate::gen_counts;

        #[test]
        fn gen_counts_contains_expected_letters() {
            let freqs = gen_counts();
            assert!(!freqs.is_empty(), "Frequency map should not be empty");
            assert!(freqs.contains_key(&'e'), "Should contain 'e'");
            assert!(freqs.contains_key(&'t'), "Should contain 't'");
            assert!(freqs.contains_key(&'a'), "Should contain 'a'");
        }
    }

    mod step_02_decryption {
        use crate::decrypt;

        #[test]
        fn decrypt_basic_shift() {
            assert_eq!(decrypt("bcd", 1), "abc");
        }

        #[test]
        fn decrypt_wrap_around() {
            assert_eq!(decrypt("abc", 3), "xyz");
        }

        #[test]
        fn decrypt_empty_string() {
            assert_eq!(decrypt("", 5), "");
        }

        #[test]
        fn decrypt_non_alpha_preserved() {
            assert_eq!(decrypt("ifmmo, xpsme!", 1), "hello, world!");
        }
    }

    mod step_03_scoring {
        use crate::{score_text, gen_counts};

        #[test]
        fn english_text_scores_higher_than_random() {
            let freqs = gen_counts();
            let english = "hello world this is a test of the emergency broadcast system";
            let random = "xzfq kcqr jwxr zw rjxx ql hjc djhjwpkb jcvtqhvf dlrjdv";
            let english_score = score_text(english, &freqs);
            let random_score = score_text(random, &freqs);
            assert!(
                english_score > random_score,
                "English-like text ({}) should score higher than random text ({})",
                english_score,
                random_score
            );
        }
    }

    mod step_04_guess {
        use crate::{guess_shift, decrypt};

        #[test]
        fn known_shift_returns_correct_shift() {
            // "hello" encrypted with forward shift 23 becomes "ebiil".
            // decrypt("ebiil", 3) recovers "hello", so guess_shift should pick shift 3.
            let encrypted = decrypt("hello", 23);
            let (shift, decrypted, _score) = guess_shift(&encrypted, 26);
            assert_eq!(shift, 3, "Should detect shift 3 (26 - 23)");
            assert_eq!(decrypted.trim(), "hello", "Should recover original message");
        }
    }
}
