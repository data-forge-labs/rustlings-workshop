# Section 6: CLI & Data Engineering Tools

*Building production-ready CLI tools, graph analytics, and connecting Rust to Python data tools.*

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

## Learning Path

1. Start with **14-CLISalad** for CLI argument parsing basics
2. Explore graph algorithms with **20-CommunityDetection** through **22-GraphVisualize**
3. Build **24-LisbonShortestPath** and **26-PageRank** for classic algorithms
4. Integrate with external databases via **25-Neo4jDataScience** and **27-RussianTrollTweets**
5. Finish with **33-CustomCLIFruitSalad** (advanced CLI) and **29-DataStructuresLessonReflection**

## Key Concepts Covered

| Rust Concept | Python Equivalent | Why It Matters |
|---|---|---|
| `clap` crate | `argparse` / `click` | CLI argument parsing |
| `petgraph` crate | `networkx` | Graph data structures |
| Dijkstra | `networkx.shortest_path` | Shortest path algorithm |
| PageRank | `networkx.pagerank` | Link analysis |
| `BinaryHeap` | `heapq` | Priority queue for Dijkstra |
| `neo4j` driver | `neo4j` Python driver | Graph database integration |
