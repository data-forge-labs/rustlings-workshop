# Workshop: LinkedListFruitSalad — Doubly-Linked List

**Goal**: Implement all functions in `src/lib.rs` to pass all **4** tests.

## Functions to Implement

### `make_fruit_list`
- **Signature**: `pub fn make_fruit_list() -> LinkedList<&'static str>`
- **Task**: Return a LinkedList with 3 fruit items
- **Tests**: test_make_fruit_list

### `shuffle_to_vec`
- **Signature**: `pub fn shuffle_to_vec(list: LinkedList<&'static str>, rng: &mut impl rand::Rng) -> Vec<&'static str>`
- **Task**: Convert LinkedList to Vec and shuffle using `rng`
- **Tests**: test_shuffle_preserves_length

### `vec_to_linked_list`
- **Signature**: `pub fn vec_to_linked_list(vec: Vec<&'static str>) -> LinkedList<&'static str>`
- **Task**: Convert a Vec back into a LinkedList
- **Tests**: test_vec_to_linked_list, test_empty_roundtrip

## Test Modules

| Module | Tests | What It Tests |
|--------|-------|---------------|
| step_01_linked_list | 4 | LinkedList creation, conversion, empty case |

## How to Run Tests
```bash
cargo test
```
