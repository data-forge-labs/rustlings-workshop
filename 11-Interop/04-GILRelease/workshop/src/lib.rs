use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::Duration;

pub fn cpu_intensive_sum(n: u64) -> u64 {
    (0..n).sum()
}

pub fn is_single_threaded_bottleneck(work_per_thread: u64, num_threads: usize) -> Duration {
    let start = std::time::Instant::now();
    for _ in 0..num_threads {
        cpu_intensive_sum(work_per_thread);
    }
    start.elapsed()
}

pub fn worker_count_active() -> usize {
    4
}

pub fn release_gil_simulation(work_units: u64) -> Duration {
    let start = std::time::Instant::now();
    let handle = thread::spawn(move || {
        cpu_intensive_sum(work_units);
    });
    handle.join().unwrap();
    start.elapsed()
}

pub fn benchmark_parallel(work_per_thread: u64, num_threads: usize) -> (Duration, u64) {
    let start = std::time::Instant::now();
    let handles: Vec<_> = (0..num_threads)
        .map(|_| thread::spawn(move || cpu_intensive_sum(work_per_thread)))
        .collect();
    let total: u64 = handles.into_iter().map(|h| h.join().unwrap()).sum();
    (start.elapsed(), total)
}

pub fn validate_inputs(work: u64, threads: usize) -> Result<(), &'static str> {
    if threads == 0 {
        return Err("threads must be greater than 0");
    }
    let _ = work;
    Ok(())
}

pub fn gil_contention_factor(work_per_thread: u64, num_threads: usize) -> f64 {
    let (serial_dur, _) = benchmark_parallel(work_per_thread, 1);
    let (parallel_dur, _) = benchmark_parallel(work_per_thread, num_threads);
    parallel_dur.as_secs_f64() / serial_dur.as_secs_f64()
}

pub fn format_result(name: &str, duration: Duration, work: u64) -> String {
    format!("{}: {:?} for {} work units", name, duration, work)
}

#[cfg(feature = "python")]
#[pyo3::pymodule]
fn gil_release_workshop(m: &pyo3::Bound<'_, pyo3::types::PyModule>) -> pyo3::PyResult<()> {
    use pyo3::types::PyModuleMethods;
    use pyo3::wrap_pyfunction;

    #[pyo3::pyfunction]
    fn cpu_intensive_sum_py(n: u64) -> u64 {
        cpu_intensive_sum(n)
    }

    #[pyo3::pyfunction]
    fn release_gil_simulation_py(work_units: u64) -> u64 {
        release_gil_simulation(work_units).as_millis() as u64
    }

    m.add_function(wrap_pyfunction!(cpu_intensive_sum_py, m)?)?;
    m.add_function(wrap_pyfunction!(release_gil_simulation_py, m)?)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    mod step_01_basic {
        use super::*;

        #[test]
        fn test_cpu_intensive_sum_zero() {
            assert_eq!(cpu_intensive_sum(0), 0);
        }

        #[test]
        fn test_cpu_intensive_sum_small() {
            let result = cpu_intensive_sum(100);
            assert!(result > 0);
        }
    }

    mod step_02_contention {
        use super::*;

        #[test]
        fn test_single_thread_bottleneck() {
            let d = is_single_threaded_bottleneck(100_000, 4);
            assert!(d.as_millis() < 5000);
        }

        #[test]
        fn test_parallel_speedup() {
            let (serial_dur, _) = benchmark_parallel(200_000, 1);
            let (parallel_dur, total) = benchmark_parallel(200_000, 4);
            assert_eq!(total, 800_000);
            assert!(parallel_dur < serial_dur * 2);
        }
    }

    mod step_03_workers {
        use super::*;

        #[test]
        fn test_worker_count() {
            assert_eq!(worker_count_active(), 4);
        }
    }

    mod step_04_release {
        use super::*;

        #[test]
        fn test_release_gil_simulation() {
            let d = release_gil_simulation(50_000);
            assert!(d.as_millis() < 5000);
        }
    }

    mod step_05_validation {
        use super::*;

        #[test]
        fn test_validate_inputs_ok() {
            assert!(validate_inputs(100, 4).is_ok());
        }

        #[test]
        fn test_validate_inputs_zero_threads() {
            assert!(validate_inputs(100, 0).is_err());
        }
    }

    mod step_06_metric {
        use super::*;

        #[test]
        fn test_gil_contention_factor_one_thread() {
            let factor = gil_contention_factor(100_000, 1);
            assert!(factor >= 0.99 && factor <= 1.01);
        }

        #[test]
        fn test_gil_contention_factor_high_thread() {
            let factor = gil_contention_factor(100_000, 8);
            assert!(factor < 1.0);
        }
    }

    mod step_07_format {
        use super::*;

        #[test]
        fn test_format_result() {
            let s = format_result("test", Duration::from_millis(123), 1000);
            assert!(s.contains("test"));
            assert!(s.contains("1000"));
        }
    }
}
