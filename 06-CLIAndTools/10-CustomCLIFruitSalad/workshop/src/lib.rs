use rand::seq::SliceRandom;

pub fn create_fruit_salad(mut fruits: Vec<String>) -> Vec<String> {
    todo!()
}

pub fn csv_to_vec(csv: &str) -> Vec<String> {
    csv.split(',')
        .map(|s| s.trim().to_string())
        .collect()
}

pub fn display_fruit_salad(fruits: &[String]) -> String {
    let mut result = String::from("Your fruit salad contains:\n");
    for fruit in fruits {
        result.push_str(&format!("{}\n", fruit));
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    mod step_01_csv_parsing {
        use super::*;

        #[test]
        fn test_csv_to_vec_basic() {
            let result = csv_to_vec("apple,pear,banana");
            assert_eq!(result, vec!["apple", "pear", "banana"]);
        }

        #[test]
        fn test_csv_to_vec_empty() {
            let result = csv_to_vec("");
            assert_eq!(result, vec![""]);
        }

        #[test]
        fn test_csv_to_vec_single_item() {
            let result = csv_to_vec("apple");
            assert_eq!(result, vec!["apple"]);
        }

        #[test]
        fn test_csv_to_vec_whitespace() {
            let result = csv_to_vec(" apple , pear , banana ");
            assert_eq!(result, vec!["apple", "pear", "banana"]);
        }
    }

    mod step_02_fruit_salad {
        use super::*;

        #[test]
        fn test_create_fruit_salad_returns_correct_count() {
            let fruits = vec!["apple".to_string(), "pear".to_string(), "banana".to_string()];
            let result = create_fruit_salad(fruits.clone());
            assert_eq!(result.len(), 3);
        }

        #[test]
        fn test_create_fruit_salad_contains_all_fruits() {
            let fruits = vec!["apple".to_string(), "pear".to_string(), "banana".to_string(), "orange".to_string(), "grape".to_string()];
            let result = create_fruit_salad(fruits.clone());
            let mut result_sorted = result.clone();
            result_sorted.sort();
            let mut fruits_sorted = fruits.clone();
            fruits_sorted.sort();
            assert_eq!(result_sorted, fruits_sorted);
        }

        #[test]
        fn test_create_fruit_salad_empty() {
            let fruits: Vec<String> = vec![];
            let result = create_fruit_salad(fruits);
            assert!(result.is_empty());
        }
    }

    mod step_03_display {
        use super::*;

        #[test]
        fn test_display_fruit_salad_multiple() {
            let fruits = vec!["apple".to_string(), "pear".to_string()];
            let result = display_fruit_salad(&fruits);
            assert_eq!(result, "Your fruit salad contains:\napple\npear\n");
        }

        #[test]
        fn test_display_fruit_salad_single() {
            let fruits = vec!["apple".to_string()];
            let result = display_fruit_salad(&fruits);
            assert_eq!(result, "Your fruit salad contains:\napple\n");
        }

        #[test]
        fn test_display_fruit_salad_empty() {
            let fruits: Vec<String> = vec![];
            let result = display_fruit_salad(&fruits);
            assert_eq!(result, "Your fruit salad contains:\n");
        }
    }
}
