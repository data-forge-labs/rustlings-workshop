# 🧪 Proptest — Property-Based Testing

*Subtitle: stop hand-writing 50 test cases. Describe the invariant and let proptest find the counter-example.*

> **Test-driven approach**: This project includes a Cargo project with progressive
> property-based tests. Each function in `src/lib.rs` starts as a `todo!()` stub.
> As you follow each section, replace `todo!()` with real code and run `cargo test`
> to watch the pass count grow. Proptest runs **32 random inputs per property**.
> Your goal: **all 8 properties pass** (256+ random inputs total).

---

## Why Property Tests for Data Pipelines?

**Python pain:** A parser reads a million rows. You wrote 5 unit tests with
hand-picked inputs. The 6th test on production data finds a buffer overflow
on negative numbers in column 3. You add the 6th test. The 7th test on
production finds a UTF-8 BOM issue. The list never ends.

**Rust fix:** A property test says "for **all** valid inputs, this invariant
holds." Proptest then *generates* thousands of inputs, **shrinking** any
counter-example to its smallest form before reporting. The test for
`count_above` becomes a single line: *the result equals the hand-written
reference*. Proptest tries negative numbers, empty vecs, all-equal inputs,
boundary values, and shrinks a failure down to `vec![-1]` with `threshold = 0`.

```rust
proptest! {
    #[test]
    fn prop_count_matches_filter(
        v in vec(-100i32..100, 0..50),
        t in -50i32..50
    ) {
        let actual = count_above(&v, t);
        let expected = v.iter().filter(|&&x| x > t).count();
        prop_assert_eq!(actual, expected);
    }
}
```

## At a Glance

| # | Concept | Rust | Python | Why it matters |
|---|---------|------|--------|----------------|
| 1 | Property-based testing | `proptest::proptest!` | `hypothesis.given(...)` | Generate inputs, not hand-pick them |
| 2 | Strategies | `proptest::collection::vec(...)` | `hypothesis.strategies.integers()` | The "any input" type |
| 3 | Random sampling | `ProptestConfig::with_cases(N)` | `@settings(max_examples=N)` | Control coverage vs speed |
| 4 | Shrinking | automatic | automatic | Failure becomes smallest reproducer |
| 5 | Invariants | `prop_assert!`, `prop_assert_eq!` | `assert ...` inside `@given` | The property the test must hold |
| 6 | Idempotence check | `sort(sort(x)) == sort(x)` | n/a | Classic property |
| 7 | Reference comparison | `count_above(x) == filter(x).count()` | n/a | Test the function via a known correct version |
| 8 | Bound check | `result >= 0.0` (epsilon) | n/a | Numerical stability across inputs |

---
