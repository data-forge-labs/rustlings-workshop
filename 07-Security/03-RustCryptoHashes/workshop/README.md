# Workshop: RustCryptoHashes

**Goal**: Implement all 6 functions in `src/lib.rs` to pass all 12 tests.

## Functions to Implement

### `simple_hash`
- **Signature**: `pub fn simple_hash(input: &str) -> String`
- **Task**: Return a hex string hash of the input (e.g., using a simple algorithm like DJB2 or SHA-256 via `sha2` crate).
- **Tests**: simple_hash_non_empty, simple_hash_different_inputs

### `xor_checksum`
- **Signature**: `pub fn xor_checksum(input: &[u8]) -> u8`
- **Task**: XOR all bytes together; return 0 for empty input.
- **Tests**: xor_checksum_basic, xor_checksum_empty, xor_checksum_single_byte

### `is_deterministic`
- **Signature**: `pub fn is_deterministic(input: &str) -> bool`
- **Task**: Return `true` if `simple_hash` produces the same result when called twice on the same input.
- **Tests**: is_deterministic_same_input

### `avalanche_effect`
- **Signature**: `pub fn avalanche_effect(input: &str, change_at: usize) -> bool`
- **Task**: Return `true` if changing the character at `change_at` produces a different hash (or `true` if index is out of bounds).
- **Tests**: avalanche_effect_changes_result, avalanche_effect_out_of_bounds

### `hash_algorithms`
- **Signature**: `pub fn hash_algorithms() -> Vec<&'static str>`
- **Task**: Return a list of hash algorithms covered (e.g., `"SHA-256"`, `"MD5"`).
- **Tests**: hash_algorithms_non_empty, hash_algorithms_includes_sha

### `hash_properties`
- **Signature**: `pub fn hash_properties() -> Vec<&'static str>`
- **Task**: Return a list of cryptographic hash properties (e.g., `"collision resistance"`).
- **Tests**: hash_properties_non_empty, hash_properties_includes_collision_resistance

## Test Modules

| Module | Tests | What It Tests |
|--------|-------|---------------|
| step_01_hashing | 5 | simple_hash and xor_checksum |
| step_02_hash_properties | 3 | Determinism and avalanche effect |
| step_03_concepts | 4 | Algorithms and properties knowledge |

## How to Run Tests
```bash
cargo test
```
