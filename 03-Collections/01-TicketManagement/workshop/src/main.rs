use ticket_management::{Ticket, index_by_status, open_ticket_titles, ticket_summaries};

fn main() {
    let tickets = vec![
        Ticket::new(1, "Login bug".into(), "Cannot log in".into(), "Open".into()),
        Ticket::new(2, "Slow query".into(), "Query takes 10s".into(), "Open".into()),
        Ticket::new(3, "Add export".into(), "CSV export".into(), "In Progress".into()),
        Ticket::new(4, "Fix typo".into(), "Typo in docs".into(), "Closed".into()),
        Ticket::new(5, "Security patch".into(), "Update deps".into(), "Open".into()),
    ];

    println!("=== Open ===");
    for title in open_ticket_titles(&tickets) {
        println!("  {title}");
    }

    println!("\n=== All ===");
    for s in ticket_summaries(&tickets) {
        println!("  {s}");
    }

    println!("\n=== By Status ===");
    let by_status = index_by_status(&tickets);
    for (status, group) in &by_status {
        println!("  {status} ({}):", group.len());
    }
}
