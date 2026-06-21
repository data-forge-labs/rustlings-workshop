use std::sync::Arc;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::TcpListener;
use tiny_redis::{command, expiry, persistence, storage};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let store = storage::new_store();
    let snapshot_path = "tiny_redis.snapshot";

    persistence::load_from_disk(snapshot_path, &store).await;
    expiry::start_cleanup(Arc::clone(&store));
    persistence::start_persistence(Arc::clone(&store), snapshot_path);

    let listener = TcpListener::bind("127.0.0.1:6379").await?;
    println!("Tiny Redis listening on 127.0.0.1:6379");
    println!("Use the client: cargo run --bin client");

    loop {
        let (socket, addr) = listener.accept().await?;
        println!("[+] Connection from {}", addr);
        let store = Arc::clone(&store);

        tokio::spawn(async move {
            if let Err(e) = handle_connection(socket, store).await {
                eprintln!("[-] Connection error from {}: {}", addr, e);
            }
            println!("[-] Disconnected: {}", addr);
        });
    }
}

async fn handle_connection(
    socket: tokio::net::TcpStream,
    store: storage::Store,
) -> Result<(), Box<dyn std::error::Error>> {
    let (reader, mut writer) = socket.into_split();
    let mut reader = BufReader::new(reader);
    let mut line = String::new();

    writer
        .write_all(b"+Welcome to Tiny Redis. Type QUIT to disconnect.\n")
        .await?;

    loop {
        line.clear();
        let bytes_read = reader.read_line(&mut line).await?;
        if bytes_read == 0 {
            break;
        }

        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }

        let response = match command::Command::parse(trimmed) {
            Ok(cmd) => {
                let should_quit = cmd == command::Command::Quit;
                let resp = command::execute(cmd, &store).await;
                if should_quit {
                    writer.write_all(resp.as_bytes()).await?;
                    break;
                }
                resp
            }
            Err(e) => format!("-Error: {}\n", e),
        };

        writer.write_all(response.as_bytes()).await?;
    }

    Ok(())
}
