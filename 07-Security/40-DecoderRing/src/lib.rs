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

fn gen_counts() -> HashMap<char, f32> {
    // Reference letter frequencies in English
    let mut eng_freq: HashMap<char, f32> = HashMap::new();

    // Accounts for 80% of all letters in English
    eng_freq.insert('e', 12.7);
    eng_freq.insert('t', 9.1);
    eng_freq.insert('a', 8.2);
    eng_freq.insert('o', 7.5);
    eng_freq.insert('i', 7.0);
    eng_freq.insert('n', 6.7);
    eng_freq.insert('s', 6.3);
    eng_freq.insert('h', 6.1);
    eng_freq.insert('r', 6.0);
    eng_freq.insert('d', 4.3);

    eng_freq
}

fn stats_analysis(text: &str) -> Vec<(char, u32, f32, Option<f32>, f32)> {
    let mut counts: HashMap<char, u32> = HashMap::new();

    for c in text.chars() {
        *counts.entry(c).or_insert(0) += 1;
    }

    let total: u32 = counts.values().sum();

    let eng_freq_map = gen_counts();

    let mut results = Vec::new();

    for (letter, count) in &counts {
        let freq = (*count as f32 / total as f32) * 100.0;
        let eng_freq = eng_freq_map.get(&letter.to_ascii_lowercase()).cloned();

        let eng_freq_diff = eng_freq.map_or(0.0, |expected_freq| (freq - expected_freq).abs());

        results.push((*letter, *count, freq, eng_freq, eng_freq_diff));
    }
    results
}

pub fn print_stats_analysis(text: &str) {
    let stats = stats_analysis(text);
    for (letter, count, freq, eng_freq, eng_freq_diff) in stats {
        println!(
            "{}: {} ({}%), English Freq: {} ({}%)",
            letter,
            count,
            freq,
            eng_freq.unwrap_or(0.0),
            eng_freq_diff
        );
    }
}

pub fn decrypt(text: &str, shift: u8) -> String {
    let mut result = String::new();

    for c in text.chars() {
        if c.is_ascii_alphabetic() {
            let base = if c.is_ascii_lowercase() { b'a' } else { b'A' };
            let offset = (c as u8 - base + shift) % 26;
            result.push((base + offset) as char);
        } else {
            result.push(c);
        }
    }

    result
}

/*
Guess Shift:

First, uses statistical analysis to determine the most likely shift.
Then, uses the most likely shift to decrypt the message.
Accepts:
 * text: the message to decrypt
 * depth: the number of shifts to try
Returns:
   * depth: the number of shifts to tried
   * shift: the most likely shift
   * decrypted: the decrypted message
*/

pub fn guess_shift(text: &str, depth: u8) -> (u8, u8, String, f32) {
    let mut max_score = 0.0;
    let mut best_shift = 0;
    let mut decrypted = String::new();

    for shift in 0..depth {
        let decrypted_text = decrypt(text, shift);
        let stats = stats_analysis(&decrypted_text);

        let mut score = 0.0;
        for (_, _, freq, eng_freq, eng_freq_diff) in stats {
            if let Some(eng_freq) = eng_freq {
                score += (1.0 - eng_freq_diff / eng_freq) * freq;
            }
        }
        //println!("Shift: {}, Score: {}", shift, score);
        if score > max_score {
            max_score = score;
            best_shift = shift;
            decrypted = decrypted_text;
        }
    }

    (depth, best_shift, decrypted, max_score)
}

use rayon::prelude::*;

// Challenge(3): How can you further optimize the scoring mechanism in guess_shift?
pub fn guess_shift_parallel(text: &str, depth: u8) -> (u8, u8, String, f32) {
    let shifts = 0..depth;
    let (max_score, best_shift, decrypted) = shifts
        .into_par_iter()
        .map(|shift| {
            let decrypted_text = decrypt(text, shift);
            let stats = stats_analysis(&decrypted_text);
            let mut score = 0.0;
            for (_, _, freq, eng_freq, eng_freq_diff) in stats {
                if let Some(eng_freq) = eng_freq {
                    score += (1.0 - eng_freq_diff / eng_freq) * freq;
                }
            }
            (score, shift, decrypted_text)
        })
        .reduce(
            || (0.0, 0, String::new()),
            |(max_score, best_shift, decrypted), (score, shift, decrypted_text)| {
                if score > max_score {
                    (score, shift, decrypted_text)
                } else {
                    (max_score, best_shift, decrypted)
                }
            },
        );

    (depth, best_shift, decrypted, max_score)

    //println!("Shift: {}, Score: {}", shift, score);
}
