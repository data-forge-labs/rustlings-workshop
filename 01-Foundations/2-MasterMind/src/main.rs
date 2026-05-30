use rand::seq::SliceRandom;
use rand::thread_rng;
use std::io::{self, Write};

fn main() {
    let mut game = mastermind::MastermindGame::new(mastermind::DEFAULT_ATTEMPTS);
    game.play();
}
