pub mod secret;
pub mod game;

pub use game::MastermindGame;
pub use secret::SecretCode;

pub const DEFAULT_ATTEMPTS: u32 = 20;
pub const HINT_POSITION_COST: u32 = 5;
pub const HINT_DIGIT_COST: u32 = 3;
