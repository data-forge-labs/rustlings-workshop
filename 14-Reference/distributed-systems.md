# Distributed Systems Concepts — Reference

## CAP Theorem

In a distributed data store, you can have at most two of three guarantees:

```
          Consistency (C)
              │
              │
              ├────────── Partition Tolerance (P)
              │
          Availability (A)
```

| System | C | A | P | Example |
|--------|---|---|---|---------|
| Traditional RDBMS | ✅ | ✅ | ❌ | PostgreSQL (single node) |
| CP systems | ✅ | ❌ | ✅ | Zookeeper, etcd |
| AP systems | ❌ | ✅ | ✅ | Cassandra, DynamoDB |
| CA (theoretical) | ✅ | ✅ | ❌ | Can't exist on unreliable networks |

- **Partition** = network split; some nodes can't talk to others
- In practice **P is mandatory** — networks are unreliable
- So the real choice is **CP vs AP**

## Consistency Models

| Model | Guarantee | Use Case |
|-------|-----------|----------|
| Strong (linearizable) | Reads see the most recent write | Banking, leader election |
| Sequential | Operations from one thread are in order | Queue, log |
| Causal | Related operations are ordered | Social feeds |
| Eventual | All replicas converge given no writes | DNS, CDN |
| Weak | No ordering guarantees | Real-time metrics |

## Conflict Resolution

### CRDTs (Conflict-Free Replicated Data Types)

Data structures that can be merged without conflicts:

| CRDT | Purpose | Rust Crate |
|------|---------|------------|
| GCounter | Grow-only counter | `crdts` |
| GSet / TwoPhaseSet | Set with add/remove | `crdts` |
| LWWReg | Last-writer-wins register | `crdts` |
| ORMap | Observed-remove map | `crdts` or `automerge` |

```rust
use crdts::{GCounter, CmRDT};
let mut a = GCounter::new();
let mut b = GCounter::new();
a.apply(a.inc("node1"));
b.apply(b.inc("node2"));
a.merge(b);
assert_eq!(a.read(), 2);
```

### Other strategies
- **LWW** (Last Writer Wins) with wall-clock timestamps
- **Vector clocks** / **Version vectors** for causality tracking
- **Application-level merge** (e.g., shopping cart union)

## Leader Election

Common algorithms for choosing a coordinator:

- **Raft**: Leader-based, uses heartbeats + log replication. `raft` crate.
- **Paxos**: More complex, leader-based quorum protocol.
- **Bully algorithm**: Highest-ID node becomes leader.
- **External**: etcd, Zookeeper, Consul as coordination services.

```rust
// Conceptual Raft — Rust ecosystem: `raft` crate by TiKV
use raft::eraftpb::Message;
use raft::RawNode;
```

## Quorum-Based Reads/Writes

```
Read quorum  + Write quorum > Total replicas
   R               W                N

Example (N=3):
  W=2, R=2  → strong consistency (2 + 2 > 3)
  W=1, R=1  → eventual consistency (1 + 1 ≤ 3)
  W=N, R=1  → write-all (slow writes, fast reads)
```

## Rust's Advantages for Distributed Systems

| Property | How Rust Helps |
|----------|---------------|
| **No GC** | Predictable latency — no stop-the-world pauses |
| **Memory safety** | No segfaults, buffer overflows, use-after-free |
| **Zero-cost abstractions** | High-level code with C-like performance |
| **`async`/`await`** | Efficient non-blocking I/O for many connections |
| **Type system** | Compile-time guarantees for protocol correctness |
| **Ecosystem** | `tokio` (async runtime), `tonic` (gRPC), `raft` (consensus), `kafka` (librdkafka bindings) |

## Python Comparison

| Concept | Python | Rust |
|---------|--------|------|
| Async networking | `asyncio` | `tokio` + `hyper` |
| gRPC | `grpcio` | `tonic` |
| Serialization | `msgpack` / `protobuf` | `serde` + `prost` |
| Shared-nothing processes | `multiprocessing` | `std::thread` + channels |
| GC pauses | Yes (stop-the-world) | No |
| Throughput (TCP echo) | ~5K req/s | ~50K+ req/s |
