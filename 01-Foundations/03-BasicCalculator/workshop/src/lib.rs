// ============================================================
// 1-BasicCalculator — Library
// ============================================================
// Replace each `todo!()` with your implementation as you
// progress through the README tutorial.
// Run `cargo test` to see your pass/fail count grow.
// ============================================================

#![allow(unused_variables)] // todo!() stubs don't use params yet
#![allow(unused_imports)]

/// Adds two integers.
/// README §4: Integer Types / §5: Variables / §6: Arithmetic
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

/// Subtracts `b` from `a`.
pub fn subtract(a: i32, b: i32) -> i32 {
    a - b
}

/// Multiplies two integers.
pub fn multiply(a: i32, b: i32) -> i32 {
    a * b
}

/// Divides `a` by `b`. Panics if `b` is 0.
/// README §9: Panics
pub fn divide(a: i32, b: i32) -> i32 {
    assert!(b != 0, "Cannot divide by zero");
    a / b
}

/// Computes `n!` using saturating arithmetic.
/// Returns `u32::MAX` (clamped) on overflow instead of panicking.
/// README §10-12: Loops, Overflow, Saturating arithmetic
pub fn factorial_safe(n: u32) -> u32 {
    let mut result: u32 = 1;
    for i in 2..=n {
        result = result.saturating_mul(i);
    }
    result
}

/// Computes `n!` using wrapping arithmetic.
/// On overflow the result wraps around (modulo 2^32).
/// README §12: Wrapping arithmetic
pub fn factorial_wrapping(n: u32) -> u32 {
    let mut result: u32 = 1;
    for i in 2..=n {
        result = result.wrapping_mul(i);
    }
    result
}

/// Returns the average of two `u32` values.
/// Uses saturating addition to avoid overflow.
/// README §15 Exercise 1
pub fn average(a: u32, b: u32) -> u32 {
    a / 2 + b / 2 + ((a % 2 + b % 2) / 2)
}

/// Safely sums a slice of row counts using saturating addition.
/// README §15 Exercise 2
pub fn total_rows(counts: &[u64]) -> u64 {
    counts.iter().copied().fold(0u64, |acc, x| acc.saturating_add(x))
}

/// Returns the minimum value in a slice of temperatures.
/// README §15 Exercise 3
pub fn min_temp(temps: &[i32]) -> i32 {
    temps.iter().copied().min().unwrap()
}

/// Returns the maximum value in a slice of temperatures.
pub fn max_temp(temps: &[i32]) -> i32 {
    temps.iter().copied().max().unwrap()
}

/// Returns the arithmetic mean of a slice of temperatures as `f64`.
pub fn avg_temp(temps: &[i32]) -> f64 {
    temps.iter().copied().sum::<i32>() as f64 / temps.len() as f64
}

/// Returns `true` if adding `a` + `b` would overflow `u32`.
/// README §15 Exercise 4
pub fn would_overflow(a: u32, b: u32) -> bool {
    a.checked_add(b).is_none()
}

// ============================================================
// Tests — organised by tutorial step
// ============================================================
// Run `cargo test` after each section to see more green checks.
// ============================================================

#[cfg(test)]
mod tests {
    use super::*;

    // ---- Step 1: Basic arithmetic (README §4-6) ----

    mod step_01_arithmetic {
        use super::super::*;

        #[test]
        fn test_add() {
            assert_eq!(add(2, 3), 5);
        }

        #[test]
        fn test_add_negative() {
            assert_eq!(add(-1, 1), 0);
        }

        #[test]
        fn test_add_zero() {
            assert_eq!(add(0, 0), 0);
        }

        #[test]
        fn test_subtract() {
            assert_eq!(subtract(10, 3), 7);
        }

        #[test]
        fn test_subtract_negative_result() {
            assert_eq!(subtract(3, 10), -7);
        }

        #[test]
        fn test_multiply() {
            assert_eq!(multiply(4, 5), 20);
        }

        #[test]
        fn test_multiply_zero() {
            assert_eq!(multiply(7, 0), 0);
        }

        #[test]
        fn test_multiply_negative() {
            assert_eq!(multiply(-3, 4), -12);
        }
    }

    // ---- Step 2: Division and panics (README §9) ----

    mod step_02_division {
        use super::super::*;

        #[test]
        fn test_divide() {
            assert_eq!(divide(10, 2), 5);
        }

        #[test]
        fn test_divide_truncates() {
            assert_eq!(divide(5, 2), 2);
        }

        #[test]
        fn test_divide_negative() {
            assert_eq!(divide(-10, 3), -3);
        }

        #[test]
        #[should_panic(expected = "Cannot divide by zero")]
        fn test_divide_by_zero() {
            divide(1, 0);
        }
    }

    // ---- Step 3: Factorial (README §10-12) ----

    mod step_03_factorial {
        use super::super::*;

        #[test]
        fn test_factorial_safe_0() {
            assert_eq!(factorial_safe(0), 1);
        }

        #[test]
        fn test_factorial_safe_1() {
            assert_eq!(factorial_safe(1), 1);
        }

        #[test]
        fn test_factorial_safe_5() {
            assert_eq!(factorial_safe(5), 120);
        }

        #[test]
        fn test_factorial_safe_10() {
            assert_eq!(factorial_safe(10), 3_628_800);
        }

        #[test]
        fn test_factorial_safe_overflow_clamps() {
            // 13! exceeds u32::MAX so saturating returns u32::MAX
            let result = factorial_safe(13);
            assert_eq!(result, u32::MAX);
        }

        #[test]
        fn test_factorial_wrapping_5() {
            assert_eq!(factorial_wrapping(5), 120);
        }

        #[test]
        fn test_factorial_wrapping_wraps() {
            // 13! wraps around modulo 2^32 — the true value (6,227,020,800) exceeds u32::MAX
            let result = factorial_wrapping(13);
            let expected = 479_001_600u32.wrapping_mul(13);
            assert_eq!(result, expected);
        }
    }

    // ---- Step 4: Saturating utilities (README §15 Ex 1-2) ----

    mod step_04_saturating_utils {
        use super::super::*;

        #[test]
        fn test_average_basic() {
            assert_eq!(average(10, 20), 15);
        }

        #[test]
        fn test_average_rounds_down() {
            assert_eq!(average(10, 11), 10);
        }

        #[test]
        fn test_average_overflow_safe() {
            // Would overflow if not using saturating arithmetic
            assert_eq!(average(u32::MAX, 1), (u32::MAX / 2) + 1);
        }

        #[test]
        fn test_total_rows_empty() {
            assert_eq!(total_rows(&[]), 0);
        }

        #[test]
        fn test_total_rows_basic() {
            assert_eq!(total_rows(&[100, 200, 150]), 450);
        }

        #[test]
        fn test_total_rows_saturating() {
            let big = u64::MAX;
            assert_eq!(total_rows(&[big, 1]), u64::MAX);
        }
    }

    // ---- Step 5: Temperature stats (README §15 Ex 3) ----

    mod step_05_temp_stats {
        use super::super::*;

        #[test]
        fn test_min_temp() {
            let temps = [23, 25, 19, 30, 28];
            assert_eq!(min_temp(&temps), 19);
        }

        #[test]
        fn test_min_temp_negative() {
            let temps = [-5, 0, 3, -10, 7];
            assert_eq!(min_temp(&temps), -10);
        }

        #[test]
        fn test_max_temp() {
            let temps = [23, 25, 19, 30, 28];
            assert_eq!(max_temp(&temps), 30);
        }

        #[test]
        fn test_max_temp_negative() {
            let temps = [-5, 0, 3, -10, 7];
            assert_eq!(max_temp(&temps), 7);
        }

        #[test]
        fn test_avg_temp() {
            let temps = [10, 20, 30];
            assert!((avg_temp(&temps) - 20.0).abs() < 1e-10);
        }

        #[test]
        fn test_avg_temp_single() {
            let temps = [42];
            assert!((avg_temp(&temps) - 42.0).abs() < 1e-10);
        }
    }

    // ---- Step 6: Overflow detection (README §15 Ex 4) ----

    mod step_06_overflow_detector {
        use super::super::*;

        #[test]
        fn test_no_overflow() {
            assert!(!would_overflow(100, 50));
        }

        #[test]
        fn test_overflow_at_max() {
            assert!(would_overflow(u32::MAX, 1));
        }

        #[test]
        fn test_overflow_exact_boundary() {
            assert!(!would_overflow(u32::MAX, 0));
        }

        #[test]
        fn test_overflow_large_values() {
            assert!(would_overflow(2_000_000_000, 2_000_000_000));
        }
    }
}
