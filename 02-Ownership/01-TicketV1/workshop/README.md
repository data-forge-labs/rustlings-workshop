# Workshop: TicketV1 — Structs, Ownership, and Methods

**Goal**: Implement all functions in `src/lib.rs` to pass all **15** tests.

## Functions to Implement

### `Ticket::new`
- **Signature**: `pub fn new(title: String, description: String, status: String) -> Ticket`
- **Task**: Validate and create a Ticket. Panic if title empty/>50 chars/has newlines, description empty/>500 chars, or status not one of "Open"/"In Progress"/"Closed"
- **Tests**: step_01_structs, step_02_validation

### `title`, `description`, `status`
- **Signature**: `pub fn title/description/status(&self) -> &String`
- **Task**: Return reference to the respective field
- **Tests**: test_new_ticket, test_borrow_does_not_move

### `set_title`, `set_description`, `set_status`
- **Signature**: `pub fn set_title/description/status(&mut self, ...: String)`
- **Task**: Update the field with the same validation as `new`
- **Tests**: step_03_setters (5 tests)

## Test Modules

| Module | Tests | What It Tests |
|--------|-------|---------------|
| step_01_structs | 2 | Struct construction, field accessors |
| step_02_validation | 6 | Input validation, panic on invalid data |
| step_03_setters | 5 | Mutable setters with validation |
| step_04_ownership | 2 | Borrow rules, shared references |

## How to Run Tests
```bash
cargo test
```
