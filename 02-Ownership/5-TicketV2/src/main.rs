use ticket_v2::{Status, Ticket};

fn main() -> Result<(), ticket_v2::TicketError> {
    let ticket = Ticket::new(
        "Fix login bug".into(),
        "SSO login broken".into(),
        Status::Open,
    )?;

    println!("{}", ticket);
    println!("Status: {}", ticket.status());
    Ok(())
}
