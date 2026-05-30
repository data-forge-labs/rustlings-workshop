# Workshop: OBRM — RAII, Drop, and Ownership

**Goal**: Implement all functions in `src/lib.rs` to pass all **11** tests.

## Functions to Implement

### `Resource::new`, `close`, `is_open`
- **Signatures**: `pub fn new(id: u32) -> Self`, `pub fn close(&mut self)`, `pub fn is_open(&self) -> bool`
- **Task**: Create a resource (open), close it, check open state. Double-close must be safe
- **Tests**: step_01_resource_lifecycle (3 tests)

### `impl Drop for Resource`
- **Task**: Print a message when resource is dropped
- **Tests**: tested via raii_demo

### `raii_demo`
- **Signature**: `pub fn raii_demo() -> Vec<String>`
- **Task**: Create and drop resources; return lifecycle messages
- **Tests**: step_02_raii_demo (2 tests)

### `ownership_transfer`
- **Signature**: `pub fn ownership_transfer() -> u32`
- **Task**: Demonstrate move semantics; return count of transferred resources
- **Tests**: step_03_ownership (1 test)

### `borrow_resource`
- **Signature**: `pub fn borrow_resource(res: &Resource) -> u32`
- **Task**: Return resource id without taking ownership
- **Tests**: step_03_ownership (2 tests)

### `obrm_concepts`
- **Signature**: `pub fn obrm_concepts() -> Vec<&'static str>`
- **Task**: Return a list of RAII/OBRM concepts covered
- **Tests**: step_04_concepts (3 tests)

## Test Modules

| Module | Tests | What It Tests |
|--------|-------|---------------|
| step_01_resource_lifecycle | 3 | Resource open/close/double-close |
| step_02_raii_demo | 2 | RAII via Drop (scope-based cleanup) |
| step_03_ownership | 3 | Move vs borrow semantics |
| step_04_concepts | 3 | Concept listing verification |

## How to Run Tests
```bash
cargo test
```
