# Workshop: MasterMind — Structs, Vec, Option, Match

**Goal**: Implement all functions in `src/lib.rs` to pass all **30** tests.

## Functions to Implement

### `has_unique_digits`
- **Signature**: `pub fn has_unique_digits(s: &str) -> bool`
- **Task**: Return `true` if `s` is exactly 4 ASCII digits, all unique
- **Tests**: step_01_validation (7 tests)

### `SecretCode`
- **Struct fields**: `digits: Vec<u8>`, `revealed_positions: Vec<bool>`, `revealed_digits: Vec<bool>`
- **Methods**: 
  - `new()` — generate random 4-digit code with unique digits
  - `evaluate_guess(&self, guess: &str) -> (usize, usize, usize)` — returns (green, yellow, red) counts
  - `can_give_position_hint() -> bool` — any unrevealed position?
  - `can_give_digit_hint() -> bool` — any unrevealed digit?
  - `give_position_hint(&mut self) -> Option<(usize, u8)>` — reveal one position
  - `give_digit_hint(&mut self) -> Option<u8>` — reveal one digit
- **Tests**: step_02_secret_code (8 tests), step_03_hints (8 tests)

### `MastermindGame`
- **Struct fields**: `secret: SecretCode`, `attempts_left: u32`, `guess_count: u32`
- **Methods**:
  - `new(max_attempts: u32) -> Self` — create a new game
  - `play(&mut self)` — run the main game loop (I/O, hints, guesses)
- **Tests**: step_04_game_setup (5 tests), step_05_integration (2 tests)

## Test Modules

| Module | Tests | What It Tests |
|--------|-------|---------------|
| step_01_validation | 7 | String validation, unique digits |
| step_02_secret_code | 8 | Struct construction, guess evaluation |
| step_03_hints | 8 | Option, hint system, exhaustion |
| step_04_game_setup | 5 | Game construction, default attempts |
| step_05_integration | 2 | Multi-scenario evaluate edge cases |

## How to Run Tests
```bash
cargo test
```
