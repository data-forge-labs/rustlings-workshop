use crate::secret::SecretCode;
use crate::{DEFAULT_ATTEMPTS, HINT_DIGIT_COST, HINT_POSITION_COST};

/// Manages game state: secret, attempts, hints, and player input.
pub struct MastermindGame {
    pub secret: SecretCode,
    pub attempts_left: u32,
    pub guess_count: u32,
}

impl MastermindGame {
    /// Creates a new game with a random secret.
    pub fn new(max_attempts: u32) -> Self {
        todo!()
    }

    /// Runs the main game loop.
    pub fn play(&mut self) {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::secret::SecretCode;

    #[test]
    fn test_game_new_has_secret() {
        let game = MastermindGame::new(10);
        assert_eq!(game.secret.digits.len(), 4);
    }

    #[test]
    fn test_game_new_attempts() {
        let game = MastermindGame::new(15);
        assert_eq!(game.attempts_left, 15);
    }

    #[test]
    fn test_game_new_guess_count_zero() {
        let game = MastermindGame::new(10);
        assert_eq!(game.guess_count, 0);
    }

    #[test]
    fn test_game_default_attempts() {
        let game = MastermindGame::new(DEFAULT_ATTEMPTS);
        assert_eq!(game.attempts_left, 20);
    }
}
