//! Reflection Questions:
//!
//! # How does the use of BinaryHeap affect the output of the program compared to
//! if a HashSet or BTreeSet was used?
//!
//! The use of a `BinaryHeap` affects the output in the following ways:
//!
//! - **Ordering**: A `BinaryHeap` is a max-heap by default, meaning that it
//!   always allows access to the maximum element. When we retrieve elements
//!   from a `BinaryHeap`, they come out in descending order until it's empty.
//!   In contrast, a `HashSet` does not maintain any order, and a `BTreeSet`
//!   maintains a sorted order (ascending).
//!
//! - **Duplicates**: `BinaryHeap` allows multiple instances of the same value
//!   (if the elements' type implements `PartialEq` and `Eq`), while both
//!   `HashSet` and `BTreeSet` only keep unique elements, rejecting duplicates.
//!
//! - **Use-Case**: A `BinaryHeap` is ideal for priority queues where access to
//!   the largest element is frequently required. `HashSet` and `BTreeSet` are
//!   better suited for ensuring element uniqueness and membership tests, with
//!   `BTreeSet` also providing sorted iteration.
//!
//! The choice between these structures should be guided by the specific needs
//! of the program, such as whether sorting is required, whether duplicates are
//! allowed, and whether the program benefits from the characteristics of a
//! priority queue.
//!
//! # Benefits of Using a BinaryHeap Over Other Collection Types
//!
//! Utilizing a `BinaryHeap` provides several advantages over other collection
//! types in specific scenarios:
//!
//! - **Efficient Maximum or Minimum Retrieval**: `BinaryHeap` allows for
//!   constant-time retrieval of the largest (or smallest, in a min-heap)
//!   element, which is particularly useful for priority queue implementations.
//!
//! - **Logarithmic Insertion and Removal**: Inserting and removing elements
//!   in a `BinaryHeap` has a time complexity of O(log n), which is efficient
//!   for collections that frequently modify their elements.
//!
//! - **Implicit Sorting**: Although not fully sorted, a `BinaryHeap` maintains
//!   a partial order that can be useful when only the most extreme elements
//!   are needed, without the overhead of full sorting.
//!
//! - **Priority Queue Functionality**: `BinaryHeap` facilitates elements
//!   processing based on priority rather than just insertion order, which is
//!   beneficial for scheduling and simulation systems.
//!
//! - **Flexibility with Duplicates**: Unlike a `HashSet` or `BTreeSet`, a
//!   `BinaryHeap` can handle multiple occurrences of the same element, enabling
//!   it to represent the frequency or weight of elements.
//!
//! Choosing a `BinaryHeap` is advantageous when the use case involves
//! prioritization and efficient access to the most extreme elements, as well as
//! when dealing with a dynamic set of data where elements are continuously
//! added and removed.
//!
//! # What Would Happen If You Changed the BinaryHeap to a Different Collection
//! Type, Such as a Vec or LinkedList?
//!
//! Changing from a `BinaryHeap` to a `Vec` or `LinkedList` would have several
//! implications for the program:
//!
//! - **Loss of Efficient Extreme Element Access**: Unlike `BinaryHeap`, neither
//!   `Vec` nor `LinkedList` provides constant-time access to the largest or
//!   smallest element. Finding these would require a linear search or sorting.
//!
//! - **Increased Complexity for Priority Queues**: Implementing a priority
//!   queue would become more complex and less efficient, as `Vec` and
//!   `LinkedList` do not have inherent priority queue properties.
//!
//! - **Different Performance Characteristics**: Insertion and removal
//!   operations would have different time complexities. For example, inserting
//!   into a `Vec` can be amortized constant time, but removal of arbitrary
//!   elements is linear. For `LinkedList`, both insertion and removal can be
//!   constant time, but only if you have a reference to the node.
//!
//! - **No Implicit Ordering**: `Vec` and `LinkedList` maintain insertion order
//!   and do not provide any sort of implicit ordering for access to elements.
//!   A full sort operation would be needed for ordered retrieval, which is less
//!   efficient than the partial ordering maintained by a `BinaryHeap`.
//!
//! - **Handling of Duplicates**: Both `Vec` and `LinkedList` allow duplicates,
//!   similar to `BinaryHeap`, so in this aspect, the behavior would remain
//!   unchanged.
//!
//! Overall, switching to `Vec` or `LinkedList` would mean losing the benefits
//! of a binary heap's structure, and it would impact the performance and
//! complexity of operations like element addition, removal, and access to
//! extremes.
//!
use rand::seq::SliceRandom;
use rand::thread_rng;
use std::cmp::Ord;
use std::collections::BinaryHeap;

#[derive(Eq, PartialEq, Debug)]
enum Fruit {
    Fig,
    Other(String),
}

// We define Figs as the highest priority by implementing Ord
impl Ord for Fruit {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self, other) {
            (Fruit::Fig, Fruit::Fig) => std::cmp::Ordering::Equal,
            (Fruit::Fig, Fruit::Other(_)) => std::cmp::Ordering::Greater,
            (Fruit::Other(_), Fruit::Fig) => std::cmp::Ordering::Less,
            (Fruit::Other(_), Fruit::Other(_)) => std::cmp::Ordering::Equal,
        }
    }
}

impl PartialOrd for Fruit {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn generate_fruit_salad() -> BinaryHeap<Fruit> {
    let mut rng = thread_rng();
    let fruits = vec![
        "Apple", "Orange", "Pear", "Peach", "Banana", "Fig", "Fig", "Fig", "Fig",
    ];
    let mut fruit_salad = BinaryHeap::new();

    let mut figs_count = 0;
    while figs_count < 2 {
        let fruit = fruits.choose(&mut rng).unwrap();
        if *fruit == "Fig" {
            figs_count += 1;
            fruit_salad.push(Fruit::Fig);
        } else {
            fruit_salad.push(Fruit::Other(fruit.to_string()));
        }
    }

    fruit_salad
}

fn main() {
    let mut fruit_salad = generate_fruit_salad();
    println!("Random Fruit Salad With Two Servings of Figs:");

    // Challenge(1): Ask the user how many fruits they would like to remove
    let fruits_to_remove = ask_fruits_to_remove();
    match fruits_to_remove {
        Some(num) => {
            for _ in 0..num {
                let removed_fruit = fruit_salad.pop();
                match removed_fruit {
                    Some(_) => println!("Removed one {:?}", removed_fruit.unwrap()),
                    None => {
                        println!("No fruit removed.");
                        break;
                    }
                }
            }
        }
        None => {}
    }

    for fruit in fruit_salad.into_sorted_vec() {
        match fruit {
            Fruit::Fig => println!("Fig"),
            Fruit::Other(fruit_name) => println!("{}", fruit_name),
        }
    }
}

// This functions list all the fruits and ask the user which one they want to eliminate, return the selected fruit by the user.
fn ask_fruits_to_remove() -> Option<usize> {
    println!("How many fruits would you like to remove? (press n to remove none)");

    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    match input.trim().parse() {
        Ok(num) => Some(num),
        Err(_) => None,
    }
}
