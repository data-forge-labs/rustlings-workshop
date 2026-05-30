use std::collections::{HashMap, LinkedList, VecDeque};

/// Return (Vec, HashMap, LinkedList, VecDeque) demonstrating each type.
pub fn demonstrate_collections() -> (Vec<i32>, HashMap<&'static str, i32>, LinkedList<&'static str>, VecDeque<&'static str>) {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    mod step_01_vec {
        #[test]
        fn test_vec_push_pop() {
            let mut v = Vec::new();
            v.push(1);
            v.push(2);
            assert_eq!(v.pop(), Some(2));
            assert_eq!(v.len(), 1);
        }

        #[test]
        fn test_vec_empty_pop() {
            let mut v: Vec<i32> = Vec::new();
            assert_eq!(v.pop(), None);
        }
    }

    mod step_02_hashmap {
        #[test]
        fn test_hashmap_insert_get() {
            let mut map = HashMap::new();
            map.insert("a", 1);
            assert_eq!(map.get("a"), Some(&1));
        }

        #[test]
        fn test_hashmap_missing_key() {
            let map: HashMap<&str, i32> = HashMap::new();
            assert_eq!(map.get("x"), None);
        }
    }

    mod step_03_vecdeque {
        #[test]
        fn test_vecdeque_push_pop() {
            let mut d = VecDeque::new();
            d.push_back(1);
            d.push_front(0);
            assert_eq!(d.pop_front(), Some(0));
            assert_eq!(d.pop_back(), Some(1));
        }
    }

    mod step_04_linked_list {
        #[test]
        fn test_linked_list_push() {
            let mut list = LinkedList::new();
            list.push_back("a");
            list.push_back("b");
            assert_eq!(list.len(), 2);
        }
    }
}
