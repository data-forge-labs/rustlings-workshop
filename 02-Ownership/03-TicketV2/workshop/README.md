# Workshop: TicketV2 — Enums, Match, Result, Custom Errors

**Goal**: Implement all functions in `src/lib.rs` to pass all **16** tests.

## Functions to Implement

### `Status::from_str`
- **Signature**: `pub fn from_str(s: &str) -> Result<Status, TicketError>`
- **Task**: Parse `"Open"`, `"In Progress"`, `"Resolved"`, `"Closed"` into `Status` variants; return `Err(InvalidStatus)` otherwise
- **Tests**: step_01_enums (6 tests)

### `impl Display for Status`
- **Signature**: `fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result`
- **Task**: Write variant name as string (e.g. `InProgress` → `"In Progress"`)
- **Tests**: step_02_match (1 test)

### `Ticket::new`
- **Signature**: `pub fn new(title: String, description: String, status: Status) -> Result<Ticket, TicketError>`
- **Task**: Validate and return `Ok(Ticket)` or the appropriate `TicketError`
- **Tests**: step_03_result (5 tests)

### `title`, `description`, `status`
- **Signatures**: `pub fn title/description(&self) -> &str`, `pub fn status(&self) -> &Status`
- **Task**: Return references to fields
- **Tests**: step_03_result

### `set_status`
- **Signature**: `pub fn set_status(&mut self, status: Status)`
- **Task**: Update status field
- **Tests**: step_04_if_let (1 test)

### `impl Display for TicketError`
- **Task**: Format each error variant with descriptive messages
- **Tests**: step_06_error_display (2 tests)

## Test Modules

| Module | Tests | What It Tests |
|--------|-------|---------------|
| step_01_enums | 6 | Enum variants, parsing with match |
| step_02_match | 1 | Display via match (exhaustive) |
| step_03_result | 5 | Result<T, E>, validation errors |
| step_04_if_let | 1 | if-let pattern for status update |
| step_05_option | 1 | Option-returning pattern |
| step_06_error_display | 2 | Custom error Display formatting |

## How to Run Tests
```bash
cargo test
```
