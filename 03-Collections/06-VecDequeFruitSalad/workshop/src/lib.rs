use std::collections::VecDeque;

pub fn make_fruit_deque() -> VecDeque<&'static str> {
    todo!()
}

pub fn format_fruit_salad(fruit: &VecDeque<&str>) -> String {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    mod step_01_vecdeque {
        #[test]
        fn test_make_fruit_deque() {
            let d = make_fruit_deque();
            assert_eq!(d.len(), 3);
        }

        #[test]
        fn test_format_non_empty() {
            let mut d = VecDeque::new();
            d.push_back("Apple");
            d.push_back("Banana");
            let s = format_fruit_salad(&d);
            assert!(s.contains("Apple"));
            assert!(s.contains("Banana"));
        }

        #[test]
        fn test_format_empty() {
            let d = VecDeque::new();
            let s = format_fruit_salad(&d);
            assert!(!s.contains("None"));
        }

        #[test]
        fn test_push_pop() {
            let mut d = VecDeque::new();
            d.push_back("First");
            d.push_back("Last");
            assert_eq!(d.pop_front(), Some("First"));
            assert_eq!(d.pop_back(), Some("Last"));
            assert!(d.is_empty());
        }

        #[test]
        fn test_pop_empty() {
            let mut d: VecDeque<&str> = VecDeque::new();
            assert_eq!(d.pop_front(), None);
            assert_eq!(d.pop_back(), None);
        }
    }
}
