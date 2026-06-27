// ============================================================
// 4-Traits — Library
// ============================================================
// Replace each `todo!()` with your implementation as you
// progress through the README tutorial.
// Run `cargo test` to watch your pass count grow.
// ============================================================

#![allow(unused_variables)]

use std::fmt;

/// A ticket with trait implementations.
/// README §2-8: Traits
/// (Add `Clone, PartialEq` to the derive macro as you progress through §5)
#[derive(Debug, Clone, PartialEq)]
pub struct Ticket {
    pub title: String,
    pub description: String,
    pub status: String,
}

/// README §2: Implementing Display (manual — like Python's __str__)
impl fmt::Display for Ticket {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}] {} — {}", self.status, self.title, self.description)
    }
}

/// README §4: Derive Debug + Clone + PartialEq
// (You can implement these manually or use derive — README covers both)

/// README §5: From/Into — convert a &str into a Ticket
impl From<&str> for Ticket {
    fn from(title: &str) -> Ticket {
        Ticket {
            title: title.to_string(),
            description: String::new(),
            status: "Open".to_string(),
        }
    }
}

/// README §3: A generic function with trait bounds
/// Formats and returns a string using Display + Debug
pub fn format_summary<T: fmt::Display + fmt::Debug>(item: &T) -> String {
    format!("Display: {}, Debug: {:?}", item, item)
}

/// README §7: A resource that demonstrates Drop
pub struct DatabaseConnection {
    pub url: String,
}

impl DatabaseConnection {
    pub fn new(url: &str) -> DatabaseConnection {
        DatabaseConnection {
            url: url.to_string(),
        }
    }
}

impl Drop for DatabaseConnection {
    fn drop(&mut self) {
        // In a real system this would close the DB connection
    }
}

// ============================================================
// Tests
// ============================================================

#[cfg(test)]
mod tests {
    use super::*;

    mod step_01_display {
        use super::*;

        #[test]
        fn test_display_format() {
            let t = Ticket {
                title: "Bug".into(),
                description: "Fix it".into(),
                status: "Open".into(),
            };
            let s = format!("{}", t);
            assert!(s.contains("Open"));
            assert!(s.contains("Bug"));
        }

        #[test]
        fn test_display_different_status() {
            let t = Ticket {
                title: "Feature".into(),
                description: "Add export".into(),
                status: "Closed".into(),
            };
            let s = format!("{}", t);
            assert!(s.contains("Closed"));
            assert!(s.contains("Feature"));
        }
    }

    mod step_02_trait_bounds {
        use super::*;

        #[test]
        fn test_format_summary() {
            let t = Ticket {
                title: "Bug".into(),
                description: "Fix it".into(),
                status: "Open".into(),
            };
            let s = format_summary(&t);
            assert!(!s.is_empty());
        }

        #[test]
        fn test_format_summary_with_string() {
            let s = format_summary(&"hello");
            assert!(s.contains("hello"));
        }
    }

    mod step_03_derive {
        use super::*;

        #[test]
        fn test_clone() {
            let t = Ticket {
                title: "Bug".into(),
                description: "Fix it".into(),
                status: "Open".into(),
            };
            let c = t.clone();
            assert_eq!(t.title, c.title);
            assert_eq!(t.description, c.description);
            assert_eq!(t.status, c.status);
        }

        #[test]
        fn test_partial_eq_equal() {
            let a = Ticket {
                title: "Bug".into(),
                description: "Fix it".into(),
                status: "Open".into(),
            };
            let b = Ticket {
                title: "Bug".into(),
                description: "Fix it".into(),
                status: "Open".into(),
            };
            assert_eq!(a, b);
        }

        #[test]
        fn test_partial_eq_unequal() {
            let a = Ticket {
                title: "Bug".into(),
                description: "Fix it".into(),
                status: "Open".into(),
            };
            let b = Ticket {
                title: "Feature".into(),
                description: "Add export".into(),
                status: "Open".into(),
            };
            assert_ne!(a, b);
        }
    }

    mod step_04_from_into {
        use super::*;

        #[test]
        fn test_from_str() {
            let t: Ticket = "Quick bug".into();
            assert_eq!(t.title, "Quick bug");
            assert_eq!(t.status, "Open");
            assert_eq!(t.description, "");
        }

        #[test]
        fn test_from_explicit() {
            let t = Ticket::from("Urgent fix");
            assert_eq!(t.title, "Urgent fix");
        }
    }

    mod step_05_drop {
        use super::*;

        #[test]
        fn test_database_connection_drop() {
            let conn = DatabaseConnection::new("postgres://localhost/db");
            assert_eq!(conn.url, "postgres://localhost/db");
        }
    }
}
