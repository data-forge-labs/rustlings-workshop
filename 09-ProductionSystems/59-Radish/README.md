# 🦀 Radish — A Redis-Compatible In-Memory KV Store in Rust

**Radish** is a lightweight, blazing-fast, in-memory key-value data store written in Rust. It implements a faithful subset of the REdis Serialization Protocol (RESP) and mirrors the single-threaded, asynchronous concurrency model that makes Redis itself so performant — all without a single `Mutex` in sight.

Built as a practical workshop project to deeply understand Redis internals, async Rust, and network protocol design from scratch.

## Features

### Core Data Operations

| Command | Syntax | Description |
|---------|--------|-------------|
| SET | `SET key value [EX seconds \| PX milliseconds]` | Store a value under a key, with optional TTL |
| GET | `GET key` | Retrieve the value for a key (returns nil if missing or expired) |
| TTL | `TTL key` | Query remaining time-to-live of a key (in seconds) |

### Time-To-Live (TTL) Support

- Attach expiry durations to any key via `EX` (seconds) or `PX` (milliseconds) on `SET`.
- `TTL` reports remaining seconds, `-1` for keys with no expiry, and `-2` for non-existent/expired keys.
- **Lazy expiration**: expired keys become invisible on access — no background threads or timers required.

### Connection Utilities

| Command | Syntax | Description |
|---------|--------|-------------|
| PING | `PING [message]` | Returns `PONG` or echoes the argument back |
| ECHO | `ECHO message` | Echoes the given message back to the client |

### Protocol Compatibility

- Fully compatible with `redis-cli` and any RESP-speaking client.
- Parses Simple Strings (`+`), Bulk Strings (`$`), Arrays (`*`), Integers (`:`), and Errors (`-`) natively.
- Returns proper RESP-encoded responses.
- Gracefully handles unknown commands with informative error messages.

## Architecture

### Concurrency Model — Single-Threaded Async

Radish deliberately avoids multi-threaded complexity. Like the original Redis, it runs an event loop on a single thread, handling thousands of concurrent connections through asynchronous I/O.

```
┌─────────────────────────────────────────────────┐
│              Tokio (current_thread)              │
│  ┌───────────────────────────────────────────┐   │
│  │             task::LocalSet                │   │
│  │                                           │   │
│  │   ┌──────────┐  ┌──────────┐  ┌───────┐  │   │
│  │   │ Client 1 │  │ Client 2 │  │  ...  │  │   │
│  │   └────┬─────┘  └────┬─────┘  └───┬───┘  │   │
│  │        │              │            │      │   │
│  │        └──────────────┼────────────┘      │   │
│  │                       ▼                   │   │
│  │            Rc<RefCell<Store>>              │   │
│  │          (zero-cost shared state)         │   │
│  └───────────────────────────────────────────┘   │
└─────────────────────────────────────────────────┘
```

- **Runtime**: `tokio::main(flavor = "current_thread")` — one OS thread, fully async.
- **Task spawning**: `tokio::task::spawn_local` — all tasks run on the same thread.
- **Shared state**: `Rc<RefCell<Store>>` — no `Arc`, no `Mutex`, zero lock contention by design.

### Module Breakdown

| Module | Responsibility |
|--------|---------------|
| `main.rs` | Entry point & runtime bootstrap |
| `server.rs` | TCP listener & connection handler |
| `resp.rs` | RESP protocol parser (encode/decode) |
| `cmd.rs` | Command routing & argument extraction |
| `store.rs` | In-memory key-value engine |
| `response.rs` | RESP response formatters & command evaluation |

### Request Lifecycle

```
redis-cli                          Radish Server
    │                                    │
    │──── *5\r\n$3\r\nSET\r\n... ──────▶│
    │                          ┌─────────┴──────────┐
    │                          │  resp.rs            │
    │                          │  Resp::decode()     │
    │                          └─────────┬──────────┘
    │                          ┌─────────┴──────────┐
    │                          │  cmd.rs             │
    │                          │  RadishCommand::    │
    │                          │  from_resp_value()  │
    │                          └─────────┬──────────┘
    │                          ┌─────────┴──────────┐
    │                          │  store.rs           │
    │                          │  Insert into        │
    │                          │  HashMap with       │
    │                          │  expiry timestamp   │
    │                          └─────────┬──────────┘
    │                          ┌─────────┴──────────┐
    │                          │  response.rs        │
    │                          │  Response::eval()   │
    │                          │  → +OK\r\n          │
    │                          └─────────┬──────────┘
    │◀─────────── +OK\r\n ───────────────│
```

## Tech Stack

| Component | Technology |
|-----------|-----------|
| Language | Rust (Edition 2024) |
| Async Runtime | Tokio (single-threaded flavor) |
| Buffer Management | `bytes` (`BytesMut`) |
| Time & Expiry | `chrono` (`DateTime<Utc>`) |
| Protocol | RESP (REdis Serialization Protocol) |

## Quick Start

```bash
git clone https://github.com/Ansh934/radish.git
cd radish
cargo run
```

In another terminal, connect with redis-cli:

```bash
redis-cli -p 7379
```

### Example Session

```
127.0.0.1:7379> PING
PONG

127.0.0.1:7379> SET greeting "Hello, Radish!" EX 120
OK

127.0.0.1:7379> GET greeting
"Hello, Radish!"

127.0.0.1:7379> TTL greeting
(integer) 118

127.0.0.1:7379> ECHO "Radish is alive!"
"Radish is alive!"
```

## Learning Objectives

This workshop introduces these advanced Rust concepts:

| Concept | How It's Used |
|---------|---------------|
| `async`/`.await` | Tokio async event loop for handling concurrent connections |
| `tokio` runtime | Single-threaded `current_thread` runtime with `LocalSet` |
| `Rc<RefCell<Store>>` | Shared state without `Arc`/`Mutex` — single-threaded interior mutability |
| `BytesMut` | Zero-copy byte buffer management from the `bytes` crate |
| RESP protocol parsing | Recursive descent parser for a wire protocol (Redis Serialization Protocol) |
| TCP networking | `tokio::net::TcpListener` and `TcpStream` for network I/O |
| `chrono` for TTL | `DateTime<Utc>` for time-to-live expiration |
| Single-threaded architecture | Understanding why Redis avoids multithreading for I/O-bound workloads |
| Pattern matching on enums | Rich `RespValue` enum with recursive parsing via `match` |
| `From` trait | `From<&str>` for case-insensitive command name matching |
| `spawn_local` | `tokio::task::spawn_local` for !Send tasks on the current thread |
