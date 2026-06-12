# Workshop: TicketManagement — Vec, HashMap, Iterators

**Goal**: Implement all functions in `src/lib.rs` to pass all **12** tests.

## Functions to Implement

### `Ticket::new`, `id`, `title`, `status`, `is_open`, `is_closed`
- **Task**: Basic ticket struct with accessors and status checks
- **Tests**: step_01_arrays_vec (4 tests)

### `open_ticket_titles`
- **Signature**: `pub fn open_ticket_titles(tickets: &[Ticket]) -> Vec<&str>`
- **Task**: Return titles of all tickets with status "Open"
- **Tests**: step_02_iterators, step_04_edge_cases

### `ticket_summaries`
- **Signature**: `pub fn ticket_summaries(tickets: &[Ticket]) -> Vec<String>`
- **Task**: Format each ticket as `"<id>: [<status>] <title>"`
- **Tests**: step_02_iterators

### `index_by_status`
- **Signature**: `pub fn index_by_status(tickets: &[Ticket]) -> HashMap<&str, Vec<&Ticket>>`
- **Task**: Group tickets by their status string
- **Tests**: step_03_hashmap, step_04_edge_cases

### `most_common_status`
- **Signature**: `pub fn most_common_status(tickets: &[Ticket]) -> Option<(&str, usize)>`
- **Task**: Find the status with the most tickets; return `None` if empty
- **Tests**: step_03_hashmap, step_04_edge_cases

### `count_by_status`
- **Signature**: `pub fn count_by_status(tickets: &[Ticket]) -> HashMap<&str, usize>`
- **Task**: Count tickets per status using `fold`
- **Tests**: step_03_hashmap

## Test Modules

| Module | Tests | What It Tests |
|--------|-------|---------------|
| step_01_arrays_vec | 4 | Struct, Vec storage, status checks |
| step_02_iterators | 2 | filter/map combinator chains |
| step_03_hashmap | 3 | HashMap index, most_common, count |
| step_04_edge_cases | 3 | Empty Vec handling for all fns |

## How to Run Tests
```bash
cargo test
```

## Exercises

* **Easy** – modify the existing function to handle an extra edge case.
* **Medium** – extend the project with a new helper function that re‑uses the core logic.

