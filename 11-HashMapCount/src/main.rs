//! # How is HashMap used in this program and what is its function?
//! 
//! - The `HashMap` is created with keys of type `i32` (the numbers from the input vector) and values of type 
//!   `u32` (the frequency of each number).
//! - As the function iterates through the input vector, it uses the `entry` method to insert a key-value 
//!   pair into the `HashMap` if the key doesn't exist, or to update the value if the key already exists.
//! - The `entry` method returns an `Entry` enum, which allows us to either insert a new value or modify
//!   an existing one. In this case, the `or_insert` method is used to set the value to 0 if the key doesn't 
//!   exist, and then increments the value by 1.
//!
//! So, the function's purpose is to create a `HashMap` that represents the frequency of each number in the input vector by iterating through the vector and updating the counts in the `HashMap`.
//!
//! # Why is or_insert(0) used in frequencies.entry(num).or_insert(0)?
//! The or_insert(0) method is used in frequencies.entry(number).or_insert(0) to set the value to 0 if the key doesn't exist, 
//! and then increments the value by 1. This ensures that the frequency count is initialized to 0 if the 
//! number is encountered for the first time, and then increments the count each time the number is encountered 
//! again. This helps in creating a HashMap that represents the frequency of each number in the input vector.
//!
//! # How does the program ensure that each number and its frequency are correctly paired in the final result?
//!
//! The program ensures that each number and its frequency are correctly paired in the final result by 
//! using the HashMap data structure. Specifically, the entry method is used to insert the number into 
//! the map if it doesn't exist, or to update the value if the key already exists. The or_insert method 
//! is then used to set the initial value to 0 if the key doesn't exist, and then increments the value 
//! by 1. This allows for the correct frequency count to be maintained for each number, ensuring that 
//! they are correctly paired in the final result.
//!

use std::collections::{BTreeMap, HashMap};

fn logic(numbers: Vec<i32>) -> HashMap<i32, u32> {
    let mut frequencies = HashMap::new();

    for number in numbers {
        let frecuency = frequencies.entry(number).or_insert(0);
        *frecuency += 1;
    }

    frequencies
}

fn main() {
    let numbers = vec![1, 2, 3, 4, 7, 7, 5, 6, 1, 7, 1, 8, 2, 2,2,2,9, 10];
    let result = logic(numbers);

    // Challenge (3): How would you modify the program to sort the final result by frequency?
    let result: BTreeMap<&i32, &u32> = result.iter().collect();

    println!("The frequency of each number in the vector is: {:?}", result);
}
