use async_clap_workshop::{parse_args, run_pipeline, run_summary};
use std::process::ExitCode;

#[tokio::main]
async fn main() -> ExitCode {
    let argv: Vec<String> = std::env::args().collect();
    let argv_ref: Vec<&str> = argv.iter().map(String::as_str).collect();

    let cli = match parse_args(&argv_ref) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("{}", e);
            return ExitCode::from(2);
        }
    };

    println!("{}", run_summary(&cli));

    match run_pipeline(&cli).await {
        Ok(msg) => {
            println!("{}", msg);
            ExitCode::SUCCESS
        }
        Err(e) => {
            eprintln!("Error: {}", e);
            ExitCode::FAILURE
        }
    }
}
