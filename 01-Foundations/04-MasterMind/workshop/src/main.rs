use rand::seq::SliceRandom;
use std::io::{self, Write};

fn main() {
    let mut game = mastermind::MastermindGame::new(mastermind::DEFAULT_ATTEMPTS);
    game.play();
}
