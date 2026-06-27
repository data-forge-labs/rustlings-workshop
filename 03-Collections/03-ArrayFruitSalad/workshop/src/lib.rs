/// Build a new array by copying elements from the start.
pub fn take_first<T: Copy>(arr: &[T; 5], n: usize) -> Vec<T> {
    arr.iter().copied().take(n).collect()
}

/// Sum all elements in a fixed-size array.
pub fn sum_array(arr: &[i32; 5]) -> i32 {
    arr.iter().sum()
}

/// Find the max element in a fixed-size array.
pub fn max_array(arr: &[i32; 5]) -> i32 {
    arr.iter().copied().max().unwrap()
}

/// Reverse a fixed-size array into a new array.
pub fn reverse_array(arr: &[i32; 5]) -> [i32; 5] {
    let mut result = *arr;
    result.reverse();
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    mod step_01_array_ops {
        #[test]
        fn test_take_first_three() {
            let arr = [10, 20, 30, 40, 50];
            assert_eq!(take_first(&arr, 3), vec![10, 20, 30]);
        }

        #[test]
        fn test_take_first_zero() {
            let arr = [1, 2, 3, 4, 5];
            assert!(take_first(&arr, 0).is_empty());
        }

        #[test]
        fn test_sum_array() {
            assert_eq!(sum_array(&[1, 2, 3, 4, 5]), 15);
        }

        #[test]
        fn test_sum_array_negatives() {
            assert_eq!(sum_array(&[-1, -2, -3, -4, -5]), -15);
        }

        #[test]
        fn test_max_array() {
            assert_eq!(max_array(&[1, 5, 3, 2, 4]), 5);
        }

        #[test]
        fn test_max_array_negative() {
            assert_eq!(max_array(&[-10, -3, -8, -1, -5]), -1);
        }

        #[test]
        fn test_reverse_array() {
            assert_eq!(reverse_array(&[1, 2, 3, 4, 5]), [5, 4, 3, 2, 1]);
        }
    }
}
