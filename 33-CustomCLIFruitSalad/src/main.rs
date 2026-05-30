/*
Usage:  

cargo run -- fruits.csv
or
cargo run -- --fruits "apple, pear"

 */
//! Reflection Questions:
//! 
//! # What is the role of the create_fruit_salad function in the lib.rs file?
//!
//! The `create_fruit_salad` function in the `lib.rs` file serves the purpose of 
//! taking a mutable `Vec<String>` representing a list of fruits, and shuffling 
//! the order of these fruits randomly. The shuffled list simulates the mixed 
//! nature of a fruit salad. This function leverages the `SliceRandom` trait 
//! provided by the `rand` crate to perform the shuffling in place.
//! 
//! 
//! # How does the program read input from either a CSV file or command-line
//! arguments?
//!
//! The program utilizes the `clap` crate to parse command-line arguments, which
//! allow the user to specify a CSV file or a list of fruits as a string of comma-
//! separated values. In the main function, the `Opts` struct is used to represent
//! the parsed input, where `csvfile` and `fruits` are optional fields. If the
//! `csvfile` option is provided, the program reads the contents of the specified
//! CSV file and converts it into a vector of strings using the `csv_to_vec`
//! function. If a CSV file is not specified, the program checks for `fruits`
//! command-line input, which is then split by commas into a vector. This logic is
//! encapsulated within a `match` statement that determines the source of the fruit
//! list to be used for the fruit salad creation.
//! 
//! # What is the purpose of the Opts struct in main.rs?
//!
//! The `Opts` struct defined in `main.rs` plays a pivotal role in the command-line
//! interface of the program. It is used with the `clap` crate to define and parse
//! the command-line arguments provided by the user. The fields within the `Opts`
//! struct represent the possible options that can be passed to the program. These
//! include specifying a CSV file through the `csvfile` field or providing a list 
//! of fruits directly through the `fruits` field. The `clap` crate automatically
//! generates help messages, version information, and validates input based on the 
//! configuration provided in the `Opts` struct.

use clap::Parser;
use fruit_salad_maker::create_fruit_salad;
use std::io::Write;

#[derive(Parser)]
#[clap(
    version = "1.0",
    author = "Your Name <your.email@example.com>",
    about = "Make a Fruit Salad"
)]
struct Opts {
    /// Fruits input as a string of comma separated values
    #[clap(short, long)]
    fruits: Option<String>,
    csvfile: Option<String>,
}

// Function that converts a csv file to a vector of strings
fn csv_to_vec(csv: &str) -> Vec<String> {
    csv.split(',')
        .map(|s| s.trim().to_string())
        .collect()
}
fn display_fruit_salad(fruits: &Vec<String>) {
    println!("Your fruit salad contains:");
    for fruit in fruits {
        println!("{}", fruit);
    }
}

fn main() {
    let opts: Opts = Opts::parse();

    // Use fruits from CSV file or command-line input
    let fruit_list = match opts.csvfile {
        Some(filename) => {
            let fruits = std::fs::read_to_string(filename)
                .expect("Could not read file");
            csv_to_vec(&fruits)
        },
        None => {
            opts.fruits.unwrap_or_default()
                .split(',')
                .map(|s| s.trim().to_string())
                .collect()
        },
    };

    // display fruit salad
    let fruit_salad = create_fruit_salad(fruit_list);
    display_fruit_salad(&fruit_salad);

    // Challenge(2): Write the fruit salad to a file
    let filename = "fruit_salad.csv";
    let mut file = std::fs::File::create(filename).expect("Could not create file");
    for fruit in fruit_salad {
        writeln!(file, "{}", fruit).expect("Could not write to file");
    }

}
