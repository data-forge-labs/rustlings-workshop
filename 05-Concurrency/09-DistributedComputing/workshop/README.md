# Workshop: Distributed Computing

**Goal**: Study the implemented `src/lib.rs` and pass all 14 tests.

## Functions to Study

### `measure_allocation_overhead`
- **Signature**: `pub fn measure_allocation_overhead(count: usize) -> usize`
- **Task**: Allocate and drop `count` Vecs to simulate allocation overhead. Already implemented.
- **Tests**: test_measure_allocation_overhead_normal, test_measure_allocation_overhead_zero

### `compute_intensive`
- **Signature**: `pub fn compute_intensive(iterations: usize) -> u64`
- **Task**: Compute the `iterations`-th Fibonacci number without GC interruptions. Already implemented.
- **Tests**: test_compute_intensive_fib_10, test_compute_intensive_fib_1, test_compute_intensive_fib_0

### `simulate_gc_pause`
- **Signature**: `pub fn simulate_gc_pause(work_items: Vec<u64>) -> u64`
- **Task**: Sum work items with simulated "GC pause" steps every 3 items. Already implemented.
- **Tests**: test_simulate_gc_pause_normal, test_simulate_gc_pause_empty, test_simulate_gc_pause_single

### `compare_throughput`
- **Signature**: `pub fn compare_throughput(data_sizes: Vec<usize>) -> Vec<f64>`
- **Task**: Return simulated compiled/interpreted throughput ratios. Already implemented.
- **Tests**: test_compare_throughput_normal, test_compare_throughput_empty

### `zero_cost_abstraction_demo`
- **Signature**: `pub fn zero_cost_abstraction_demo(values: Vec<i32>) -> i32`
- **Task**: Sum all even values using iterator chaining (zero-cost abstraction). Already implemented.
- **Tests**: test_zero_cost_sum_evens, test_zero_cost_all_odd, test_zero_cost_empty, test_zero_cost_negative

## Test Modules

| Module | Tests | What It Tests |
|--------|-------|---------------|
| step_01_memory_efficiency | 5 | Allocation overhead and GC pause simulation |
| step_02_compute_performance | 5 | Fibonacci, throughput ratios |
| step_03_zero_cost | 4 | Zero-cost iterator abstraction |

## How to Run Tests
```bash
cargo test
```
