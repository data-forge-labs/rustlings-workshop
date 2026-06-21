# TinyRedis — Build a Redis Clone in Rust

> **Test-driven approach**: This project includes a Cargo project with progressive unit tests. Each function in `src/lib.rs` starts as a `todo!()` stub. As you follow each section, replace `todo!()` with real code and run `cargo test` to watch the pass count grow. Your goal: **all 27 tests pass**.

---

## What Is TinyRedis?

A concurrent, persistent key-value store with a TCP interface — a tiny Redis built from scratch.

### Python equivalent

```python
import redis

r = redis.Redis()
r.set("key", "value", ex=3600)
print(r.get("key"))           # b'value'
print(r.ttl("key"))           # 3599
r.delete("key")
# All hidden behind a library — the server is a black box
```

In this project you'll learn to build the **server itself** in Rust — and along the way you'll discover **multi-threaded async TCP**, **`Arc<Mutex<>>` shared state**, **`thiserror` custom errors**, **background tasks**, and **disk persistence**.

### Topics covered

| # | Concept | Why it matters |
|---|---------|----------------|
| 1 | `Arc<Mutex<HashMap>>` | Thread-safe shared state across async tasks |
| 2 | Tokio multi-threaded TCP | Accept concurrent client connections |
| 3 | `thiserror` derive macro | Structured custom error types with zero boilerplate |
| 4 | Typed command enum | Model a protocol with Rust enums and match guards |
| 5 | `tokio::spawn` + `time::interval` | Background tasks for periodic cleanup |
| 6 | `serde` JSON snapshots | Persist and restore in-memory state to disk |
| 7 | CLI client | Build a Rust TCP client with formatted output |
| 8 | Unit + integration tests | Test-driven development before wiring TCP |

---

## Table of Contents
1. [Introduction](#1-introduction)
2. [Prerequisites](#2-prerequisites)
3. [Architecture Overview](#3-architecture-overview)
4. [Step 1: Error Handling First](#4-step-1-error-handling-first)
5. [Step 2: The Storage Engine](#5-step-2-the-storage-engine)
6. [Step 3: The Command Parser](#6-step-3-the-command-parser)
7. [Step 4: Testing Before Wiring](#7-step-4-testing-before-wiring)
8. [Step 5: Background Key Expiry](#8-step-5-background-key-expiry)
9. [Step 6: Executing Commands](#9-step-6-executing-commands)
10. [Step 7: The TCP Server](#10-step-7-the-tcp-server)
11. [Step 8: Disk Persistence](#11-step-8-disk-persistence)
12. [Step 9: The CLI Client](#12-step-9-the-cli-client)
13. [Step 10: Integration Tests](#13-step-10-integration-tests)
14. [Running the Complete System](#14-running-the-complete-system)
15. [Exercises](#15-exercises)
16. [Summary](#16-summary)

## 1. Introduction

Redis is one of the most battle-tested pieces of software in the world. It is a TCP server that accepts commands like `SET`, `GET`, `DEL`, and `EXPIRE` from multiple clients simultaneously, keeps data in memory, optionally expires keys after a timeout, and persists data to disk.

The production Redis is written in C and is extraordinarily fast. Ours will be written in Rust and will be correct, safe, and concurrent by construction.

By the end you will have a running server that:

- Accepts TCP connections from multiple clients at the same time
- Supports `SET`, `GET`, `DEL`, `EXISTS`, `EXPIRE`, `TTL`, `DBSIZE`, and `PING`
- Expires keys in the background automatically
- Saves a snapshot to disk every 30 seconds and loads it on startup
- Ships with a CLI client you can type commands into

**Python → Rust**: In Python, you would use `redis-py` as a client and run the real Redis server as a binary. In Rust, we build the server itself — handling raw TCP streams, parsing commands, managing shared state, and persisting to disk. This is the kind of systems-level control where Rust shines: zero-cost abstractions for concurrency, memory safety without a garbage collector, and compile-time guarantees about thread safety.

## 2. Prerequisites

Before starting, you should have completed or be familiar with:

- **[Project 34: DataRace](../../05-Concurrency/03-DataRace/README.md)** — `Arc`, `Mutex`, shared-state concurrency
- **[Project 35: Futures](../../05-Concurrency/02-Futures/README.md)** — `async`/`.await`, Tokio runtime
- **[Project 3: TicketV2](../../02-Ownership/03-TicketV2/README.md)** — Enums, `Result`, `thiserror`
- **[Project 37: ConversionErrorHandling](../../02-Ownership/06-ConversionErrorHandling/README.md)** — `?` operator, `From` trait
- Familiarity with TCP/IP networking basics

## 3. Architecture Overview

```
┌──────────────────────────────────────────────────┐
│               Tokio Runtime                      │
│            (multi-threaded)                      │
│                                                  │
│  ┌──────────────────────────────────────────┐    │
│  │          TcpListener :6379               │    │
│  │  ┌─────────┐  ┌─────────┐  ┌──────┐     │    │
│  │  │ Client1 │  │ Client2 │  │ ...  │     │    │
│  │  └────┬────┘  └────┬────┘  └──┬───┘     │    │
│  │       └─────────────┼─────────┘          │    │
│  │                     ▼                    │    │
│  │        Arc<Mutex<HashMap<String,Entry>>> │    │
│  │         (thread-safe shared state)       │    │
│  └──────────────────────────────────────────┘    │
│                                                  │
│  ┌─────────────────┐  ┌──────────────────────┐   │
│  │ expiry cleanup  │  │ persistence snapshot │   │
│  │ (1s interval)   │  │ (30s interval)       │   │
│  └─────────────────┘  └──────────────────────┘   │
└──────────────────────────────────────────────────┘
```

### Module Breakdown

| Module | File | Responsibility |
|--------|------|----------------|
| `error.rs` | Error types | `thiserror`-derived `RedisError` enum |
| `storage.rs` | Data engine | `Arc<Mutex<HashMap>>` with `Entry` and TTL support |
| `command.rs` | Protocol layer | Typed `Command` enum, parser, and executor |
| `expiry.rs` | Background task | Periodic cleanup of expired keys |
| `persistence.rs` | Disk I/O | JSON snapshot save/load with `serde` |
| `main.rs` | TCP server | Accept connections, dispatch commands |
| `bin/client.rs` | CLI client | Connect, read user input, format responses |

### Design Decision: `Arc<Mutex<>>` vs `Rc<RefCell<>>

| Approach | Threading | Locking | When to use |
|----------|-----------|---------|-------------|
| `Rc<RefCell<>>` | Single-threaded | Zero-cost runtime checks | When all tasks run on one OS thread |
| `Arc<Mutex<>>` | Multi-threaded | Mutex lock/unlock | When tasks may run on different threads |

TinyRedis uses `Arc<Mutex<>>` because Tokio's default runtime is multi-threaded with work-stealing. This means any task can run on any thread, so shared state must be thread-safe. Radish (Project 59) takes the opposite approach with `Rc<RefCell<>>` on a single-threaded runtime — zero locking overhead, but all tasks must stay on one thread.

**Python → Rust**: In Python, the GIL protects all shared mutable state automatically. In Rust, you must choose explicitly: `Rc<RefCell<>>` (single-threaded, zero overhead) or `Arc<Mutex<>>` (multi-threaded, lock overhead). The compiler enforces this choice — you cannot accidentally share `Rc` across threads.

## 4. Step 1: Error Handling First

Do not start writing logic until you know what can go wrong. This is the lesson from every production system you will ever build.

Create `src/error.rs`:

```rust
use thiserror::Error;

#[derive(Error, Debug)]
pub enum RedisError {
    #[error("unknown command: '{0}'")]
    UnknownCommand(String),

    #[error("wrong number of arguments for '{0}'")]
    WrongArgCount(String),

    #[error("invalid argument: {0}")]
    InvalidArgument(String),

    #[error("empty command")]
    EmptyCommand,

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
}
```

### What `thiserror` Does

`thiserror` generates the `Display` and `Error` implementations automatically from the `#[error("...")]` attributes. The `#[from]` on the last two variants means `RedisError` implements `From<std::io::Error>` and `From<serde_json::Error>`, so the `?` operator converts those errors automatically.

**Python equivalent:**

```python
class RedisError(Exception):
    pass

class UnknownCommand(RedisError):
    def __init__(self, cmd):
        super().__init__(f"unknown command: '{cmd}'")

class EmptyCommand(RedisError):
    pass
```

In Python you subclass `Exception` for each error type. In Rust, `thiserror` gives you structured, typed errors with zero boilerplate — a single enum with variants, not a class hierarchy.

## 5. Step 2: The Storage Engine

This is the heart of the server. Everything else is built around it.

Create `src/storage.rs`:

```rust
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

#[derive(Debug, Clone)]
pub struct Entry {
    pub value: String,
    pub expires_at: Option<Instant>,
}

impl Entry {
    pub fn new(value: String) -> Self {
        Entry { value, expires_at: None }
    }

    pub fn with_expiry(value: String, ttl: Duration) -> Self {
        Entry { value, expires_at: Some(Instant::now() + ttl) }
    }

    pub fn is_expired(&self) -> bool {
        self.expires_at.map_or(false, |exp| Instant::now() > exp)
    }
}

pub type Store = Arc<Mutex<HashMap<String, Entry>>>;

pub fn new_store() -> Store {
    Arc::new(Mutex::new(HashMap::new()))
}
```

### Design Decisions

| Decision | Why |
|----------|-----|
| `Option<Instant>` for expiry | Most keys never expire. `None` means "no expiry" — cleaner than a sentinel value like `-1`. |
| `map_or(false, ...)` | One line instead of `if/else` chains. `None` → `false` (never expired), `Some` → compare. |
| `Arc<Mutex<HashMap>>` | `Arc` for shared ownership across threads, `Mutex` for exclusive access when mutating. |
| `type Store = ...` | Type alias hides the nested generic. Callers write `Store` instead of `Arc<Mutex<HashMap<String, Entry>>>`. |

**Python equivalent:**

```python
from dataclasses import dataclass
from datetime import datetime, timedelta
from typing import Optional
import threading

@dataclass
class Entry:
    value: str
    expires_at: Optional[datetime] = None

    @classmethod
    def with_expiry(cls, value: str, ttl: timedelta):
        return cls(value=value, expires_at=datetime.utcnow() + ttl)

    def is_expired(self) -> bool:
        if self.expires_at is None:
            return False
        return datetime.utcnow() > self.expires_at

store: dict[str, Entry] = {}
lock = threading.Lock()
```

In Python, you use a plain `dict` and `threading.Lock`. In Rust, `Arc<Mutex<HashMap>>` makes the thread-safety contract explicit in the type signature — the compiler verifies that every access is safe.

## 6. Step 3: The Command Parser

Rust's enum system is perfect for modeling a protocol. Each command variant carries exactly the data it needs, nothing more.

Create `src/command.rs` (add the `Command` enum and `parse` method):

```rust
use crate::error::RedisError;
use std::time::Duration;

#[derive(Debug, PartialEq)]
pub enum Command {
    Ping,
    Set { key: String, value: String, ttl: Option<Duration> },
    Get { key: String },
    Del { key: String },
    Exists { key: String },
    Expire { key: String, seconds: u64 },
    Ttl { key: String },
    DbSize,
    Quit,
}

impl Command {
    pub fn parse(input: &str) -> Result<Command, RedisError> {
        let parts: Vec<&str> = input.trim().split_whitespace().collect();
        if parts.is_empty() {
            return Err(RedisError::EmptyCommand);
        }
        match parts[0].to_uppercase().as_str() {
            "PING" => Ok(Command::Ping),
            "DBSIZE" => Ok(Command::DbSize),
            "QUIT" => Ok(Command::Quit),
            "GET" if parts.len() == 2 => Ok(Command::Get {
                key: parts[1].to_string(),
            }),
            "DEL" if parts.len() == 2 => Ok(Command::Del {
                key: parts[1].to_string(),
            }),
            "EXISTS" if parts.len() == 2 => Ok(Command::Exists {
                key: parts[1].to_string(),
            }),
            "TTL" if parts.len() == 2 => Ok(Command::Ttl {
                key: parts[1].to_string(),
            }),
            "SET" if parts.len() == 3 => Ok(Command::Set {
                key: parts[1].to_string(),
                value: parts[2].to_string(),
                ttl: None,
            }),
            "SET" if parts.len() == 5 && parts[3].to_uppercase() == "EX" => {
                let secs = parts[4].parse::<u64>().map_err(|_| {
                    RedisError::InvalidArgument("EX requires a positive integer".into())
                })?;
                Ok(Command::Set {
                    key: parts[1].to_string(),
                    value: parts[2].to_string(),
                    ttl: Some(Duration::from_secs(secs)),
                })
            }
            "EXPIRE" if parts.len() == 3 => {
                let secs = parts[2].parse::<u64>().map_err(|_| {
                    RedisError::InvalidArgument("EXPIRE requires a positive integer".into())
                })?;
                Ok(Command::Expire {
                    key: parts[1].to_string(),
                    seconds: secs,
                })
            }
            cmd => Err(RedisError::UnknownCommand(cmd.to_string())),
        }
    }
}
```

### Match Guards

The parsing logic uses `match` with guards. The guard `if parts.len() == 2` after the pattern means the arm only matches when the condition is also true:

```
"GET" if parts.len() == 2  →  only matches "GET key" (2 parts)
"GET"                       →  would match "GET" alone too (but we don't have this arm)
```

If you send `GET` with no key, no arm matches and the catch-all returns `UnknownCommand`. This is cleaner than nesting `if/else` inside match arms.

**Python equivalent:**

```python
def parse(input_str: str):
    parts = input_str.strip().split()
    if not parts:
        raise EmptyCommand()

    cmd = parts[0].upper()
    if cmd == "PING":
        return Ping()
    elif cmd == "GET" and len(parts) == 2:
        return Get(key=parts[1])
    elif cmd == "SET" and len(parts) == 3:
        return Set(key=parts[1], value=parts[2])
    # ...
    else:
        raise UnknownCommand(cmd)
```

Rust's `match` is exhaustive — the compiler ensures every variant is handled. Python's `if/elif` chain is not checked at compile time.

### Note on the Protocol

Values in our protocol cannot contain spaces. `SET name "John Doe"` would not work. This is a deliberate simplification. Real Redis uses a binary-safe protocol called RESP that handles arbitrary bytes in values. Radish (Project 59) implements the full RESP protocol.

## 7. Step 4: Testing Before Wiring

Before moving on to the TCP server, write tests for the command parser. This is the test-driven approach — verify the foundation is solid before building on it.

Add a test module to `src/command.rs`:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_ping() {
        assert_eq!(Command::parse("PING").unwrap(), Command::Ping);
        assert_eq!(Command::parse("ping").unwrap(), Command::Ping);
        assert_eq!(Command::parse("  PING  ").unwrap(), Command::Ping);
    }

    #[test]
    fn parse_set() {
        let cmd = Command::parse("SET mykey myvalue").unwrap();
        assert_eq!(cmd, Command::Set {
            key: "mykey".to_string(),
            value: "myvalue".to_string(),
            ttl: None,
        });
    }

    #[test]
    fn parse_set_with_expiry() {
        let cmd = Command::parse("SET session abc123 EX 3600").unwrap();
        assert_eq!(cmd, Command::Set {
            key: "session".to_string(),
            value: "abc123".to_string(),
            ttl: Some(Duration::from_secs(3600)),
        });
    }

    #[test]
    fn empty_command_returns_error() {
        assert!(Command::parse("").is_err());
        assert!(Command::parse("   ").is_err());
    }

    #[test]
    fn unknown_command_returns_error() {
        assert!(Command::parse("HGET key field").is_err());
    }

    #[test]
    fn wrong_arg_count_returns_error() {
        assert!(Command::parse("GET").is_err());
        assert!(Command::parse("SET key").is_err());
        assert!(Command::parse("SET key value EX notanumber").is_err());
    }
}
```

Run `cargo test` — all 13 parser tests should pass. If they do not pass before you write the server, you know the foundation is broken.

## 8. Step 5: Background Key Expiry

A background task runs every second and removes keys whose expiry time has passed. This is called **lazy expiry with periodic cleanup** — exactly how Redis does it.

Create `src/expiry.rs`:

```rust
use crate::storage::Store;
use std::time::{Duration, Instant};
use tokio::time;

pub fn start_cleanup(store: Store) {
    tokio::spawn(async move {
        let mut interval = time::interval(Duration::from_secs(1));
        loop {
            interval.tick().await;
            let mut store = store.lock().unwrap();
            let now = Instant::now();
            store.retain(|_, entry| entry.expires_at.map_or(true, |exp| now < exp));
        }
    });
}
```

### How It Works

| Step | Code | Description |
|------|------|-------------|
| 1 | `tokio::spawn(async move { ... })` | Creates a background async task |
| 2 | `time::interval(Duration::from_secs(1))` | Wakes up every second |
| 3 | `store.lock().unwrap()` | Acquires the mutex lock |
| 4 | `store.retain(...)` | Keeps only entries where the closure returns `true` |

`retain` is an iterator method on `HashMap` that keeps only the entries for which the closure returns `true`. We return `true` when there is no expiry (keep it forever) or when the expiry is in the future (not yet expired). Everything else is dropped.

**Python equivalent:**

```python
import asyncio
import threading

async def cleanup_loop(store, lock):
    while True:
        await asyncio.sleep(1)
        with lock:
            now = datetime.utcnow()
            expired = [k for k, v in store.items() if v.is_expired()]
            for k in expired:
                del store[k]
```

The difference: Rust shares the store across the background task and all connection handlers through `Arc<Mutex<T>>` and the compiler verifies that sharing is safe. In Python, you pass the same `dict` and `lock` objects manually.

## 9. Step 6: Executing Commands

Now write the function that takes a parsed `Command` and produces a response string.

Add to `src/command.rs`:

```rust
use crate::storage::Store;
use std::time::Instant;

pub async fn execute(cmd: Command, store: &Store) -> String {
    match cmd {
        Command::Ping => "+PONG\n".to_string(),
        Command::Set { key, value, ttl } => {
            let entry = match ttl {
                Some(duration) => crate::storage::Entry::with_expiry(value, duration),
                None => crate::storage::Entry::new(value),
            };
            store.lock().unwrap().insert(key, entry);
            "+OK\n".to_string()
        }
        Command::Get { key } => {
            let mut store = store.lock().unwrap();
            match store.get(&key) {
                Some(entry) if entry.is_expired() => {
                    store.remove(&key);
                    "$-1\n".to_string()
                }
                Some(entry) => format!("+{}\n", entry.value),
                None => "$-1\n".to_string(),
            }
        }
        Command::Del { key } => {
            let removed = store.lock().unwrap().remove(&key).is_some();
            if removed { ":1\n".to_string() } else { ":0\n".to_string() }
        }
        Command::Exists { key } => {
            let store = store.lock().unwrap();
            let exists = store.get(&key).map_or(false, |e| !e.is_expired());
            if exists { ":1\n".to_string() } else { ":0\n".to_string() }
        }
        Command::Expire { key, seconds } => {
            let mut store = store.lock().unwrap();
            if let Some(entry) = store.get_mut(&key) {
                entry.expires_at = Some(Instant::now() + std::time::Duration::from_secs(seconds));
                ":1\n".to_string()
            } else {
                ":0\n".to_string()
            }
        }
        Command::Ttl { key } => {
            let store = store.lock().unwrap();
            match store.get(&key) {
                None => ":-2\n".to_string(),
                Some(entry) => match entry.expires_at {
                    None => ":-1\n".to_string(),
                    Some(exp) => {
                        let now = Instant::now();
                        if now > exp { ":-2\n".to_string() }
                        else { format!(":{}\n", (exp - now).as_secs()) }
                    }
                },
            }
        }
        Command::DbSize => {
            let count = store.lock().unwrap().len();
            format!(":{}\n", count)
        }
        Command::Quit => "+OK\n".to_string(),
    }
}
```

### Lazy Expiry on GET

Notice the `Get` arm:

```rust
Some(entry) if entry.is_expired() => {
    store.remove(&key);
    "$-1\n".to_string()
}
```

Even if the background task has not run yet, a `GET` on an expired key will remove it immediately and return nil. The guard pattern in `match` (`if entry.is_expired()`) is something you only get in Rust — no other mainstream language has this built into pattern matching.

### Response Format

| Prefix | Meaning | Example |
|--------|---------|---------|
| `+` | Simple string | `+OK\n`, `+PONG\n`, `+value\n` |
| `:` | Integer | `:1\n`, `:0\n`, `:3\n` |
| `$-1` | Nil (key missing/expired) | `$-1\n` |
| `-` | Error | `-Error: unknown command\n` |

## 10. Step 7: The TCP Server

The main event. Create `src/main.rs`:

```rust
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

    writer.write_all(b"+Welcome to Tiny Redis. Type QUIT to disconnect.\n").await?;

    loop {
        line.clear();
        let bytes_read = reader.read_line(&mut line).await?;
        if bytes_read == 0 { break; }

        let trimmed = line.trim();
        if trimmed.is_empty() { continue; }

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
```

### What Happens With Multiple Clients

Trace through what happens when two clients connect:

1. **Client A** connects. `accept()` returns, a task is spawned for A, and the loop immediately goes back to `accept()`.
2. **Client B** connects. Another task is spawned for B.
3. **Client A** sends `SET counter 0`. The task for A locks the store, inserts the entry, releases the lock, and sends `+OK`.
4. **Client B** sends `GET counter` at the same time. The task for B tries to lock the store. If A's task holds it, B waits. If A has released it, B gets it immediately.

The `Mutex` ensures only one client modifies the store at any moment. The `Arc` ensures both clients see the same store. The Tokio runtime ensures neither task blocks a thread while waiting for I/O.

**Python equivalent:**

```python
async def handle_client(reader, writer, store, lock):
    while True:
        line = await reader.readline()
        if not line:
            break
        cmd = parse(line.decode().strip())
        with lock:
            response = execute(cmd, store)
        writer.write(response.encode())
        await writer.drain()

async def main():
    store = {}
    lock = asyncio.Lock()
    server = await asyncio.start_server(
        lambda r, w: handle_client(r, w, store, lock),
        "127.0.0.1", 6379)
    async with server:
        await server.serve_forever()
```

The Rust version is structurally similar. The key difference: Rust's `Arc<Mutex<>>` makes the thread-safety contract explicit in the type system. Python relies on the GIL and `asyncio.Lock` for coroutine safety — but there is no compiler check that you locked before accessing shared state.

## 11. Step 8: Disk Persistence

Redis calls its persistence mechanism RDB snapshots. Every few seconds it serializes the entire dataset to disk. On startup it reads the snapshot back. Ours works the same way.

Replace the placeholder `src/persistence.rs` with the full implementation:

```rust
use crate::storage::{Entry, Store};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};
use tokio::time;

#[derive(Serialize, Deserialize)]
struct PersistedEntry {
    value: String,
    expires_at_unix: Option<u64>,
}

#[derive(Serialize, Deserialize)]
struct Snapshot {
    entries: HashMap<String, PersistedEntry>,
}

pub async fn load_from_disk(path: &str, store: &Store) {
    if !Path::new(path).exists() {
        println!("No snapshot found at {}. Starting fresh.", path);
        return;
    }
    match tokio::fs::read_to_string(path).await {
        Err(e) => eprintln!("Could not read snapshot: {}", e),
        Ok(content) => match serde_json::from_str::<Snapshot>(&content) {
            Err(e) => eprintln!("Could not parse snapshot: {}", e),
            Ok(snapshot) => {
                let now_unix = unix_now();
                let mut store = store.lock().unwrap();
                for (key, pe) in snapshot.entries {
                    let expires_at = pe.expires_at_unix.and_then(|exp_unix| {
                        if exp_unix > now_unix {
                            let remaining = exp_unix - now_unix;
                            Some(Instant::now() + Duration::from_secs(remaining))
                        } else {
                            None // already expired, skip
                        }
                    });
                    if pe.expires_at_unix.map_or(true, |_| expires_at.is_some()) {
                        store.insert(key, Entry { value: pe.value, expires_at });
                    }
                }
                println!("Loaded {} keys from {}", store.len(), path);
            }
        },
    }
}

pub fn start_persistence(store: Store, path: &str) {
    let path = path.to_string();
    tokio::spawn(async move {
        let mut interval = time::interval(Duration::from_secs(30));
        loop {
            interval.tick().await;
            save_snapshot(&store, &path).await;
        }
    });
}

async fn save_snapshot(store: &Store, path: &str) {
    let snapshot = {
        let store = store.lock().unwrap();
        let now = Instant::now();
        let now_unix = unix_now();
        let entries = store.iter()
            .filter(|(_, e)| e.expires_at.map_or(true, |exp| now < exp))
            .map(|(k, e)| {
                let expires_at_unix = e.expires_at.map(|exp| {
                    let remaining = (exp - now).as_secs();
                    now_unix + remaining
                });
                (k.clone(), PersistedEntry {
                    value: e.value.clone(),
                    expires_at_unix,
                })
            })
            .collect();
        Snapshot { entries }
    };
    match serde_json::to_string_pretty(&snapshot) {
        Err(e) => eprintln!("Serialization failed: {}", e),
        Ok(json) => match tokio::fs::write(path, json).await {
            Err(e) => eprintln!("Could not write snapshot: {}", e),
            Ok(_) => println!("[snapshot] Saved {} keys to {}", snapshot.entries.len(), path),
        },
    }
}

fn unix_now() -> u64 {
    SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs()
}
```

### Key Design Decisions

| Decision | Why |
|----------|-----|
| `Instant` → Unix timestamp | `Instant` is relative to an unknown clock origin — it does not survive restarts. Unix timestamps are absolute. |
| Lock minimum time | Build the snapshot synchronously, drop the lock, then write to disk asynchronously. Holding a lock across `async` is a common mistake. |
| Skip expired keys on save | No point persisting keys that are already dead. |
| Skip expired keys on load | If the server was down longer than a key's TTL, the key should not come back. |

### What `serde` Does

The `#[derive(Serialize, Deserialize)]` on `PersistedEntry` and `Snapshot` generates all the JSON serialization code automatically. Without it you would write hundreds of lines of manual serialization logic.

**Python equivalent:**

```python
import json
from dataclasses import dataclass, asdict

@dataclass
class PersistedEntry:
    value: str
    expires_at_unix: int | None

@dataclass
class Snapshot:
    entries: dict[str, PersistedEntry]

def save_snapshot(store, path):
    snapshot = Snapshot(entries={
        k: PersistedEntry(v.value, v.expires_at_unix)
        for k, v in store.items()
    })
    with open(path, 'w') as f:
        json.dump(asdict(snapshot), f, indent=2)
```

Python's `json.dump` with `dataclasses.asdict` is one line. Rust's `serde_json::to_string_pretty` is one line too — the `#[derive]` macro does the heavy lifting.

## 12. Step 9: The CLI Client

Connecting with `nc` works but is not a great experience. Let us write a proper client with a prompt.

Create `src/bin/client.rs`:

```rust
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
            Err(e) => { eprintln!("Input error: {}", e); break; }
            Ok(_) => {}
        }

        let trimmed = input.trim();
        if trimmed.is_empty() { continue; }

        writer.write_all(format!("{}\n", trimmed).as_bytes()).await?;

        let mut response = String::new();
        server_reader.read_line(&mut response).await?;
        let resp = response.trim();

        if resp.starts_with('+')      { println!("{}", &resp[1..]); }
        else if resp.starts_with(':') { println!("(integer) {}", &resp[1..]); }
        else if resp == "$-1"         { println!("(nil)"); }
        else if resp.starts_with('-') { println!("Error: {}", &resp[1..]); }
        else                          { println!("{}", resp); }

        if trimmed.to_uppercase() == "QUIT" { break; }
    }
    println!("Goodbye.");
    Ok(())
}
```

### Why Two Runtimes?

The server uses `#[tokio::main]` (multi-threaded async for TCP). The client also uses `#[tokio::main]` for async TCP, but reads from stdin synchronously with `io::stdin().lock().read_line()`. This is the pragmatic approach — async for the network, sync for the terminal.

The optional address argument (`std::env::args().nth(1)`) means you can connect to a remote server:

```bash
cargo run --bin client -- 192.168.1.5:6379
```

## 13. Step 10: Integration Tests

With all modules built, write integration tests that test the full command execution path against a real store.

Create `tests/integration.rs`:

```rust
use std::time::Duration;
use tiny_redis::command::{execute, Command};
use tiny_redis::storage::new_store;

#[tokio::test]
async fn set_and_get() {
    let store = new_store();
    execute(Command::Set {
        key: "name".to_string(),
        value: "Alice".to_string(),
        ttl: None,
    }, &store).await;
    let response = execute(Command::Get { key: "name".to_string() }, &store).await;
    assert_eq!(response, "+Alice\n");
}

#[tokio::test]
async fn get_missing_key_returns_nil() {
    let store = new_store();
    let response = execute(Command::Get { key: "ghost".to_string() }, &store).await;
    assert_eq!(response, "$-1\n");
}

#[tokio::test]
async fn del_removes_key() {
    let store = new_store();
    execute(Command::Set {
        key: "temp".to_string(),
        value: "value".to_string(),
        ttl: None,
    }, &store).await;
    let del_response = execute(Command::Del { key: "temp".to_string() }, &store).await;
    assert_eq!(del_response, ":1\n");
    let get_response = execute(Command::Get { key: "temp".to_string() }, &store).await;
    assert_eq!(get_response, "$-1\n");
}

#[tokio::test]
async fn expired_key_returns_nil() {
    let store = new_store();
    execute(Command::Set {
        key: "short".to_string(),
        value: "lived".to_string(),
        ttl: Some(Duration::from_millis(1)),
    }, &store).await;
    tokio::time::sleep(Duration::from_millis(10)).await;
    let response = execute(Command::Get { key: "short".to_string() }, &store).await;
    assert_eq!(response, "$-1\n");
}

#[tokio::test]
async fn dbsize_counts_active_keys() {
    let store = new_store();
    execute(Command::Set { key: "a".to_string(), value: "1".to_string(), ttl: None }, &store).await;
    execute(Command::Set { key: "b".to_string(), value: "2".to_string(), ttl: None }, &store).await;
    execute(Command::Set { key: "c".to_string(), value: "3".to_string(), ttl: None }, &store).await;
    let response = execute(Command::DbSize, &store).await;
    assert_eq!(response, ":3\n");
}
```

The `expired_key_returns_nil` test is particularly satisfying — it sets a key with a 1ms TTL, sleeps 10ms, then verifies the key is gone. This tests real time-based behaviour.

## 14. Running the Complete System

### Start the Server

```bash
cargo run --bin server
```

You should see:

```
No snapshot found at tiny_redis.snapshot. Starting fresh.
Tiny Redis listening on 127.0.0.1:6379
Use the client: cargo run --bin client
```

### Connect with the Client

In another terminal:

```bash
cargo run --bin client
```

Try a session:

```
Connected to Tiny Redis at 127.0.0.1:6379
Server: Welcome to Tiny Redis. Type QUIT to disconnect.
> PING
PONG
> SET username zeeshan
OK
> GET username
zeeshan
> SET session abc123 EX 30
OK
> TTL session
(integer) 29
> DBSIZE
(integer) 2
> EXISTS username
(integer) 1
> DEL username
(integer) 1
> GET username
(nil)
> QUIT
OK
Goodbye.
```

### Test Concurrent Access

Open a second client terminal while the first is still running. Both work simultaneously — the store is shared, changes from one client visible to the other.

### Test Persistence

1. Start the server, set some keys
2. Stop the server (`Ctrl+C`)
3. Restart the server — your non-expired keys come back from the snapshot

## 15. Exercises

### Easy: Add a `KEYS` Command

Implement `KEYS *` that returns all keys in the store:

```
> SET a 1
OK
> SET b 2
OK
> KEYS *
1) "a"
2) "b"
```

**Hints:**
- Add `Keys { pattern: String }` to the `Command` enum
- In the `parse` method, handle `"KEYS"` with `parts.len() == 2`
- In `execute`, lock the store and collect matching keys
- Use `glob` or simple string matching for the pattern

### Medium: Add an `MGET` Command

Implement `MGET key1 key2 ...` that returns multiple values at once:

```
> SET a 1
OK
> SET b 2
OK
> MGET a b c
1) "1"
2) "2"
3) (nil)
```

**Hints:**
- `MGET` takes a variable number of arguments
- Parse with `parts.len() >= 2` (command + at least one key)
- Lock the store once, then look up each key
- Return a formatted list of responses

### Hard: Add a `KEYS` Pattern with Glob Support

Extend `KEYS` to support glob patterns like `user:*` or `session:?`:

```
> SET user:1 alice
OK
> SET user:2 bob
OK
> SET session:abc token
OK
> KEYS user:*
1) "user:1"
2) "user:2"
```

**Hints:**
- Use the `glob` crate for pattern matching
- Or implement a simple matcher: `*` matches any sequence, `?` matches one character
- Filter the store's keys against the pattern

## 16. Summary

| Concept | How TinyRedis Uses It | Python Equivalent |
|---------|----------------------|-------------------|
| `Arc<Mutex<HashMap>>` | Thread-safe shared store across async tasks | `dict` + `threading.Lock` |
| `thiserror` | Custom `RedisError` with `#[from]` auto-conversion | `class RedisError(Exception)` |
| `enum` + match guards | Typed `Command` with parsing via `match` | `dataclass` + `if/elif` |
| `tokio::spawn` | Background expiry task and connection handlers | `asyncio.create_task` |
| `tokio::time::interval` | Periodic 1s cleanup and 30s snapshot | `asyncio.sleep` loop |
| `serde` + `Serialize`/`Deserialize` | JSON snapshot persistence | `json.dump` + `dataclasses.asdict` |
| `Option<Instant>` | TTL expiry — `None` means no expiry | `Optional[datetime]` |
| `map_or`, `retain` | Functional iteration without if/else chains | ternary, list comprehension |
| TCP `into_split` | Separate reader/writer halves of a connection | `asyncio.open_connection` |
| Integration tests | Test full command execution without TCP | `unittest` + mock server |

### What to Learn Next

Now that you have built a concurrent, persistent key-value store, you are ready to:

- **Upgrade to RESP protocol** — Radish (Project 59) implements the full Redis wire protocol with `Rc<RefCell<>>` on a single-threaded runtime
- **Add data types** — lists, hashes, sets (like real Redis)
- **Add replication** — primary/replica with AOF or RDB propagation
- **Benchmark** — measure throughput with `wrk` or a custom load test
- **Add an HTTP API** — put Axum in front of the store for a REST interface

### Further Reading

- [Redis protocol spec (RESP)](https://redis.io/docs/reference/protocol-spec/)
- [Tokio tutorial](https://tokio.rs/tokio/tutorial)
- [thiserror documentation](https://docs.rs/thiserror)
- [serde guide](https://serde.rs/guide/)
