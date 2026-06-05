# Section 6: CLI & Data Engineering Tools

*Building production-ready CLI tools, graph analytics, and connecting Rust to Python data tools.*

---

## Why This Section?

### The Problem — Slow Python CLIs & Graph Tools

Python's CLI tools for data processing share a common fate:

```python
# python analyze_network.py --input graph.csv --algorithm pagerank
import pandas as pd
import networkx as nx
import sys
import argparse

# Parse args
parser = argparse.ArgumentParser()
parser.add_argument("--input", required=True)
parser.add_argument("--algorithm", default="centrality")
args = parser.parse_args()

# Load graph
df = pd.read_csv(args.input)
G = nx.from_pandas_edgelist(df, "source", "target")

# Compute — this will be SLOW for large graphs
if args.algorithm == "pagerank":
    pr = nx.pagerank(G)  # minutes for 1M nodes
```

**The problems with Python for CLI data tools:**

```
┌─────────────────────────────────────────────────────┐
│  Python CLI Tool Problems                            │
│                                                      │
│  1. Startup time: 300-500ms (import tax)             │
│     ├─ import pandas     → 200ms                     │
│     ├─ import networkx   → 150ms                     │
│     └─ import argparse   → 50ms                      │
│                                                      │
│  2. Memory: graph nodes are PyObjects                │
│     ├─ 1M nodes → ~300 MB (Python)                   │
│     └─ 1M nodes → ~32 MB (Rust/petgraph)            │
│                                                      │
│  3. Speed: pure Python loops are 50-100x slower      │
│     ├─ PageRank in Python → 45 seconds               │
│     └─ PageRank in Rust  → < 1 second                │
│                                                      │
│  4. Distribution: "pip install" hell                 │
│     └─ Rust compiles to a single binary              │
└─────────────────────────────────────────────────────┘
```

### The Rust Solution — Instant Startup, Blazing Algorithms

```rust
use clap::Parser;
use petgraph::graph::Graph;
use petgraph::algo::dijkstra;

#[derive(Parser)]
struct Cli {
    #[arg(long)]
    input: String,
    #[arg(long, default_value = "centrality")]
    algorithm: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    let graph: Graph<(), f64> = load_graph(&cli.input)?;
    // Computing — milliseconds, not minutes
    process_algorithm(&graph, &cli.algorithm);
    Ok(())
}
```

Rust compiles to a **single binary** with near-zero startup time — ideal for CLI tools that run frequently in data pipelines.

---

## What You'll Learn

| # | Concept | Rust Type / Module | Python Equivalent | Purpose |
|---|---------|--------------------|------------------|---------|
| 1 | CLI argument parsing | `clap` crate | `argparse`, `click` | Declarative, typed CLI args |
| 2 | Graph data structures | `petgraph::Graph` | `networkx.Graph` | Directed & undirected graphs |
| 3 | Graph algorithms | `petgraph::algo` | `networkx.algorithms` | Dijkstra, DFS, BFS, etc. |
| 4 | Strongly connected components | Kosaraju's algorithm | `networkx.kosaraju` | Community detection |
| 5 | Centrality | Degree, closeness, betweenness | `networkx.centrality` | Node importance |
| 6 | Shortest path | Dijkstra | `networkx.shortest_path` | Routing & optimization |
| 7 | PageRank | Iterative power method | `networkx.pagerank` | Link analysis / ranking |
| 8 | Priority queue | `BinaryHeap<T>` | `heapq` | Dijkstra's core data structure |
| 9 | Graph DB integration | Neo4j driver | `neo4j` Python driver | Graph database queries |
| 10 | ASCII visualization | `rasciigraph` crate | `matplotlib` (overkill) | Quick terminal charts |
| 11 | Package layout | `lib.rs` + `main.rs` | Package/module | Separate logic from entry point |
| 12 | Library re-exports | `pub use` | `__init__.py` | Control public API surface |

---

## Concepts at a Glance

### 1. `clap` — CLI Argument Parsing

```rust
use clap::Parser;

#[derive(Parser)]
#[command(name = "graph-tool")]
struct Args {
    /// Input file path
    #[arg(short, long)]
    input: String,

    /// Algorithm to run
    #[arg(short, long, default_value = "pagerank")]
    algorithm: String,
}

fn main() {
    let args = Args::parse();
    println!("Input: {}", args.input);
}
```

In Python: `argparse.ArgumentParser()` or `@click.command()`

### 2. `petgraph` — Graph Data Structures

```rust
use petgraph::graph::{Graph, NodeIndex};
use petgraph::Undirected;

// Create a graph
let mut graph: Graph<&str, f64, Undirected> = Graph::new_undirected();
let a = graph.add_node("Alice");
let b = graph.add_node("Bob");
graph.add_edge(a, b, 1.5);  // weight = 1.5
```

In Python: `networkx.Graph()`

### 3. Dijkstra's Algorithm

```rust
use petgraph::algo::dijkstra;

let distances = dijkstra(&graph, start_node, None, |e| *e.weight());
// Returns HashMap<NodeIndex, f64> — shortest distances from start
```

In Python: `networkx.single_source_dijkstra_path_length(G, source)`

### 4. PageRank

```rust
// Iterative PageRank implementation
let mut ranks = HashMap::new();
for node in graph.node_indices() {
    ranks.insert(node, 1.0 / n as f64);
}
for _ in 0..max_iter {
    // ... iterative power method
}
```

In Python: `networkx.pagerank(G)`

### 5. `BinaryHeap` — Priority Queue

Used as the core data structure for Dijkstra:

```rust
use std::collections::BinaryHeap;

let mut heap = BinaryHeap::new();
heap.push(std::cmp::Reverse((0, start_node)));  // min-heap via Reverse
```

In Python: `heapq.heappush(heap, (distance, node))`

### 6. Graph DB Integration (Neo4j)

Connect Rust to Neo4j for production graph analytics — like `neo4j` Python driver.

### 7. `lib.rs` + `main.rs` — Separation of Concerns

```
src/
├── lib.rs    ← public API, all logic
└── main.rs   ← CLI entry point, calls lib.rs
```

This pattern lets you **test the library** without the CLI and reuse the library in other projects.

---

## Prerequisites

- Completed [Section 3: Collections](../03-Collections/README.md)
- Understand `HashMap`, iterators, and basic error handling

## Projects in This Section

| # | Project | Concepts | Format |
|---|---------|----------|--------|
| 14 | **CLISalad** — CLI with clap arg parsing | `clap` derive, `std::env`, pattern matching, `std::io` | Project |
| 20 | **CommunityDetection** — Kosaraju's SCC algorithm | `petgraph`, directed graphs, SCC, DFS, graph transposition | Project |
| 21 | **UFCGraphCentrality** — centrality on UFC data | `UnGraph`, degree/closeness centrality, `NodeIndex` | Project |
| 22 | **GraphVisualize** — ASCII bar charts | `rasciigraph`, ASCII visualization, data scaling | Project |
| 24 | **LisbonShortestPath** — Dijkstra's algorithm | Dijkstra, weighted graphs, `BinaryHeap` as priority queue | Project |
| 25 | **Neo4jDataScience** — Neo4j graph DB | Neo4j integration, centrality algorithms | Project |
| 26 | **PageRank** — PageRank algorithm | PageRank, iterative ranking, damping factor, link analysis | Project |
| 27 | **RussianTrollTweets** — Neo4j analysis | Graph DB analysis, influence detection, social graph modeling | Project |
| 29 | **DataStructuresLessonReflection** — graph DS reflection | Graph vs other DS, centrality metrics, community detection | Reflection |
| 31 | **FullyConnectedGraph** — graph connectivity | Graph connectivity, `HashMap` memoization | Project |
| 33 | **CustomCLIFruitSalad** — advanced CLI + CSV | `clap` derive, CSV reading, `lib.rs`/`main.rs` separation, modules | Project |
| 60 | **RatatuiTUI** — terminal dashboard | `ratatui`, `crossterm`, `TestBackend`, immediate-mode UI, layouts, widgets, event loop | Project |
| 61 | **AsyncClap** — async CLI with subcommands | `clap` derive, `#[tokio::main]`, `ExitCode`, subcommand trees, JSON config | Project |

## Learning Path

1. Start with **01-CLISalad** for CLI argument parsing basics
2. Explore graph algorithms with **02-CommunityDetection** through **04-GraphVisualize**
3. Build **05-LisbonShortestPath** and **07-PageRank** for classic algorithms
4. Integrate with external databases via **06-Neo4jDataScience** and **08-RussianTrollTweets**
5. Finish with **10-CustomCLIFruitSalad** (advanced CLI)
6. **11-RatatuiTUI** — terminal dashboards for monitoring pipelines
7. **12-AsyncClap** — async subcommand CLIs for ETL tools
