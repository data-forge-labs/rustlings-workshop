// ============================================================
// 0-Intro — Library
// ============================================================
// Replace each `todo!()` with your implementation as you
// progress through the README tutorial.
// Run `cargo test` to watch your pass count grow.
// ============================================================

#![allow(unused_variables)]
#![allow(unused_imports)]

/// Converts Celsius to Fahrenheit.
/// README §5: Functions
pub fn celsius_to_fahrenheit(c: f64) -> f64 {
    c * 9.0 / 5.0 + 32.0
}

/// Converts Fahrenheit to Celsius.
/// README §5: Exercise
pub fn fahrenheit_to_celsius(f: f64) -> f64 {
    (f - 32.0) * 5.0 / 9.0
}

/// Classifies a temperature as "cold", "mild", or "hot".
/// Demonstrates `if`/`else` as an expression.
/// README §7: If/Else
pub fn classify_temp(temp: i32) -> &'static str {
    if temp < 10 {
        "cold"
    } else if (temp < 30) && (temp >= 10) {
        "mild"
    } else {
        "hot"
    }
}

/// Counts how many values in a fixed array are positive (> 0).
/// Demonstrates `for` loops and `if`.
/// README §8: Loops
pub fn count_positive(values: [i32; 5]) -> usize {
    let mut cnt = 0;
    for i in values {
        if i > 0 {
            cnt += 1;
        }
    }
    cnt
}

/// Sums all values in a fixed array of 5 i32s.
/// README §8: Loops (exercise)
pub fn sum_five(values: [i32; 5]) -> i32 {
    let mut s = 0;
    for t in values {
        s += t;
    }
    s
}

/// Classifies a data row given as a tuple `(id, value, is_valid)`.
/// Demonstrates tuple destructuring.
/// README §9: Tuples
pub fn categorize_row(row: (u32, f64, bool)) -> &'static str {
    let (t, value, is_valid) = row;
    if is_valid && value > 0.0 {
        "ok"
    } else if !is_valid {
        "invalid"
    } else {
        "zero"
    }
}

/// Returns the largest value in a fixed array of 5 i32s.
/// Demonstrates fixed-size arrays and `for` ranges.
/// README §10: Arrays
pub fn max_of_five(values: [i32; 5]) -> i32 {
    let mut max = values[0];
    for v in values {
        if v > max {
            max = v;
        }
    }
    max
}

/// Counts how many readings in a 5-element array are "hot" (>= 30).
/// Returns (count, label) where label is "few", "some", or "many".
/// Demonstrates combining everything: arrays, loops, if, tuples.
/// README §11: Putting It All Together
pub fn hot_readings_summary(readings: [i32; 5]) -> (usize, &'static str) {
    let mut count: usize = 0;
    for v in readings {
        if v >= 30 {
            count += 1;
        }
    }
    let label = if count == 1 || count == 2 {
        "some"
    } else if count > 2 {
        "many"
    } else {
        "few"
    };
    (count, label)
}

// ============================================================
// Tests
// ============================================================

#[cfg(test)]
mod tests {
    use super::*;

    // ---- Step 1: Functions (README §5) ----

    mod step_01_functions {
        use super::super::*;

        #[test]
        fn test_celsius_to_fahrenheit_freezing() {
            assert!((celsius_to_fahrenheit(0.0) - 32.0).abs() < 1e-10);
        }

        #[test]
        fn test_celsius_to_fahrenheit_boiling() {
            assert!((celsius_to_fahrenheit(100.0) - 212.0).abs() < 1e-10);
        }

        #[test]
        fn test_celsius_to_fahrenheit_negative() {
            assert!((celsius_to_fahrenheit(-40.0) - (-40.0)).abs() < 1e-10);
        }

        #[test]
        fn test_fahrenheit_to_celsius_freezing() {
            assert!((fahrenheit_to_celsius(32.0) - 0.0).abs() < 1e-10);
        }

        #[test]
        fn test_fahrenheit_to_celsius_boiling() {
            assert!((fahrenheit_to_celsius(212.0) - 100.0).abs() < 1e-10);
        }
    }

    // ---- Step 2: If/Else as expression (README §7) ----

    mod step_02_if_else {
        use super::super::*;

        #[test]
        fn test_classify_cold() {
            assert_eq!(classify_temp(5), "cold");
        }

        #[test]
        fn test_classify_threshold_cold() {
            assert_eq!(classify_temp(10), "mild"); // 10 is not < 10
        }

        #[test]
        fn test_classify_mild() {
            assert_eq!(classify_temp(20), "mild");
        }

        #[test]
        fn test_classify_threshold_hot() {
            assert_eq!(classify_temp(30), "hot"); // 30 is not < 30
        }

        #[test]
        fn test_classify_hot() {
            assert_eq!(classify_temp(35), "hot");
        }
    }

    // ---- Step 3: Loops (README §8) ----

    mod step_03_loops {
        use super::super::*;

        #[test]
        fn test_count_positive_mixed() {
            assert_eq!(count_positive([10, -3, 25, 0, 7]), 3);
        }

        #[test]
        fn test_count_positive_all() {
            assert_eq!(count_positive([1, 2, 3, 4, 5]), 5);
        }

        #[test]
        fn test_count_positive_none() {
            assert_eq!(count_positive([-1, -2, 0, -3, -4]), 0);
        }

        #[test]
        fn test_sum_five_basic() {
            assert_eq!(sum_five([10, 20, 30, 40, 50]), 150);
        }

        #[test]
        fn test_sum_five_with_negatives() {
            assert_eq!(sum_five([5, -2, 3, -1, 0]), 5);
        }

        #[test]
        fn test_sum_five_all_zeros() {
            assert_eq!(sum_five([0, 0, 0, 0, 0]), 0);
        }
    }

    // ---- Step 4: Tuples (README §9) ----

    mod step_04_tuples {
        use super::super::*;

        #[test]
        fn test_categorize_row_ok() {
            assert_eq!(categorize_row((1, 5.0, true)), "ok");
        }

        #[test]
        fn test_categorize_row_zero() {
            assert_eq!(categorize_row((2, 0.0, true)), "zero");
        }

        #[test]
        fn test_categorize_row_invalid() {
            assert_eq!(categorize_row((3, 5.0, false)), "invalid");
        }

        #[test]
        fn test_categorize_row_negative_value() {
            // Negative value: not invalid (is_valid == true) and not > 0 → "zero"
            assert_eq!(categorize_row((4, -1.0, true)), "zero");
        }
    }

    // ---- Step 5: Arrays (README §10) ----

    mod step_05_arrays {
        use super::super::*;

        #[test]
        fn test_max_of_five_positive() {
            assert_eq!(max_of_five([3, 1, 4, 1, 5]), 5);
        }

        #[test]
        fn test_max_of_five_negative() {
            assert_eq!(max_of_five([-2, -8, -1, -9, -5]), -1);
        }

        #[test]
        fn test_max_of_five_mixed() {
            assert_eq!(max_of_five([-1, 0, 7, 3, 2]), 7);
        }
    }

    // ---- Step 6: Putting It All Together (README §11) ----

    mod step_06_combined {
        use super::super::*;

        #[test]
        fn test_hot_readings_few() {
            let (n, l) = hot_readings_summary([10, 20, 22, 18, 25]);
            assert_eq!(n, 0);
            assert_eq!(l, "few");
        }

        #[test]
        fn test_hot_readings_some() {
            let (n, l) = hot_readings_summary([25, 30, 28, 22, 18]);
            assert_eq!(n, 1);
            assert_eq!(l, "some");
        }

        #[test]
        fn test_hot_readings_many() {
            let (n, l) = hot_readings_summary([22, 28, 31, 35, 30]);
            assert_eq!(n, 3);
            assert_eq!(l, "many");
        }
    }
}
