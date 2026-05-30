//! # Reflection Questions:
//! 
//! # How does the push method work in a Vector in Rust?
//!
//! The `push` method in Rust is used to add an element to the end of a `Vec<T>`,
//! which is a growable array type. This method modifies the vector in place, and
//! it requires a mutable reference to the vector. Here's how it works internally:
//!
//! - First, it checks if there is enough capacity in the vector to accommodate
//!   the new element. The capacity is the total amount of space allocated for the
//!   vector, which may be larger than the number of elements (`len`).
//!
//! - If the capacity is sufficient, the element is appended at the end, and the
//!   length of the vector (`len`) is incremented by one.
//!
//! - If the capacity is not sufficient, a new memory block is allocated with more
//!   space (typically doubling the capacity), and the existing elements are moved
//!   to the new block. Then, the new element is appended, and the length is
//!   updated.
//!
//! This method ensures that the vector always has enough space to add new
//! elements, while aiming to minimize the number of allocations needed as the
//! vector grows.
//!
//! ## Example
//!
//! ```rust
//! let mut numbers = Vec::new();
//! numbers.push(1); // numbers now contains [1]
//! numbers.push(2); // numbers now contains [1, 2]
//! ```
//!
//! The `push` method abstracts away these details, providing a simple and
//! efficient interface for working with dynamic arrays in Rust.
//! 
//! # What does the pop method do, and what does it return?
//!
//! The `pop` method in Rust is used to remove the last element from a vector 
//! and return it. This method operates on a mutable reference to the vector, 
//! modifying it in place.
//!
//! ## Behavior of pop
//!
//! - If the vector is non-empty, `pop` removes the last element, decreases the 
//!   vector's length by one, and returns `Some(value)`, where `value` is the 
//!   value of the element that was removed.
//! - If the vector is empty, no elements are removed, and the method returns 
//!   `None`, indicating that there are no elements to pop.
//!
//! The `pop` method provides a safe way to handle the removal of elements from 
//! a vector without directly managing memory or indices, ensuring that the 
//! operation does not cause a buffer underflow.
//!
//! ## Example Usage
//!
//! ```rust
//! let mut numbers = vec![1, 2, 3];
//! assert_eq!(numbers.pop(), Some(3)); // numbers is now [1, 2]
//! assert_eq!(numbers.pop(), Some(2)); // numbers is now [1]
//! assert_eq!(numbers.pop(), Some(1)); // numbers is now []
//! assert_eq!(numbers.pop(), None);    // numbers is still []
//! ```
//! # Why is it necessary to declare the Vector as mutable for these operations?
//!
//! In Rust, mutability is a core concept that determines whether the contents 
//! of a variable can be changed. For operations like `push` and `pop` on a 
//! `Vec<T>`, which modify the vector's size and contents, it is necessary to 
//! declare the vector as mutable to allow these changes to occur.
//!
//! ## Mutability for Safety
//!
//! - **Safety**: Rust's type system ensures safe memory access. Mutability is 
//!   part of this system, preventing accidental changes to data that is not 
//!   intended to be modified.
//!
//! - **Concurrency**: Mutable references have restrictions that prevent data 
//!   races when used with Rust's concurrency features.
//!
//! ## Immutable by Default
//!
//! - Rust variables are immutable by default to encourage developers to think 
//!   carefully about where and when data should be allowed to change, leading 
//!   to more predictable and less error-prone code.
//!
//! ## Declaring Mutability
//!
//! - To perform operations that change the vector, you must explicitly declare 
//!   it mutable using the `mut` keyword:
//!
//! ```rust
//! let mut numbers = Vec::new();
//! numbers.push(4); // Allowed as numbers is mutable
//! ```
//!
//! Without the `mut` keyword, these operations would result in a compile-time 
//! error, as Rust enforces strict controls over mutability to uphold safety 
//! guarantees.
use rand::thread_rng;
use rand::prelude::SliceRandom;

fn main() {
    // Create a vector of fruits.
    let fruit_salad = vec!["apple", "banana", "cherry", "dates", "elderberries"];
    println!("Original fruit salad: {:?}, length: {}\n", fruit_salad, fruit_salad.len()); // Challenge(3): Print the length of the fruit salad

    // Uncommenting the following line will cause a compilation error because fruit_salad is immutable.
    // fruit_salad.push("figs");

    // To mutate the vector, we need to declare it as mutable:
    let mut fruit_salad = vec!["apple", "banana", "cherry", "dates", "elderberries"];
    fruit_salad.sort(); // Challenge(2): Sort the fruit salad
    fruit_salad.push("figs");
    println!("Modified fruit salad: {:?}, length: {}\n", fruit_salad, fruit_salad.len());

    // Challenge(1): Remove a fruit from the salad 
    let mut rng = thread_rng();
    let fruit_to_remove = *fruit_salad.choose(&mut rng).expect("Should have chosen a fruit");
    println!("Fruit to remove: {}", fruit_to_remove);

    remove_fruit(&mut fruit_salad, fruit_to_remove);
    println!("Fruit salad after removal: {:?}, length: {}\n", fruit_salad, fruit_salad.len());
}

fn remove_fruit(fruit_salad: &mut Vec<&str>, fruit_to_remove: &str) {
    if let Some(index) = fruit_salad.iter().position(|&x| x == fruit_to_remove) {
        fruit_salad.remove(index);
    }
}