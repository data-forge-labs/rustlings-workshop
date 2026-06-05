fn main() {
    println!("Rust Logging — Python loguru equivalent");
    // Call all lib functions
    let levels = rust_logging::demo_log_levels();
    println!("Log levels demo: {:?}", levels);

    let structured = rust_logging::log_structured_data("event", "startup");
    println!("Structured log: {}", structured);

    let spans = rust_logging::tracing_demo(2);
    println!("Tracing spans: {:?}", spans);

    let (with_log, without_log) = rust_logging::logging_overhead(1000);
    println!("Logging overhead: {}ns vs {}ns", with_log, without_log);

    let equivalents = rust_logging::loguru_equivalents();
    for (rust, py) in &equivalents {
        println!("{}  ->  {}", rust, py);
    }
}
