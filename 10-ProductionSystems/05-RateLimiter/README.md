# 🦀 Distributed Rate Limiter — gRPC + Redis + Sliding Window

> **Test-driven approach**: This project includes a Cargo project with progressive unit tests. Each function in `workshop/src/lib.rs` starts as a `todo!()` stub. As you follow each section, replace `todo!()` with real code and run `cd workshop && cargo test` to watch the pass count grow. Your goal: **all 10 tests pass**.

---

## What Is This Project?

A production-ready distributed rate limiter using gRPC, Redis, and the sliding window algorithm — the infrastructure piece that protects APIs from abuse.

### Python equivalent

```python
import redis

r = redis.Redis()
key = "user:123:api_calls"
count = r.incr(key)
r.expire(key, 60)  # 1-minute window
if count > 100:
    print("Rate limit exceeded")
```

In this project you'll learn to build this in Rust — and along the way
you'll discover **gRPC with Tonic**, **sliding window algorithms**, **Redis Lua scripts**, **clock abstraction**, and **property-based testing**.

### Topics covered

| # | Concept | Why it matters |
|---|---------|----------------|
| 1 | Rate limiting algorithms | Fixed window, token bucket, sliding window trade-offs |
| 2 | gRPC with Tonic | Binary-serialized async services |
| 3 | Redis Lua scripts | Atomic multi-step operations |
| 4 | Clock abstraction | Testable time-dependent code |
| 5 | Sliding window | Accurate rate limiting without burst exploits |
| 6 | Policy system | Pattern-matched rate limit rules |
| 7 | Health checks | Application-level readiness probes |
| 8 | `tokio::join!` | Concurrent await for parallel Redis fetches |

---

## Table of Contents

1. [Project Overview](#1-project-overview)
2. [Prerequisites](#2-prerequisites)
3. [Concept: Rate Limiting Algorithms](#3-concept-rate-limiting-algorithms)
4. [Concept: gRPC with Tonic](#4-concept-grpc-with-tonic)
5. [Concept: Redis Lua Scripts](#5-concept-redis-lua-scripts)
6. [Concept: Clock Abstraction](#6-concept-clock-abstraction)
7. [Concept: Sliding Window Implementation](#7-concept-sliding-window-implementation)
8. [Concept: Policy System](#8-concept-policy-system)
9. [Concept: Health Checks](#9-concept-health-checks)
10. [Putting It All Together](#10-putting-it-all-together)
11. [Exercises](#11-exercises)
12. [Summary](#12-summary)

---

## 1. Project Overview

Rate limiting controls how many requests a client can make within a time window. It protects against DDoS, enforces fair usage, and controls costs. This project builds a distributed rate limiter as a gRPC microservice using Redis for shared state.

### Why Rust for this?

| Factor | Rust | Python | Go |
|--------|------|--------|-----|
| Latency | Sub-millisecond | GC pauses | GC pauses |
| Concurrency | Compile-time safe | GIL limits | Safe but GC |
| Memory | Predictable, no leaks | High per-object overhead | Moderate |

---

## 2. Prerequisites

- Completed [14-03-RedisAsync](../../14-DataInfrastructure/03-RedisAsync/README.md) — Redis async basics
- Completed [09-04-Proptest](../../09-ObservabilityAndTesting/04-Proptest/README.md) — property-based testing
- Completed [05-13-AsyncPatterns](../../05-Concurrency/13-AsyncPatterns/README.md) — Tokio async patterns

---

## 3. Concept: Rate Limiting Algorithms

### Fixed Window

Divide time into fixed intervals. Count requests in each window.

```
Window 1: 00:00-00:59 → 100 requests allowed
Window 2: 01:00-01:59 → 100 requests allowed
```

**Problem**: A user can make 100 requests at 00:59 and 100 at 01:00 — 200 requests in 2 seconds.

### Token Bucket

A bucket fills with tokens at a constant rate. Each request consumes a token.

- Bucket capacity: 100 tokens
- Refill rate: 10 tokens/second
- Handles bursts naturally (accumulated tokens)

### Sliding Window (our choice)

Combines accuracy with efficiency by weighing the previous window's contribution.

```
At timestamp T (45 seconds into a 60-second window):

weight = (60 - 45) / 60 = 0.25
estimated = current_requests + (previous_requests × weight)
```

**Why sliding window**: No burst exploits, smooth enforcement, only 2 Redis keys per limit.

### Python comparison

```python
# Fixed window — simple but has burst problem
import time

def is_allowed_fixed(key, limit, window):
    current_window = int(time.time() // window)
    count = redis.incr(f"{key}:{current_window}")
    redis.expire(f"{key}:{current_window}", window)
    return count <= limit
```

---

## 4. Concept: gRPC with Tonic

gRPC uses HTTP/2 and Protocol Buffers — binary-serialized, multiplexed, language-agnostic.

### Proto definition

```protobuf
service RateLimiter {
  rpc Acquire(AcquireRequest) returns (AcquireResponse);
}

message AcquireRequest {
  string key = 1;
  int32 tokens = 2;
}

message AcquireResponse {
  bool allowed = 1;
  int32 remaining = 2;
  int64 reset_after = 3;
}
```

### Rust service (simplified)

```rust
#[tonic::async_trait]
impl RateLimiter for RateLimiterImpl {
    async fn acquire(&self, request: Request<AcquireRequest>)
        -> Result<Response<AcquireResponse>, Status>
    {
        let req = request.get_ref();
        // ... rate limit logic ...
        Ok(Response::new(AcquireResponse {
            allowed: true,
            remaining: 99,
            reset_after: 1234567890,
        }))
    }
}
```

### Python comparison

```python
# Python: FastAPI + grpcio
from grpc import aio

class RateLimiterServicer(rate_limiter_pb2_grpc.RateLimiterServicer):
    async def Acquire(self, request, context):
        return rate_limiter_pb2.AcquireResponse(allowed=True, remaining=99)
```

---

## 5. Concept: Redis Lua Scripts

Redis Lua scripts execute atomically on the server — no race conditions between operations.

### Why Lua?

```rust
// This script runs atomically on Redis:
static SCRIPT: LazyLock<redis::Script> = LazyLock::new(|| {
    redis::Script::new(r#"
        local key = KEYS[1]
        local increment = tonumber(ARGV[1])
        local ttl = tonumber(ARGV[2])

        local new_value = redis.call('INCRBY', key, increment)
        redis.call('EXPIRE', key, ttl)

        return new_value - increment
    "#)
});
```

The script returns `new_value - increment` because we need the count *before* this request.

### Python comparison

```python
# Python: Redis-py also supports Lua scripts
SCRIPT = """
local key = KEYS[1]
local increment = tonumber(ARGV[1])
local new_value = redis.call('INCRBY', key, increment)
return new_value - increment
"""
result = r.eval(SCRIPT, 1, key, increment)
```

---

## 6. Concept: Clock Abstraction

Time-dependent code is impossible to test without a clock abstraction.

### The problem

```rust
// This is untestable — you can't control SystemTime::now()
let now = SystemTime::now();
```

### The solution: Clock trait

```rust
pub trait Clock {
    fn now(&self) -> SystemTime;
}

pub struct SystemClock;
impl Clock for SystemClock {
    fn now(&self) -> SystemTime { SystemTime::now() }
}

// In tests:
struct MockClock { now: SystemTime }
impl Clock for MockClock {
    fn now(&self) -> SystemTime { self.now }
}
```

### Python comparison

```python
# Python: mock.patch is the standard approach
from unittest.mock import patch

@patch('time.time', return_value=1761948300.0)
def test_rate_limit(mock_time):
    assert is_allowed("key", 100, 60)
```

In Rust, the `Clock` trait gives you the same control without mocking frameworks.

---

## 7. Concept: Sliding Window Implementation

### The algorithm

```rust
impl<C: Clock> RateLimitAlgorithm for SlidingWindow<C> {
    fn try_acquire(&self, attempt: &AcquireAttempt)
        -> Result<(u32, SystemTime), RateLimitError>
    {
        let now = self.clock.now();
        let window_ms = attempt.window_duration.as_millis();
        let unix_now = to_unix_millis(now);

        // How far into the current window?
        let time_in_current = unix_now % window_ms;
        let remaining = window_ms - time_in_current;
        let reset_after = now + Duration::from_millis(remaining as u64);

        // Fast path: current window alone exceeds limit
        if attempt.current_window_requests
            .saturating_add(attempt.tokens_to_acquire)
            > attempt.max_tokens_per_window
        {
            return Err(RateLimitError::RateLimitExceeded(reset_after));
        }

        // Weighted sliding window
        let weight = remaining as f64 / window_ms as f64;
        let used = attempt.current_window_requests.saturating_add(
            (attempt.previous_window_requests as f64 * weight).round() as u32,
        );

        let possible = used.saturating_add(attempt.tokens_to_acquire);
        if possible > attempt.max_tokens_per_window {
            Err(RateLimitError::RateLimitExceeded(reset_after))
        } else {
            Ok((attempt.max_tokens_per_window - possible, reset_after))
        }
    }
}
```

### Visual walkthrough

```
Window: 60 seconds, limit: 100 requests

Time 12:05:00 (start)     Time 12:05:30 (middle)     Time 12:05:45 (late)
├─────────────────────────┤├─────────────────────────┤├────────────────────┤
│ Prev: 80 │ Curr: 0     ││ Prev: 80 │ Curr: 30    ││ Prev:80│ Curr:30 │
└─────────────────────────┘└─────────────────────────┘└────────────────────┘
Weight: 100% prev          Weight: 50% prev           Weight: 25% prev
Estimated: 80              Estimated: 70              Estimated: 50
```

---

## 8. Concept: Policy System

Different endpoints need different rate limits. The policy system uses pattern matching with priority.

### Configuration

```toml
[default_policy]
max_tokens = 10
window_secs = 60

[[policies]]
pattern = "user.login"
type = "exact"
max_tokens = 5
priority = 100

[[policies]]
pattern = "user."
type = "prefix"
max_tokens = 100
priority = 50
```

### Matching logic

```rust
pub fn find_policy(&self, key: &str) -> &PolicyDefinition {
    self.policies.iter()
        .filter(|rule| match rule.pattern_type {
            PatternType::Exact => rule.pattern == key,
            PatternType::Prefix => key.starts_with(&rule.pattern),
        })
        .max_by_key(|rule| rule.priority)
        .map(|rule| &rule.policy)
        .unwrap_or(&self.default_policy)
}
```

---

## 9. Concept: Health Checks

Application-level health checks verify Redis connectivity.

```rust
async fn check_health(&self) -> HealthCheckResponse {
    let mut conn = self.conn.clone();
    match redis::cmd("PING").query_async::<String>(&mut conn).await {
        Ok(_) => HealthCheckResponse { healthy: true, message: "OK".into() },
        Err(e) => HealthCheckResponse { healthy: false, message: e.to_string() },
    }
}
```

---

## 10. Putting It All Together

### Architecture

```
┌─────────────────┐
│  gRPC Clients   │  (Any language)
└────────┬────────┘
         │ gRPC
         ▼
┌─────────────────────────────┐
│   Rate Limiter Service      │
│   ┌─────────────────────┐   │
│   │  Sliding Window     │   │  ← Algorithm
│   └──────────┬──────────┘   │
│   ┌──────────▼──────────┐   │
│   │  Redis Client       │   │  ← State
│   └─────────────────────┘   │
└──────────────┬──────────────┘
               ▼
         ┌───────────┐
         │   Redis   │
         └───────────┘
```

### Key design decisions

| Decision | Rationale |
|----------|-----------|
| Service (not library) | Centralized logic, polyglot clients |
| Sliding window | No burst exploits, 2 keys per limit |
| Clock trait | Deterministic testing |
| Lua scripts | Atomic Redis operations |
| `thiserror` | Typed errors callers can match on |

---

## 11. Exercises

### Easy — Fixed window algorithm

Implement `FixedWindow` that divides time into fixed intervals:

```rust
struct FixedWindow<C: Clock> { clock: C }

impl<C: Clock> RateLimitAlgorithm for FixedWindow<C> {
    fn try_acquire(&self, attempt: &AcquireAttempt)
        -> Result<(u32, SystemTime), RateLimitError>
    {
        // Your code here
        todo!()
    }
}
```

### Medium — Token bucket

Implement a token bucket that refills at a constant rate:

```rust
struct TokenBucket<C: Clock> {
    clock: C,
    tokens: f64,
    last_refill: SystemTime,
    refill_rate: f64,  // tokens per second
    capacity: f64,
}
```

### Hard — gRPC server

Build the full Tonic gRPC server that exposes the `Acquire` and `CheckHealth` RPCs, connects to Redis, and uses the policy system.

---

## 12. Summary

| Concept | Rust | Python equivalent |
|---------|------|-------------------|
| Rate limiting | `SlidingWindow` struct | redis-py `INCR` + `EXPIRE` |
| gRPC | `tonic` + protobuf | `grpcio` / FastAPI |
| Lua scripts | `redis::Script` | `r.eval(SCRIPT, ...)` |
| Clock abstraction | `Clock` trait + `MockClock` | `unittest.mock.patch` |
| Sliding window | Weighted previous window | Manual implementation |
| Policy matching | `PatternType` enum + priority | Config dict lookup |
| Health checks | `PING` + response struct | Docker healthcheck |
| `tokio::join!` | Concurrent await | `asyncio.gather` |

---

## Exercises

* **Easy** — implement the fixed window algorithm
* **Medium** — implement a token bucket with refill rate
* **Hard** — build the full gRPC server with Redis integration
