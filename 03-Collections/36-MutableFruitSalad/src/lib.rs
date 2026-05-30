use rand::seq::SliceRandom;

pub fn remove_fruit(fruit_salad: &mut Vec<&str>, fruit_to_remove: &str) -> bool {
    todo!()
}

pub fn add_fruit<'a>(fruit_salad: &mut Vec<&'a str>, fruit: &'a str) {
    todo!()
}

pub fn sort_fruits(fruit_salad: &mut Vec<&str>) {
    todo!()
}

pub fn pick_random_fruit<'a>(fruit_salad: &[&'a str], rng: &mut impl rand::Rng) -> Option<&'a str> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::thread_rng;

    mod step_01_mutability {
        #[test]
        fn test_add_fruit() {
            let mut salad = vec!["apple", "banana"];
            add_fruit(&mut salad, "cherry");
            assert_eq!(salad.len(), 3);
            assert_eq!(salad[2], "cherry");
        }

        #[test]
        fn test_remove_fruit_exists() {
            let mut salad = vec!["apple", "banana", "cherry"];
            let removed = remove_fruit(&mut salad, "banana");
            assert!(removed);
            assert_eq!(salad.len(), 2);
            assert!(!salad.contains(&"banana"));
        }

        #[test]
        fn test_remove_fruit_not_found() {
            let mut salad = vec!["apple", "banana"];
            let removed = remove_fruit(&mut salad, "grape");
            assert!(!removed);
            assert_eq!(salad.len(), 2);
        }

        #[test]
        fn test_sort_fruits() {
            let mut salad = vec!["banana", "apple", "cherry"];
            sort_fruits(&mut salad);
            assert_eq!(salad, vec!["apple", "banana", "cherry"]);
        }

        #[test]
        fn test_pick_random_fruit() {
            let salad = vec!["apple", "banana"];
            let mut rng = thread_rng();
            let picked = pick_random_fruit(&salad, &mut rng);
            assert!(picked.is_some());
            assert!(salad.contains(&picked.unwrap()));
        }

        #[test]
        fn test_pick_random_empty() {
            let salad: Vec<&str> = vec![];
            let mut rng = thread_rng();
            assert_eq!(pick_random_fruit(&salad, &mut rng), None);
        }
    }
}
