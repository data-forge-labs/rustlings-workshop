use std::collections::HashMap;

pub fn count_frequencies(numbers: Vec<i32>) -> HashMap<i32, u32> {
    todo!()
}

pub fn most_frequent(numbers: &[i32]) -> Option<(i32, u32)> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    mod step_01_frequencies {
        #[test]
        fn test_empty() {
            let result = count_frequencies(vec![]);
            assert!(result.is_empty());
        }

        #[test]
        fn test_single_element() {
            let result = count_frequencies(vec![5]);
            assert_eq!(result.get(&5), Some(&1));
        }

        #[test]
        fn test_multiple_occurrences() {
            let result = count_frequencies(vec![1, 2, 2, 3, 3, 3]);
            assert_eq!(result.get(&1), Some(&1));
            assert_eq!(result.get(&2), Some(&2));
            assert_eq!(result.get(&3), Some(&3));
        }

        #[test]
        fn test_all_same() {
            let result = count_frequencies(vec![7, 7, 7, 7]);
            assert_eq!(result.get(&7), Some(&4));
        }
    }

    mod step_02_most_frequent {
        #[test]
        fn test_most_frequent_basic() {
            let nums = vec![1, 2, 2, 3, 3, 3];
            assert_eq!(most_frequent(&nums), Some((3, 3)));
        }

        #[test]
        fn test_most_frequent_empty() {
            assert_eq!(most_frequent(&[]), None);
        }

        #[test]
        fn test_most_frequent_tie() {
            let nums = vec![1, 1, 2, 2];
            let result = most_frequent(&nums);
            assert!(result == Some((1, 2)) || result == Some((2, 2)));
        }
    }
}
