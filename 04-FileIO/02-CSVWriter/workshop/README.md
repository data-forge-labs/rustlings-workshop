# Workshop: CSV Writer

**Goal**: Implement all functions in `src/lib.rs` to pass all 4 tests.

## Functions to Implement

### `apply_discount`
- **Signature**: `pub fn apply_discount(product: &Product) -> Product`
- **Task**: Return a new `Product` with `price` reduced by `DISCOUNT` (10%), keeping the same name.
- **Tests**: test_apply_discount, test_apply_discount_zero

### `total_savings`
- **Signature**: `pub fn total_savings(products: &[Product]) -> f64`
- **Task**: Sum the total discount amount across all products (each product's price * DISCOUNT).
- **Tests**: test_total_savings, test_total_savings_empty

## Structs

### `Product`
- Fields: `name: String`, `price: f64`
- Derives `Debug`, `Deserialize`, `Serialize` with `PascalCase` renaming.

## Test Modules

| Module | Tests | What It Tests |
|--------|-------|---------------|
| step_01_discount | 4 | Discount application and total savings calculation |

## How to Run Tests
```bash
cargo test
```

## Exercises

* **Easy** – modify the existing function to handle an extra edge case.
* **Medium** – extend the project with a new helper function that re‑uses the core logic.

