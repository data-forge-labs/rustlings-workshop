# Workshop: Lisbon Shortest Path

**Goal**: Study the implemented `src/lib.rs` and pass all 14 tests.

## Functions to Study

### `build_weighted_adjacency`
- **Signature**: `pub fn build_weighted_adjacency(edges: &[WeightedEdge]) -> HashMap<usize, Vec<(usize, f64)>>`
- **Task**: Build an undirected weighted adjacency list. Already implemented.

### `dijkstra`
- **Signature**: `pub fn dijkstra(adj: &HashMap<usize, Vec<(usize, f64)>>, start: usize) -> HashMap<usize, f64>`
- **Task**: Compute shortest-path distances from `start` using Dijkstra's algorithm. Already implemented.

### `shortest_path`
- **Signature**: `pub fn shortest_path(adj: &HashMap<usize, Vec<(usize, f64)>>, start: usize, end: usize) -> Option<Vec<usize>>`
- **Task**: Reconstruct the shortest path between two nodes. Already implemented.

### `path_weight`
- **Signature**: `pub fn path_weight(edges: &[WeightedEdge], path: &[usize]) -> Option<f64>`
- **Task**: Compute the total weight of a given path through the graph. Already implemented.

### `format_path`
- **Signature**: `pub fn format_path(path: &[usize]) -> String`
- **Task**: Format a path as `"0 -> 1 -> 2"`. Already implemented.

## Structs

### `WeightedEdge`
- Fields: `from: usize`, `to: usize`, `weight: f64`

## Test Modules

| Module | Tests | What It Tests |
|--------|-------|---------------|
| step_01_graph_setup | 3 | Weighted adjacency list building |
| step_02_dijkstra | 3 | Dijkstra shortest-path distances |
| step_03_path_reconstruction | 5 | Path reconstruction, weight, no-path case |
| step_04_formatting | 3 | Path-to-string formatting |

## How to Run Tests
```bash
cargo test
```
