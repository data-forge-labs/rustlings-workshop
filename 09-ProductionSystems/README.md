# Section 9: Production Systems — Building Real-World Services

*Production-grade Rust: building networked services, async I/O, wire protocols, and in-memory data stores.*

## Prerequisites

- Completed [Section 5: Concurrency](../05-Concurrency/README.md)
- Understand `async`/`.await` and Tokio from project 8

## Projects in This Section

| # | Project | Concepts | Format |
|---|---------|----------|--------|
| 59 | **Radish** — Redis-compatible KV store | `tokio` async, RESP protocol, TCP networking, `Rc<RefCell>`, `BytesMut`, TTL expiry | Project |

## Learning Path

1. Build **59-Radish** to create a production-grade Redis-compatible server

## Key Concepts Covered

| Rust Concept | Python Equivalent | Why It Matters |
|---|---|---|
| `tokio` | `asyncio` | Async I/O runtime |
| TCP networking | `asyncio.start_server` | Network services |
| `BytesMut` | `bytearray` | Zero-copy buffering |
| `Rc<RefCell>` | N/A (GC handles) | Single-threaded shared state |
| RESP protocol | N/A (custom) | Wire protocol design |
| `chrono` | `datetime` | Time handling with TTL |
