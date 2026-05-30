//! Shuffle a vector
//! # What is a Vector and how is different from a arrays?
//! In Rust, a vector is a dynamic array that can grow or shrink in size. It is a collection type that is part of 
//! the standard library and is represented by the Vec type.
//! 
//! The main difference between arrays and vectors in Rust is that arrays have a fixed size determined at compile time,
//! while vectors can chane in size at runtime. Vectors also have additionla methods for dynamic operations such as adding,
//! removing, and modifying elements, while arrays do not have these methods.
//!
//! # What is the use of the SliceRangom trait from the rand crate in the program?
//! The `SliceRandom` trait from the `rand` crate is used in the program to shuffle the elements of the vector. It provides the
//! `shuffle` whitch shuffles the elements of the slice in a random order. In this case, it's used to shuffle the elments of the 
//! `fruit` array before printing out the "Fruit salad".
//!
//! # Why is enumerate method used while printing the fruits? What functionality does it provide?
//! The `enumerate` method is used to iterate over the elements of the `fruit` array and provide both the index and the value
//! of each element in the iteration.
//! This functionality is useful when you want to access both the index and the value of each element in a collection, for example,
//! when you want to print out the index along with the value in a formatted output.

use rand::rngs::ThreadRng;
use rand::seq::SliceRandom;
use rand::{thread_rng, Rng};

// The static array of all fruits
const FRUITS: [&str; 10] = ["Orange", "Apple", "Banana", "Pear", "Grape", "Watermelon", "Strawberry", "Cherry", "Plum", "Peach"];

fn main() {
    // Create a random number generator
    let mut rng = thread_rng();

    // Get a random number between 1 and FRUITS.len()
    let fruit_count = rng.gen_range(1..=FRUITS.len());

    // Challenge(3): Select `fruit_count` random fruits 
    let mut fruit: Vec<&str> = select_random_fruits(fruit_count, FRUITS.as_slice(), &mut rng);

    // Challenge(2): Select a random fruit from the salad
    let random_fruit = fruit.choose(&mut rng);
    println!("Random fruit: {}", random_fruit.unwrap());


    // Scramble (shuffle) the vector
    fruit.shuffle(&mut rng);

    // Print out the fruit salad
    println!("Fruit salad:");
    for (i, item) in fruit.iter().enumerate() {
        if i != fruit.len() - 1 {
            print!("{}, ", item);
        } else {
            println!("{}", item);
        }
    }
}

// Select `fruit_count` random fruits
fn select_random_fruits(fruit_count: usize, fruits: &[&'static str], rng: &mut ThreadRng) -> Vec<&'static str> {
    let mut selected_fruits = Vec::new();
    for _ in 0..fruit_count {
        let random_index = rng.gen_range(0..fruits.len());
        selected_fruits.push(fruits[random_index]);
    }
    selected_fruits
}
