# Radish — Build a Redis-Compatible KV Store in Rust

## Why Build a Redis Server from Scratch?

**Python pain:** `redis-py` is just a *client* — it connects to an external Redis server. You never see the wire protocol, the event loop, or the shared-state management:

```python
r = redis.Redis()
r.set("key", "value")  # TCP connect -> RESP encode -> send -> wait -> parse -> return
# all hidden behind a one-liner
```

**Rust fix:** Radish is a full Redis-compatible server built from scratch — raw TCP streams, RESP parsing byte-by-byte, async I/O, TTL expiry, shared state with zero-cost concurrency:

```rust
pub(crate) async fn run() -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind("127.0.0.1:7379").await?;
    let store = Store::new();                     // Rc<RefCell<HashMap<...>>>
    let local = task::LocalSet::new();            // single-threaded Tokio
    local.run_until(async move {
        loop {
            let (mut stream, _) = listener.accept().await?;
            let store_clone = Rc::clone(&store);
            task::spawn_local(async move {
                let mut buf = [0; 512];
                loop {
                    let n = stream.read(&mut buf).await?;
                    if n == 0 { break; }
                    // Parse RESP bytes -> execute -> write response
                }
            });
        }
    }).await;
}
```

Single-threaded Tokio with `Rc<RefCell<>>` gives zero-lock shared state — matching Redis's own design — without needing `Arc` or `Mutex`.

## At a Glance

| # | Concept | Rust | Python | Why it matters |
|---|---------|------|--------|----------------|
| 1 | Async/await | `tokio` runtime | `asyncio` | Concurrent TCP I/O without threads |
| 2 | TCP networking | `tokio::net::TcpListener` | `asyncio.start_server` | Accept and manage client connections |
| 3 | Recursive enum | `RespValue::Array(Vec<RespValue>)` | `Union[str, int, list, None]` | Model nested RESP protocol variants |
| 4 | Pattern matching | `match first { b'*' => ... }` | `if/elif` / `match/case` | Parse wire protocol by first byte |
| 5 | `From` trait | `impl From<&str> for CommandType` | dict lookup with `.upper()` | Case-insensitive command parsing |
| 6 | `Rc<RefCell>` | `Rc<RefCell<Store>>` | mutable object (GIL) | Single-threaded shared state, zero locks |
| 7 | `HashMap` store | `HashMap<String, StoreValue>` | `dict` | In-memory key-value data engine |
| 8 | `BytesMut` | `bytes::BytesMut` | `bytearray` | Zero-copy byte buffer for network I/O |
| 9 | `chrono` | `chrono::DateTime<Utc>` | `datetime.datetime` | TTL expiry timestamp tracking |
| 10 | `spawn_local` | `task::spawn_local` | `asyncio.create_task` | Run `!Send` futures on the same thread |

---

## Concepts at a Glance

**1-2. Async/await with Tokio** — Python's `asyncio` and Rust's `tokio` both provide non-blocking I/O. Tokio defaults to multi-threaded work-stealing; Radish uses `current_thread` flavor to match Redis's single-threaded design, enabling zero-lock shared state.

**3-4. Recursive enum & pattern matching** — RESP is a recursive protocol (arrays can contain arrays). Rust's `enum RespValue` with `Array(Vec<RespValue>)` models this natively. Python's equivalent is `Union[str, int, list, bytes, None]`, but without compiler-verified exhaustiveness.

**5. From trait** — Python uses a dict or `if/elif` chain for name-to-enum mapping. Rust's `From<&str>` trait is the canonical conversion — the compiler enforces every command name produces a valid `CommandType`, and `match` is exhaustive.

**6. Rc<RefCell>** — Python objects are freely mutable because the GIL protects everything. Rust distinguishes: `Rc<RefCell<T>>` for single-threaded (zero overhead) vs `Arc<Mutex<T>>` for multi-threaded (locking overhead). The type system enforces thread safety at compile time.

**7. HashMap store** — Rust's `HashMap<String, StoreValue>` is the same as Python's `dict` — O(1) average lookup. Rust's `entry()` API replaces Python's `dict.setdefault()` pattern for atomic insert-or-update.

**8. BytesMut** — Python's `bytearray` is a mutable byte buffer. Rust's `BytesMut` provides zero-copy slicing — split a buffer into segments without copying data. Critical for high-performance network protocol parsing.

**9. Chrono DateTime** — Python's `datetime.datetime.utcnow()` maps to `chrono::Utc::now()`. Both support adding durations and comparing timestamps. Radish uses it for lazy TTL expiry checking.

**10. spawn_local** — Python's `asyncio.create_task()` always works because of the GIL. Rust's `spawn_local` is explicit about running !Send futures on one thread — the compiler won't let you share `Rc` across threads.

---

## Table of Contents
1. [Introduction](#1-introduction)
2. [Prerequisites](#2-prerequisites)
3. [Architecture Overview](#3-architecture-overview)
4. [Step 1: Project Setup and Dependencies](#4-step-1-project-setup-and-dependencies)
5. [Step 2: RESP Protocol — Parsing and Encoding](#5-step-2-resp-protocol--parsing-and-encoding)
6. [Step 3: Command Routing — Parsing Redis Commands](#6-step-3-command-routing--parsing-redis-commands)
7. [Step 4: In-Memory Store with TTL Support](#7-step-4-in-memory-store-with-ttl-support)
8. [Step 5: Response Evaluation — Command Logic](#8-step-5-response-evaluation--command-logic)
9. [Step 6: TCP Server — Accepting Connections with Tokio](#9-step-6-tcp-server--accepting-connections-with-tokio)
10. [Step 7: Main Entry Point — Tying It All Together](#10-step-7-main-entry-point--tying-it-all-together)
11. [Putting It All Together: Run and Test](#11-putting-it-all-together-run-and-test)
12. [Complete Code Reference](#12-complete-code-reference)
13. [Exercises](#13-exercises)
14. [Summary](#14-summary)

## 1. Introduction

**Radish** is a lightweight, blazing-fast, in-memory key-value data store written in Rust. It implements the REdis Serialization Protocol (RESP) — the wire protocol that Redis uses — and supports a subset of Redis commands (`SET`, `GET`, `TTL`, `PING`, `ECHO`).

This is the **capstone project** of the course. It ties together everything you have learned:
- **Async I/O** with Tokio (Project 8)
- **TCP networking** — `TcpListener` and `TcpStream`
- **Enums and pattern matching** for protocol parsing (Project 5)
- **Interior mutability** with `Rc<RefCell<>>` for shared state without `Arc`/`Mutex` (Project 34)
- **HashMap** for the key-value store (Project 6)
- **Chrono** for time-to-live (TTL) expiry
- **Zero-copy buffers** with the `bytes` crate

**Python → Rust**: In Python, you would use `redis-py` as a client and run the real Redis server as a binary. In Rust, we are building the *server itself* from scratch — handling raw TCP streams, parsing a wire protocol byte-by-byte, and managing shared state. This is the kind of systems-level control where Rust truly shines: zero-cost abstractions for network I/O, memory safety without a garbage collector, and fearless concurrency.

### Data Engineering Motivation

Why build a Redis clone? In production data pipelines, Redis is used everywhere:
- **Result caching** — store intermediate ETL results with TTL
- **Rate limiting** — atomic counters with expiry
- **Session stores** — fast key-value lookups
- **Message brokers** — Redis Streams for event-driven pipelines

Understanding how Redis works under the hood — especially the async event loop and the wire protocol — makes you a better data engineer. You will know exactly when to reach for Redis and why it performs the way it does.

## 2. Prerequisites

Before starting, you should have completed or be familiar with:

- **[Project 3: TicketV1](../02-Ownership/01-TicketV1/README.md)** — Ownership, borrowing, `struct`, `impl`
- **[Project 5: TicketV2](../02-Ownership/03-TicketV2/README.md)** — Enums, `Result`, `match`
- **[Project 6: TicketManagement](../03-Collections/01-TicketManagement/README.md)** — `HashMap`, iterators
- **[Project 8: Futures](../05-Concurrency/02-Futures/README.md)** — `async`/`.await`, Tokio runtime
- **[Project 34: DataRace](../05-Concurrency/03-DataRace/README.md)** — `Arc`, `Mutex`, `Rc`, `RefCell`
- Familiarity with TCP/IP networking basics
- `redis-cli` installed locally (for testing)

## 3. Architecture Overview

Radish follows a single-threaded, fully async architecture — just like the original Redis:

```
┌────────────────────────────────────────────┐
│              Tokio Runtime                  │
│          (current_thread flavor)            │
│  ┌──────────────────────────────────────┐   │
│  │          task::LocalSet              │   │
│  │                                      │   │
│  │  ┌─────────┐  ┌─────────┐  ┌──────┐ │   │
│  │  │ Client1 │  │ Client2 │  │ ...  │ │   │
│  │  └────┬────┘  └────┬────┘  └──┬───┘ │   │
│  │       │             │         │      │   │
│  │       └─────────────┼─────────┘      │   │
│  │                     ▼                │   │
│  │          Rc<RefCell<Store>>          │   │
│  │        (zero-cost shared state)      │   │
│  └──────────────────────────────────────┘   │
└────────────────────────────────────────────┘
```

### Module Breakdown

| Module | File | Responsibility |
|--------|------|----------------|
| `resp.rs` | RESP protocol | Parse raw bytes into `RespValue`, encode `RespValue` back to bytes |
| `cmd.rs` | Command routing | Decode a `RespValue` array into a typed `RadishCommand` |
| `store.rs` | Data engine | In-memory `HashMap` with optional TTL expiry |
| `response.rs` | Command logic | Evaluate a command against the store and produce a RESP response |
| `server.rs` | TCP server | Listen for connections, read bytes, dispatch commands |
| `main.rs` | Entry point | Bootstrap the Tokio runtime |

### Request Lifecycle

```
redis-cli                         Radish Server
    │                                    │
    │─── *3\r\n$3\r\nSET\r\n... ──────▶ │
    │                          ┌────────┴────────┐
    │                          │  resp.rs         │
    │                          │  decode() →      │
    │                          │  RespValue::Array │
    │                          └────────┬────────┘
    │                          ┌────────┴────────┐
    │                          │  cmd.rs          │
    │                          │  from_resp_value │
    │                          │  → RadishCommand │
    │                          └────────┬────────┘
    │                          ┌────────┴────────┐
    │                          │  store.rs        │
    │                          │  set/get/ttl     │
    │                          │  on HashMap      │
    │                          └────────┬────────┘
    │                          ┌────────┴────────┐
    │                          │  response.rs     │
    │                          │  eval() →        │
    │                          │  Resp::encode()  │
    │                          └────────┬────────┘
    │◀─────────── +OK\r\n ──────────────│
```

In Python (using `redis-py`), you would never see any of this — it is all hidden behind `r.set("key", "value")`. Building it yourself in Rust reveals exactly what happens on the wire.

## 4. Step 1: Project Setup and Dependencies

Create a new Cargo project:

```bash
cargo new radish
cd radish
```

Edit `Cargo.toml`:

```toml
[package]
name = "radish"
version = "0.1.0"
edition = "2024"

[dependencies]
bytes = "1.11.1"
chrono = "0.4.44"
tokio = { version = "1.52.3", features = ["full"] }
```

### Dependencies Explained

| Crate | Why We Need It | Python Equivalent |
|-------|----------------|-------------------|
| `tokio` | Async runtime for TCP I/O and task spawning | `asyncio` |
| `bytes` | Zero-copy byte buffer management (`BytesMut`) | `bytearray` (manual) |
| `chrono` | `DateTime<Utc>` for TTL expiry calculations | `datetime.datetime` |

**Python → Rust**: In Python, you would `pip install redis` to get a *client*. Here we are installing server-side crates: `tokio` is like `asyncio` but with a work-stealing runtime, `bytes` gives us efficient buffer slicing (similar to Python's `memoryview`), and `chrono` is the Rust equivalent of `datetime`.

## 5. Step 2: RESP Protocol — Parsing and Encoding

The **REdis Serialization Protocol** (RESP) is a simple wire protocol. Every message is one of these types:

| RESP Type | First Byte | Example Wire Format | Python Equivalent |
|-----------|------------|---------------------|-------------------|
| Simple String | `+` | `+OK\r\n` | `"OK"` |
| Error | `-` | `-ERR msg\r\n` | `RedisError` exception |
| Integer | `:` | `:1000\r\n` | `int` |
| Bulk String | `$` | `$6\r\nfoobar\r\n` | `bytes` or `str` |
| Array | `*` | `*2\r\n...` | `list` |
| Null | `$` | `$-1\r\n` | `None` |

### Python Comparison

In Python with `redis-py`:

```python
import redis
r = redis.Redis()
r.set("key", "value")  # This sends: *3\r\n$3\r\nSET\r\n$3\r\nkey\r\n$5\r\nvalue\r\n
```

You never see the wire format. In Rust, we parse and encode every byte ourselves.

### Implementing RespValue

Create `src/resp.rs`:

```rust
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub(crate) enum RespValue {
    SimpleString(String),  // +OK\r\n
    Integer(i64),          // :1000\r\n
    BulkString(String),    // $6\r\nfoobar\r\n
    Array(Vec<RespValue>), // *2\r\n...
    Error(String),         // -ERR msg\r\n
    Null,                  // $-1\r\n
}
```

This is a **recursive enum** — `Array` contains `Vec<RespValue>`, which can themselves be arrays, bulk strings, etc. This mirrors the recursive nature of the RESP protocol.

In Python, this would be a `Union` type:

```python
from typing import Union
RespValue = Union[str, int, list, bytes, None]
```

Rust's `enum` is safer — every variant is explicitly listed and the compiler enforces that you handle all of them in `match` statements.

### Reading Lines (CRLF-terminated)

RESP uses `\r\n` (CRLF) as its line terminator. We write a helper to split on `\r\n`:

```rust
fn read_line(buf: &[u8]) -> Option<(&[u8], &[u8])> {
    if buf.len() < 2 { return None; }
    let pos = buf.windows(2).position(|w| w == b"\r\n")?;
    Some((&buf[..pos], &buf[pos + 2..]))
}
```

- `windows(2)` slides a 2-byte window over the buffer — like `zip(buf, buf[1:])` in Python.
- `position(|w| w == b"\r\n")` finds the first CRLF — this is the Rust equivalent of `buf.index(b'\r\n')`.
- Returns `None` (the `?` operator) if no CRLF is found — this is the Rust equivalent of Python's `try/except` or a `return None` for partial reads.

### The Decoder (Parser)

The decoder reads the first byte to determine the type, then parses the rest:

```rust
pub(crate) fn decode(buf: &[u8]) -> Option<(RespValue, &[u8])> {
    let first = *buf.first()?;
    match first {
        b'*' => { /* Array: read count, then decode that many values */ }
        b'+' => { /* Simple String: read line */ }
        b':' => { /* Integer: read line, parse as i64 */ }
        b'-' => { /* Error: read line */ }
        b'$' => { /* Bulk String: read length, then that many bytes */ }
        _ => None,
    }
}
```

**Pattern**: The decoder returns `Option<(RespValue, &[u8])>` — the parsed value *plus* the remaining unparsed bytes. This allows chaining: decode an array header, then decode N elements in a loop, each time consuming from the remaining buffer.

In Python, you might write:

```python
def decode(buf):
    first = buf[0:1]
    if first == b'*':
        line, rest = read_line(buf[1:])
        count = int(line)
        values = []
        for _ in range(count):
            value, rest = decode(rest)
            values.append(value)
        return values, rest
    elif first == b'+':
        line, rest = read_line(buf[1:])
        return line.decode(), rest
    # etc.
```

Rust's version is nearly identical, but:
- Pattern matching (`match first { ... }`) is more concise than `if/elif` chains.
- `?` gives us early None without explicit `if` checks.
- Ownership and borrowing are explicit — the returned `&[u8]` slices borrow from the input buffer, zero-copy.

### The Encoder

Encoding converts a `RespValue` back to wire bytes:

```rust
pub(crate) fn encode(value: &RespValue) -> Vec<u8> {
    match value {
        RespValue::SimpleString(s) => format!("+{}\r\n", s).into_bytes(),
        RespValue::BulkString(s)   => format!("${}\r\n{}\r\n", s.len(), s).into_bytes(),
        RespValue::Integer(i)      => format!(":{}\r\n", i).into_bytes(),
        RespValue::Error(e)        => format!("-{}\r\n", e).into_bytes(),
        RespValue::Null            => b"$-1\r\n".to_vec(),
        RespValue::Array(arr) => {
            let mut out = format!("*{}\r\n", arr.len()).into_bytes();
            for value in arr { out.extend(Self::encode(value)); }
            out
        }
    }
}
```

**Convenience helpers** for common RESP responses:

```rust
pub(crate) fn encode_simple_string(s: &str) -> Vec<u8> { Self::encode(&RespValue::SimpleString(s.to_string())) }
pub(crate) fn encode_bulk_string(s: &str) -> Vec<u8> { Self::encode(&RespValue::BulkString(s.to_string())) }
pub(crate) fn encode_error(e: &str) -> Vec<u8> { Self::encode(&RespValue::Error(e.to_string())) }
pub(crate) fn encode_null() -> Vec<u8> { Self::encode(&RespValue::Null) }
```

## 6. Step 3: Command Routing — Parsing Redis Commands

After parsing RESP, we have a `RespValue::Array` — the first element is the command name, the rest are arguments. We need to map this to a typed command.

Create `src/cmd.rs`:

### CommandType Enum

```rust
#[derive(Debug, PartialEq)]
pub(crate) enum CommandType {
    Ping,
    Echo,
    Set,
    Get,
    Ttl,
    Unknown(String),
}
```

### Converting Command Names (Case-Insensitive)

Redis commands are case-insensitive. We implement `From<&str>` — the trait for infallible conversions:

```rust
impl From<&str> for CommandType {
    fn from(s: &str) -> Self {
        match s.to_uppercase().as_str() {
            "PING" => CommandType::Ping,
            "ECHO" => CommandType::Echo,
            "SET"  => CommandType::Set,
            "GET"  => CommandType::Get,
            "TTL"  => CommandType::Ttl,
            _      => CommandType::Unknown(s.to_string()),
        }
    }
}
```

**Python → Rust**: In Python, you would write:

```python
class CommandType(Enum):
    PING = auto()
    ECHO = auto()
    SET = auto()
    GET = auto()
    TTL = auto()
    UNKNOWN = auto()

def command_from_str(s: str) -> CommandType:
    match = s.upper()
    return {
        "PING": CommandType.PING,
        "ECHO": CommandType.ECHO,
        "SET": CommandType.SET,
        "GET": CommandType.GET,
        "TTL": CommandType.TTL,
    }.get(match, CommandType.UNKNOWN)
```

Rust's `From<&str>` trait is more structured — it tells the compiler "this is *the* way to convert a `&str` to a `CommandType`". The match statement with `.to_uppercase().as_str()` is exhaustive and compiler-verified.

### RadishCommand Struct

```rust
pub(crate) struct RadishCommand {
    cmd: CommandType,
    args: Vec<String>,
}
```

The `from_resp_value` method extracts the command and arguments from a parsed RESP array:

```rust
fn from_resp_value(value: RespValue) -> Option<Self> {
    match value {
        RespValue::Array(mut items) if !items.is_empty() => {
            let first_item = items.remove(0);
            let cmd_str = match first_item {
                RespValue::BulkString(s) => s,
                RespValue::SimpleString(s) => s,
                _ => return None,
            };
            let cmd = CommandType::from(cmd_str.as_str());
            let args = items.into_iter()
                .filter_map(|item| match item {
                    RespValue::BulkString(s) => Some(s),
                    RespValue::SimpleString(s) => Some(s),
                    _ => None,
                })
                .collect();
            Some(RadishCommand { cmd, args })
        }
        _ => None,
    }
}
```

**Key patterns:**
- `if !items.is_empty()` — a **match guard**, like a Python `if` condition on a match/case pattern.
- `items.remove(0)` — pops the first element, leaving the rest as arguments. In Python: `cmd_str = items.pop(0)`.
- `filter_map` — Rust's equivalent of `[x for x in items if condition]`, combining filter and map into one iterator adapter.

## 7. Step 4: In-Memory Store with TTL Support

Create `src/store.rs`. This is the data engine — a `HashMap` wrapped in `Rc<RefCell<>>` for shared mutable state.

### The StoreValue

```rust
#[derive(Debug)]
pub(crate) struct StoreValue {
    value: RespValue,
    expiry: Option<DateTime<Utc>>,
}
```

Each stored value has an optional expiry timestamp. If `expiry` is `Some` and the current time is past it, the value is considered expired.

### The Store

```rust
pub(crate) struct Store {
    data: HashMap<String, StoreValue>,
}

impl Store {
    pub(crate) fn new() -> SharedStore {
        Rc::new(RefCell::new(Store {
            data: HashMap::new(),
        }))
    }
}
```

**Why `Rc<RefCell<>>` instead of `Arc<Mutex<>>`?**

This is the most important design decision in Radish. Because we use a **single-threaded async runtime** (`tokio::main(flavor = "current_thread")`), all tasks run on one OS thread. This means:

- We can use `Rc` (reference-counted, not thread-safe) instead of `Arc` (atomically reference-counted, thread-safe).
- We can use `RefCell` (runtime borrow checking, not thread-safe) instead of `Mutex` (lock-based, thread-safe).
- **Zero lock contention** — no mutex, no atomic operations, no performance cost for shared state.

```rust
pub(crate) type SharedStore = Rc<RefCell<Store>>;
```

**Python → Rust**: In Python, there is no distinction between single-threaded and multi-threaded shared state — the GIL protects everything. In Rust, the type system enforces this distinction:
- `Rc<RefCell<T>>` — single-threaded, zero overhead (Python equivalent: just a mutable variable)
- `Arc<Mutex<T>>` — multi-threaded, locking overhead (Python equivalent: `threading.Lock`)

### SET with Optional TTL

```rust
pub(crate) fn set(&mut self, key: String, value: RespValue, expiry_ms: Option<i64>) {
    let expiry = match expiry_ms {
        Some(ms) => Some(Utc::now() + Duration::milliseconds(ms)),
        None => None,
    };
    self.data.insert(key, StoreValue { value, expiry });
}
```

**Python equivalent:**
```python
def set(self, key, value, expiry_ms=None):
    expiry = datetime.utcnow() + timedelta(milliseconds=expiry_ms) if expiry_ms else None
    self.data[key] = StoreValue(value, expiry)
```

### GET with Lazy Expiration

```rust
pub(crate) fn get(&self, key: &str) -> Option<&RespValue> {
    self.data.get(key).and_then(|store_value| {
        if let Some(expiry) = store_value.expiry {
            if Utc::now() > expiry { return None; }
        }
        Some(&store_value.value)
    })
}
```

**Lazy expiration**: We check expiry on access, not with a background timer. This is Redis's approach too — keys are only removed when someone tries to read them (or during periodic sampling).

**`and_then`**: The Rust equivalent of chaining `if` checks. If `self.data.get(key)` returns `Some`, we apply the closure. If it returns `None` (key not found), we short-circuit to `None`.

### TTL Query

```rust
pub(crate) fn ttl(&self, key: &str) -> i64 {
    match self.data.get(key) {
        Some(store_value) => match store_value.expiry {
            Some(expiry) if expiry > Utc::now() => expiry.signed_duration_since(Utc::now()).num_seconds(),
            Some(_) => -2,  // expired
            None => -1,     // no expiry set
        },
        None => -2,  // key does not exist
    }
}
```

**TTL return values** (matching Redis semantics):
- `>= 0` — remaining seconds
- `-1` — key exists but has no expiry
- `-2` — key does not exist or is expired

## 8. Step 5: Response Evaluation — Command Logic

Create `src/response.rs`. This module takes a parsed `RadishCommand` and produces the RESP-encoded response bytes.

```rust
pub(crate) struct Response {
    pub(crate) data: Vec<u8>,
}
```

The `eval` method is the heart of the server — a big `match` on the command type:

```rust
impl Response {
    pub(crate) fn eval(cmd: &RadishCommand, store: &SharedStore) -> Self {
        let data = match cmd.cmd_type() {
            CommandType::Ping => {
                if cmd.args().is_empty() { Resp::encode_simple_string("PONG") }
                else { Resp::encode_bulk_string(&cmd.args()[0]) }
            }
            CommandType::Echo => {
                match cmd.args().get(0) {
                    Some(arg) => Resp::encode_bulk_string(arg),
                    None => Resp::encode_error("ECHO command requires an argument"),
                }
            }
            CommandType::Set => {
                // Parse key, value, optional EX/PX
                let args = cmd.args();
                if args.len() < 2 {
                    return Response { data: Resp::encode_error("SET requires key and value") };
                }
                let key = args[0].clone();
                let value = args[1].clone();
                let mut expiry_ms: Option<i64> = None;
                let mut i = 2;
                while i < args.len() {
                    match args[i].to_uppercase().as_str() {
                        "EX" => { i += 1; expiry_ms = Some(args[i].parse::<i64>().ok()?.saturating_mul(1000)); }
                        "PX" => { i += 1; expiry_ms = Some(args[i].parse::<i64>().ok()?); }
                        _ => return Response { data: Resp::encode_error("Unknown option") };
                    }
                    i += 1;
                }
                store.borrow_mut().set(key, RespValue::BulkString(value), expiry_ms);
                Resp::encode_simple_string("OK")
            }
            CommandType::Get => {
                match cmd.args().get(0) {
                    Some(key) => {
                        let store_ref = store.borrow();
                        match store_ref.get(key) {
                            Some(value) => Resp::encode(value),
                            None => Resp::encode_null(),
                        }
                    }
                    None => Resp::encode_error("GET requires a key"),
                }
            }
            CommandType::Ttl => {
                match cmd.args().get(0) {
                    Some(key) => {
                        let store_ref = store.borrow();
                        let ttl = store_ref.ttl(key);
                        Resp::encode(&RespValue::Integer(ttl))
                    }
                    None => Resp::encode_error("TTL requires a key"),
                }
            }
            CommandType::Unknown(name) => Resp::encode_error(&format!("unknown command: {}", name)),
        };
        Response { data }
    }
}
```

**Pattern**: Each command handler:
1. Validates arguments (Rust's `match` arms make missing-argument checks explicit).
2. Operates on the store through `store.borrow()` (immutable) or `store.borrow_mut()` (mutable).
3. Returns RESP-encoded bytes via `Resp::encode_*` helpers.

In Python's `redis-py`, the equivalent logic lives on the *client* side (converting Python calls to wire bytes). Here, it lives on the *server* side — this is the actual command execution engine.

## 9. Step 6: TCP Server — Accepting Connections with Tokio

Create `src/server.rs`. This is where the async magic happens.

```rust
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;
use tokio::task;
use std::rc::Rc;
```

### The Event Loop

```rust
pub(crate) async fn run() -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind("127.0.0.1:7379").await?;
    let store = Store::new();
    let local = task::LocalSet::new();

    local.run_until(async move {
        loop {
            let (mut stream, addr) = match listener.accept().await {
                Ok(res) => res,
                Err(e) => { eprintln!("accept error: {}", e); continue; }
            };
            let store_clone = Rc::clone(&store);

            task::spawn_local(async move {
                let mut buf = [0; 512];
                loop {
                    match stream.read(&mut buf).await {
                        Ok(0) => break,  // client disconnected
                        Ok(read_count) => {
                            let cmd = RadishCommand::from_bytes(&buf[..read_count]);
                            match cmd {
                                Some(cmd) => {
                                    let response = Response::eval(&cmd, &store_clone);
                                    if let Err(err) = stream.write_all(&response.data).await {
                                        eprintln!("write error: {}", err); break;
                                    }
                                }
                                None => {
                                    let _ = stream.write_all(b"-ERR invalid command\r\n").await;
                                }
                            }
                        }
                        Err(err) => { eprintln!("read error: {}", err); break; }
                    }
                }
            });
        }
    }).await;
    Ok(())
}
```

### How It Works

| Step | Code | Description |
|------|------|-------------|
| 1 | `TcpListener::bind(...)` | Bind to port 7379 and start listening |
| 2 | `task::LocalSet::new()` | Create a local task set (for `!Send` tasks with `Rc`) |
| 3 | `listener.accept().await` | Wait for a connection — non-blocking |
| 4 | `Rc::clone(&store)` | Share the store reference with the new connection handler |
| 5 | `task::spawn_local(...)` | Spawn a new async task for each connection |
| 6 | `stream.read(&mut buf).await` | Read raw bytes from the TCP stream |
| 7 | `RadishCommand::from_bytes(...)` | Parse bytes → RESP → command |
| 8 | `Response::eval(...)` | Execute the command against the store |
| 9 | `stream.write_all(...).await` | Write the RESP response back to the client |

**Python → Rust** (with `asyncio`):

```python
async def handle_client(reader, writer, store):
    while True:
        buf = await reader.read(512)
        if not buf:
            break
        cmd = RadishCommand.from_bytes(buf)
        if cmd:
            response = Response.eval(cmd, store)
            writer.write(response.data)
            await writer.drain()
        else:
            writer.write(b"-ERR invalid command\r\n")
            await writer.drain()

async def main():
    server = await asyncio.start_server(
        lambda r, w: handle_client(r, w, store), "127.0.0.1", 7379)
    async with server:
        await server.serve_forever()
```

The Rust version is structurally nearly identical. The key differences:
- Rust uses `tokio::net::TcpListener` vs Python's `asyncio.start_server`.
- Rust explicitly handles the `match` on `Result` — Python raises exceptions.
- Rust uses `Rc<RefCell<>>` for shared state; Python uses regular object references (GIL-protected).

### Why `spawn_local` instead of `spawn`?

`tokio::task::spawn` requires the future to be `Send` (thread-safe). Our connection handlers capture `Rc<RefCell<Store>>`, which is `!Send` (not thread-safe). `spawn_local` allows `!Send` futures — but they must run on the same thread. We wrap everything inside `task::LocalSet::new().run_until(...)` which provides a single-threaded context.

## 10. Step 7: Main Entry Point — Tying It All Together

Create `workshop/src/main.rs`:

```rust
mod cmd;
mod resp;
mod response;
mod server;
mod store;

use server::Server;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    println!("Logs from your program will appear here!");
    if let Err(e) = Server::run().await {
        eprintln!("Server error: {}", e);
    }
}
```

**`#[tokio::main(flavor = "current_thread")]`**: This attribute macro transforms the `async fn main()` into a proper entry point that initializes the Tokio runtime. The `current_thread` flavor means a single OS thread — all tasks, all connections, all I/O on one thread.

In Python:
```python
import asyncio

def main():
    asyncio.run(server_main())

if __name__ == "__main__":
    main()
```

Both start a single-threaded async event loop. The difference: Rust does it at compile time with a macro attribute, while Python does it at runtime with `asyncio.run()`.

## 11. Putting It All Together: Run and Test

### Build and Run

```bash
cd 59-Radish/radish-master
cd workshop && cargo run
```

You should see:
```
Logs from your program will appear here!
Starting server on 127.0.0.1:7379
```

### Test with redis-cli

Open another terminal and run:

```bash
redis-cli -p 7379
```

Try the commands:

```
127.0.0.1:7379> PING
PONG

127.0.0.1:7379> ECHO "Hello, Radish!"
"Hello, Radish!"

127.0.0.1:7379> SET greeting "Hello, Radish!" EX 120
OK

127.0.0.1:7379> GET greeting
"Hello, Radish!"

127.0.0.1:7379> TTL greeting
(integer) 118

127.0.0.1:7379> GET nonexistent
(nil)
```

### Data Flow Summary

```
redis-cli                          Radish Server
    │                                    │
    │──── *3\r\n$3\r\nSET\r\n... ──────▶│  (1) TcpListener accepts connection
    │                          ┌────────┴────────┐
    │                          │  server.rs       │  (2) stream.read(&mut buf)
    │                          │  loop:           │
    │                          │  read → parse →  │
    │                          │  eval → respond  │
    │                          └────────┬────────┘
    │                          ┌────────┴────────┐
    │                          │  resp.rs         │  (3) Resp::decode()
    │                          │  "*3\r\n$3\r\n   │
    │                          │  SET\r\n..."     │
    │                          │  → RespValue::   │
    │                          │  Array([BulkStr, │
    │                          │  BulkStr, BulkStr])│
    │                          └────────┬────────┘
    │                          ┌────────┴────────┐
    │                          │  cmd.rs          │  (4) from_resp_value()
    │                          │  → RadishCommand │
    │                          │  {Ping|Set|Get|..}│
    │                          └────────┬────────┘
    │                          ┌────────┴────────┐
    │                          │  response.rs     │  (5) eval() → Vec<u8>
    │                          │  match cmd_type  │
    │                          │  → encode result │
    │                          └────────┬────────┘
    │◀─────────── +OK\r\n ──────────────│
```

## 12. Complete Code Reference

The complete source code is in the `radish-master/` subdirectory. Here is a summary of every file:

### `Cargo.toml` — 3 dependencies: `tokio`, `bytes`, `chrono`

### `workshop/src/main.rs` — 15 lines
- Declares modules: `cmd`, `resp`, `response`, `server`, `store`
- Single-threaded Tokio runtime (`current_thread`)
- Calls `Server::run().await`

### `src/resp.rs` — 149 lines
- `RespValue` enum: `SimpleString`, `Integer`, `BulkString`, `Array`, `Error`, `Null`
- `Resp::read_line()` — splits buffer on `\r\n`
- `Resp::decode()` — parses bytes → `(RespValue, remaining_bytes)`
- `Resp::encode()` — converts `RespValue` → wire bytes
- Convenience: `encode_simple_string`, `encode_bulk_string`, `encode_error`, `encode_null`

### `src/cmd.rs` — 70 lines
- `CommandType` enum: `Ping`, `Echo`, `Set`, `Get`, `Ttl`, `Unknown(String)`
- `From<&str>` for `CommandType` — case-insensitive name matching
- `RadishCommand` struct: `cmd` + `args: Vec<String>`
- `from_bytes()` — decode bytes → RESP → command
- `from_resp_value()` — extract command name and arguments from a `RespValue::Array`

### `src/store.rs` — 64 lines
- `StoreValue` struct: `value: RespValue` + `expiry: Option<DateTime<Utc>>`
- `Store` struct: `data: HashMap<String, StoreValue>`
- `Store::new()` — creates `Rc<RefCell<Store>>`
- `set()` — stores value with optional TTL
- `get()` — retrieves value with lazy expiration check
- `ttl()` — returns remaining seconds, `-1`, or `-2`

### `src/response.rs` — 101 lines
- `Response` struct: `data: Vec<u8>`
- `Response::eval()` — matches on `CommandType`, validates args, operates on store, returns RESP bytes

### `src/server.rs` — 79 lines
- `Server` struct
- `Server::run()` — async method:
  1. Binds `TcpListener` to `127.0.0.1:7379`
  2. Creates `SharedStore` (`Rc<RefCell<Store>>`)
  3. Creates `LocalSet` for `!Send` tasks
  4. Accepts connections in a loop
  5. Spawns a `spawn_local` task per connection
  6. Each task: read → parse → eval → write (in a loop)

## 13. Exercises

### Easy: Add a `DEL` Command

Implement the `DEL` command that removes a key from the store:

```
127.0.0.1:7379> SET temp "will be deleted" EX 3600
OK
127.0.0.1:7379> DEL temp
(integer) 1
127.0.0.1:7379> GET temp
(nil)
```

**Hints:**
- Add `Del` to `CommandType` enum in `cmd.rs`
- Add `"DEL" => CommandType::Del` in the `From<&str>` match
- In `response.rs`, add a `CommandType::Del` arm that calls `store.borrow_mut().data.remove(key)`
- In `store.rs`, add a `del(&mut self, key: &str) -> bool` method

### Medium: Add `EXISTS` and `STRLEN` Commands

```
127.0.0.1:7379> SET name "Radish"
OK
127.0.0.1:7379> EXISTS name
(integer) 1
127.0.0.1:7379> STRLEN name
(integer) 6
127.0.0.1:7379> EXISTS nonexistent
(integer) 0
```

**Hints:**
- `EXISTS` returns 1 if the key exists and is not expired, 0 otherwise
- `STRLEN` returns the length of the string value, 0 if key missing
- Both follow the same pattern: add to `CommandType`, add to `From<&str>`, add eval arm

### Hard: Implement `INCR` and Atomic Increment

`INCR` atomically increments a numeric value stored at a key:

```
127.0.0.1:7379> SET counter 0
OK
127.0.0.1:7379> INCR counter
(integer) 1
127.0.0.1:7379> INCR counter
(integer) 2
127.0.0.1:7379> GET counter
"2"
```

**Hints:**
- `INCR` should parse the existing value as an integer, add 1, store it back
- Return the new value as an `Integer` RESP response
- How should you handle the case where the key does not exist? (Redis: treat as 0)
- How should you handle the case where the value is not a number? (Redis: return an error)

### Challenge: Add an `INCR` command to a concurrent version

Change the server to use `Arc<Mutex<Store>>` instead of `Rc<RefCell<Store>>`, then switch to multi-threaded Tokio (`#[tokio::main]` without `current_thread`). Does `INCR` still work correctly? Why or why not?

## 14. Summary

| Concept | How Radish Uses It | Python Equivalent |
|---------|-------------------|-------------------|
| `async`/`.await` | Tokio event loop for concurrent connections | `asyncio` |
| `TcpListener` / `TcpStream` | Accept and read from TCP connections | `asyncio.start_server` |
| Recursive `enum` | `RespValue` with `Array(Vec<RespValue>)` | `Union[str, int, list, bytes, None]` |
| `match` exhaustive | Parse RESP first byte, route command types | `if/elif` chain or `match/case` |
| `From<&str>` trait | Case-insensitive command name conversion | `dict` lookup with `.upper()` |
| `Rc<RefCell<>>` | Single-threaded shared state (zero-lock) | Regular object reference (GIL) |
| `HashMap<String, StoreValue>` | In-memory key-value data engine | `dict` |
| `Option<DateTime<Utc>>` | TTL expiry timestamps | `Optional[datetime]` |
| `and_then`, `filter_map` | Chained option handling on iterators | Chained `if` checks / list comprehensions |
| `saturating_mul` | Safe integer multiplication for TTL conversion | `*` (may overflow silently in Python) |
| `spawn_local` | Run `!Send` tasks on the same thread | `asyncio.create_task` |
| `String::from_utf8_lossy` | Convert raw bytes to string safely | `.decode(errors='replace')` |

### What to Learn Next

Now that you have built a production-grade async TCP server, you are ready to:

- **Extend it** with more Redis commands (LPUSH, LRANGE, EXPIRE, etc.)
- **Add persistence** — snapshot to disk (RDB) or append-only log (AOF)
- **Add a client** — write a Rust `redis-cli` equivalent using `tokio::net::TcpStream`
- **Make it distributed** — shard data across multiple Radish instances with consistent hashing

### Further Reading

- The paper [RESP protocol spec](https://redis.io/docs/reference/protocol-spec/)
- Redis source code — the `server.c` event loop
- Tokio documentation on `LocalSet` and `spawn_local`
- The `bytes` crate for zero-copy buffer management
