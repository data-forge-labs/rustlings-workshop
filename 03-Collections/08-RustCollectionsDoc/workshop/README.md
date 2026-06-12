# Workshop: RustCollectionsDoc — Word Counter & PriorityQueue

**Goal**: Implement all functions in `src/lib.rs` to pass all **8** tests.

## Functions to Implement

### `word_counter`
- **Signature**: `pub fn word_counter(text: &str) -> HashMap<String, u32>`
- **Task**: Count word frequencies in a text (split by whitespace, case-sensitive)
- **Tests**: step_01_word_counter (3 tests: empty, basic, case_sensitive)

### `Item` struct with Ord
- **Fields**: `priority: u32`, `value: String`
- **Task**: Implement `Ord` so higher-priority items come first (reverse priority)
- **Tests**: step_02_priority_queue (test_item_ord, test_item_eq)

### `PriorityQueue`
- **Fields**: `items: BinaryHeap<Item>`
- **Methods**: `new()`, `push(item)`, `pop() -> Option<Item>`, `len() -> usize`
- **Task**: Wrap BinaryHeap with proper ordering
- **Tests**: step_02_priority_queue (test_priority_queue_new, test_priority_queue_push_pop)

## Test Modules

| Module | Tests | What It Tests |
|--------|-------|---------------|
| step_01_word_counter | 3 | HashMap word counting |
| step_02_priority_queue | 5 | BinaryHeap, Ord impl, push/pop order |

## How to Run Tests
```bash
cargo test
```

## Exercises

* **Easy** – modify the existing function to handle an extra edge case.
* **Medium** – extend the project with a new helper function that re‑uses the core logic.

