use rand::seq::SliceRandom;
use std::collections::BinaryHeap;

#[derive(Eq, PartialEq, Debug)]
pub enum Fruit {
    Fig,
    Other(String),
}

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

pub fn generate_fruit_salad() -> BinaryHeap<Fruit> {
    let fruits = vec![
        Fruit::Fig,
        Fruit::Other("Apple".into()),
        Fruit::Other("Banana".into()),
        Fruit::Other("Cherry".into()),
    ];
    fruits.into_iter().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    mod step_01_fruit_ord {
        #[test]
        fn test_fig_greater_than_other() {
            let fig = Fruit::Fig;
            let other = Fruit::Other("Apple".into());
            assert!(fig > other);
        }

        #[test]
        fn test_fig_equal_fig() {
            let a = Fruit::Fig;
            let b = Fruit::Fig;
            assert_eq!(a, b);
        }

        #[test]
        fn test_other_equal_other() {
            let a = Fruit::Other("A".into());
            let b = Fruit::Other("B".into());
            assert_eq!(a, b);
        }
    }

    mod step_02_generate {
        #[test]
        fn test_generate_contains_figs() {
            let salad = generate_fruit_salad();
            assert!(salad.iter().any(|f| *f == Fruit::Fig));
        }

        #[test]
        fn test_generate_non_empty() {
            let salad = generate_fruit_salad();
            assert!(!salad.is_empty());
        }
    }
}
