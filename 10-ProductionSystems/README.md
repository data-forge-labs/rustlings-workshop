# Section 10: Production Systems — Building Real-World Services

*Production-grade Rust: building networked services, async I/O, wire protocols, and in-memory data stores.*

---

## Why This Section?

### The Problem — Python Async Services Hit Walls

Python's `asyncio` is great for I/O-bound tasks, but production services face limits:

```python
# Python async Redis-like server
import asyncio

class SimpleKV:
    def __init__(self):
        self.store = {}

    async def handle(self, reader, writer):
        data = await reader.read(1024)
        cmd = data.decode().strip()

        if cmd.startswith("GET"):
            key = cmd.split()[1]
            result = self.store.get(key, "nil")
        elif cmd.startswith("SET"):
            _, key, value = cmd.split()
            self.store[key] = value
            result = "OK"

        writer.write(f"{result}\r\n".encode())
        await writer.drain()
        writer.close()
```

**The problems at scale:**

```
┌─────────────────────────────────────────────────────┐
│  Python Async Service Bottlenecks                    │
│                                                      │
│  ┌─────────────────────────────────────────────┐    │
│  │  1. Single event loop — all coroutines share │    │
│  │     one thread on one CPU core               │    │
│  ├─────────────────────────────────────────────┤    │
│  │  2. GC pauses — stop-the-world at any time  │    │
│  │     → latency spikes of 50-200ms             │    │
│  ├─────────────────────────────────────────────┤    │
│  │  3. Memory overhead — each connection costs │    │
│  │     ~10 KB in Python (objects, buffers)      │    │
│  ├─────────────────────────────────────────────┤    │
│  │  4. Object overhead — dict-based storage    │    │
│  │     ~200 bytes per key-value pair            │    │
│  └─────────────────────────────────────────────┘    │
└─────────────────────────────────────────────────────┘
```

### The Rust Solution — Tokio + Zero-Cost Abstraction

```rust
use tokio::net::TcpListener;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use std::collections::HashMap;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind("127.0.0.1:6379").await?;
    let store: HashMap<String, String> = HashMap::new();

    loop {
        let (mut socket, _) = listener.accept().await?;
        let store = store.clone();  // Arc<RwLock<...>> in real code
        tokio::spawn(async move {
            // Handle connection
        });
    }
}
```

**Rust advantages:**
- **No event loop bottleneck** — `tokio` uses work-stealing across all cores
- **No GC pauses** — deterministic RAII cleanup
- **~2 KB per connection** — minimal memory overhead
- **~32 bytes per key-value** — tight structs, not Python dicts

---

## What You'll Learn

| # | Concept | Rust Type / Module | Python Equivalent | Purpose |
|---|---------|--------------------|------------------|---------|
| 1 | Async runtime | `tokio` | `asyncio` | Multi-threaded async executor |
| 2 | TCP networking | `tokio::net` | `asyncio.start_server` | Async TCP server/client |
| 3 | Zero-copy buffering | `bytes::BytesMut` | `bytearray` | Efficient buffer management |
| 4 | Shared state | `Arc<RwLock<T>>` | N/A (GC handles) | Thread-safe shared data |
| 5 | Wire protocol | RESP | N/A | Redis serialization protocol |
| 6 | TTL expiry | `chrono` + `HashMap` | `time` + `dict` | Time-based key expiration |
| 7 | Async I/O traits | `AsyncRead`, `AsyncWrite` | Async context mgrs | Non-blocking read/write |
| 8 | Task spawning | `tokio::spawn` | `asyncio.create_task` | Concurrent connection handling |

---

## Concepts at a Glance

### 1. `tokio` — The Async Runtime

Tokio is to Rust what `asyncio` is to Python — but multi-threaded:

```
  Python asyncio:
  ┌─────────────────────────────┐
  │         Event Loop          │
  │  [task1] [task2] [task3]   │
  │  ─────────────────────────► │
  │         1 CPU core          │
  └─────────────────────────────┘

  Rust Tokio:
  ┌─────────────────────────────┐
  │     Work-Stealing Scheduler │
  │  [task1] [task2] [task3]   │
  │  ────┬──────┬──────┬──────►│
  │   CPU0  CPU1  CPU2  CPU3   │
  └─────────────────────────────┘
```

### 2. `tokio::net` — Async TCP

```rust
let listener = TcpListener::bind("127.0.0.1:6379").await?;
loop {
    let (mut socket, addr) = listener.accept().await?;
    tokio::spawn(async move {
        let mut buf = vec![0; 1024];
        loop {
            let n = socket.read(&mut buf).await?;
            if n == 0 { break; }
            // process data
        }
    });
}
```

### 3. `BytesMut` — Zero-Copy Buffering

```rust
use bytes::BytesMut;

let mut buf = BytesMut::with_capacity(64);
buf.extend_from_slice(b"SET key value\r\n");
// Split without copying:
let first_line = buf.split_to(16);  // zero-copy
```

### 4. RESP Protocol

Redis Serialization Protocol (RESP) is simple and human-readable:

```
  Client → Server: *3\r\n$3\r\nSET\r\n$3\r\nkey\r\n$5\r\nvalue\r\n
  Server → Client: +OK\r\n

  Types:
    +OK\r\n      → Simple String (status)
    -Error\r\n   → Error
    :1\r\n       → Integer
    $5\r\nhello\r\n  → Bulk String
    *2\r\n...\r\n    → Array
```

### 5. TTL Expiry Pattern

```rust
struct Entry {
    value: String,
    expires_at: Option<Instant>,
}

impl Entry {
    fn is_expired(&self) -> bool {
        self.expires_at
            .map(|exp| Instant::now() > exp)
            .unwrap_or(false)
    }
}
```

---

## Prerequisites

- Completed [Section 5: Concurrency](../05-Concurrency/README.md)
- Understand `async`/`.await` and Tokio from project 8

## Projects in This Section

| # | Project | Concepts | Format |
|---|---------|----------|--------|
| # | Project | Concepts | Format |
|---|---------|----------|--------|
| 59 | **Radish** — Redis-compatible KV store | `tokio` async, RESP protocol, TCP networking, `Rc<RefCell>`, `BytesMut`, TTL expiry | Project |
| 60 | **AxumShop** — Shop Manager API with Axum | `axum::Router`, `tokio` async, `sqlx` async DB, `serde` JSON, `tower-http` CORS, `tower-sessions`, `FromRequestParts` auth, SHA-256 hashing, DB transactions | Workshop |
| 61 | **AxumAuth** — JWT + Bearer middleware for Axum 0.8 | `jsonwebtoken` 9, HS256 sign/verify, typed `Claims`, role-based access, refresh tokens, `kid` header inspection | Workshop |
| 62 | **OpenTelemetry** — Traces, spans, and correlation IDs | `tracing` 0.1, `tracing-subscriber` JSON output, OTel attribute model, `AtomicU64` pipeline metrics, `Uuid` correlation ids | Workshop |

## Learning Path

1. Build **01-Radish** to create a production-grade Redis-compatible server
2. Build **02-AxumShop** to create a full async web API with Axum, matching a FastAPI project end-to-end
3. **03-AxumAuth** adds JWT bearer auth + role checks — drop-in for any Axum service
4. **04-OpenTelemetry** adds structured JSON logging, spans, and atomic metrics — the OTel data model without a collector
