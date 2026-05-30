//! # What are the main collection types provided in Rust's standard library? When might you use each one?
//!
//! The Rust standard library categorizes collections into four major types:
//! - Sequences: such as `Vec` and `LinkedList`, for ordered lists.
//! - Maps: like `HashMap` and `BTreeMap`, for key-value storage.
//! - Sets: `HashSet` and `BTreeSet`, for unique element storage.
//! - Specialized structures: `BinaryHeap`, for priority queues.
//!
//! ## Choosing Collection Types
//!
//! Each collection is suited for specific scenarios, based on performance and functionality.
//! - `Vec`: Dynamic arrays, for flexible storage needs.
//! - `HashMap`: Key-value pairs, ideal for lookup operations.
//! - `HashSet`: Unique elements, ensures no duplicates.
//! - `VecDeque`: Double-ended queue, for FIFO or LIFO operations.
//! - `LinkedList`: Doubly-linked list, for efficient insertion and removal.
//! - `BTreeMap`: Key-value storage with sorted keys, for ordered maps.
//! - `BTreeSet`: Unique elements in sorted order, when order matters.
//! - `BinaryHeap`: Priority queue, for prioritized element processing.
//!
//! # How does the documentation recommend deciding between a Vec or HashMap? When would other collections be more optimal?
//!
//! The documentation recommends using `Vec` for most scenarios due to its performance and
//! flexibility. `HashMap` is preferred for efficient key-value lookup needs.
//!
//! ## When to Choose Other Collections
//!
//! - `VecDeque` for FIFO/LIFO operations.
//! - `LinkedList` when frequent insertion and removal are crucial.
//! - `BTreeMap` and `BTreeSet` for ordered data.
//! - `BinaryHeap` for priority-based element processing.
//!
//! Selection is based on specific use cases and performance requirements.
//!
//! According to the docs, what are some key factors to consider when choosing between HashMap and BTreeMap?
//!
//! Key factors when deciding between `HashMap` and `BTreeMap` include:
//! - **Performance**: `HashMap` generally offers faster access times (O(1)) ideal for
//!   high-speed lookups.
//! - **Order**: `BTreeMap` maintains elements in a sorted order, providing O(log n)
//!   access times, suitable for scenarios requiring ordered data or range queries.
//!
//! The choice depends on the application's specific needs regarding performance and
//! data ordering.
//!
//! # What are some differences in performance characteristics between Vec, VecDeque, and LinkedList? When does this matter?
//!
//! - `Vec`: Efficient for contiguous storage and fast access, but less so for frequent
//!   insertions/deletions at the beginning or middle.
//! - `VecDeque`: Optimized for fast insertions and removals at both ends, making it ideal
//!   for queue-like structures.
//! - `LinkedList`: Allows for quick insertions and deletions anywhere in the list, but
//!   suffers from slower access times due to its non-contiguous nature.
//!
//! The choice among these depends on the specific access and modification patterns of
//! your application.
//!
//! # How could using entry API methods like or_insert help optimize accumulators and other use cases?
//!
//! The `entry` API's `or_insert` method optimizes accumulators and similar patterns by:
//! - Reducing the need for multiple lookups in collections like `HashMap`.
//! - Efficiently inserting or updating values with a single operation.
//! - Simplifying code logic for checking the existence of keys before insertion.
//!
//! This approach is particularly useful in scenarios requiring frequent modifications to a collection based on key presence.

use std::collections::{BinaryHeap, HashMap};

fn main() {
    println!("Hello, world!");

    // Challenge(1): Implement a word counter with HashMap
    let sample_text = "Hello, world! This is a sample text. This is another sample text.";
    let word_count = word_counter(sample_text);
    println!("Word count: {:?}", word_count);

    println!();

    // Challenge(2): Implement a priority queue using BinaryHeap.
    let mut priority_queue = PriorityQueue::new();
    priority_queue.push(Item {
        priority: 1,
        value: "A".to_string(),
    });
    priority_queue.push(Item {
        priority: 2,
        value: "B".to_string(),
    });
    priority_queue.push(Item {
        priority: 3,
        value: "C".to_string(),
    });
    println!("Priority Queue: {:?}", priority_queue);
    priority_queue.pop();
    println!("Priority Queue: {:?}", priority_queue);
}

fn word_counter(text: &str) -> HashMap<String, u32> {
    let mut word_count = HashMap::new();
    for word in text.split_whitespace() {
        *word_count.entry(word.to_string()).or_insert(0) += 1;
    }
    word_count
}

use std::cmp::Ordering;

#[derive(Debug)]
struct PriorityQueue {
    items: BinaryHeap<Item>,
}

impl PriorityQueue {
    fn new() -> Self {
        Self {
            items: BinaryHeap::new(),
        }
    }

    fn push(&mut self, item: Item) {
        self.items.push(item);
    }

    fn pop(&mut self) -> Option<Item> {
        self.items.pop()
    }
}

#[derive(Eq, Debug)]
#[allow(dead_code)]
struct Item {
    priority: u32,
    value: String,
}

impl PartialOrd for Item {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(&other))
    }
}

impl Ord for Item {
    fn cmp(&self, other: &Self) -> Ordering {
        self.priority.cmp(&other.priority).then(other.value.cmp(&self.value))
    }
}

impl PartialEq for Item {
    fn eq(&self, other: &Self) -> bool {
        self.priority == other.priority &&
        self.value == other.value
    }
}

// Challenge(5): Write tests for a custom Rust struct that implements Ord and other traits to be usable in BTreeMap and BTreeSet.
#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::{BTreeMap, BTreeSet};

    #[test]
    fn test_item_btree_map() {
        let mut btree_map = BTreeMap::new();
        btree_map.insert(Item { priority: 1, value: "A".to_string() }, "A's value");
        btree_map.insert(Item { priority: 2, value: "B".to_string() }, "B's value");
        btree_map.insert(Item { priority: 3, value: "C".to_string() }, "C's value");

        assert_eq!(btree_map.len(), 3);

        btree_map.remove(&Item { priority: 1, value: "A".to_string() });

        assert!(!btree_map.contains_key(&Item { priority: 1, value: "A".to_string() }));
        assert!(btree_map.contains_key(&Item { priority: 2, value: "B".to_string() }));
        assert!(btree_map.contains_key(&Item { priority: 3, value: "C".to_string() }));
    }

    #[test]
    fn test_item_btree_set() {
        let mut btree_set = BTreeSet::new();
        btree_set.insert(Item { priority: 1, value: "A".to_string() });
        btree_set.insert(Item { priority: 2, value: "B".to_string() });
        btree_set.insert(Item { priority: 2, value: "B".to_string() });

        assert_eq!(btree_set.len(), 2);

        btree_set.remove(&Item { priority: 1, value: "A".to_string() });

        assert!(!btree_set.contains(&Item { priority: 1, value: "A".to_string() }));
        assert!(btree_set.contains(&Item { priority: 2, value: "B".to_string() }));

    }
}