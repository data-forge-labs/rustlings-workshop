use std::collections::{BinaryHeap, HashMap};
use std::cmp::Ordering;

pub fn word_counter(text: &str) -> HashMap<String, u32> {
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        *map.entry(word.to_string()).or_insert(0) += 1;
    }
    map
}

#[derive(Debug, Eq)]
pub struct Item {
    pub priority: u32,
    pub value: String,
}

impl Ord for Item {
    fn cmp(&self, other: &Self) -> Ordering {
        self.priority.cmp(&other.priority)
    }
}

impl PartialOrd for Item {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Item {
    fn eq(&self, other: &Self) -> bool {
        self.priority == other.priority && self.value == other.value
    }
}

#[derive(Debug)]
pub struct PriorityQueue {
    items: BinaryHeap<Item>,
}

impl PriorityQueue {
    pub fn new() -> Self {
        PriorityQueue { items: BinaryHeap::new() }
    }

    pub fn push(&mut self, item: Item) {
        self.items.push(item);
    }

    pub fn pop(&mut self) -> Option<Item> {
        self.items.pop()
    }

    pub fn len(&self) -> usize {
        self.items.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod step_01_word_counter {
        #[test]
        fn test_word_counter_empty() {
            let result = word_counter("");
            assert!(result.is_empty());
        }

        #[test]
        fn test_word_counter_basic() {
            let result = word_counter("hello hello world");
            assert_eq!(result.get("hello"), Some(&2));
            assert_eq!(result.get("world"), Some(&1));
        }

        #[test]
        fn test_word_counter_case_sensitive() {
            let result = word_counter("Hello hello");
            assert_eq!(result.len(), 2);
        }
    }

    mod step_02_priority_queue {
        #[test]
        fn test_priority_queue_new() {
            let pq = PriorityQueue::new();
            assert_eq!(pq.len(), 0);
        }

        #[test]
        fn test_priority_queue_push_pop() {
            let mut pq = PriorityQueue::new();
            pq.push(Item { priority: 1, value: "low".into() });
            pq.push(Item { priority: 3, value: "high".into() });
            pq.push(Item { priority: 2, value: "mid".into() });
            assert_eq!(pq.pop().unwrap().value, "high");
            assert_eq!(pq.pop().unwrap().value, "mid");
            assert_eq!(pq.pop().unwrap().value, "low");
            assert_eq!(pq.pop(), None);
        }

        #[test]
        fn test_item_ord() {
            let high = Item { priority: 3, value: "a".into() };
            let low = Item { priority: 1, value: "b".into() };
            assert!(high > low);
        }

        #[test]
        fn test_item_eq() {
            let a = Item { priority: 1, value: "x".into() };
            let b = Item { priority: 1, value: "x".into() };
            assert_eq!(a, b);
        }
    }
}
