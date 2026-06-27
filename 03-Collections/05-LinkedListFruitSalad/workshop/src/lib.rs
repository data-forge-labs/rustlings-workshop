use std::collections::LinkedList;

pub fn make_fruit_list() -> LinkedList<&'static str> {
    let mut list = LinkedList::new();
    list.push_back("Apple");
    list.push_back("Banana");
    list.push_back("Cherry");
    list
}

pub fn shuffle_to_vec(list: LinkedList<&'static str>, rng: &mut impl rand::Rng) -> Vec<&'static str> {
    let mut vec: Vec<&str> = list.into_iter().collect();
    vec.shuffle(rng);
    vec
}

pub fn vec_to_linked_list(vec: Vec<&'static str>) -> LinkedList<&'static str> {
    vec.into_iter().collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::rng;

    mod step_01_linked_list {
        #[test]
        fn test_make_fruit_list() {
            let list = make_fruit_list();
            assert_eq!(list.len(), 3);
        }

        #[test]
        fn test_shuffle_preserves_length() {
            let list = make_fruit_list();
            let mut rng = rng();
            let shuffled = shuffle_to_vec(list, &mut rng);
            assert_eq!(shuffled.len(), 3);
        }

        #[test]
        fn test_vec_to_linked_list() {
            let vec = vec!["a", "b", "c"];
            let list = vec_to_linked_list(vec);
            assert_eq!(list.len(), 3);
        }

        #[test]
        fn test_empty_roundtrip() {
            let vec: Vec<&str> = vec![];
            let list = vec_to_linked_list(vec);
            assert!(list.is_empty());
        }
    }
}
