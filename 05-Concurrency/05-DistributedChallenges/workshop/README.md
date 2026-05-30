# Workshop: Distributed Challenges

**Goal**: Implement all functions in `src/lib.rs` to pass all 17 tests.

## Functions to Implement

### `validate_cap_pair`
- **Signature**: `pub fn validate_cap_pair(consistency: bool, availability: bool, partition_tolerance: bool) -> &'static str`
- **Task**: Return "CP", "AP", or "CA" for valid CAP combinations, or "Invalid".
- **Tests**: test_cp_valid, test_ap_valid, test_ca_no_partition, test_all_three_invalid, test_none_invalid

### `simulate_eventual_consistency`
- **Signature**: `pub fn simulate_eventual_consistency(writes: Vec<&str>) -> Vec<&str>`
- **Task**: Deduplicate writes while preserving first-occurrence order (eventual convergence).
- **Tests**: test_eventual_consistency_dedup, test_eventual_consistency_empty, test_eventual_consistency_no_duplicates

### `merge_crdt_values`
- **Signature**: `pub fn merge_crdt_values(local: Vec<&str>, remote: Vec<&str>) -> Vec<&str>`
- **Task**: Merge two value sets as a CRDT (union without duplicates).
- **Tests**: test_crdt_merge_no_overlap, test_crdt_merge_with_overlap, test_crdt_merge_empty_local

### `simulate_leader_election`
- **Signature**: `pub fn simulate_leader_election(node_count: usize) -> usize`
- **Task**: Return the elected leader ID (highest-numbered node, or 0 for empty).
- **Tests**: test_leader_election_single_node, test_leader_election_multiple_nodes, test_leader_election_empty

### `simulate_quorum_read`
- **Signature**: `pub fn simulate_quorum_read(writes: Vec<usize>, quorum_size: usize) -> Option<usize>`
- **Task**: Find the first value that appears at least `quorum_size` times, or `None`.
- **Tests**: test_quorum_read_reaches_quorum, test_quorum_read_no_quorum, test_quorum_read_empty_writes

## Test Modules

| Module | Tests | What It Tests |
|--------|-------|---------------|
| step_01_cap_theorem | 5 | CAP theorem validation |
| step_02_consistency | 6 | Eventual consistency and CRDT merge |
| step_03_distributed_patterns | 6 | Leader election and quorum reads |

## How to Run Tests
```bash
cargo test
```
