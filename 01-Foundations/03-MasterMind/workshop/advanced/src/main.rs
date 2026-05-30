use clap::Parser;

#[derive(Parser)]
#[command(name = "mastermind", about = "Code-breaking game")]
struct Args {
    /// Maximum number of attempts
    #[arg(short, long, default_value_t = 20)]
    max_attempts: u32,
}

fn main() {
    let args = Args::parse();
    let mut game = mastermind_advanced::MastermindGame::new(args.max_attempts);
    game.play();
}
