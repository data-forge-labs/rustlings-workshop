use rand::seq::SliceRandom;
use std::collections::{HashMap, HashSet};

pub fn generate_fruit() -> &'static str {
    todo!()
}

pub fn collect_unique_fruits(count: usize) -> (HashSet<&'static str>, HashMap<&'static str, u32>) {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    mod step_01_generate {
        #[test]
        fn test_generate_fruit_returns_valid() {
            let valid = ["Apple", "Banana", "Cherry", "Date", "Elderberry",
                         "Fig", "Grape", "Honeydew"];
            let fruit = generate_fruit();
            assert!(valid.contains(&fruit));
        }
    }

    mod step_02_hashset {
        #[test]
        fn test_collect_unique_fruits() {
            let (set, counter) = collect_unique_fruits(10);
            assert!(set.len() <= 8);
            assert!(!set.is_empty());
        }

        #[test]
        fn test_hashset_no_duplicates() {
            let mut set = HashSet::new();
            set.insert("a");
            set.insert("a");
            assert_eq!(set.len(), 1);
        }

        #[test]
        fn test_hashset_insert() {
            let mut set = HashSet::new();
            set.insert("apple");
            assert!(set.contains("apple"));
        }
    }
}
