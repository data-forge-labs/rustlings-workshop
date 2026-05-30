# Workshop: PageRank

**Goal**: Study the implemented `src/lib.rs` and pass all 22 tests.

## Functions to Study

### `build_outlinks`
- **Signature**: `pub fn build_outlinks(edges: &[(usize, usize)]) -> HashMap<usize, Vec<usize>>`
- **Task**: Build outlink map from directed edge list. Already implemented.

### `build_inlinks`
- **Signature**: `pub fn build_inlinks(edges: &[(usize, usize)]) -> HashMap<usize, Vec<usize>>`
- **Task**: Build inlink map from directed edge list. Already implemented.

### `page_rank`
- **Signature**: `pub fn page_rank(edges: &[(usize, usize)], node_count: usize, damping: f64, iterations: usize) -> HashMap<usize, f64>`
- **Task**: Compute PageRank scores iteratively with teleportation. Already implemented.

### `top_pages`
- **Signature**: `pub fn top_pages(scores: &HashMap<usize, f64>, n: usize) -> Vec<(usize, f64)>`
- **Task**: Return the top `n` pages by PageRank score. Already implemented.

### `score_delta`
- **Signature**: `pub fn score_delta(prev: &HashMap<usize, f64>, curr: &HashMap<usize, f64>) -> f64`
- **Task**: Compute sum of absolute differences between two score maps. Already implemented.

## Test Modules

| Module | Tests | What It Tests |
|--------|-------|---------------|
| step_01_graph_setup | 7 | Outlink and inlink map building |
| step_02_pagerank | 7 | PageRank computation with various parameters |
| step_03_analysis | 8 | Top pages and score delta convergence check |

## How to Run Tests
```bash
cargo test
```
