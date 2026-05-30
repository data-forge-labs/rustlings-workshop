use std::collections::{BTreeMap, HashMap, VecDeque};

/// Demonstrate key collection differences.
/// Returns (Vec, VecDeque, HashMap, BTreeMap) of sample data.
pub fn collection_examples() -> (Vec<i32>, VecDeque<i32>, HashMap<&'static str, i32>, BTreeMap<&'static str, i32>) {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    mod step_01_vec_vs_vecdeque {
        #[test]
        fn test_vec_push() {
            let mut v = Vec::new();
            v.push(1);
            v.push(2);
            assert_eq!(v.len(), 2);
        }

        #[test]
        fn test_vecdeque_push_front_back() {
            let mut d = VecDeque::new();
            d.push_front(1);
            d.push_back(2);
            assert_eq!(d.pop_front(), Some(1));
            assert_eq!(d.pop_back(), Some(2));
        }
    }

    mod step_02_hashmap_vs_btreemap {
        #[test]
        fn test_hashmap_insert_lookup() {
            let mut map = HashMap::new();
            map.insert("a", 1);
            assert_eq!(map.get("a"), Some(&1));
        }

        #[test]
        fn test_btreemap_insert_lookup() {
            let mut map = BTreeMap::new();
            map.insert("c", 3);
            map.insert("a", 1);
            map.insert("b", 2);
            let keys: Vec<&&str> = map.keys().collect();
            assert_eq!(keys, vec![&"a", &"b", &"c"]);
        }
    }
}
