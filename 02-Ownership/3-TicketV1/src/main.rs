use ticket_v1::Ticket;

fn main() {
    let mut ticket = Ticket::new(
        "Fix login bug".into(),
        "Users cannot log in with SSO".into(),
        "Open".into(),
    );
    println!("Created: [{}] {}", ticket.status(), ticket.title());

    ticket.set_status("In Progress".into());
    println!("Updated: [{}] {}", ticket.status(), ticket.title());
}
