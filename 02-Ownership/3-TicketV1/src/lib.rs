// ============================================================
// 3-TicketV1 — Library
// ============================================================
// Replace each `todo!()` with your implementation as you
// progress through the README tutorial.
// Run `cargo test` to watch your pass count grow.
// ============================================================

#![allow(unused_variables)]

/// Represents a ticket in the tracking system.
/// README §4: Structs
pub struct Ticket {
    title: String,
    description: String,
    status: String,
}

impl Ticket {
    /// Create a new Ticket with validation.
    /// panics if title/description/status are invalid.
    /// README §4: Structs, §6: Validation
    pub fn new(title: String, description: String, status: String) -> Ticket {
        todo!()
    }

    /// README §5: Methods, §11: References
    pub fn title(&self) -> &String {
        todo!()
    }

    /// README §5: Methods, §11: References
    pub fn description(&self) -> &String {
        todo!()
    }

    /// README §5: Methods, §11: References
    pub fn status(&self) -> &String {
        todo!()
    }

    /// README §7: Setters
    pub fn set_title(&mut self, title: String) {
        todo!()
    }

    /// README §7: Setters
    pub fn set_description(&mut self, description: String) {
        todo!()
    }

    /// README §7: Setters
    pub fn set_status(&mut self, status: String) {
        todo!()
    }
}

// ============================================================
// Tests — grouped by README section
// ============================================================

#[cfg(test)]
mod tests {
    use super::*;

    mod step_01_structs {
        use super::*;

        #[test]
        fn test_new_ticket() {
            let t = Ticket::new(
                "Bug".into(),
                "Fix it".into(),
                "Open".into(),
            );
            assert_eq!(t.title(), "Bug");
            assert_eq!(t.description(), "Fix it");
            assert_eq!(t.status(), "Open");
        }

        #[test]
        fn test_new_ticket_in_progress() {
            let t = Ticket::new(
                "Feature".into(),
                "Add export".into(),
                "In Progress".into(),
            );
            assert_eq!(t.status(), "In Progress");
        }
    }

    mod step_02_validation {
        use super::*;

        #[test]
        #[should_panic(expected = "Title cannot be empty")]
        fn test_empty_title_panics() {
            Ticket::new("".into(), "desc".into(), "Open".into());
        }

        #[test]
        #[should_panic(expected = "Title too long")]
        fn test_title_too_long_panics() {
            Ticket::new("a".repeat(51), "desc".into(), "Open".into());
        }

        #[test]
        #[should_panic(expected = "Title cannot contain newlines")]
        fn test_title_with_newline_panics() {
            Ticket::new("bug\nfix".into(), "desc".into(), "Open".into());
        }

        #[test]
        #[should_panic(expected = "Description cannot be empty")]
        fn test_empty_description_panics() {
            Ticket::new("Bug".into(), "".into(), "Open".into());
        }

        #[test]
        #[should_panic(expected = "Description too long")]
        fn test_description_too_long_panics() {
            Ticket::new("Bug".into(), "a".repeat(501), "Open".into());
        }

        #[test]
        #[should_panic(expected = "Invalid status")]
        fn test_invalid_status_panics() {
            Ticket::new("Bug".into(), "desc".into(), "Bogus".into());
        }
    }

    mod step_03_setters {
        use super::*;

        #[test]
        fn test_set_title() {
            let mut t = Ticket::new("Bug".into(), "desc".into(), "Open".into());
            t.set_title("New title".into());
            assert_eq!(t.title(), "New title");
        }

        #[test]
        fn test_set_description() {
            let mut t = Ticket::new("Bug".into(), "desc".into(), "Open".into());
            t.set_description("new desc".into());
            assert_eq!(t.description(), "new desc");
        }

        #[test]
        fn test_set_status() {
            let mut t = Ticket::new("Bug".into(), "desc".into(), "Open".into());
            t.set_status("Closed".into());
            assert_eq!(t.status(), "Closed");
        }

        #[test]
        #[should_panic(expected = "Title cannot be empty")]
        fn test_set_empty_title_panics() {
            let mut t = Ticket::new("Bug".into(), "desc".into(), "Open".into());
            t.set_title("".into());
        }

        #[test]
        #[should_panic(expected = "Invalid status")]
        fn test_set_invalid_status_panics() {
            let mut t = Ticket::new("Bug".into(), "desc".into(), "Open".into());
            t.set_status("Invalid".into());
        }
    }

    mod step_04_ownership {
        use super::*;

        #[test]
        fn test_borrow_does_not_move() {
            let t = Ticket::new("Bug".into(), "desc".into(), "Open".into());
            let title = t.title();
            let status = t.status();
            assert_eq!(title, "Bug");
            assert_eq!(status, "Open");
        }

        #[test]
        fn test_mut_borrow_then_imm_borrow() {
            let mut t = Ticket::new("Bug".into(), "desc".into(), "Open".into());
            t.set_status("Closed".into());
            assert_eq!(t.status(), "Closed");
        }
    }
}
