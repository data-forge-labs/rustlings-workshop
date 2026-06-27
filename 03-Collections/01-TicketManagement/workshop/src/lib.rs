// ============================================================
// 01-TicketManagement — Library
// ============================================================
// Replace each `todo!()` with your implementation as you
// progress through the README tutorial.
// Run `cargo test` to watch your pass count grow.
// ============================================================

#![allow(unused_variables)]

use std::collections::HashMap;

/// A ticket with id, title, description, and status.
/// README §2-3: Arrays, Vec
#[derive(Debug, Clone)]
pub struct Ticket {
    id: u32,
    title: String,
    description: String,
    status: String,
}

impl Ticket {
    /// README §3: Vec — storing tickets
    pub fn new(id: u32, title: String, description: String, status: String) -> Ticket {
        Ticket { id, title, description, status }
    }

    pub fn id(&self) -> u32 {
        self.id
    }

    pub fn title(&self) -> &str {
        &self.title
    }

    /// README §4: Slices — views into data
    pub fn status(&self) -> &str {
        &self.status
    }

    pub fn is_open(&self) -> bool {
        self.status == "Open"
    }

    pub fn is_closed(&self) -> bool {
        self.status == "Closed"
    }
}

/// Index tickets by status using HashMap.
/// README §7: HashMap — key-value store
pub fn index_by_status(tickets: &[Ticket]) -> HashMap<&str, Vec<&Ticket>> {
    let mut map: HashMap<&str, Vec<&Ticket>> = HashMap::new();
    for ticket in tickets {
        map.entry(ticket.status()).or_default().push(ticket);
    }
    map
}

/// Find the most common status using iterators.
/// README §5-6: Iterators, combinators
pub fn most_common_status(tickets: &[Ticket]) -> Option<(&str, usize)> {
    let counts = count_by_status(tickets);
    counts.into_iter().max_by_key(|&(_, count)| count)
}

/// Return titles of all open tickets using filter + map.
/// README §6: Iterator combinators
pub fn open_ticket_titles(tickets: &[Ticket]) -> Vec<&str> {
    tickets.iter().filter(|t| t.is_open()).map(|t| t.title()).collect()
}

/// Format each ticket as "N: [Status] Title" using map.
/// README §6: map combinator
pub fn ticket_summaries(tickets: &[Ticket]) -> Vec<String> {
    tickets.iter().map(|t| format!("{}: [{}] {}", t.id(), t.status(), t.title())).collect()
}

/// Count tickets per status using fold.
/// README §6: fold combinator
pub fn count_by_status(tickets: &[Ticket]) -> HashMap<&str, usize> {
    tickets.iter().fold(HashMap::new(), |mut acc, t| {
        *acc.entry(t.status()).or_insert(0) += 1;
        acc
    })
}

// ============================================================
// Tests
// ============================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_tickets() -> Vec<Ticket> {
        vec![
            Ticket::new(1, "Login bug".into(), "Cannot log in".into(), "Open".into()),
            Ticket::new(2, "Slow query".into(), "Query takes 10s".into(), "Open".into()),
            Ticket::new(3, "Add export".into(), "CSV export".into(), "In Progress".into()),
            Ticket::new(4, "Fix typo".into(), "Typo in docs".into(), "Closed".into()),
            Ticket::new(5, "Security patch".into(), "Update deps".into(), "Open".into()),
        ]
    }

    mod step_01_arrays_vec {
        use super::*;

        #[test]
        fn test_ticket_new() {
            let t = Ticket::new(1, "Bug".into(), "Fix it".into(), "Open".into());
            assert_eq!(t.id(), 1);
            assert_eq!(t.title(), "Bug");
        }

        #[test]
        fn test_is_open() {
            let t = Ticket::new(1, "Bug".into(), "Fix it".into(), "Open".into());
            assert!(t.is_open());
            assert!(!t.is_closed());
        }

        #[test]
        fn test_is_closed() {
            let t = Ticket::new(1, "Bug".into(), "Fix it".into(), "Closed".into());
            assert!(t.is_closed());
            assert!(!t.is_open());
        }

        #[test]
        fn test_vec_len() {
            let tickets = sample_tickets();
            assert_eq!(tickets.len(), 5);
        }
    }

    mod step_02_iterators {
        use super::*;

        #[test]
        fn test_open_ticket_titles() {
            let tickets = sample_tickets();
            let open = open_ticket_titles(&tickets);
            assert_eq!(open.len(), 3);
            assert!(open.contains(&"Login bug"));
            assert!(open.contains(&"Slow query"));
            assert!(open.contains(&"Security patch"));
        }

        #[test]
        fn test_ticket_summaries() {
            let tickets = sample_tickets();
            let summaries = ticket_summaries(&tickets);
            assert_eq!(summaries.len(), 5);
            assert!(summaries[0].contains("1"));
            assert!(summaries[0].contains("Login bug"));
            assert!(summaries[0].contains("Open"));
        }
    }

    mod step_03_hashmap {
        use super::*;

        #[test]
        fn test_index_by_status() {
            let tickets = sample_tickets();
            let idx = index_by_status(&tickets);
            assert_eq!(idx.get("Open").unwrap().len(), 3);
            assert_eq!(idx.get("In Progress").unwrap().len(), 1);
            assert_eq!(idx.get("Closed").unwrap().len(), 1);
        }

        #[test]
        fn test_most_common_status() {
            let tickets = sample_tickets();
            let result = most_common_status(&tickets);
            assert_eq!(result, Some(("Open", 3)));
        }

        #[test]
        fn test_count_by_status() {
            let tickets = sample_tickets();
            let counts = count_by_status(&tickets);
            assert_eq!(*counts.get("Open").unwrap(), 3);
            assert_eq!(*counts.get("In Progress").unwrap(), 1);
            assert_eq!(*counts.get("Closed").unwrap(), 1);
        }
    }

    mod step_04_edge_cases {
        use super::*;

        #[test]
        fn test_empty_tickets() {
            let tickets: Vec<Ticket> = vec![];
            let open = open_ticket_titles(&tickets);
            assert!(open.is_empty());
        }

        #[test]
        fn test_most_common_empty() {
            let tickets: Vec<Ticket> = vec![];
            assert_eq!(most_common_status(&tickets), None);
        }

        #[test]
        fn test_index_by_status_empty() {
            let tickets: Vec<Ticket> = vec![];
            let idx = index_by_status(&tickets);
            assert!(idx.is_empty());
        }
    }
}
