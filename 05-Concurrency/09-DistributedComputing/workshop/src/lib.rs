//! # Rust for Distributed Systems
//!
//! This module demonstrates why Rust is well-suited for distributed systems
//! by comparing its performance characteristics (no GC, zero-cost abstractions,
//! predictable memory) with interpreted/garbage-collected languages like
//! Python, Ruby, and JavaScript.

/// Simulate allocation overhead by allocating and dropping `count` Vecs on the heap.
/// Returns the number of allocations performed.
pub fn measure_allocation_overhead(count: usize) -> usize {
    for _ in 0..count {
        let mut v: Vec<u64> = Vec::with_capacity(1000);
        for j in 0..1000 {
            v.push(j as u64);
        }
    }
    count
}

/// Simulate a compute-intensive task (iterative Fibonacci calculation)
/// without GC interruptions. Returns the `iterations`-th Fibonacci number.
pub fn compute_intensive(iterations: usize) -> u64 {
    if iterations == 0 {
        return 0;
    }
    let mut a = 0u64;
    let mut b = 1u64;
    for _ in 1..iterations {
        let next = a.saturating_add(b);
        a = b;
        b = next;
    }
    b
}

/// Simulate GC pauses by processing work items with occasional
/// "collection" steps. Returns the total sum of processed items.
pub fn simulate_gc_pause(work_items: Vec<u64>) -> u64 {
    let mut total = 0u64;
    for (i, item) in work_items.iter().enumerate() {
        total += item;
        if (i + 1) % 3 == 0 {
            let mut pause: u64 = 0;
            for _ in 0..1000 {
                pause = pause.wrapping_add(1);
            }
        }
    }
    total
}

/// Return simulated throughput ratios (compiled / interpreted)
/// for different data sizes. Larger sizes show higher ratios
/// as compiled languages benefit from lack of GC overhead.
pub fn compare_throughput(data_sizes: Vec<usize>) -> Vec<f64> {
    data_sizes
        .iter()
        .map(|&size| {
            let base_ratio = 1.5;
            let size_factor = (size as f64).sqrt() * 0.1;
            base_ratio + size_factor
        })
        .collect()
}

/// Demonstrate zero-cost abstraction by using iterators.
/// Returns the sum of all even values in the input.
pub fn zero_cost_abstraction_demo(values: Vec<i32>) -> i32 {
    values.iter().filter(|&&x| x % 2 == 0).sum()
}

#[cfg(test)]
mod tests {
    mod step_01_memory_efficiency {
        use crate::{measure_allocation_overhead, simulate_gc_pause};

        #[test]
        fn test_measure_allocation_overhead_normal() {
            let result = measure_allocation_overhead(10);
            assert_eq!(result, 10);
        }

        #[test]
        fn test_measure_allocation_overhead_zero() {
            let result = measure_allocation_overhead(0);
            assert_eq!(result, 0);
        }

        #[test]
        fn test_simulate_gc_pause_normal() {
            let items = vec![1, 2, 3, 4, 5];
            let total = simulate_gc_pause(items);
            assert_eq!(total, 15);
        }

        #[test]
        fn test_simulate_gc_pause_empty() {
            let items: Vec<u64> = vec![];
            let total = simulate_gc_pause(items);
            assert_eq!(total, 0);
        }

        #[test]
        fn test_simulate_gc_pause_single() {
            let items = vec![42];
            let total = simulate_gc_pause(items);
            assert_eq!(total, 42);
        }
    }

    mod step_02_compute_performance {
        use crate::{compare_throughput, compute_intensive};

        #[test]
        fn test_compute_intensive_fib_10() {
            let result = compute_intensive(10);
            assert_eq!(result, 55);
        }

        #[test]
        fn test_compute_intensive_fib_1() {
            let result = compute_intensive(1);
            assert_eq!(result, 1);
        }

        #[test]
        fn test_compute_intensive_fib_0() {
            let result = compute_intensive(0);
            assert_eq!(result, 0);
        }

        #[test]
        fn test_compare_throughput_normal() {
            let sizes = vec![100, 1000, 10000];
            let ratios = compare_throughput(sizes);
            assert_eq!(ratios.len(), 3);
            assert!(ratios[0] < ratios[2]);
        }

        #[test]
        fn test_compare_throughput_empty() {
            let sizes: Vec<usize> = vec![];
            let ratios = compare_throughput(sizes);
            assert!(ratios.is_empty());
        }
    }

    mod step_03_zero_cost {
        use crate::zero_cost_abstraction_demo;

        #[test]
        fn test_zero_cost_sum_evens() {
            let values = vec![1, 2, 3, 4, 5, 6];
            let sum = zero_cost_abstraction_demo(values);
            assert_eq!(sum, 12);
        }

        #[test]
        fn test_zero_cost_all_odd() {
            let values = vec![1, 3, 5, 7];
            let sum = zero_cost_abstraction_demo(values);
            assert_eq!(sum, 0);
        }

        #[test]
        fn test_zero_cost_empty() {
            let values: Vec<i32> = vec![];
            let sum = zero_cost_abstraction_demo(values);
            assert_eq!(sum, 0);
        }

        #[test]
        fn test_zero_cost_negative() {
            let values = vec![-2, -1, 0, 1, 2];
            let sum = zero_cost_abstraction_demo(values);
            assert_eq!(sum, 0);
        }
    }
}
