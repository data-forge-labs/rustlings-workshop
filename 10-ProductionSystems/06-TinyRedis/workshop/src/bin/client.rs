use std::io::{self, BufRead, Write};
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::TcpStream;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "127.0.0.1:6379".to_string());

    let stream = TcpStream::connect(&addr).await?;
    println!("Connected to Tiny Redis at {}", addr);
    println!("Commands: PING  SET key value [EX secs]  GET key");
    println!("          DEL key  EXISTS key  EXPIRE key secs");
    println!("          TTL key  DBSIZE  QUIT");
    println!();

    let (reader, mut writer) = stream.into_split();
    let mut server_reader = BufReader::new(reader);

    let mut welcome = String::new();
    server_reader.read_line(&mut welcome).await?;
    println!("Server: {}", welcome.trim());

    let stdin = io::stdin();
    loop {
        print!("> ");
        io::stdout().flush()?;

        let mut input = String::new();
        match stdin.lock().read_line(&mut input) {
            Ok(0) => break,
            Err(e) => {
                eprintln!("Input error: {}", e);
                break;
            }
            Ok(_) => {}
        }

        let trimmed = input.trim();
        if trimmed.is_empty() {
            continue;
        }

        writer
            .write_all(format!("{}\n", trimmed).as_bytes())
            .await?;

        let mut response = String::new();
        server_reader.read_line(&mut response).await?;
        let resp = response.trim();

        if resp.starts_with('+') {
            println!("{}", &resp[1..]);
        } else if resp.starts_with(':') {
            println!("(integer) {}", &resp[1..]);
        } else if resp == "$-1" {
            println!("(nil)");
        } else if resp.starts_with('-') {
            println!("Error: {}", &resp[1..]);
        } else {
            println!("{}", resp);
        }

        if trimmed.to_uppercase() == "QUIT" {
            break;
        }
    }

    println!("Goodbye.");
    Ok(())
}
