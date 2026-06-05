# Project 21: Graph Centrality on UFC Data -- Degree and Closeness Centrality

> **Test-driven approach**: This project includes a Cargo project with progressive
> unit tests. Each function in `workshop/src/lib.rs` starts as a `todo!()` stub. As you
> follow each section, replace `todo!()` with real code and run `cd workshop && cargo test` to
> watch the pass count grow. Your goal: **all 14 tests pass**.

## Why Compute Centrality Without `networkx`?

**Python pain:** `nx.degree_centrality` and `nx.closeness_centrality` are one-liners — but every node is a Python object, every BFS step is a Python function call. On a 1K-node UFC fight graph that's ~2.5s where Rust's BFS does it in ~0.05s.

**Rust fix:** Operate directly on flat `HashMap` and `Vec` with `VecDeque` BFS — no wrapper overhead, no Python call dispatch:

```rust
pub fn closeness_centrality(adj: &HashMap<usize, Vec<usize>>) -> HashMap<usize, f64> {
    let mut scores = HashMap::new();
    for &start in adj.keys() {
        let mut dist = HashMap::new();
        let mut queue = VecDeque::new();
        dist.insert(start, 0);
        queue.push_back(start);
        // BFS accumulates distances at machine speed
    }
    scores
}
```

## At a Glance

| # | Concept | Rust | Python | Why it matters |
|---|---------|------|--------|----------------|
| 1 | Undirected adjacency list | `HashMap` + dual insert | `defaultdict(list)` bidirectional | Store undirected graph |
| 2 | Degree centrality | `v.len() as f64 / (N-1)` | `nx.degree_centrality` | Fraction of nodes a fighter has fought |
| 3 | BFS distances | `VecDeque` + `HashMap<usize, usize>` | `collections.deque` + dict | Shortest-path distances |
| 4 | Closeness centrality | `reachable / sum_dist` | `nx.closeness_centrality` | How quickly a fighter reaches others |
| 5 | Max by float | `.max_by(partial_cmp)` | `max(dict, key=dict.get)` | `f64` has no `Ord`, use `partial_cmp` |
| 6 | Sorted formatted output | sort keys, `format!("{:.4}")` | f-string | Deterministic score display |

---

## Table of Contents

1. [Introduction](#1-introduction)
2. [Prerequisites](#2-prerequisites)
3. [Concept: Undirected Adjacency List](#3-concept-undirected-adjacency-list)
4. [Concept: Degree Centrality](#4-concept-degree-centrality)
5. [Concept: Closeness Centrality with BFS](#5-concept-closeness-centrality-with-bfs)
6. [Concept: Analysis Utilities](#6-concept-analysis-utilities)
7. [Putting It All Together](#7-putting-it-all-together)
8. [Summary](#8-summary)

## 1. Introduction

Centrality measures identify the most important nodes in a graph. In data engineering, centrality is used for influencer detection in social networks, identifying critical infrastructure nodes, and ranking entities in knowledge graphs.

Given a graph of UFC fighters where edges represent fights between two fighters, you will compute **degree centrality** (who fought the most opponents) and **closeness centrality** (who can reach others most quickly through the fight network).

In Python, you would use `networkx.degree_centrality` and `networkx.closeness_centrality`. In Rust, you implement both from scratch using `HashMap`, BFS, and basic math.

## 2. Prerequisites

- `HashMap` and `VecDeque` (covered in Section 3)
- Iterators and closures
- Basic graph representation (adjacency list)

## 3. Concept: Undirected Adjacency List

### Explanation

An undirected graph means if `a` is connected to `b`, then `b` is also connected to `a`. In Python:

```python
graph = {0: [], 1: [], 2: []}
for a, b in edges:
    graph[a].append(b)
    graph[b].append(a)
```

In Rust:

```rust
use std::collections::HashMap;

pub fn build_adjacency(edges: &[(usize, usize)]) -> HashMap<usize, Vec<usize>> {
    let mut adj: HashMap<usize, Vec<usize>> = HashMap::new();
    for &(a, b) in edges {
        adj.entry(a).or_default().push(b);
        adj.entry(b).or_default().push(a);
    }
    adj
}
```

The key difference: each edge adds entries in **both** directions.

## 4. Concept: Degree Centrality

### Explanation

Degree centrality is the fraction of other nodes a node is connected to. Formula:

```
degree_centrality(v) = degree(v) / (N - 1)
```

where N is the total number of nodes. In Python with networkx:

```python
import networkx as nx
G = nx.Graph([(0,1), (0,2), (1,2)])
cent = nx.degree_centrality(G)
# {0: 1.0, 1: 1.0, 2: 1.0}  (each connected to 2 of 2 possible)
```

In Rust:

```rust
pub fn degree_centrality(adj: &HashMap<usize, Vec<usize>>) -> HashMap<usize, f64>
```

For each node, divide its neighbor count by `N-1`. If `N <= 1`, use 1.0 as the denominator to avoid division by zero.

### Applying to Our Project

The result is a `HashMap<usize, f64>` mapping each fighter node to a score between 0 and 1. A score of 1.0 means the fighter has fought every other fighter.

## 5. Concept: Closeness Centrality with BFS

### Explanation

Closeness centrality measures how close a node is to all other nodes:

```
closeness(v) = reachable / sum_of_shortest_distances
```

where `reachable` is the number of other nodes v can reach, and `sum_of_shortest_distances` is the sum of shortest path lengths to those nodes.

In Python with networkx:

```python
cent = nx.closeness_centrality(G)
```

To compute it manually, you perform BFS from each node. Python's BFS using `collections.deque`:

```python
from collections import deque

def bfs_distances(graph, start):
    visited = {start: 0}
    queue = deque([start])
    while queue:
        node = queue.popleft()
        for neighbor in graph[node]:
            if neighbor not in visited:
                visited[neighbor] = visited[node] + 1
                queue.append(neighbor)
    return visited
```

In Rust, `VecDeque` serves the same role as `deque`:

```rust
use std::collections::VecDeque;

pub fn closeness_centrality(adj: &HashMap<usize, Vec<usize>>) -> HashMap<usize, f64>
```

For each start node:
1. BFS to compute distances to all reachable nodes
2. Sum distances (excluding self), count reachable nodes
3. Compute `reachable / sum_dist` (return 0.0 if unreachable or isolated)

### Applying to Our Project

In a star graph (center connected to all leaves), the center has closeness 1.0 (directly connected to all), while a leaf has lower closeness because it must go through the center.

## 6. Concept: Analysis Utilities

### Explanation

Two helper functions make the results human-readable:

- **`most_central_node`**: finds the node with the highest centrality score using `max_by` with `partial_cmp` (since `f64` doesn't implement `Ord`):

```rust
pub fn most_central_node(centrality: &HashMap<usize, f64>) -> Option<usize>
```

In Python: `max(scores, key=scores.get)`

- **`format_centrality`**: formats scores as `"node: score"` strings sorted by node ID:

```rust
pub fn format_centrality(scores: &HashMap<usize, f64>) -> Vec<String>
```

In Python:

```python
sorted(f"{k}: {v:.4f}" for k, v in scores.items())
```

### Applying to Our Project

These utilities let you answer "which fighter is most central?" and print all scores in a clean format.

## 7. Putting It All Together

Open `workshop/src/lib.rs` and implement each function:

1. **`build_adjacency`** -- for each `(a, b)`, push `b` to `a`'s list and `a` to `b`'s list
2. **`degree_centrality`** -- `v.len() as f64 / max(1, N-1) as f64`
3. **`closeness_centrality`** -- BFS from each node, sum distances, compute `reachable / sum_dist`
4. **`most_central_node`** -- `.max_by()` with `partial_cmp`
5. **`format_centrality`** -- sort keys, format as `"{}: {:.4}"`

Run `cd workshop && cargo test` after each step. Groups: `step_01_graph_building` (3 tests), `step_02_degree_centrality` (3 tests), `step_03_closeness_centrality` (4 tests), `step_04_analysis` (4 tests).

## 8. Summary

| Concept | Rust Equivalent | Python Equivalent | Used In |
|---|---|---|---|
| Undirected adjacency list | `HashMap<usize, Vec<usize>>` with dual insertion | `defaultdict(list)` | `build_adjacency` |
| Degree centrality | `v.len() / (N-1)` | `nx.degree_centrality` | `degree_centrality` |
| BFS distances | `VecDeque` + `HashMap<usize, usize>` | `collections.deque` + dict | `closeness_centrality` |
| Closeness centrality | `reachable / sum_dist` | `nx.closeness_centrality` | `closeness_centrality` |
| Max by float key | `.max_by(partial_cmp)` | `max(dict, key=dict.get)` | `most_central_node` |
| Sorted formatted output | Sort keys, `format!("{:.4}")` | f-string formatting | `format_centrality` |
