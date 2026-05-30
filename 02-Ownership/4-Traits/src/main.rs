use traits::{Ticket, DatabaseConnection, format_summary};

fn main() {
    let ticket = Ticket {
        title: "Fix login bug".into(),
        description: "SSO login broken".into(),
        status: "Open".into(),
    };

    println!("{}", ticket);
    println!("{:?}", ticket);
    println!("{}", format_summary(&ticket));

    let quick: Ticket = "Quick bug".into();
    println!("{}", quick);

    let _conn = DatabaseConnection::new("postgres://localhost/db");
}
