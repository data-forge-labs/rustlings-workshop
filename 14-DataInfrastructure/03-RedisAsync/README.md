# 🦀 Redis Async — Cache + Streams with redis-rs

*Subtitle: serve hot reads, push to streams, claim idempotency, and group consumers — all in async Rust.*

> **Test-driven approach**: This project includes a Cargo project with progressive
> unit tests. Each function in `src/lib.rs` starts as a `todo!()` stub. As you
> follow each section, replace `todo!()` with real code and run `cargo test` to
> watch the pass count grow. Your goal: **all 14 tests pass**.

---

## Why async Redis in Rust?

**Python pain:** `redis-py` is sync by default, and `aioredis` (now
folded back into `redis-py`) is callback-shaped. Caches under load need
*pooled* connections, *batched* pipelines, and *atomic* SETNX-style
operations. Python often ends up ad-hoc.

```python
# Race condition: two workers both see the lock as free.
if not redis.set("lock:job-42", "1", nx=True, ex=60):
    raise AlreadyRunning()  # the second worker is already past this check
```

**Rust fix:** `redis-rs` with the `tokio-comp` + `connection-manager`
features gives you a clonable `ConnectionManager` (auto-reconnect,
multiplexed) and a typed `set_nx` that returns a real `bool`. The
borrow checker makes it impossible to write the broken `if !set(...)`
pattern because `set` returns a value, not a side-effect.

## At a Glance

| # | Concept | Rust | Python | Why it matters |
|---|---------|------|--------|----------------|
| 1 | Multiplexed connection | `ConnectionManager` | `redis.asyncio.Redis` | One connection, many tasks |
| 2 | Connection URL | `redis://:pw@host:port/db` | `redis://...` | Same wire format |
| 3 | Cache key naming | `order_key(&Uuid)` | `f"order:{id}"` | Centralized prefixing |
| 4 | TTL bands | `ttl_for(CacheKind)` | per-call `ex=` | Strategy table |
| 5 | Streams | `XADD` / `XREADGROUP` | `xadd` / `xreadgroup` | Persistent queue inside Redis |
| 6 | Consumer groups | `ConsumerGroup` | `xgroup_create` | Per-key load balancing |
| 7 | Idempotency | `IdempotencyClaim` | `SETNX` | At-most-once work scheduling |
| 8 | Cache stats | `CacheStats` | dict + `INCR` | Hit-ratio observability |

---

## Table of Contents
1. [Introduction](#1-introduction)
2. [Prerequisites](#2-prerequisites)
3. [Setup: Create the Project from Scratch](#3-setup-create-the-project-from-scratch)
4. [Concept: redis-rs ConnectionManager](#4-concept-redis-rs-connectionmanager)
5. [Concept: Streams vs Lists](#5-concept-streams-vs-lists)
6. [Concept: Consumer Groups](#6-concept-consumer-groups)
7. [Concept: SETNX for Idempotency](#7-concept-setnx-for-idempotency)
8. [Step-by-Step Implementation](#8-step-by-step-implementation)
9. [Running End-to-End](#9-running-end-to-end)
10. [Summary](#10-summary)

---

## 1. Introduction

This is the **read-side** project of the data infrastructure section.
Redis serves three roles:

- **Cache** — hot order/customer reads
- **Stream** — lightweight event log alternative to Kafka for
  low-volume work queues
- **Idempotency store** — `SETNX` claims that prevent duplicate work

The library exposes pure-data helpers (`order_key`, `ttl_for`,
`ConsumerGroup`, `IdempotencyClaim`, `CacheStats`); `main.rs` does the
async I/O against a local Redis container.

## 2. Prerequisites

- Rust 1.75+ (1.96 recommended)
- Docker Compose: `docker compose up -d redis`
- Concept: async/await (Section 05/02-Futures)
- Concept: serde (Section 04/02-CSVWriter)

## 3. Setup: Create the Project from Scratch

```bash
cargo new --lib redis_async
cd redis_async
# Replace Cargo.toml with workshop/Cargo.toml
cargo test   # 14 tests should fail with "not yet implemented"
```

## 4. Concept: redis-rs ConnectionManager

`ConnectionManager` wraps a single multiplexed TCP connection and is
**`Clone`**. Clone it per task — no need for a pool. The manager
handles reconnects under the hood, so transient network blips don't
crash the program.

```rust
let client = redis::Client::open("redis://localhost:6379/0")?;
let mut conn = redis::aio::ConnectionManager::new(client).await?;
let _: String = conn.set("hello", "world").await?;
```

## 5. Concept: Streams vs Lists

- `LPUSH` / `RPOP` — fast, but no persistence per consumer
- `XADD` / `XREAD` — append-only log, persistent, replayable
- `XADD` / `XREADGROUP` — append-only log + per-consumer-group
  cursors (Kafka-style consumer groups inside Redis)

We use streams because the next project (Kafka) already does the list
pattern; Redis streams give us a "Redis-only" mini-Kafka for tests
where Kafka is overkill.

## 6. Concept: Consumer Groups

`XGROUP CREATE stream:orders svc-billing $ MKSTREAM` creates a
group named `svc-billing` that reads from the **end** of the stream
($) by default. Each consumer in the group gets a unique subset of
messages; the group tracks per-consumer cursors in Redis itself.

## 7. Concept: SETNX for Idempotency

The classic "have I done this before?" check:

```rust
let is_first: bool = conn.set_nx("dedup:event:<id>", "1").await?;
if is_first { do_work().await?; }
```

`set_nx` is atomic on the Redis side. We wrap it in
`IdempotencyClaim` to centralize the key naming and TTL policy.

## 8. Step-by-Step Implementation

Work through the tests in order:

```bash
cargo test step_01_connection   # 2 tests
cargo test step_02_domain       # 2 tests
cargo test step_03_keys         # 4 tests
cargo test step_04_ttl          # 1 test
cargo test step_05_stream       # 2 tests
cargo test step_06_consumer_group
cargo test step_07_idempotency  # 2 tests
cargo test step_08_stats        # 4 tests
```

All `todo!()` bodies live in `src/lib.rs`.

## 9. Running End-to-End

```bash
# From the section root
docker compose up -d redis
cargo run --release --manifest-path 03-RedisAsync/workshop/Cargo.toml
```

You should see:

```
INFO connected to redis: redis://localhost:6379/0
INFO cached order key=order:... hit=true
INFO xadd stream=stream:orders id=...
INFO setnx result first=true second=false
INFO consumer group stream=stream:orders group=demo-group ...
INFO cache stats hits=1 misses=0 ratio=1
```

## 10. Summary

| Concept | Where used | Next project |
|---------|-----------|--------------|
| `ConnectionManager` | `main.rs` | 04-ClickHouseIngestion (HTTP sink) |
| `ttl_for(CacheKind)` | `main.rs` | 08-UnifiedPipeline (cache layer) |
| `StreamEntry` | `entry_to_fields` | 05-ApacheIggy (alternative broker) |
| `ConsumerGroup` | `main.rs` | 07-CdcPipeline (Debezium routing) |
| `IdempotencyClaim` | `main.rs` | 07-CdcPipeline (leader dedup) |
| `CacheStats` | `main.rs` | 08-UnifiedPipeline (observability) |

## Further Reading

- [redis-rs async docs](https://docs.rs/redis/0.27/redis/aio/index.html) — `ConnectionManager` and pipelines
- [Redis Streams tutorial](https://redis.io/docs/latest/develop/data-types/streams/) — `XADD`/`XREADGROUP` semantics
- [Consumer groups in Redis](https://redis.io/docs/latest/commands/xreadgroup/) — load balancing
- [Idempotency keys in distributed systems](https://martin.kleppmann.com/blog/2016/02/08/how-to-do-distributed-locking.html) — Redlock and limits
