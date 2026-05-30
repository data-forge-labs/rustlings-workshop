# Project 25: Graph Centrality Algorithms -- Degree, Closeness, and Betweenness

> **Test-driven approach**: This project includes a Cargo project with progressive
> unit tests. Each function in `workshop/src/lib.rs` starts as a `todo!()` stub. As you
> follow each section, replace `todo!()` with real code and run `cd workshop && cargo test` to
> watch the pass count grow. Your goal: **all 14 tests pass**.

## Table of Contents

1. [Introduction](#1-introduction)
2. [Prerequisites](#2-prerequisites)
3. [Concept: Graph Struct](#3-concept-graph-struct)
4. [Concept: Degree Centrality](#4-concept-degree-centrality)
5. [Concept: Closeness Centrality](#5-concept-closeness-centrality)
6. [Concept: Betweenness Centrality](#6-concept-betweenness-centrality)
7. [Concept: Analysis and Ranking](#7-concept-analysis-and-ranking)
8. [Putting It All Together](#8-putting-it-all-together)
9. [Summary](#9-summary)

## 1. Introduction

Centrality algorithms identify the most important nodes in a graph. This project implements three classic measures: degree, closeness, and betweenness centrality. These are foundational for social network analysis, infrastructure planning, and recommendation systems.

In Python, you would use `networkx` or the Neo4j Graph Data Science library via the `neo4j` Python driver. In Rust, you build all three algorithms from scratch using `HashMap`, `VecDeque`, and `Vec`.

## 2. Prerequisites

- Structs and associated functions
- `HashMap`, `VecDeque`, iterators
- Prior centrality project (03-UFCGraphCentrality) is helpful but not required

## 3. Concept: Graph Struct

### Explanation

Instead of using a raw `HashMap`, wrap the adjacency list in a `Graph` struct:

```rust
pub struct Graph {
    pub adjacency: HashMap<usize, Vec<usize>>,
}
```

In Python, this would be a class:

```python
class Graph:
    def __init__(self, edges):
        self.adjacency = defaultdict(list)
        for a, b in edges:
            self.adjacency[a].append(b)
            self.adjacency[b].append(a)
```

The constructor is `build_graph`:

```rust
pub fn build_graph(edges: &[(usize, usize)]) -> Graph
```

This converts edge tuples into an undirected adjacency list, ensuring every node mentioned has an entry.

## 4. Concept: Degree Centrality

### Explanation

Degree centrality counts how many direct neighbors a node has, normalized by the maximum possible (N-1):

```
degree(v) = degree(v) / (N - 1)
```

In Python with networkx:

```python
import networkx as nx
G = nx.Graph([(0,1), (0,2), (1,2)])
cent = nx.degree_centrality(G)
```

In Rust:

```rust
pub fn compute_degree_centrality(graph: &Graph) -> HashMap<usize, f64>
```

This iterates over the adjacency list and divides each neighbor count by `(N - 1)`.

### Applying to Our Project

For a star graph where node 0 connects to all others, node 0 has degree centrality 1.0, while leaves have `1/(N-1)`.

## 5. Concept: Closeness Centrality

### Explanation

Closeness centrality measures how quickly a node can reach all others:

```
closeness(v) = reachable / sum_of_shortest_paths
```

In Python with networkx:

```python
cent = nx.closeness_centrality(G)
```

The implementation performs BFS from each node to compute shortest-path distances. Using `VecDeque` for BFS:

```rust
pub fn compute_closeness_centrality(graph: &Graph) -> HashMap<usize, f64>
```

For each node:
1. BFS tracking distances in a `HashMap<usize, usize>`
2. Sum distances to all reachable nodes (excluding self)
3. Return `reachable / sum_dist`, or 0.0 if no other nodes are reachable

### Applying to Our Project

In a line graph `0-1-2`, the middle node (1) has the highest closeness because it has the shortest average distance to all others.

## 6. Concept: Betweenness Centrality

### Explanation

Betweenness centrality measures how often a node lies on the shortest path between other pairs of nodes:

```
betweenness(v) = sum over all pairs (s,t) of (number of shortest paths through v) / (total shortest paths from s to t)
```

In Python with networkx:

```python
cent = nx.betweenness_centrality(G)
```

A simplified implementation uses BFS from each node to count how many times each other node appears on shortest paths:

```rust
pub fn compute_betweenness_centrality(graph: &Graph) -> HashMap<usize, f64>
```

For each source node:
1. BFS to find distances and count of shortest paths to each node
2. Track which nodes are predecessors on shortest paths
3. Accumulate "dependency" scores from farthest nodes back to the source

Nodes that lie on many shortest paths get a high betweenness score.

### Applying to Our Project

In a line graph `0-1-2`, node 1 lies on the only shortest path between 0 and 2, so it has high betweenness. In a fully connected triangle, no node lies on any shortest path (the paths are direct), so all have zero betweenness.

## 7. Concept: Analysis and Ranking

### Explanation

Two utility functions help interpret centrality results:

- **`top_n_central`**: returns the top N nodes by score, sorted descending. In Python: `sorted(scores.items(), key=lambda x: -x[1])[:n]`

```rust
pub fn top_n_central(scores: &HashMap<usize, f64>, n: usize) -> Vec<(usize, f64)>
```

- **`centrality_summary`**: returns (min, max, mean) of all scores. In Python: `min(scores.values()), max(scores.values()), sum(scores.values()) / len(scores)`

```rust
pub fn centrality_summary(scores: &HashMap<usize, f64>) -> (f64, f64, f64)
```

## 8. Putting It All Together

Open `workshop/src/lib.rs` and implement each function:

1. **`build_graph`** -- iterate edges, insert bidirectionally with `entry().or_default()`
2. **`compute_degree_centrality`** -- for each node, `neighbors.len() as f64 / (N-1) as f64`
3. **`compute_closeness_centrality`** -- BFS from each node using `VecDeque`, compute sum of distances
4. **`compute_betweenness_centrality`** -- Brandes' algorithm: BFS from each source, accumulate dependencies
5. **`top_n_central`** -- collect into `Vec<(usize, f64)>`, sort by score descending, truncate
6. **`centrality_summary`** -- fold values for min/max, divide sum by count for mean

Run `cd workshop && cargo test` after each step. Groups: `step_01_graph_creation` (3 tests), `step_02_degree` (3 tests), `step_03_closeness` (2 tests), `step_04_betweenness` (2 tests), `step_05_analysis` (4 tests).

## 9. Summary

| Concept | Rust Equivalent | Python Equivalent | Used In |
|---|---|---|---|
| Graph struct | `struct Graph { adjacency: HashMap... }` | Class with `defaultdict` | `build_graph` |
| Degree centrality | `neighbors.len() / (N-1)` | `nx.degree_centrality` | `compute_degree_centrality` |
| Closeness centrality | BFS from each node, sum distances | `nx.closeness_centrality` | `compute_closeness_centrality` |
| Betweenness centrality | Brandes' BFS-based algorithm | `nx.betweenness_centrality` | `compute_betweenness_centrality` |
| Top-N ranking | Collect, sort by score desc, truncate | `sorted(dict.items(), key=-x[1])` | `top_n_central` |
| Summary stats | Fold for min/max, sum/count for mean | `min/max/sum/len` | `centrality_summary` |
