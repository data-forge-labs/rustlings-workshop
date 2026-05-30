//! # Reflection Questions:
//!
//! # How does the generate_fruit() function ensure a random fruit is chosen every
//! time it's called?
//!
//! The `generate_fruit()` function  ensures that a random fruit is chosen every
//! time it is called by using the `SliceRandom` trait from the `rand` crate, which
//! provides the method `choose` to randomly select an element from a slice. Here
//! is a conceptual outline of the implementation:
//!
//! ```rust
//! use rand::seq::SliceRandom; // Import the necessary trait from the `rand` crate.
//! use rand::thread_rng;
//!
//! fn generate_fruit() -> &'static str {
//!     let fruits = ["apple", "banana", "cherry", "date", "fig"]; // An array of fruits.
//!     let mut rng = thread_rng(); // Create a random number generator.
//!     *fruits.choose(&mut rng).unwrap() // Randomly select and return a fruit reference.
//! }
//! ```
//!
//! By calling `choose` on the array of fruits with a mutable reference to a
//! random number generator, a random element is selected from the slice. The
//! `unwrap` is used to extract the fruit reference from the `Option` returned
//! by `choose`, which is safe in this context because the slice is non-empty.
//! This method provides an easy and efficient way to select a random element,
//! ensuring that a different fruit can be chosen each time the function is
//! called.
//!
//! # What is the purpose of using a HashSet in this program?
//!
//! In the context of the program from
//! `{week-1/Lesson 2 - Rust Sets, Graphs and Misc Data Structures/hashset-fruit/src/main.rs:main}`,
//! a `HashSet` is used to store a unique collection of fruit names. As the
//! `generate_fruit` function is called repeatedly to generate random fruits, the
//! `HashSet` ensures that each fruit name is stored only once, regardless of how
//! many times it is generated. This is due to the property of a `HashSet` that
//! each element must be unique within the set.
//!
//! The program aims to report the number of unique fruits generated after a
//! certain number of iterations. By leveraging the `HashSet`, it can efficiently
//! keep track of unique fruits without manual checks for duplicates, which would
//! be necessary if a `Vec` or an array was used instead.
//!
//! The use of a `HashSet` also simplifies the code and improves performance, as
//! it offers constant time complexity, \(O(1)\), for insertions and lookups,
//! making it an ideal data structure for this use case.
//!
//! # What would happen if you changed the HashSet to a different collection type,
//! such as a Vec or LinkedList?
//!
//! If the `HashSet` in `{week-1/Lesson 2 - Rust Sets, Graphs and Misc Data
//! Structures/hashset-fruit/src/main.rs:main}` were changed to a `Vec` or
//! `LinkedList`, several implications could arise:
//!
//! - **Using a Vec**: This would allow duplicates since a `Vec` does not enforce
//!   uniqueness of elements. The performance for checking if an element exists
//!   before adding it would also become O(n), as it would require iterating
//!   through the entire vector. Additionally, insertion time would be O(1) only
//!   if adding to the end; otherwise, it could be O(n) due to shifting elements.
//!
//! - **Using a LinkedList**: This would also permit duplicates. Performance for
//!   checking existence of an element would be O(n), as it requires traversal
//!   from the beginning or end until the element is found. Insertion times would
//!   be O(1) only at the beginning or end, but O(n) for insertion in the middle.
//!
//! Both changes would necessitate additional logic to maintain the unique set of
//! fruit names that the `HashSet` provides by default. The overall performance
//! for common operations would likely degrade due to the more expensive search
//! and potential insertion costs compared to the `HashSet`.
//!

use rand::seq::SliceRandom;
use rand::thread_rng;
use std::collections::{HashMap, HashSet};

fn generate_fruit() -> &'static str {
    let fruits = [
        "Apple",
        "Banana",
        "Cherry",
        "Date",
        "Elderberry",
        "Fig",
        "Grape",
        "Honeydew",
    ];
    let mut rng = thread_rng();
    fruits.choose(&mut rng).unwrap()
}

fn main() {
    let mut fruit_set = HashSet::new();
    let mut fruit_counter = HashMap::new();
    let number_of_fruits = read_user_input(); // Challenge(1): Read the number of fruits from the user

    println!("Generating {} random fruits...", number_of_fruits);
    for _ in 0..number_of_fruits {
        let fruit = generate_fruit();
        fruit_set.insert(fruit);
        fruit_counter
            .entry(fruit)
            .and_modify(|count| *count += 1)
            .or_insert(1); // Challenge(2): Count the number of times each fruit is generated
    }

    println!("Number of unique fruits generated: {}", fruit_set.len());
    println!(
        "Number of times each fruit was generated: {:?}",
        fruit_counter
    ); // Challenge(3): Print the number of times each fruit was generated
}

// This function read a number of random fruits from the user (cmd line) and return this number
fn read_user_input() -> usize {
    let mut input = String::new();
    println!("Enter the number of fruits you want to generate: ");
    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let input: usize = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Error: Invalid input. Please enter a number.");
            0
        }
    };
    input
}
