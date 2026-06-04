pub const FRUITS: [&str; 10] = [
    "Orange", "Apple", "Banana", "Pear", "Grape",
    "Watermelon", "Strawberry", "Cherry", "Plum", "Peach",
];

pub fn select_random_fruits<'a>(fruit_count: usize, fruits: &[&'a str], rng: &mut impl rand::Rng) -> Vec<&'a str> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::rng;

    mod step_01_select {
        #[test]
        fn test_select_zero_fruits() {
            let mut rng = rng();
            let result = select_random_fruits(0, &FRUITS, &mut rng);
            assert!(result.is_empty());
        }

        #[test]
        fn test_select_one_fruit() {
            let mut rng = rng();
            let result = select_random_fruits(1, &FRUITS, &mut rng);
            assert_eq!(result.len(), 1);
            assert!(FRUITS.contains(&result[0]));
        }

        #[test]
        fn test_select_multiple_fruits() {
            let mut rng = rng();
            let result = select_random_fruits(5, &FRUITS, &mut rng);
            assert_eq!(result.len(), 5);
        }

        #[test]
        fn test_select_all_fruits() {
            let mut rng = rng();
            let result = select_random_fruits(FRUITS.len(), &FRUITS, &mut rng);
            assert_eq!(result.len(), FRUITS.len());
        }
    }
}
