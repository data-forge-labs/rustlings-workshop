# Workshop: VecDequeFruitSalad — Double-Ended Queue

**Goal**: Implement all functions in `src/lib.rs` to pass all **5** tests.

## Functions to Implement

### `make_fruit_deque`
- **Signature**: `pub fn make_fruit_deque() -> VecDeque<&'static str>`
- **Task**: Return a VecDeque with 3 fruit items
- **Tests**: test_make_fruit_deque

### `format_fruit_salad`
- **Signature**: `pub fn format_fruit_salad(fruit: &VecDeque<&str>) -> String`
- **Task**: Format all fruits into a comma-separated string
- **Tests**: test_format_non_empty, test_format_empty

### (push/pop pattern)
- **Task**: Use `push_back`, `pop_front`, `pop_back` operations
- **Tests**: test_push_pop, test_pop_empty

## Test Modules

| Module | Tests | What It Tests |
|--------|-------|---------------|
| step_01_vecdeque | 5 | VecDeque creation, formatting, push/pop, empty |

## How to Run Tests
```bash
cargo test
```
