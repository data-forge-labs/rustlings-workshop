# Section 13: Actor Model — Concurrent State Machines

*From `mpsc` to `ractor` to a complete ETL pipeline. The actor model: single owner of state, message in, message out.*

---

## Why This Section?

The actor model solves a class of problems that threads and locks handle badly:
**mutable state shared by many senders**. An Airflow DAG, a Kafka consumer, a
rate-limiter — all of these are "one piece of state, many producers/consumers,
sequential processing." Threads + locks work, but race conditions and ordering
bugs are constant. Actors eliminate the problem by making the state private to
one task that processes messages one at a time.

```
                    ┌──────────────────────┐
   sender A ───►    │                      │    ───► reply to A
                    │  Actor (1 task)      │
   sender B ───►    │  + private state     │    ───► reply to B
                    │  + mailbox (mpsc)    │
   sender C ───►    │                      │    ───► reply to C
                    └──────────────────────┘
```

This section shows you:

1. **DIY Actor** — build one from `tokio::sync::mpsc` + `oneshot` in 30 lines
2. **Ractor** — the production `ractor` crate with `cast`, `call`, supervision
3. **ETL Pipeline** — compose three actors (source, transform, sink) for a data pipeline

## Projects in This Section

| # | Project | Concepts | Format |
|---|---------|----------|--------|
| 01 | **DIY-Actor** — Build an actor from `mpsc` + `oneshot` | `tokio::sync::mpsc` mailbox, `oneshot` reply, fire-and-forget tell, request-response ask, graceful shutdown | Workshop |
| 02 | **Ractor** — Production-grade actor framework | `ractor::Actor` trait, `pre_start` / `handle`, `cast` (tell), `call` (ask), `CallResult`, `ActorRef` | Workshop |
| 03 | **ETLPipeline** — Source → Transform → Sink as actors | Pipeline composition, bounded channels for backpressure, atomic per-stage metrics, drop-sender-to-close | Workshop |

## Learning Path

1. **01-DIY-Actor** — internalize the pattern: 1 task, 1 `mpsc` loop, mutable state. 30 lines, no crate.
2. **02-Ractor** — the production wrapper: typed messages, RPC with `CallResult`, supervision, clustering (opt-in).
3. **03-ETLPipeline** — compose three actors. Bounded channels give backpressure for free; per-stage metrics show you which stage is slow.

## Prerequisites

- Section 5: Concurrency (threads, channels, async/await)
- Section 5: Futures and Tokio basics

## Why Actors, Not Locks?

| Concern | `Arc<Mutex<T>>` | Actor |
|---------|----------------|-------|
| Read-modify-write race | manual care | impossible (single owner) |
| Lock contention under load | grows linearly | bounded by mailbox size |
| Composability | shared state is hard to split | drop in a channel between two actors |
| Restart on failure | data lost / lock poisoned | restart the task, replay mailbox |
| Backpressure | none | `mpsc::channel(n)` |

Use actors when you have: a single piece of mutable state, multiple producers
or consumers, sequential processing semantics, or a need for backpressure.
Use locks when you have: simple, low-contention shared state, or need to
read it from many tasks concurrently without coordination.

