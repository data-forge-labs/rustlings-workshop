//! # Reflection Questions:
//!
//! # How does the use of BTreeSet affect the output of the program compared to
//! if a HashSet was used?
//!
//! The use of a `BTreeSet` instead of a `HashSet` affects the output of the
//! program in terms of element ordering:
//!
//! - **Ordering**: `BTreeSet` maintains its elements in sorted order, which
//!   means that when iterating over a `BTreeSet`, the elements will be returned
//!   in a predictable order according to the natural ordering of the elements
//!   (e.g., alphabetical order for strings). In contrast, a `HashSet` does not
//!   maintain any particular order of its elements, so the iteration order is
//!   unpredictable.
//!
//! - **Performance**: For certain operations like looking up an item, a
//!   `BTreeSet` could have slower performance compared to a `HashSet`, as
//!   `BTreeSet` has O(log n) lookup time, whereas `HashSet` typically offers
//!   O(1) average-case performance.
//!
//! If the program relies on the sorted nature of the elements or the order of
//! iteration, then using a `BTreeSet` would be necessary. If the program only
//! requires uniqueness of elements and faster access is preferred, then a
//! `HashSet` would be more suitable.
//!
//! # What are the benefits of using a BTreeSet over other collection types?
//!
//! Using a `BTreeSet` over other collection types like `HashSet`, `Vec`, or
//! `LinkedList` has several benefits:
//!
//! - **Sorted Order**: `BTreeSet` automatically keeps its elements in sorted
//!   order. This is beneficial when sorted data is required for the application
//!   logic, such as maintaining a leaderboard or retrieving elements in a
//!   specific range.
//!
//! - **Consistent Iteration Order**: Unlike `HashSet`, which has an unpredictable
//!   iteration order, `BTreeSet` iterates over elements in a consistent, sorted
//!   manner, which can be important for repeatability in certain applications.
//!
//! - **Range Queries**: `BTreeSet` allows efficient range queries, meaning you
//!   can easily get all elements between two values, which is not directly
//!   supported by `HashSet`, `Vec`, or `LinkedList`.
//!
//! - **Memory Overhead**: While `HashSet` might have faster access times, it
//!   typically has higher memory overhead due to hashing. `BTreeSet` may use
//!   memory more efficiently, especially with a large number of elements.
//!
//! - **Element Insertion and Removal**: For `Vec` and `LinkedList`, inserting or
//!   removing elements in the middle of the collection is costly (O(n) for `Vec`,
//!   O(1) for `LinkedList` if you have a reference to the node). However, a
//!   `BTreeSet` can do these operations in O(log n) time, which can be more
//!   efficient, especially for large datasets.
//!
//! The choice of using a `BTreeSet` should be based on the specific requirements
//! of the application, particularly when ordered data and range queries are
//! critical to the application's functionality.
//!
//! # What would happen if you changed the BTreeSet to a different collection type,
//! such as a Vec or LinkedList?
//!
//! Changing a `BTreeSet` to a `Vec` or `LinkedList` would have several
//! implications for the program:
//!
//! - **Unsorted Elements**: Both `Vec` and `LinkedList` do not maintain any
//!   inherent order among elements. As a result, the elements would not be
//!   automatically sorted as they are in a `BTreeSet`.
//!
//! - **Duplicates Allowed**: Unlike `BTreeSet`, which ensures that each element
//!   is unique, both `Vec` and `LinkedList` allow duplicates. Code that assumes
//!   uniqueness would need to be revised to handle potential duplicates.
//!
//! - **Performance Changes**: Insertion into a `Vec` is O(1) at the end but
//!   can be O(n) anywhere else due to shifting elements. For a `LinkedList`,
//!   insertion is O(1) at the beginning or end, but finding the insertion point
//!   can be O(n). `BTreeSet` guarantees O(log n) for insertions and lookups,
//!   so depending on the use case, changing the collection type could lead to
//!   performance degradation or improvement.
//!
//! - **Iteration**: Iteration over a `Vec` or `LinkedList` is straightforward
//!   and efficient, but the order would be as per insertion rather than sorted.
//!
//! - **Range Queries**: `Vec` and `LinkedList` do not support efficient range
//!   queries like a `BTreeSet`. Implementing such functionality would require
//!   additional logic and could be less performant.
//!
//! If the application depends on the ordered nature of the `BTreeSet`, switching
//! to a `Vec` or `LinkedList` would require significant changes to both the data
//! handling and the related algorithms.
//!
use rand::seq::SliceRandom;
use rand::thread_rng;
use std::collections::{BTreeSet, HashMap};

fn main() {
    let fruits = vec![
        "apple",
        "banana",
        "cherry",
        "date",
        "elderberry",
        "fig",
        "grape",
        "honeydew",
    ];
    let amounts = [1, 3, 5, 7, 9];

    let mut rng = thread_rng();

    let mut fruit_counter = HashMap::new();

    for amount in amounts.iter() {
        let mut fruit_set = BTreeSet::new();
        let mut shuffled_fruits = fruits.clone();
        shuffled_fruits.shuffle(&mut rng);

        for fruit in shuffled_fruits {
            fruit_set.insert(fruit);

            // Challenge(3): Count the number of times each fruit is generated
            fruit_counter
                .entry(fruit)
                .and_modify(|count| *count += 1)
                .or_insert(1);

            if fruit_set.len() >= *amount {
                break;
            }
        }

        println!("{}: {:?}", amount, fruit_set);
    }

    // Challenge(1): Read a fruit to delete from the user
    let mut fruits_set: BTreeSet<_> = fruits.iter().copied().collect();
    println!("Fruits: {:?}", fruits_set);

    let index = select_fruit(fruits.as_slice());

    match index {
        Some(index) => {
            fruits_set.remove(fruits.get(index).unwrap());
            println!("Fruits: {:?}", fruits_set);
        }
        None => {
            println!("No fruit selected.");
        }
    }

    // Challenge(2): Print the fruits in reverse
    println!("Fruits: {:?}", fruits_set.iter().rev().collect::<Vec<_>>());

    println!("Fruit Counter: {:?}", fruit_counter);
}

// This functions list all the fruits and ask the user which one they want to eliminate, return the selected fruit by the user.
fn select_fruit(fruits: &[&str]) -> Option<usize> {
    println!("Select a fruit to eliminate:");
    for (i, fruit) in fruits.iter().enumerate() {
        println!("{}: {}", i, fruit);
    }
    println!("n: None");

    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    match input.trim().parse() {
        Ok(num) => Some(num),
        Err(_) => None,
    }
}
