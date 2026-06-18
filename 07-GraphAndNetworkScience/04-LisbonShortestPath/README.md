# Project 24: Dijkstra's Algorithm -- Shortest Path on Weighted Graphs

> **Test-driven approach**: This project includes a Cargo project with progressive
> unit tests. Each function in `workshop/src/lib.rs` starts as a `todo!()` stub. As you
> follow each section, replace `todo!()` with real code and run `cd workshop && cargo test` to
> watch the pass count grow. Your goal: **all 14 tests pass**.

## What Is This Project?

Dijkstra's shortest-path algorithm on weighted graphs — route planning with `BinaryHeap`.

### Python equivalent

```python
import networkx as nx

G = nx.DiGraph()
G.add_edge("A", "B", weight=4)
G.add_edge("B", "C", weight=2)
path = nx.shortest_path(G, "A", "C", weight="weight")
```

```rust
use std::collections::{HashMap, BinaryHeap};
use std::cmp::Reverse;

pub fn dijkstra(adj: &HashMap<usize, Vec<(usize, f64)>>, start: usize) -> HashMap<usize, f64> {
    let mut dist = HashMap::new();
    let mut heap = BinaryHeap::new();
    dist.insert(start, 0.0);
    heap.push(Reverse((0.0, start)));
    while let Some(Reverse((d, node))) = heap.pop() {
        if d > dist[&node] { continue; }
        for &(neighbor, w) in &adj[&node] {
            let nd = d + w;
            if nd < *dist.get(&neighbor).unwrap_or(&f64::INFINITY) {
                dist.insert(neighbor, nd);
                heap.push(Reverse((nd, neighbor)));
            }
        }
    }
    dist
}
```

## At a Glance

| # | Concept | Rust | Python | Why it matters |
|---|---------|------|--------|----------------|
| 1 | Weighted edge | `struct WeightedEdge` + derive | `@dataclass` | Model edge with from, to, weight |
| 2 | Weighted adjacency list | `HashMap<usize, Vec<(usize, f64)>>` | `defaultdict(list[tuple])` | Store weighted graph |
| 3 | Min-heap priority queue | `BinaryHeap<Reverse<(f64, usize)>>` | `heapq` | Always expand closest unvisited node |
| 4 | Dijkstra relaxation | distance compare + update | `nx.shortest_path` | Compute shortest distances |
| 5 | Path reconstruction | `prev` map, walk backwards | `prev` dict + reverse | Reconstruct actual path nodes |
| 6 | Path weight computation | `windows(2)` over edges | `zip(path, path[1:])` | Sum edge weights along a path |
| 7 | Path formatting | `.map().join(" -> ")` | `" -> ".join(map(str, path))` | Display path as readable string |

---

---

## Table of Contents

1. [Introduction](#1-introduction)
2. [Prerequisites](#2-prerequisites)
3. [Concept: Weighted Edges and Adjacency](#3-concept-weighted-edges-and-adjacency)
4. [Concept: Dijkstra's Algorithm](#4-concept-dijkstras-algorithm)
5. [Concept: Shortest Path Reconstruction](#5-concept-shortest-path-reconstruction)
6. [Concept: Path Weight and Formatting](#6-concept-path-weight-and-formatting)
7. [Putting It All Together](#7-putting-it-all-together)
8. [Summary](#8-summary)

## 1. Introduction

Finding the shortest path through a weighted graph is a core data-engineering problem: it appears in route planning, network routing, and ETL dependency optimization. Dijkstra's algorithm solves the single-source shortest path problem for graphs with non-negative weights.

Given a graph of Lisbon locations with distances as edge weights, you will compute shortest paths between any two points.

In Python, you would use `networkx.shortest_path(G, source, target, weight='weight')`. In Rust, you implement Dijkstra from scratch using `BinaryHeap` as a priority queue.

## 2. Prerequisites

- Struct definitions and `impl` blocks
- `HashMap`, `BinaryHeap`, iterators
- Basic understanding of graph representation

## 3. Concept: Weighted Edges and Adjacency

### Explanation

A weighted edge has `from`, `to`, and `weight`. In Python, this might be a tuple or a dataclass:

```python
from dataclasses import dataclass

@dataclass
class WeightedEdge:
    from_node: int
    to_node: int
    weight: float
```

In Rust, you define a struct with `#[derive(Debug, Clone, PartialEq)]`:

```rust
#[derive(Debug, Clone, PartialEq)]
pub struct WeightedEdge {
    pub from: usize,
    pub to: usize,
    pub weight: f64,
}
```

An undirected weighted adjacency list maps each node to its neighbors with weights:

```python
adj = defaultdict(list)
for e in edges:
    adj[e.from].append((e.to, e.weight))
    adj[e.to].append((e.from, e.weight))
```

In Rust:

```rust
pub fn build_weighted_adjacency(edges: &[WeightedEdge]) -> HashMap<usize, Vec<(usize, f64)>>
```

## 4. Concept: Dijkstra's Algorithm

### Explanation

Dijkstra's algorithm uses a priority queue to always expand the closest unvisited node. In Python:

```python
import heapq

def dijkstra(adj, start):
    distances = {start: 0}
    heap = [(0, start)]
    while heap:
        dist, node = heapq.heappop(heap)
        if dist > distances.get(node, float('inf')):
            continue
        for neighbor, weight in adj.get(node, []):
            new_dist = dist + weight
            if new_dist < distances.get(neighbor, float('inf')):
                distances[neighbor] = new_dist
                heapq.heappush(heap, (new_dist, neighbor))
    return distances
```

In Rust, `BinaryHeap` is a max-heap by default, so you wrap tuples in `Reverse`:

```rust
use std::cmp::Reverse;
use std::collections::BinaryHeap;

pub fn dijkstra(adj: &HashMap<usize, Vec<(usize, f64)>>, start: usize) -> HashMap<usize, f64>
```

- `BinaryHeap::push(Reverse((dist, node)))` acts as a min-heap
- Skip stale entries where `dist > distances[&node]`
- Update neighbors when a shorter path is found

### Applying to Our Project

`dijkstra` computes distances from `start` to all reachable nodes. Nodes not reachable are absent from the result.

## 5. Concept: Shortest Path Reconstruction

### Explanation

To reconstruct the actual path (not just the distance), track `prev` -- the previous node on the shortest path:

```python
def shortest_path(adj, start, end):
    if start == end:
        return [start]
    distances = {start: 0}
    prev = {}
    heap = [(0, start)]
    while heap:
        dist, node = heapq.heappop(heap)
        if node == end:
            break
        for neighbor, weight in adj.get(node, []):
            new_dist = dist + weight
            if new_dist < distances.get(neighbor, float('inf')):
                distances[neighbor] = new_dist
                prev[neighbor] = node
                heapq.heappush(heap, (new_dist, neighbor))
    # Reconstruct
    path = []
    curr = end
    while curr != start:
        if curr not in prev:
            return None
        path.append(curr)
        curr = prev[curr]
    path.append(start)
    path.reverse()
    return path
```

In Rust:

```rust
pub fn shortest_path(adj: &HashMap<usize, Vec<(usize, f64)>>, start: usize, end: usize) -> Option<Vec<usize>>
```

- Uses a `prev: HashMap<usize, usize>` to store predecessors
- Reconstructs by walking backwards from `end` to `start`
- Returns `None` if no path exists

## 6. Concept: Path Weight and Formatting

### Explanation

Given a path (sequence of nodes), compute its total weight and format it:

```rust
pub fn path_weight(edges: &[WeightedEdge], path: &[usize]) -> Option<f64>
```

Builds a lookup map from `(from, to)` to weight (both directions). Uses `path.windows(2)` to iterate consecutive pairs.

In Python:

```python
def path_weight(edges, path):
    lookup = {(e.from, e.to): e.weight for e in edges}
    lookup.update({(e.to, e.from): e.weight for e in edges})
    total = 0.0
    for i in range(len(path) - 1):
        key = (path[i], path[i+1])
        if key not in lookup:
            return None
        total += lookup[key]
    return total
```

**Formatting** joins nodes with " -> ":

```rust
pub fn format_path(path: &[usize]) -> String
```

In Python: `" -> ".join(map(str, path))`

## 7. Putting It All Together

Open `workshop/src/lib.rs` and implement:

1. **`build_weighted_adjacency`** -- for each edge, add `(to, weight)` to `from`'s list and vice versa
2. **`dijkstra`** -- `BinaryHeap<Reverse<(f64, usize)>>`, relax neighbors, skip stale entries
3. **`shortest_path`** -- extend Dijkstra with `prev` map, reconstruct path, return `None` if unreachable
4. **`path_weight`** -- build bidirectional lookup with `HashMap<(usize, usize), f64>`, `windows(2)` to sum
5. **`format_path`** -- `iter().map(|n| n.to_string()).join(" -> ")`

Run `cd workshop && cargo test` after each step. Groups: `step_01_graph_setup` (3 tests), `step_02_dijkstra` (3 tests), `step_03_path_reconstruction` (5 tests), `step_04_formatting` (3 tests).

## 8. Summary

| Concept | Rust Equivalent | Python Equivalent | Used In |
|---|---|---|---|
| Weighted edge struct | `struct WeightedEdge` with derive | `dataclass` or tuple | `WeightedEdge` |
| Weighted adjacency | `HashMap<usize, Vec<(usize, f64)>>` | `defaultdict(list[tuple])` | `build_weighted_adjacency` |
| Min-heap | `BinaryHeap<Reverse<...>>` | `heapq` | `dijkstra`, `shortest_path` |
| Dijkstra distances | Priority queue with relaxation | `nx.shortest_path` | `dijkstra` |
| Path reconstruction | `prev` map, walk backwards | `prev` dict, reverse | `shortest_path` |
| Path weight | `HashMap<(usize,usize), f64>` + `windows(2)` | dict lookup + loop | `path_weight` |
| Path formatting | `map + join(" -> ")` | `" -> ".join(map(str, path))` | `format_path` |

## Exercises

* **Easy** – modify the existing function to handle an extra edge case.
* **Medium** – extend the project with a new helper function that re‑uses the core logic.

