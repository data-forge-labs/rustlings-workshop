/// Return the longer of two string slices (basic lifetime demo)
pub fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    todo!()
}

/// Return the first element of a slice (lifetime elision)
pub fn first<'a>(items: &'a [i32]) -> &'a i32 {
    todo!()
}

/// A struct with a reference (requires lifetime annotation)
pub struct Bookmark<'a> {
    pub title: &'a str,
    pub url: &'a str,
}

impl<'a> Bookmark<'a> {
    pub fn new(title: &'a str, url: &'a str) -> Self {
        todo!()
    }

    pub fn display(&self) -> String {
        todo!()
    }
}

/// Demonstrate move semantics
pub fn move_demo(s: String) -> String {
    todo!()
}

/// Demonstrate Copy types (i32 is Copy)
pub fn copy_demo(x: i32) -> i32 {
    todo!()
}

/// Return a list of lifetime/ownership concepts
pub fn lifetime_concepts() -> Vec<&'static str> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    mod step_01_lifetime_functions {
        use super::*;

        #[test]
        fn test_longest_returns_longer() {
            let result = longest("abc", "defg");
            assert_eq!(result, "defg");
        }

        #[test]
        fn test_longest_equal_length() {
            let result = longest("abc", "xyz");
            assert_eq!(result, "abc");
        }

        #[test]
        fn test_longest_with_empty() {
            let result = longest("", "hello");
            assert_eq!(result, "hello");
        }

        #[test]
        fn test_first_non_empty() {
            let items = vec![1, 2, 3];
            let result = first(&items);
            assert_eq!(*result, 1);
        }

        #[test]
        fn test_first_single_element() {
            let items = vec![42];
            let result = first(&items);
            assert_eq!(*result, 42);
        }

        #[test]
        #[should_panic]
        fn test_first_empty_slice() {
            let items: Vec<i32> = vec![];
            first(&items);
        }
    }

    mod step_02_struct_lifetimes {
        use super::*;

        #[test]
        fn test_bookmark_new() {
            let bm = Bookmark::new("Rust Lang", "https://www.rust-lang.org");
            assert_eq!(bm.title, "Rust Lang");
            assert_eq!(bm.url, "https://www.rust-lang.org");
        }

        #[test]
        fn test_bookmark_display() {
            let bm = Bookmark::new("Example", "https://example.com");
            let display = bm.display();
            assert!(display.contains("Example"));
            assert!(display.contains("https://example.com"));
        }

        #[test]
        fn test_bookmark_display_format() {
            let bm = Bookmark::new("Rust", "https://rust-lang.org");
            let display = bm.display();
            assert_eq!(display, "Rust - https://rust-lang.org");
        }
    }

    mod step_03_move_vs_copy {
        use super::*;

        #[test]
        fn test_move_demo_returns_string() {
            let input = String::from("hello");
            let output = move_demo(input);
            assert_eq!(output, "hello");
        }

        #[test]
        fn test_move_demo_appends_text() {
            let input = String::from("world");
            let output = move_demo(input);
            assert!(output.len() > 0);
        }

        #[test]
        fn test_copy_demo_returns_input() {
            let x = 42;
            let y = copy_demo(x);
            assert_eq!(y, 42);
            // x is still usable because i32 is Copy
            assert_eq!(x, 42);
        }

        #[test]
        fn test_copy_demo_zero() {
            let x = 0;
            let y = copy_demo(x);
            assert_eq!(y, 0);
        }
    }

    mod step_04_concepts {
        use super::*;

        #[test]
        fn test_lifetime_concepts_non_empty() {
            let concepts = lifetime_concepts();
            assert!(!concepts.is_empty(), "should list at least one concept");
        }

        #[test]
        fn test_lifetime_concepts_includes_lifetime_annotation() {
            let concepts = lifetime_concepts();
            let found = concepts.iter().any(|c| c.contains("'a") || c.contains("lifetime"));
            assert!(found, "concepts should include lifetime annotations");
        }

        #[test]
        fn test_lifetime_concepts_includes_borrow() {
            let concepts = lifetime_concepts();
            let found = concepts.iter().any(|c| c.to_lowercase().contains("borrow"));
            assert!(found, "concepts should include borrow");
        }
    }
}
