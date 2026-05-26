import random

class SecretCode:
    def __init__(self):
        self.digits = random.sample(range(10), 4)
        self._revealed_positions = set()
        self._revealed_digits = set()

    def evaluate_guess(self, guess: str) -> tuple[int, int, int]:
        guess_digits = [int(ch) for ch in guess]
        secret = self.digits.copy()

        green = sum(1 for g, s in zip(guess_digits, secret) if g == s)

        unmatched_secret = [s for g, s in zip(guess_digits, secret) if g != s]
        unmatched_guess = [g for g, s in zip(guess_digits, secret) if g != s]

        yellow = 0
        for g in unmatched_guess:
            if g in unmatched_secret:
                yellow += 1
                unmatched_secret.remove(g)
        red = 4 - green - yellow
        return green, yellow, red

    def can_give_position_hint(self) -> bool:
        return len(self._revealed_positions) < 4

    def can_give_digit_hint(self) -> bool:
        return len(self._revealed_digits) < 4

    def give_position_hint(self) -> tuple[int, int] | None:
        if not self.can_give_position_hint():
            return None
        available = [i for i in range(4) if i not in self._revealed_positions]
        pos = random.choice(available)
        self._revealed_positions.add(pos)
        return pos, self.digits[pos]

    def give_digit_hint(self) -> int | None:
        if not self.can_give_digit_hint():
            return None
        available = [d for d in self.digits if d not in self._revealed_digits]
        digit = random.choice(available)
        self._revealed_digits.add(digit)
        return digit


class MastermindGame:
    DEFAULT_ATTEMPTS = 20
    HINT_POSITION_COST = 5
    HINT_DIGIT_COST = 3

    def __init__(self, max_attempts: int = DEFAULT_ATTEMPTS):
        self.secret = SecretCode()
        self.attempts_left = max_attempts
        self.guess_count = 0

    def play(self):
        self._display_welcome()
        while self.attempts_left > 0:
            print(f"\nAttempts left: {self.attempts_left}")
            guess = self._get_user_input()

            if guess == "help":
                self._handle_hint()
                continue

            self.guess_count += 1
            green, yellow, red = self.secret.evaluate_guess(guess)
            self._display_feedback(green, yellow, red)

            if green == 4:
                print(f"\n🎉 Congratulations! You cracked the code in {self.guess_count} actual guesses.")
                return

            self.attempts_left -= 1

        print(f"\n❌ Game Over! The secret code was {''.join(map(str, self.secret.digits))}.")

    def _display_welcome(self):
        print("=" * 40)
        print("   Welcome to Mastermind!")
        print("   Guess the 4-digit code (digits 0-9, no repeats)")
        print(f"   You have {self.attempts_left} attempts. Type 'help' for hints.")
        print("=" * 40)

    def _get_user_input(self) -> str:
        while True:
            user_input = input("Enter guess (or 'help'): ").strip().lower()
            if user_input == "help":
                return user_input
            if len(user_input) != 4 or not user_input.isdigit():
                print("Invalid input. Please enter exactly 4 digits (e.g., 1234).")
                continue
            if len(set(user_input)) != 4:
                print("Digits must be unique (no repeats). Try again.")
                continue
            return user_input

    def _display_feedback(self, green: int, yellow: int, red: int):
        print(f"🟢: {green}   🟡: {yellow}   🔴: {red}")

    def _handle_hint(self):
        if self.attempts_left <= 0:
            print("You don't have enough attempts to use a hint.")
            return

        pos_available = self.secret.can_give_position_hint()
        dig_available = self.secret.can_give_digit_hint()

        if not pos_available and not dig_available:
            print("All hints already revealed. No more help available.")
            return

        print("\n--- Hint Menu ---")
        menu_options = []
        if pos_available:
            print("1. Reveal one digit and its correct position (costs 5 attempts)")
            menu_options.append('1')
        else:
            print("1. (No more position hints available)")
        if dig_available:
            print("2. Reveal a correct digit (costs 3 attempts)")
            menu_options.append('2')
        else:
            print("2. (No more digit hints available)")

        choice = input("Choose 1 or 2 (or press Enter to cancel): ").strip()
        if choice not in menu_options:
            print("Hint cancelled.")
            return

        if choice == '1':
            cost = self.HINT_POSITION_COST
            if self.attempts_left < cost:
                print(f"Not enough attempts. You need {cost} but have {self.attempts_left}.")
                return
            pos, digit = self.secret.give_position_hint()
            self.attempts_left -= cost
            print(f"Hint: Digit {digit} is at position {pos+1}.")
            print(f"({cost} attempts deducted)")
        else:
            cost = self.HINT_DIGIT_COST
            if self.attempts_left < cost:
                print(f"Not enough attempts. You need {cost} but have {self.attempts_left}.")
                return
            digit = self.secret.give_digit_hint()
            self.attempts_left -= cost
            print(f"Hint: The code contains the digit {digit}.")
            print(f"({cost} attempts deducted)")

        if self.attempts_left <= 0:
            print("\nYou've used up your last attempts on a hint.")


if __name__ == "__main__":
    game = MastermindGame()
    game.play()
