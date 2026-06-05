use gil_release_workshop::{
    benchmark_parallel, cpu_intensive_sum, gil_contention_factor, is_single_threaded_bottleneck,
    release_gil_simulation, worker_count_active,
};
use std::error::Error;
use std::time::Instant;

fn main() -> Result<(), Box<dyn Error>> {
    let workers = worker_count_active();
    println!("Active CPU workers: {}", workers);
    println!();

    let n = 5_000_000;
    let start = Instant::now();
    let s = cpu_intensive_sum(n);
    println!("cpu_intensive_sum({}) = {} in {:?}", n, s, start.elapsed());

    let (dur, total) = benchmark_parallel(1_000_000, 4);
    println!("4 threads did {} ops in {:?}", total, dur);

    let factor = gil_contention_factor(500_000, 4);
    println!("GIL contention factor (4 thr): {:.3}", factor);

    let sim = release_gil_simulation(500_000);
    println!("release_gil_simulation: {:?}", sim);

    let (one_t, _) = benchmark_parallel(1_000_000, 1);
    let (four_t, _) = benchmark_parallel(1_000_000, 4);
    println!(
        "1 thr: {:?} | 4 thr: {:?} | speedup: {:.2}x",
        one_t,
        four_t,
        one_t.as_secs_f64() / four_t.as_secs_f64()
    );

    let _ = is_single_threaded_bottleneck(100_000, 4);

    Ok(())
}
