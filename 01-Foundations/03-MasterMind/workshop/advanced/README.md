# Workshop: MasterMind Advanced — Modules, CLI, Docs

**Goal**: Implement all functions in `src/lib.rs`, `src/secret.rs`, and `src/game.rs` to pass all tests.

> **Basic version** also available in [`../`](../) — complete it first before starting this advanced workshop.

## Structure

```
src/
├── lib.rs       — Library root, re-exports public API
├── main.rs      — Binary entry point with `clap` CLI args
├── secret.rs    — `SecretCode` struct and implementation
└── game.rs      — `MastermindGame` struct and implementation
```

## Functions to Implement

### `secret.rs` — `SecretCode`
- `new()` — generate random 4-digit code with unique digits
- `from_digits(digits: Vec<u8>) -> Self` — create from explicit digits (for testing)
- `evaluate_guess(&self, guess: &str) -> (usize, usize, usize)` — returns (green, yellow, red)
- `can_give_position_hint() -> bool` — any unrevealed position?
- `can_give_digit_hint() -> bool` — any unrevealed digit?
- `give_position_hint(&mut self) -> Option<(usize, u8)>` — reveal one position
- `give_digit_hint(&mut self) -> Option<u8>` — reveal one digit

### `game.rs` — `MastermindGame`
- `new(max_attempts: u32) -> Self` — create a new game
- `play(&mut self)` — run the main game loop (I/O, hints, guesses)

### `main.rs` — CLI
- Parse `--max-attempts` argument with `clap::Parser`

## How to Run

```bash
cargo run                   # 20 attempts (default)
cargo run -- --max-attempts 15   # 15 attempts
```

## How to Run Tests

```bash
cargo test
```

## New Concepts in This Version

| Concept | File | Purpose |
|---------|------|---------|
| `mod secret; pub use` | `lib.rs` | Module declarations and re-exports |
| `clap::Parser` | `main.rs` | Type-safe CLI argument parsing |
| `///` doc comments | `secret.rs`, `game.rs` | Rustdoc documentation |
| `#[cfg(test)] mod tests` | Each source file | Inline unit tests |
| Library + binary crate | `Cargo.toml` | Separate logic from I/O |
