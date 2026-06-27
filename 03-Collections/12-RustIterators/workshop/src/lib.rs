/// Sum all elements using fold.
pub fn sum_with_fold(numbers: &[i32]) -> i32 {
    numbers.iter().fold(0, |acc, &x| acc + x)
}

pub fn keep_even(numbers: &[i32]) -> Vec<i32> {
    numbers.iter().copied().filter(|x| x % 2 == 0).collect()
}

pub fn double_all(numbers: &[i32]) -> Vec<i32> {
    numbers.iter().map(|x| x * 2).collect()
}

pub fn take_first_n<T: Clone>(items: &[T], n: usize) -> Vec<T> {
    items.iter().cloned().take(n).collect()
}

pub fn skip_first_n<T: Clone>(items: &[T], n: usize) -> Vec<T> {
    items.iter().cloned().skip(n).collect()
}

pub fn reverse_slice<T: Clone>(items: &[T]) -> Vec<T> {
    items.iter().cloned().rev().collect()
}

pub fn flatten<T: Clone>(nested: Vec<Vec<T>>) -> Vec<T> {
    nested.into_iter().flatten().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    mod step_01_fold {
        #[test]
        fn test_sum_with_fold() {
            assert_eq!(sum_with_fold(&[1, 2, 3, 4, 5]), 15);
        }

        #[test]
        fn test_sum_with_fold_empty() {
            assert_eq!(sum_with_fold(&[]), 0);
        }
    }

    mod step_02_filter {
        #[test]
        fn test_keep_even() {
            assert_eq!(keep_even(&[1, 2, 3, 4, 5, 6]), vec![2, 4, 6]);
        }

        #[test]
        fn test_keep_even_all_odd() {
            let result: Vec<i32> = keep_even(&[1, 3, 5]);
            assert!(result.is_empty());
        }
    }

    mod step_03_map {
        #[test]
        fn test_double_all() {
            assert_eq!(double_all(&[1, 2, 3]), vec![2, 4, 6]);
        }

        #[test]
        fn test_double_all_empty() {
            let result: Vec<i32> = double_all(&[]);
            assert!(result.is_empty());
        }
    }

    mod step_04_take_skip {
        #[test]
        fn test_take_first_n() {
            assert_eq!(take_first_n(&[1, 2, 3, 4, 5], 3), vec![1, 2, 3]);
        }

        #[test]
        fn test_take_more_than_len() {
            assert_eq!(take_first_n(&[1, 2], 5), vec![1, 2]);
        }

        #[test]
        fn test_skip_first_n() {
            assert_eq!(skip_first_n(&[1, 2, 3, 4, 5], 3), vec![4, 5]);
        }

        #[test]
        fn test_skip_all() {
            let result: Vec<i32> = skip_first_n(&[1, 2], 5);
            assert!(result.is_empty());
        }
    }

    mod step_05_rev {
        #[test]
        fn test_reverse_slice() {
            assert_eq!(reverse_slice(&[1, 2, 3]), vec![3, 2, 1]);
        }
    }

    mod step_06_flatten {
        #[test]
        fn test_flatten() {
            let nested = vec![vec![1, 2], vec![3, 4], vec![5]];
            assert_eq!(flatten(nested), vec![1, 2, 3, 4, 5]);
        }
    }
}
