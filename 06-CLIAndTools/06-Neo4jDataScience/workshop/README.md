# Workshop: Neo4j Data Science

**Goal**: Implement all functions in `src/lib.rs` to pass all 14 tests.

## Functions to Implement

### `build_graph`
- **Signature**: `pub fn build_graph(edges: &[(usize, usize)]) -> Graph`
- **Task**: Build a `Graph` struct containing an undirected adjacency list.
- **Tests**: test_build_graph_simple, test_build_graph_empty, test_build_graph_disconnected

### `compute_degree_centrality`
- **Signature**: `pub fn compute_degree_centrality(graph: &Graph) -> HashMap<usize, f64>`
- **Task**: Compute degree centrality normalized by N-1.
- **Tests**: test_degree_fully_connected, test_degree_star, test_degree_single_node

### `compute_closeness_centrality`
- **Signature**: `pub fn compute_closeness_centrality(graph: &Graph) -> HashMap<usize, f64>`
- **Task**: Compute closeness centrality using BFS shortest paths.
- **Tests**: test_closeness_line, test_closeness_isolated

### `compute_betweenness_centrality`
- **Signature**: `pub fn compute_betweenness_centrality(graph: &Graph) -> HashMap<usize, f64>`
- **Task**: Compute betweenness centrality for each node.
- **Tests**: test_betweenness_line, test_betweenness_fully_connected

### `top_n_central`
- **Signature**: `pub fn top_n_central(scores: &HashMap<usize, f64>, n: usize) -> Vec<(usize, f64)>`
- **Task**: Return the top `n` nodes by centrality score, sorted descending.
- **Tests**: test_top_n_central_basic, test_top_n_central_larger_than_map

### `centrality_summary`
- **Signature**: `pub fn centrality_summary(scores: &HashMap<usize, f64>) -> (f64, f64, f64)`
- **Task**: Compute min, max, and mean of centrality scores.
- **Tests**: test_centrality_summary_basic, test_centrality_summary_single

## Structs

### `Graph`
- Field: `adjacency: HashMap<usize, Vec<usize>>`

## Test Modules

| Module | Tests | What It Tests |
|--------|-------|---------------|
| step_01_graph_creation | 3 | Graph building from edges |
| step_02_degree | 3 | Degree centrality |
| step_03_closeness | 2 | Closeness centrality |
| step_04_betweenness | 2 | Betweenness centrality |
| step_05_analysis | 4 | Top-N and summary statistics |

## How to Run Tests
```bash
cargo test
```
