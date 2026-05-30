/// The hidden 4-digit code with hint-tracking state.
pub struct SecretCode {
    digits: Vec<u8>,
    revealed_positions: Vec<bool>,
    revealed_digits: Vec<bool>,
}

impl SecretCode {
    /// Creates a new random 4-digit code with no hints revealed.
    pub fn new() -> Self {
        todo!()
    }

    /// Creates a code from an explicit digit vector (for testing).
    pub fn from_digits(digits: Vec<u8>) -> Self {
        todo!()
    }

    /// Compares a guess (exactly 4 digits) with the secret.
    /// Returns (green, yellow, red) — exact matches, wrong-position, none.
    pub fn evaluate_guess(&self, guess: &str) -> (usize, usize, usize) {
        todo!()
    }

    /// Returns true if at least one position hint remains.
    pub fn can_give_position_hint(&self) -> bool {
        todo!()
    }

    /// Returns true if at least one digit hint remains.
    pub fn can_give_digit_hint(&self) -> bool {
        todo!()
    }

    /// Reveals one unrevealed position. Returns Some((index, digit)) or None.
    pub fn give_position_hint(&mut self) -> Option<(usize, u8)> {
        todo!()
    }

    /// Reveals one unrevealed digit (without position). Returns Some(digit) or None.
    pub fn give_digit_hint(&mut self) -> Option<u8> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn known_secret() -> SecretCode {
        SecretCode {
            digits: vec![1, 2, 3, 4],
            revealed_positions: vec![false; 4],
            revealed_digits: vec![false; 10],
        }
    }

    #[test]
    fn test_new_has_four_digits() {
        let s = SecretCode::new();
        assert_eq!(s.digits.len(), 4);
    }

    #[test]
    fn test_new_digits_unique() {
        let s = SecretCode::new();
        let mut seen = [false; 10];
        for &d in &s.digits {
            assert!(!seen[d as usize]);
            seen[d as usize] = true;
        }
    }

    #[test]
    fn test_evaluate_all_green() {
        let s = known_secret();
        assert_eq!(s.evaluate_guess("1234"), (4, 0, 0));
    }

    #[test]
    fn test_evaluate_all_yellow() {
        let s = known_secret();
        assert_eq!(s.evaluate_guess("4321"), (0, 4, 0));
    }

    #[test]
    fn test_evaluate_no_match() {
        let s = known_secret();
        assert_eq!(s.evaluate_guess("5678"), (0, 0, 4));
    }

    #[test]
    fn test_position_hint_exhausts() {
        let mut s = known_secret();
        for _ in 0..4 { s.give_position_hint(); }
        assert!(s.give_position_hint().is_none());
    }
}
