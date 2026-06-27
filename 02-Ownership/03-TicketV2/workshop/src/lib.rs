// ============================================================
// 5-TicketV2 — Library
// ============================================================
// Replace each `todo!()` with your implementation as you
// progress through the README tutorial.
// Run `cargo test` to watch your pass count grow.
// ============================================================

#![allow(unused_variables)]

use std::fmt;

// === Status Enum ===
// README §2: Enums — better than booleans
#[derive(Debug, Clone, PartialEq)]
pub enum Status {
    Open,
    InProgress,
    Resolved,
    Closed,
}

impl Status {
    /// Parse a string into a Status variant.
    /// README §4: Match — exhaustive pattern matching
    pub fn from_str(s: &str) -> Result<Status, TicketError> {
        match s {
            "Open" => Ok(Status::Open),
            "In Progress" => Ok(Status::InProgress),
            "Resolved" => Ok(Status::Resolved),
            "Closed" => Ok(Status::Closed),
            other => Err(TicketError::InvalidStatus(other.to_string())),
        }
    }
}

impl fmt::Display for Status {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Status::Open => write!(f, "Open"),
            Status::InProgress => write!(f, "In Progress"),
            Status::Resolved => write!(f, "Resolved"),
            Status::Closed => write!(f, "Closed"),
        }
    }
}

// === Custom Error Types ===
// README §8: Custom error types

#[derive(Debug, Clone, PartialEq)]
pub enum TicketError {
    EmptyTitle,
    TitleTooLong { max: usize, actual: usize },
    EmptyDescription,
    DescriptionTooLong { max: usize, actual: usize },
    InvalidStatus(String),
}

impl fmt::Display for TicketError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TicketError::EmptyTitle => write!(f, "Title cannot be empty"),
            TicketError::TitleTooLong { max, actual } => {
                write!(f, "Title too long: max {} but got {}", max, actual)
            }
            TicketError::EmptyDescription => write!(f, "Description cannot be empty"),
            TicketError::DescriptionTooLong { max, actual } => {
                write!(f, "Description too long: max {} but got {}", max, actual)
            }
            TicketError::InvalidStatus(s) => write!(f, "Invalid status: {}", s),
        }
    }
}

// === Ticket Struct ===
// README §4: Enums with data (status is now Status, not String)
#[derive(Debug, Clone, PartialEq)]
pub struct Ticket {
    title: String,
    description: String,
    status: Status,
}

impl Ticket {
    /// Create a Ticket, returning an error if validation fails.
    /// README §7: Result<T, E> — recoverable errors
    pub fn new(title: String, description: String, status: Status) -> Result<Ticket, TicketError> {
        if title.is_empty() {
            return Err(TicketError::EmptyTitle);
        }
        if title.len() > 50 {
            return Err(TicketError::TitleTooLong {
                max: 50,
                actual: title.len(),
            });
        }
        if description.is_empty() {
            return Err(TicketError::EmptyDescription);
        }
        if description.len() > 500 {
            return Err(TicketError::DescriptionTooLong {
                max: 500,
                actual: description.len(),
            });
        }
        Ok(Ticket {
            title,
            description,
            status,
        })
    }

    /// README §7-9
    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn description(&self) -> &str {
        &self.description
    }

    pub fn status(&self) -> &Status {
        &self.status
    }

    /// README §5: if let — set status when you only care about one variant
    pub fn set_status(&mut self, status: Status) {
        self.status = status;
    }
}

impl fmt::Display for Ticket {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}] {} — {}", self.status, self.title, self.description)
    }
}

// ============================================================
// Tests
// ============================================================

#[cfg(test)]
mod tests {
    use super::*;

    mod step_01_enums {
        use super::*;

        #[test]
        fn test_status_variants() {
            let open = Status::Open;
            assert_eq!(open, Status::Open);
        }

        #[test]
        fn test_status_parse_open() {
            assert_eq!(Status::from_str("Open"), Ok(Status::Open));
        }

        #[test]
        fn test_status_parse_in_progress() {
            assert_eq!(Status::from_str("In Progress"), Ok(Status::InProgress));
        }

        #[test]
        fn test_status_parse_resolved() {
            assert_eq!(Status::from_str("Resolved"), Ok(Status::Resolved));
        }

        #[test]
        fn test_status_parse_closed() {
            assert_eq!(Status::from_str("Closed"), Ok(Status::Closed));
        }

        #[test]
        fn test_status_parse_invalid() {
            assert!(Status::from_str("Bogus").is_err());
        }
    }

    mod step_02_match {
        use super::*;

        #[test]
        fn test_status_display() {
            assert_eq!(format!("{}", Status::Open), "Open");
            assert_eq!(format!("{}", Status::InProgress), "In Progress");
            assert_eq!(format!("{}", Status::Resolved), "Resolved");
            assert_eq!(format!("{}", Status::Closed), "Closed");
        }
    }

    mod step_03_result {
        use super::*;

        #[test]
        fn test_ticket_new_valid() {
            let t = Ticket::new("Bug".into(), "Fix it".into(), Status::Open);
            assert!(t.is_ok());
            let t = t.unwrap();
            assert_eq!(t.title(), "Bug");
            assert_eq!(t.description(), "Fix it");
            assert_eq!(t.status(), &Status::Open);
        }

        #[test]
        fn test_ticket_new_empty_title() {
            let t = Ticket::new("".into(), "desc".into(), Status::Open);
            assert_eq!(t, Err(TicketError::EmptyTitle));
        }

        #[test]
        fn test_ticket_new_title_too_long() {
            let t = Ticket::new("a".repeat(51), "desc".into(), Status::Open);
            assert_eq!(
                t,
                Err(TicketError::TitleTooLong { max: 50, actual: 51 })
            );
        }

        #[test]
        fn test_ticket_new_empty_description() {
            let t = Ticket::new("Bug".into(), "".into(), Status::Open);
            assert_eq!(t, Err(TicketError::EmptyDescription));
        }

        #[test]
        fn test_ticket_new_description_too_long() {
            let t = Ticket::new("Bug".into(), "a".repeat(501), Status::Open);
            assert_eq!(
                t,
                Err(TicketError::DescriptionTooLong {
                    max: 500,
                    actual: 501
                })
            );
        }
    }

    mod step_04_if_let {
        use super::*;

        #[test]
        fn test_set_status() {
            let mut t = Ticket::new("Bug".into(), "desc".into(), Status::Open).unwrap();
            t.set_status(Status::Closed);
            assert_eq!(t.status(), &Status::Closed);
        }
    }

    mod step_05_option {
        use super::*;

        #[test]
        fn test_status_from_str_option() {
            // Option version using Status::from_str returning Result
            let s = Status::from_str("Open");
            assert!(s.is_ok());
        }
    }

    mod step_06_error_display {
        use super::*;

        #[test]
        fn test_error_display() {
            let err = TicketError::EmptyTitle;
            let msg = format!("{}", err);
            assert!(!msg.is_empty());
        }

        #[test]
        fn test_error_display_too_long() {
            let err = TicketError::TitleTooLong { max: 50, actual: 100 };
            let msg = format!("{}", err);
            assert!(msg.contains("50"));
        }
    }
}
