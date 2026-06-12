# Section 7: Graph & Network Science — `petgraph`, PageRank, Neo4j

*From a single 6-node shortest-path demo to 1M-node PageRank on real social graphs. Rust + `petgraph` + Neo4j for production data-engineering analytics.*

---

## Why This Section?

### The Problem — Python Graph Tooling is Slow at Scale

A Python data engineer reaching for graph analytics hits the wall fast:

```python
# python — every layer adds latency and memory
import networkx as nx
import pandas as pd
G = nx.from_pandas_edgelist(df, "source", "target")
pr = nx.pagerank(G)            # 1M nodes → 45 seconds + 1.5 GB RAM
sp = nx.single_source_dijkstra_path_length(G, source)  # another 30s
```

**The Python tax on graph work:**

```
┌─────────────────────────────────────────────────────┐
│  Python Graph Tool Problems                          │
│                                                      │
│  1. Memory: each node/edge is a PyObject             │
│     ├─ 1M nodes, 5M edges → ~1.5 GB heap            │
│     └─ Same graph in Rust/petgraph → ~150 MB         │
│                                                      │
│  2. Speed: pure-Python loops are 30-100x slower      │
│     ├─ PageRank on 1M nodes → 45s (Python)           │
│     └─ PageRank on 1M nodes → <1s (Rust)             │
│                                                      │
│  3. Startup: importing networkx + pandas = 1s+       │
│     └─ One-shot CLI invocations pay this every time  │
│                                                      │
│  4. Distribution: no single-binary deploy            │
│     └─ Rust compiles to one statically linked binary │
└─────────────────────────────────────────────────────┘
```

### The Rust Solution — Native-Speed Graph Analytics

```rust
use petgraph::graph::UnGraph;
use petgraph::algo::{dijkstra, kosaraju_scc};

let graph: UnGraph<&str, f64> = build_from_csv("data/edges.csv")?;
let distances = dijkstra(&graph, start, None, |e| *e.weight());
let sccs = kosaraju_scc(&graph);  // community detection
let pr = pagerank(&graph, 0.85, 100);  // <1s for 1M nodes
```

Same `petgraph` types can be persisted to a Neo4j graph database, visualized as ASCII bar charts, or partitioned into strongly connected components — all in a single Rust binary.

---

## Concepts at a Glance

### 1. `petgraph` — Directed and Undirected Graphs

```rust
use petgraph::graph::{Graph, UnGraph};
use petgraph::Undirected;

let mut g: UnGraph<&str, f64> = UnGraph::new_undirected();
let a = g.add_node("Alice");
let b = g.add_node("Bob");
let c = g.add_node("Carol");
g.add_edge(a, b, 1.0);
g.add_edge(b, c, 2.5);
g.add_edge(a, c, 4.0);
```

In Python: `G = nx.Graph(); G.add_edge("Alice", "Bob", weight=1.0)` — same idea, slower, heavier.

### 2. Strongly Connected Components (Kosaraju)

```rust
use petgraph::algo::kosaraju_scc;

let sccs: Vec<Vec<_>> = kosaraju_scc(&graph);
for (i, component) in sccs.iter().enumerate() {
    println!("Community {}: {} nodes", i, component.len());
}
```

Kosaraju = two DFS passes (forward + transposed). Rust's DFS is iterative and zero-allocation; the same code in Python recurses and boxes everything.

### 3. Dijkstra's Shortest Path

```rust
use petgraph::algo::dijkstra;
use std::collections::HashMap;

let distances: HashMap<NodeIndex, f64> =
    dijkstra(&graph, start_node, None, |e| *e.weight());
```

Internally Dijkstra uses a `BinaryHeap` (max-heap) wrapped in `Reverse` to act as a min-heap — the same trick as Python's `heapq`.

### 4. PageRank (Iterative Power Method)

```rust
fn pagerank(g: &Graph, damping: f64, iters: usize) -> HashMap<NodeIndex, f64> {
    let n = g.node_count() as f64;
    let mut ranks: HashMap<_, f64> = g.node_indices().map(|i| (i, 1.0 / n)).collect();
    for _ in 0..iters {
        let mut next = HashMap::new();
        for node in g.node_indices() {
            let mut sum = 0.0;
            for neighbor in g.neighbors(node) {
                let out_degree = g.neighbors(neighbor).count() as f64;
                if out_degree > 0.0 {
                    sum += ranks[&neighbor] / out_degree;
                }
            }
            next.insert(node, (1.0 - damping) / n + damping * sum);
        }
        ranks = next;
    }
    ranks
}
```

In Python: `nx.pagerank(G, alpha=0.85)` — works, but for 1M nodes you'll feel it.

### 5. Neo4j Integration

```rust
// Connect Rust to Neo4j for production graph analytics — like the `neo4j` Python driver
let graph = neo4j::Graph::new("bolt://localhost:7687", "neo4j", "password").await?;
let result = graph.execute("MATCH (n:Person) RETURN n.name LIMIT 10").await?;
```

### 6. ASCII Graph Visualization

```rust
use rasciigraph::{plot, Config};

let data = vec![("PageRank-A", 0.42), ("PageRank-B", 0.31), ("PageRank-C", 0.18)];
println!("{}", plot(data, Config::default().with_height(5)));
```

### 7. HashMap Memoization for Connectivity

```rust
fn count_connected(g: &Graph, start: NodeIndex, seen: &mut HashSet<NodeIndex>) -> usize {
    if !seen.insert(start) { return 0; }
    let mut count = 1;
    for n in g.neighbors(start) {
        count += count_connected(g, n, seen);
    }
    count
}
```

---

## Prerequisites

- Completed [Section 5: Concurrency](../../../../../05-Concurrency/README.md) — some projects use parallel iteration (Rayon)
- Comfortable with `HashMap`, `HashSet`, iterators, closures
- For Neo4j projects: Docker installed (we run Neo4j in a container)

## Projects in This Section

| # | Project | Concepts | Format |
|---|---------|----------|--------|
| 01 | **CommunityDetection** — Kosaraju's SCC algorithm | `petgraph`, directed graphs, SCC, DFS, graph transposition | Workshop |
| 02 | **UFCGraphCentrality** — centrality on UFC data | `UnGraph`, degree/closeness centrality, `NodeIndex` | Workshop |
| 03 | **GraphVisualize** — ASCII bar charts | `rasciigraph`, ASCII visualization, data scaling | Workshop |
| 04 | **LisbonShortestPath** — Dijkstra's algorithm | Dijkstra, weighted graphs, `BinaryHeap` as priority queue | Workshop |
| 05 | **Neo4jDataScience** — Neo4j graph DB | Neo4j integration, centrality algorithms (degree, closeness, betweenness, eigenvector) | Workshop |
| 06 | **PageRank** — PageRank algorithm | PageRank, iterative ranking, damping factor, link analysis | Workshop |
| 07 | **RussianTrollTweets** — Neo4j analysis | Graph DB analysis, influence detection, social graph modeling | Workshop |
| 08 | **FullyConnectedGraph** — graph connectivity | Graph connectivity, `HashMap`/`HashSet` memoization | Workshop |

## Learning Path

1. **01-CommunityDetection** — get fluent in `petgraph` basics: build a graph, traverse it, partition it
2. **02-UFCGraphCentrality** — apply centrality to a real dataset (UFC fighter records)
3. **03-GraphVisualize** — turn graph data into something you can see in the terminal
4. **04-LisbonShortestPath** — Dijkstra + `BinaryHeap`, the workhorse of routing
5. **05-Neo4jDataScience** — when your graph outgrows RAM, store it in Neo4j
6. **06-PageRank** — the algorithm that made Google
7. **07-RussianTrollTweets** — end-to-end social-graph analysis on a real dataset
8. **08-FullyConnectedGraph** — connectivity, components, and why this matters for resilience

---

## How This Section Fits in the Course

- **Builds on**: Section 3 (Collections, especially `HashMap`/`HashSet` for memoization), Section 5 (Rayon for parallel graph algorithms)
- **Sets up for**: Section 9 (Observability — sometimes you monitor a graph pipeline), Section 10 (production services that expose graph queries via REST)

For terminal applications (CLIs, TUIs), revisit [Section 6: Terminal Apps](../../../../../06-TerminalApps/README.md).

## Exercises

* **Easy** – modify the existing function to handle an extra edge case.
* **Medium** – extend the project with a new helper function that re‑uses the core logic.

