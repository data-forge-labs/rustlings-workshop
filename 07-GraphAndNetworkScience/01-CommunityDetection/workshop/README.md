# Workshop: Community Detection

**Goal**: Study the implemented `src/lib.rs` and pass all 18 tests.

## Functions to Study

### `build_adjacency_list`
- **Signature**: `pub fn build_adjacency_list(edges: &[(usize, usize)]) -> HashMap<usize, Vec<usize>>`
- **Task**: Build a directed graph adjacency list from edge tuples. Already implemented.

### `dfs_order`
- **Signature**: `pub fn dfs_order(graph: &HashMap<usize, Vec<usize>>, start: usize) -> Vec<usize>`
- **Task**: Perform DFS traversal from `start`, returning visited node order. Already implemented.

### `reverse_graph`
- **Signature**: `pub fn reverse_graph(graph: &HashMap<usize, Vec<usize>>) -> HashMap<usize, Vec<usize>>`
- **Task**: Reverse all edge directions in the graph. Already implemented.

### `kosaraju_scc`
- **Signature**: `pub fn kosaraju_scc(edges: &[(usize, usize)], node_count: usize) -> Vec<Vec<usize>>`
- **Task**: Compute strongly connected components using Kosaraju's algorithm. Already implemented.

### `count_distinct_users`
- **Signature**: `pub fn count_distinct_users() -> usize`
- **Task**: Count distinct Twitter usernames from the embedded dataset. Already implemented.

### `top_n_users`
- **Signature**: `pub fn top_n_users(n: usize) -> Vec<(&'static str, usize)>`
- **Task**: Return the top `n` most frequent Twitter usernames. Already implemented.

## Test Modules

| Module | Tests | What It Tests |
|--------|-------|---------------|
| step_01_graph_ops | 9 | Build adjacency, DFS, reverse graph |
| step_02_kosaraju | 5 | Kosaraju SCC on various graph structures |
| step_03_twitter_data | 4 | Distinct user count and top-N frequency |

## How to Run Tests
```bash
cargo test
```

## Exercises

* **Easy** – modify the existing function to handle an extra edge case.
* **Medium** – extend the project with a new helper function that re‑uses the core logic.

