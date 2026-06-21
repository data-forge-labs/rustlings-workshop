# Project 31: Graph Connectivity -- Checking Fully Connected Graphs with Memoization

> **Test-driven approach**: This project includes a Cargo project with progressive
> unit tests. Each function in `workshop/src/lib.rs` starts as a `todo!()` stub. As you
> follow each section, replace `todo!()` with real code and run `cd workshop && cargo test` to
> watch the pass count grow. Your goal: **all 8 tests pass**.

---

## What Is This Project?

Checking graph connectivity with memoization — efficiently determining if all nodes are reachable.

### Python equivalent

```python
def is_fully_connected(nodes, edges):
    adj = {n: [] for n in nodes}
    for a, b in edges:
        adj[a].append(b)
        adj[b].append(a)
    visited = set()
    dfs(nodes[0], adj, visited)
    return len(visited) == len(nodes)
```

```rust
pub fn fully_connected_graph(nodes: &[i32], edges: &[(i32, i32)]) -> bool {
    let mut memory = HashMap::new();
    for i in 0..nodes.len() {
        if !fully_connected_node(i, nodes, edges, &mut memory) {
            return false;
        }
    }
    true
}
// 50K edge lookups → 0.01s (vs 500K lookups → 0.5-1s)
```

In this project you'll learn to build this in Rust — and along the way
you'll discover **graph connectivity**, **DFS**, and **memoization**.

### Topics covered

| # | Concept | Why it matters |
|---|---------|----------------|
| 1 | Edge existence check | Pattern match on slice iter |
| 2 | Memoization cache | Store verified connections |
| 3 | Full connectivity | Verify graph is fully connected |
| 4 | Complete graph generation | Generate all unique undirected pairs |

---

---

## Table of Contents

1. [Introduction](#1-introduction)
2. [Prerequisites](#2-prerequisites)
3. [Concept: Checking Direct Connections](#3-concept-checking-direct-connections)
4. [Concept: Single Node Connectivity](#4-concept-single-node-connectivity)
5. [Concept: Full Graph Connectivity](#5-concept-full-graph-connectivity)
6. [Concept: Generating Complete Graphs](#6-concept-generating-complete-graphs)
7. [Putting It All Together](#7-putting-it-all-together)
8. [Summary](#8-summary)

## 1. Introduction

A graph is "fully connected" (also called a complete graph) when every node is directly connected to every other node. Checking connectivity is a fundamental graph operation used in network validation, cluster analysis, and data integrity verification.

In this project, you will build functions to check whether a graph is fully connected, using `HashMap` for memoization to avoid redundant lookups. You will also generate the edges for a complete graph from a list of nodes.

In Python, you would write manual loops over edge lists. In Rust, you use `HashMap` as a cache and iterate over slices.

## 2. Prerequisites

- Basic Rust: functions, loops, slices
- `HashMap` usage (covered in Section 3)
- The `continue` keyword for skipping loop iterations

## 3. Concept: Checking Direct Connections

### Explanation

Given two nodes and a list of undirected edges, check if they are directly connected. An edge is undirected, meaning `(a, b)` connects both `a` to `b` and `b` to `a`.

In Python:

```python
def connected_nodes(node_a, node_b, edges):
    for left, right in edges:
        if (left == node_a and right == node_b) or (left == node_b and right == node_a):
            return True
    return False
```

In Rust, you iterate the slice with pattern matching:

```rust
pub fn connected_nodes(node_a: i32, node_b: i32, edges: &[(i32, i32)]) -> bool {
    for (left, right) in edges {
        if (*left == node_a && *right == node_b) || (*left == node_b && *right == node_a) {
            return true;
        }
    }
    false
}
```

The function checks both orderings because edges are undirected.

## 4. Concept: Single Node Connectivity

### Explanation

Check whether a specific node is connected to all other nodes in the graph. A `memory: HashMap<i32, i32>` cache stores already-verified connections to avoid rechecking.

In Python:

```python
def fully_connected_node(node_index, nodes, edges, memory):
    center = nodes[node_index]
    for node in nodes:
        if node == center:
            continue
        if node in memory:
            continue
        if not connected_nodes(center, node, edges):
            return False
        memory[center] = node
        memory[node] = center
    return True
```

In Rust:

```rust
pub fn fully_connected_node(
    node_index: usize,
    nodes: &[i32],
    edges: &[(i32, i32)],
    memory: &mut HashMap<i32, i32>,
) -> bool
```

The function:
1. Gets the center node at `node_index`
2. Iterates all other nodes
3. Skips if already in cache (`memory.contains_key(node)`)
4. Returns `false` the moment any connection is missing
5. On success, caches both directions

### Applying to Our Project

If node 1 is connected to nodes 2, 3, and 4, then `fully_connected_node(0, &[1,2,3,4], &[(1,2),(1,3),(1,4)], &mut memory)` returns `true`. When checking node 2 later, the cache already knows about the `(1,2)` edge, saving one lookup.

## 5. Concept: Full Graph Connectivity

### Explanation

A graph is fully connected if every node is connected to every other node. This function iterates all nodes and uses `fully_connected_node` with a shared cache:

```rust
pub fn fully_connected_graph(nodes: &[i32], edges: &[(i32, i32)]) -> bool
```

In Python:

```python
def fully_connected_graph(nodes, edges):
    memory = {}
    for i in range(len(nodes)):
        if not fully_connected_node(i, nodes, edges, memory):
            return False
    return True
```

The shared `memory` cache means that once connection `(a, b)` is verified, it never needs to be checked again. This optimization matters for large graphs.

### Applying to Our Project

For three nodes `[1,2,3]` with edges connecting all pairs `[(1,2),(1,3),(2,3)]`, the function returns `true`. If even one edge is missing, it returns `false`.

## 6. Concept: Generating Complete Graphs

### Explanation

Generate all edges for a complete graph from a list of nodes. A complete graph on N nodes has N*(N-1)/2 edges.

In Python:

```python
def generate_fully_connected_edges(nodes):
    edges = []
    for i in range(len(nodes)):
        for j in range(i + 1, len(nodes)):
            edges.append((nodes[i], nodes[j]))
    return edges
```

In Rust:

```rust
pub fn generate_fully_connected_edges(nodes: &[i32]) -> Vec<(i32, i32)>
pub fn generate_nodes(nodes_quantity: i32) -> Vec<i32>
```

`generate_nodes` creates node IDs `0..nodes_quantity`. `generate_fully_connected_edges` generates all unique undirected pairs.

### Applying to Our Project

`generate_nodes(3)` produces `[0, 1, 2]`. `generate_fully_connected_edges(&[1, 2, 3])` produces `[(1,2), (1,3), (2,3)]`.

## 7. Putting It All Together

Open `workshop/src/lib.rs` and implement:

1. **`connected_nodes`** -- iterate edges, check both orderings
2. **`fully_connected_node`** -- iterate all nodes, skip self and cached, check connection with `connected_nodes`, update cache
3. **`fully_connected_graph`** -- iterate all indices, call `fully_connected_node` with a shared mutable `HashMap`
4. **`generate_fully_connected_edges`** -- double loop with `j = i + 1`
5. **`generate_nodes`** -- loop from 0 to `nodes_quantity - 1`

Run `cd workshop && cargo test` after each step. Groups: `step_01_connected_nodes` (2 tests), `step_02_fully_connected_node` (2 tests), `step_03_fully_connected_graph` (2 tests), `step_04_generators` (2 tests).

## 8. Summary

| Concept | Rust Equivalent | Python Equivalent | Used In |
|---|---|---|---|
| Edge existence check | Slice iteration with pattern matching | For loop with tuple unpacking | `connected_nodes` |
| Memoization cache | `&mut HashMap<i32, i32>` | Dict `memory` | `fully_connected_node` |
| Skip loop iteration | `continue` | `continue` | `fully_connected_node` |
| Full connectivity | Iterate all nodes, shared cache | For loop with shared dict | `fully_connected_graph` |
| Complete graph generation | Double loop `j = i + 1` | Nested `for` with `range` | `generate_fully_connected_edges` |
| Node ID generation | Loop with `push` | List comprehension `range` | `generate_nodes` |

## Exercises

* **Easy** – modify the existing function to handle an extra edge case.
* **Medium** – extend the project with a new helper function that re‑uses the core logic.

