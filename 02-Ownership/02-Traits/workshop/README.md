# Workshop: Traits — Display, Derive, From, Drop

**Goal**: Implement all trait impls and functions in `src/lib.rs` to pass all **10** tests.

## Functions to Implement

### `impl Display for Ticket`
- **Signature**: `impl fmt::Display for Ticket { fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result }`
- **Task**: Format as `"<title> [<status>]: <description>"`
- **Tests**: step_01_display (2 tests)

### `impl From<&str> for Ticket`
- **Signature**: `impl From<&str> for Ticket`
- **Task**: Convert `&str` into a Ticket (title = str, status = "Open", description = "")
- **Tests**: step_04_from_into (2 tests)

### `format_summary`
- **Signature**: `pub fn format_summary<T: Display + Debug>(item: &T) -> String`
- **Task**: Return `format!("{} {:?}", item, item)`
- **Tests**: step_02_trait_bounds (2 tests)

### `DatabaseConnection::new` + `Drop`
- **Tasks**: `new(url)` creates a connection; `Drop::drop` prints a closing message
- **Tests**: step_05_drop (1 test)

### Derive macros
- **Task**: Add `#[derive(Clone, PartialEq)]` on `Ticket`
- **Tests**: step_03_derive (3 tests)

## Test Modules

| Module | Tests | What It Tests |
|--------|-------|---------------|
| step_01_display | 2 | Display formatting (like `__str__`) |
| step_02_trait_bounds | 2 | Generic fn with trait bounds |
| step_03_derive | 3 | Clone, PartialEq via derive |
| step_04_from_into | 2 | From/Into conversion |
| step_05_drop | 1 | Drop trait (RAII teardown) |

## How to Run Tests
```bash
cargo test
```

## Exercises

* **Easy** – modify the existing function to handle an extra edge case.
* **Medium** – extend the project with a new helper function that re‑uses the core logic.

