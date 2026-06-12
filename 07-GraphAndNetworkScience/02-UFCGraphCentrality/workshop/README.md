# Workshop: UFC Graph Centrality

**Goal**: Study the implemented `src/lib.rs` and pass all 13 tests.

## Functions to Study

### `build_adjacency`
- **Signature**: `pub fn build_adjacency(edges: &[(usize, usize)]) -> HashMap<usize, Vec<usize>>`
- **Task**: Build an undirected adjacency list from edge tuples. Already implemented.

### `degree_centrality`
- **Signature**: `pub fn degree_centrality(adj: &HashMap<usize, Vec<usize>>) -> HashMap<usize, f64>`
- **Task**: Compute degree centrality normalized by N-1. Already implemented.

### `closeness_centrality`
- **Signature**: `pub fn closeness_centrality(adj: &HashMap<usize, Vec<usize>>) -> HashMap<usize, f64>`
- **Task**: Compute closeness centrality using BFS shortest-path distances. Already implemented.

### `most_central_node`
- **Signature**: `pub fn most_central_node(centrality: &HashMap<usize, f64>) -> Option<usize>`
- **Task**: Return the node with the highest centrality score. Already implemented.

### `format_centrality`
- **Signature**: `pub fn format_centrality(scores: &HashMap<usize, f64>) -> Vec<String>`
- **Task**: Format centrality scores as `"node: score"` strings sorted by node ID. Already implemented.

## Test Modules

| Module | Tests | What It Tests |
|--------|-------|---------------|
| step_01_graph_building | 3 | Adjacency list building |
| step_02_degree_centrality | 3 | Degree centrality calculation |
| step_03_closeness_centrality | 3 | Closeness centrality (BFS-based) |
| step_04_analysis | 4 | Most central node and formatting |

## How to Run Tests
```bash
cargo test
```

## Exercises

* **Easy** – modify the existing function to handle an extra edge case.
* **Medium** – extend the project with a new helper function that re‑uses the core logic.

