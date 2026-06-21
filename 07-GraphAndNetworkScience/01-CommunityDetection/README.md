# Project 20: Community Detection with Kosaraju's SCC Algorithm

> **Test-driven approach**: This project includes a Cargo project with progressive
> unit tests. Each function in `workshop/src/lib.rs` starts as a `todo!()` stub. As you
> follow each section, replace `todo!()` with real code and run `cd workshop && cargo test` to
> watch the pass count grow. Your goal: **all 17 tests pass**.

---

## What Is This Project?

Kosaraju's strongly-connected-components algorithm — finding communities in directed graphs.

### Python equivalent

```python
import networkx as nx

G = nx.DiGraph()
G.add_edges_from([(0, 1), (1, 2), (2, 0), (3, 4)])
sccs = list(nx.strongly_connected_components(G))
print(sccs)  # [{0, 1, 2}, {3}, {4}]
```

```rust
pub fn kosaraju_scc(edges: &[(usize, usize)], node_count: usize) -> Vec<Vec<usize>> {
    let graph = build_adjacency_list(edges);
    let rev = reverse_graph(&graph);
    let mut visited = vec![false; node_count];
    let mut stack = Vec::new();
    for n in 0..node_count {
        if !visited[n] { dfs_post(n, &graph, &mut visited, &mut stack); }
    }
    // second pass on reversed graph...
}
```

In this project you'll learn to build this in Rust — and along the way
you'll discover **adjacency lists**, **iterative DFS**, and **Kosaraju's algorithm**.

### Topics covered

| # | Concept | Why it matters |
|---|---------|----------------|
| 1 | Adjacency list | Store directed graph edges |
| 2 | Iterative DFS | Avoid stack overflow on deep graphs |
| 3 | Graph transposition | Reverse edges for Kosaraju pass 2 |
| 4 | Kosaraju's algorithm | Find strongly connected components |
| 5 | `HashSet` / `HashMap` | Distinct count, frequency ranking |

---

---

## Table of Contents

1. [Introduction](#1-introduction)
2. [Prerequisites](#2-prerequisites)
3. [Concept: Adjacency List](#3-concept-adjacency-list)
4. [Concept: DFS Traversal](#4-concept-dfs-traversal)
5. [Concept: Reverse Graph](#5-concept-reverse-graph)
6. [Concept: Kosaraju's SCC Algorithm](#6-concept-kosarajus-scc-algorithm)
7. [Concept: Twitter Data Analysis](#7-concept-twitter-data-analysis)
8. [Putting It All Together](#8-putting-it-all-together)
9. [Summary](#9-summary)

## 1. Introduction

Strongly Connected Components (SCCs) are a fundamental graph concept: groups of nodes where every node can reach every other node. In data engineering, SCC detection is used for community detection in social networks, detecting cycles in dependency graphs, and optimizing distributed computations.

You will implement Kosaraju's algorithm, which finds all SCCs in a directed graph using two DFS passes. Then you will apply it to analyze retweet patterns among Twitter accounts.

In Python, you would use `networkx.strongly_connected_components`. In Rust, you build the algorithm from scratch using `HashMap`, `HashSet`, and `Vec`.

## 2. Prerequisites

- Basic Rust: ownership, borrowing, vectors, loops
- Familiarity with `HashMap` and `HashSet` (covered in Section 3)
- No prior graph experience required

## 3. Concept: Adjacency List

### Explanation

In Python, a directed graph is often stored as a dict of lists:

```python
graph = {0: [1], 1: [2], 2: [0]}
```

In Rust, this is a `HashMap<usize, Vec<usize>>`:

```rust
use std::collections::HashMap;

let mut graph: HashMap<usize, Vec<usize>> = HashMap::new();
graph.entry(0).or_default().push(1);
graph.entry(1).or_default().push(2);
graph.entry(2).or_default().push(0);
```

The `entry()` API with `or_default()` creates an empty `Vec` if the key is missing, then returns a mutable reference to insert into.

### Applying to Our Project

The `build_adjacency_list` function converts a list of `(from, to)` edge tuples into this structure:

```rust
pub fn build_adjacency_list(edges: &[(usize, usize)]) -> HashMap<usize, Vec<usize>>
```

It iterates over edges, adds `to` to `from`'s neighbor list, and ensures `to` also has an entry (even if it has no outgoing edges).

## 4. Concept: DFS Traversal

### Explanation

Depth-first search explores a graph by going as deep as possible before backtracking. In Python:

```python
def dfs_order(graph, start):
    visited = set()
    order = []
    stack = [start]
    while stack:
        node = stack.pop()
        if node not in visited:
            visited.add(node)
            order.append(node)
            for neighbor in reversed(graph.get(node, [])):
                if neighbor not in visited:
                    stack.append(neighbor)
    return order
```

In Python, `reversed()` ensures neighbors are processed in order. Rust's `iter().rev()` does the same:

```rust
pub fn dfs_order(graph: &HashMap<usize, Vec<usize>>, start: usize) -> Vec<usize>
```

The function uses a `HashSet` for visited tracking and a `Vec` as a stack. `visited.insert(node)` returns `true` if the node was newly inserted.

### Applying to Our Project

The `dfs_order` function returns the order nodes are visited during DFS from a start node. This is the building block for Kosaraju's first pass.

## 5. Concept: Reverse Graph

### Explanation

A reverse graph has all edges flipped: if the original has `a -> b`, the reverse has `b -> a`. In Python:

```python
def reverse_graph(graph):
    reversed = {node: [] for node in graph}
    for node, neighbors in graph.items():
        for neighbor in neighbors:
            reversed.setdefault(neighbor, []).append(node)
    return reversed
```

In Rust:

```rust
pub fn reverse_graph(graph: &HashMap<usize, Vec<usize>>) -> HashMap<usize, Vec<usize>>
```

It iterates each node's neighbors and pushes the source node into the neighbor's list in the reversed map.

## 6. Concept: Kosaraju's SCC Algorithm

### Explanation

Kosaraju's algorithm finds SCCs in two passes:

**Pass 1**: Perform DFS on every node, recording the order in which nodes finish (post-order). Push each finished node onto a stack.

**Pass 2**: Pop nodes from the stack. For each unvisited node, run DFS on the **reversed** graph to collect one SCC.

In Python with networkx:

```python
import networkx as nx
G = nx.DiGraph([(0,1), (1,2), (2,0), (2,3), (3,4), (4,3)])
sccs = list(nx.strongly_connected_components(G))
# Result: [{0, 1, 2}, {3, 4}]
```

The Rust implementation uses two recursive `dfs_post` and `dfs_collect` helper functions nested inside `kosaraju_scc`:

```rust
fn dfs_post(node: usize, graph: &HashMap<usize, Vec<usize>>, visited: &mut Vec<bool>, stack: &mut Vec<usize>)
fn dfs_collect(node: usize, graph: &HashMap<usize, Vec<usize>>, visited: &mut Vec<bool>, component: &mut Vec<usize>)
```

- `dfs_post` marks visited and, after exploring all neighbors, pushes the node onto `stack`.
- `dfs_collect` collects all nodes reachable in the reversed graph into one component.

### Applying to Our Project

```rust
pub fn kosaraju_scc(edges: &[(usize, usize)], node_count: usize) -> Vec<Vec<usize>>
```

This function:
1. Builds the adjacency list from edges
2. Builds the reverse graph
3. Runs `dfs_post` on all unvisited nodes (Pass 1, fills stack)
4. Pops from stack and runs `dfs_collect` on the reversed graph (Pass 2, collects SCCs)
5. Returns all SCCs, each sorted ascending

## 7. Concept: Twitter Data Analysis

### Explanation

The project includes a dataset of 140 Twitter retweet relationships stored as `TWITTER_USERNAMES`. A pair `(retweeter, original)` represents a directed retweet edge.

You will:
1. **Count distinct users** using a `HashSet` to deduplicate
2. **Find top N most active users** using a `HashMap` to count occurrences, then sort by count descending

In Python:

```python
from collections import Counter
users = ["ten_gop", "leroylovesusa", ...]  # repeated
distinct = len(set(users))
top_3 = Counter(users).most_common(3)
```

In Rust:

```rust
pub fn count_distinct_users() -> usize
pub fn top_n_users(n: usize) -> Vec<(&'static str, usize)>
```

`top_n_users` builds a frequency map, collects into a `Vec`, sorts by `(count desc, name asc)`, and truncates.

## 8. Putting It All Together

Open `workshop/src/lib.rs` and replace each `todo!()` with the correct implementation:

1. **`build_adjacency_list`** -- iterate edges, use `entry().or_default()` to add neighbors
2. **`dfs_order`** -- iterative DFS with `HashSet` and `Vec` stack
3. **`reverse_graph`** -- flip all edges
4. **`kosaraju_scc`** -- two-pass algorithm using recursive helpers
5. **`count_distinct_users`** -- collect `TWITTER_USERNAMES` into a `HashSet`, return length
6. **`top_n_users`** -- count with `HashMap`, sort by count descending, truncate to `n`

Run `cd workshop && cargo test` after each step to verify. The tests are grouped: `step_01_graph_ops` (8 tests), `step_02_kosaraju` (5 tests), `step_03_twitter_data` (4 tests).

## 9. Summary

| Concept | Rust Equivalent | Python Equivalent | Used In |
|---|---|---|---|
| Adjacency list | `HashMap<usize, Vec<usize>>` | `dict[int, list[int]]` | `build_adjacency_list` |
| DFS traversal | Iterative with `Vec` stack + `HashSet` | Stack-based DFS | `dfs_order`, Kosaraju passes |
| Reverse graph | HashMap reversal | Dict comprehension | `reverse_graph` |
| Kosaraju SCC | Two-pass with post-order + reverse DFS | `nx.strongly_connected_components` | `kosaraju_scc` |
| Counting distinct | `HashSet` | `set()` | `count_distinct_users` |
| Frequency ranking | `HashMap` + sort by value | `collections.Counter` | `top_n_users` |

## Exercises

* **Easy** – modify the existing function to handle an extra edge case.
* **Medium** – extend the project with a new helper function that re‑uses the core logic.

