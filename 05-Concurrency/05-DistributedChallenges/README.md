# Consistency Models — Eventual vs Strong, CAP Theorem

> **Test-driven approach**: This project includes a Cargo project with progressive unit tests. Each function in `workshop/src/lib.rs` starts as a `todo!()` stub. As you follow each section, replace `todo!()` with real code and run `cd workshop && cargo test` to watch the pass count grow. Your goal: **all 14 tests pass**.

## Table of Contents
1. [Introduction](#1-introduction)
2. [Prerequisites](#2-prerequisites)
3. [Concept: CAP theorem](#3-concept-cap-theorem)
4. [Concept: Eventual consistency](#4-concept-eventual-consistency)
5. [Concept: CRDT merge](#5-concept-crdt-merge)
6. [Concept: Leader election](#6-concept-leader-election)
7. [Concept: Quorum reads](#7-concept-quorum-reads)
8. [Putting It All Together](#8-putting-it-all-together)
9. [Exercises](#9-exercises)
10. [Summary](#10-summary)

## 1. Introduction

Distributed databases like Cassandra, DynamoDB, and Riak make trade-offs between consistency, availability, and partition tolerance. Understanding these trade-offs is critical for data engineers building reliable pipelines.

This project implements core distributed systems primitives in Rust — CAP validation, eventual consistency, CRDT-style merges, leader election, and quorum-based reads.

**Data engineering context**: When you read from a distributed data store, you need to know whether the data is guaranteed to be up-to-date (strong consistency) or may be stale (eventual consistency). This affects how you design ETL pipelines.

Python has no direct equivalent — these are system design concepts, not language features. We implement them in Rust to get a feel for the logic.

## 2. Prerequisites

- Basic Rust syntax and `Vec` operations
- No prior distributed systems knowledge required

## 3. Concept: CAP theorem

### Explanation

The CAP theorem states a distributed data store can only provide two of three guarantees simultaneously:

- **C**onsistency — every read receives the most recent write or an error
- **A**vailability — every request receives a (non-error) response, without guarantee it contains the most recent write
- **P**artition Tolerance — the system continues to operate despite network partitions

In practice, networks are unreliable, so you must choose P, then decide between C and A: CP systems (strong consistency) or AP systems (eventual consistency).

### Python comparison

```python
def validate_cap(consistency, availability, partition_tolerance):
    # CAP theorem: you can only have 2 of 3
    if consistency and availability and not partition_tolerance:
        return "CA"
    if consistency and not availability and partition_tolerance:
        return "CP"
    if not consistency and availability and partition_tolerance:
        return "AP"
    return "Invalid"
```

### Applying to our project

```rust
pub fn validate_cap_pair(
    consistency: bool,
    availability: bool,
    partition_tolerance: bool,
) -> &'static str {
    match (consistency, availability, partition_tolerance) {
        (true, true, false) => "CA",
        (true, false, true) => "CP",
        (false, true, true) => "AP",
        _ => "Invalid",
    }
}
```

## 4. Concept: Eventual consistency

### Explanation

Eventual consistency means that given enough time without new writes, all replicas will converge to the same value. In practice, this means a read may return stale data.

The `simulate_eventual_consistency` function deduplicates writes — simulating the convergence behavior: if the same value is written multiple times, the system eventually settles on the unique set.

```rust
pub fn simulate_eventual_consistency(writes: Vec<&str>) -> Vec<&str> {
    let mut seen = std::collections::HashSet::new();
    let mut result = Vec::new();
    for item in writes {
        if seen.insert(item) {
            result.push(item);
        }
    }
    result
}
```

### Python comparison

```python
def simulate_eventual_consistency(writes: list[str]) -> list[str]:
    # Deduplicate to simulate convergence
    seen = set()
    result = []
    for item in writes:
        if item not in seen:
            seen.add(item)
            result.append(item)
    return result
```

## 5. Concept: CRDT merge

### Explanation

Conflict-free Replicated Data Types (CRDTs) allow concurrent updates across replicas that automatically merge without conflicts. A simple **Last-Writer-Wins** or **additive merge** (union) approach:

```rust
pub fn merge_crdt_values(local: Vec<&str>, remote: Vec<&str>) -> Vec<&str> {
    let mut merged: Vec<&str> = local.clone();
    for item in remote {
        if !merged.contains(&item) {
            merged.push(item);
        }
    }
    merged
}
```

### Python comparison

```python
def merge_crdt_values(local: list[str], remote: list[str]) -> list[str]:
    # Set union — idempotent merge
    return list(set(local) | set(remote))
```

The Rust version preserves insertion order and uses `contains` instead of a `HashSet` for simplicity. In production, CRDTs use `HashSet` or `BTreeSet`.

## 6. Concept: Leader election

### Explanation

Leader election picks a single node to coordinate work. A simple approach: pick the node with the highest ID (bully algorithm style).

```rust
pub fn simulate_leader_election(node_count: usize) -> usize {
    if node_count == 0 { return 0; }
    node_count - 1  // Highest ID wins
}
```

### Python comparison

```python
def simulate_leader_election(node_count: int) -> int:
    if node_count == 0:
        return 0
    return node_count - 1  # Highest ID wins
```

In real systems, leader election uses algorithms like Raft, Paxos, or Zab. This simplifed model shows the concept.

## 7. Concept: Quorum reads

### Explanation

Quorum ensures consistency by requiring a minimum number of replicas to agree on a read. Given a set of writes with values, find the value that appears at least `quorum_size` times.

```rust
pub fn simulate_quorum_read(writes: Vec<usize>, quorum_size: usize) -> Option<usize> {
    use std::collections::HashMap;
    let mut counts = HashMap::new();
    for &w in &writes {
        *counts.entry(w).or_insert(0) += 1;
    }
    for (value, count) in counts {
        if count >= quorum_size {
            return Some(value);
        }
    }
    None
}
```

### Python comparison

```python
from collections import Counter

def simulate_quorum_read(writes: list[int], quorum_size: int) -> int | None:
    counts = Counter(writes)
    for value, count in counts.items():
        if count >= quorum_size:
            return value
    return None
```

## 8. Putting It All Together

Implement each function in `workshop/src/lib.rs`:

| Function | Concept | Test Module | Tests |
|---|---|---|---|
| `validate_cap_pair()` | CAP theorem logic | `step_01_cap_theorem` | 5 |
| `simulate_eventual_consistency()` | Eventual consistency | `step_02_consistency` | 3 |
| `merge_crdt_values()` | CRDT additive merge | `step_02_consistency` | 3 |
| `simulate_leader_election()` | Leader election | `step_03_distributed_patterns` | 3 |
| `simulate_quorum_read()` | Quorum-based reads | `step_03_distributed_patterns` | 3 |

## 9. Exercises

**Easy**: Modify `merge_crdt_values` to use `HashSet` internally for O(1) deduplication.

**Medium**: Implement a quorum write function that returns `true` if a value is written to enough replicas.

**Hard**: Simulate a full Raft-like state machine: leader election, log replication, and commit indexing.

## 10. Summary

| Concept | Rust Implementation | Python Equivalent |
|---|---|---|
| CAP validation | Enum match on bools | `if/elif` checks |
| Eventual consistency | Dedup via HashSet | `set()` dedup |
| CRDT merge | Vector union | `set` union |
| Leader election | Highest-ID selection | `max(range)` |
| Quorum read | HashMap frequency count | `collections.Counter` |
