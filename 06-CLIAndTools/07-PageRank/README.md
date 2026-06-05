# Project 26: PageRank Algorithm -- Iterative Ranking with Damping Factor

> **Test-driven approach**: This project includes a Cargo project with progressive
> unit tests. Each function in `workshop/src/lib.rs` starts as a `todo!()` stub. As you
> follow each section, replace `todo!()` with real code and run `cd workshop && cargo test` to
> watch the pass count grow. Your goal: **all 18 tests pass**.

## Why Run PageRank Without `networkx`?

**Python pain:** `nx.pagerank` is a one-liner, but behind it `networkx` wraps every node and edge in Python objects, and the power method runs as pure Python loops. On a 100K-node web crawl that's *minutes*:

```
networkx PageRank (100K nodes, 50 iters):  ~3-5 minutes
Rust PageRank (100K nodes, 50 iters):      ~0.2 seconds
```

**Rust fix:** Power method directly on flat `HashMap` structures — iterating at machine speed:

```rust
pub fn page_rank(edges: &[(usize, usize)], node_count: usize,
                 damping: f64, iterations: usize) -> HashMap<usize, f64> {
    let outlinks = build_outlinks(edges);
    let inlinks = build_inlinks(edges);
    let mut ranks = HashMap::new();
    for i in 0..node_count { ranks.insert(i, 1.0 / node_count as f64); }
    for _ in 0..iterations {
        for node in 0..node_count {
            let sum: f64 = inlinks.get(&node).map_or(0.0, |inlinks| {
                inlinks.iter().map(|u| ranks[u] / outlinks.get(u).map_or(1.0, |v| v.len() as f64)).sum()
            });
            ranks.insert(node, (1.0 - damping) / node_count as f64 + damping * sum);
        }
    }
    ranks
}
```

## At a Glance

| # | Concept | Rust | Python | Why it matters |
|---|---------|------|--------|----------------|
| 1 | Outlinks map | `HashMap<usize, Vec<usize>>` from `(from, to)` | `defaultdict(list)` | Map each node to outgoing edges |
| 2 | Inlinks map | `HashMap<usize, Vec<usize>>` from `(to, from)` | `defaultdict(list)` | Map each node to incoming edges |
| 3 | Iterative PageRank | power method + damping | `nx.pagerank` | Compute rank scores iteratively |
| 4 | Damping factor | `damping: f64` (default 0.85) | `alpha` parameter | Control random-jump probability |
| 5 | Top-N ranking | collect, sort desc, truncate | `sorted(..., key=-x[1])` | Find highest-ranked pages |
| 6 | Convergence checking | sum of absolute deltas | `sum(abs(curr[k]-v) ...)` | Detect when ranks stabilize |

---

---

## Table of Contents

1. [Introduction](#1-introduction)
2. [Prerequisites](#2-prerequisites)
3. [Concept: Outlinks and Inlinks](#3-concept-outlinks-and-inlinks)
4. [Concept: Iterative PageRank Computation](#4-concept-iterative-pagerank-computation)
5. [Concept: Top Pages and Convergence](#5-concept-top-pages-and-convergence)
6. [Putting It All Together](#6-putting-it-all-together)
7. [Summary](#7-summary)

## 1. Introduction

PageRank is the algorithm that powered Google's early search engine. It ranks nodes in a directed graph by simulating a random surfer who follows links with probability `damping` and jumps to a random page with probability `1 - damping`. The importance of a page is determined by how many important pages link to it.

This algorithm is widely used in data engineering for link analysis, recommendation systems, and influence scoring.

In Python, you would use `networkx.pagerank(G, alpha=0.85)`. In Rust, you implement the iterative power method from scratch using `HashMap`.

## 2. Prerequisites

- `HashMap` and iterators
- Understanding of directed graphs (outlinks vs inlinks)
- Basic floating-point arithmetic

## 3. Concept: Outlinks and Inlinks

### Explanation

A directed edge `(from, to)` means "from links to to". Outlinks are edges leaving a node; inlinks are edges entering a node.

In Python:

```python
outlinks = defaultdict(list)
inlinks = defaultdict(list)
for f, t in edges:
    outlinks[f].append(t)
    inlinks[t].append(f)
```

In Rust:

```rust
pub fn build_outlinks(edges: &[(usize, usize)]) -> HashMap<usize, Vec<usize>>
pub fn build_inlinks(edges: &[(usize, usize)]) -> HashMap<usize, Vec<usize>>
```

- `build_outlinks`: for each `(from, to)`, push `to` into `from`'s list
- `build_inlinks`: for each `(from, to)`, push `from` into `to`'s list

A "dangling node" (a node with no outlinks) simply won't appear as a key in the outlinks map.

## 4. Concept: Iterative PageRank Computation

### Explanation

The PageRank formula for each node:

```
PR(v) = (1 - d) / N + d * sum(PR(u) / out_degree(u) for u in inlinks[v])
```

where `d` is the damping factor (typically 0.85) and `N` is the total number of nodes.

In Python with networkx:

```python
import networkx as nx
G = nx.DiGraph([(0,1), (1,2), (2,0)])
ranks = nx.pagerank(G, alpha=0.85)
```

The Rust implementation iteratively updates ranks:

```rust
pub fn page_rank(edges: &[(usize, usize)], node_count: usize, damping: f64, iterations: usize) -> HashMap<usize, f64>
```

Steps:
1. Initialize all ranks to `1.0 / node_count`
2. For each iteration:
   - For each node, compute `rank_sum = sum of ranks[inlink] / out_degree(inlink)`
   - New rank = `(1.0 - damping) / node_count + damping * rank_sum`
3. Return the final ranks

Key points:
- The sum of all ranks always equals 1.0 (they form a probability distribution)
- With `damping = 1.0`, a node with no outlinks absorbs all rank; with `damping = 0.0`, all nodes have equal rank

### Applying to Our Project

For a two-node mutual link graph `(0->1, 1->0)`, both nodes converge to rank 0.5. For a single node with no edges, rank remains 1.0.

## 5. Concept: Top Pages and Convergence

### Explanation

Two analysis functions:

- **`top_pages`**: returns the N highest-ranked pages:

```rust
pub fn top_pages(scores: &HashMap<usize, f64>, n: usize) -> Vec<(usize, f64)>
```

In Python: `sorted(scores.items(), key=lambda x: -x[1])[:n]`

- **`score_delta`**: computes the sum of absolute changes between two iterations, useful for convergence checking:

```rust
pub fn score_delta(prev: &HashMap<usize, f64>, curr: &HashMap<usize, f64>) -> f64
```

In Python:

```python
def score_delta(prev, curr):
    return sum(abs(curr.get(k, 0) - v) for k, v in prev.items())
```

### Applying to Our Project

`score_delta` helps determine when ranks have stabilized. A delta below a threshold (e.g., 1e-6) means the algorithm has converged.

## 6. Putting It All Together

Open `workshop/src/lib.rs` and implement:

1. **`build_outlinks`** -- for each `(from, to)`, push `to` to `from`'s entry
2. **`build_inlinks`** -- for each `(from, to)`, push `from` to `to`'s entry
3. **`page_rank`** -- initialize ranks to `1/N`, iterate updating each node's rank using the formula with damping
4. **`top_pages`** -- collect into `Vec<(usize, f64)>`, sort by score descending, truncate to `n`
5. **`score_delta`** -- iterate `prev`, sum `abs(curr.get(k, 0) - v)`

Run `cd workshop && cargo test` after each step. Groups: `step_01_graph_setup` (6 tests), `step_02_pagerank` (6 tests), `step_03_analysis` (6 tests).

## 7. Summary

| Concept | Rust Equivalent | Python Equivalent | Used In |
|---|---|---|---|
| Outlinks map | `HashMap<usize, Vec<usize>>` from `(from, to)` | `defaultdict(list)` | `build_outlinks` |
| Inlinks map | `HashMap<usize, Vec<usize>>` from `(to, from)` | `defaultdict(list)` | `build_inlinks` |
| PageRank iteration | `(1-d)/N + d * sum(rank/in_degree)` | `nx.pagerank` | `page_rank` |
| Damping factor | Parameter `damping: f64` (default 0.85) | `alpha` parameter | `page_rank` |
| Top-N ranking | Collect, sort desc, truncate | `sorted(dict.items(), key=-x[1])` | `top_pages` |
| Convergence check | Sum of absolute deltas | `sum(abs(curr[k]-v) for k,v in prev.items())` | `score_delta` |
