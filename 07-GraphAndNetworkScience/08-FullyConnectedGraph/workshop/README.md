# Workshop: Fully Connected Graph

**Goal**: Study the implemented `src/lib.rs` and pass all 8 tests.

## Functions to Study

### `connected_nodes`
- **Signature**: `pub fn connected_nodes(node_a: i32, node_b: i32, edges: &[(i32, i32)]) -> bool`
- **Task**: Check if two nodes are directly connected by any edge. Already implemented.

### `fully_connected_node`
- **Signature**: `pub fn fully_connected_node(node_index: usize, nodes: &[i32], edges: &[(i32, i32)], memory: &mut HashMap<i32, i32>) -> bool`
- **Task**: Check if a specific node is connected to all other nodes. Already implemented.

### `fully_connected_graph`
- **Signature**: `pub fn fully_connected_graph(nodes: &[i32], edges: &[(i32, i32)]) -> bool`
- **Task**: Check if the entire graph is fully connected (every node to every other). Already implemented.

### `generate_fully_connected_edges`
- **Signature**: `pub fn generate_fully_connected_edges(nodes: &[i32]) -> Vec<(i32, i32)>`
- **Task**: Generate all possible edges for a fully connected graph. Already implemented.

### `generate_nodes`
- **Signature**: `pub fn generate_nodes(nodes_quantity: i32) -> Vec<i32>`
- **Task**: Generate a list of nodes numbered 0 to nodes_quantity-1. Already implemented.

## Test Modules

| Module | Tests | What It Tests |
|--------|-------|---------------|
| step_01_connected_nodes | 2 | Direct edge existence check |
| step_02_fully_connected_node | 2 | Node-to-all connectivity |
| step_03_fully_connected_graph | 2 | Full graph connectivity |
| step_04_generators | 2 | Node and edge generation |

## How to Run Tests
```bash
cargo test
```
