# 🦀 Rust for Python Data Engineers — Guess the Number Game

> **Test-driven approach**: This project includes a Cargo project with progressive unit tests. Each function in `workshop/src/lib.rs` starts as a `todo!()` stub. As you follow each section, replace `todo!()` with real code and run `cd workshop && cargo test` to watch the pass count grow. The `workshop/src/main.rs` file provides a runnable game (calling the same functions) — use `cargo run` to play. Your goal: **all 20 tests pass**.

---

## What Is This Game?

The computer picks a random number between 1 and 100. You have 7 attempts to guess it. After each guess the program tells you "Too high!", "Too low!", or "You win!". It's the classic beginner game — and in Rust it's the smallest program that exercises several important concepts at once.

### Python version

```python
import random

secret = random.randint(1, 100)
attempts = 7

for attempt in range(1, attempts + 1):
    guess = int(input(f"Attempt {attempt}/{attempts} > "))
    if guess == secret:
        print("You win!")
        break
    elif guess > secret:
        print("Too high!")
    else:
        print("Too low!")
else:
    print(f"Out of attempts! The secret was {secret}.")
```

Five lines of logic — but Python hides a lot: `int()` can throw, strings are always safe, and there's only one `str` type. In Rust, each of those assumptions becomes an explicit concept.

### Rust version

```rust
let guess: u32 = input.trim().parse().expect("Please enter a number");
match check_guess(secret, guess) {
    GuessOutcome::Correct => println!("You win!"),
    GuessOutcome::TooHigh => println!("Too high!"),
    GuessOutcome::TooLow  => println!("Too low!"),
}
```

### Topics covered in this project

| # | Concept | Why it matters |
|---|---------|----------------|
| 1 | External crate (`rand`) | Adding dependencies — Rust's `pip install` |
| 2 | `String` vs `&str` | Owned text (heap) vs borrowed views — the biggest adjustment for Python devs |
| 3 | Custom `enum` | Type-safe "one of these" — compiler tracks every variant |
| 4 | `#[derive]` macros | Auto-implement `Debug`, `PartialEq`, etc. |
| 5 | `std::io` — `read_line`, `flush` | Reading from stdin with mutable buffers |
| 6 | `Result<T, E>` and `.parse()` | Rust's error type — compiler forces you to handle failures |
| 7 | `match` (basic) | Pattern-based dispatch on `Result` and enums |
| 8 | `?` operator | Propagate errors up without writing `match` blocks |
| 9 | `.expect("msg")` | Crash with a message on `Err` — fine for small programs |
| 10 | `continue` | Skip to next loop iteration |

> **Coming up next**: [03-BasicCalculator](../../03-BasicCalculator/README.md) deepens integers, overflow, and `panic!`. [04-MasterMind](../../04-MasterMind/README.md) then uses the same I/O and `match` ideas on a more complex game with `struct`, `Vec`, and `Option`.

---

## Table of Contents

1. [Project Overview](#1-project-overview)
2. [Prerequisites](#2-prerequisites)
3. [Concept: Custom `enum`](#3-concept-custom-enum)
4. [Concept: Adding an External Crate](#4-concept-adding-an-external-crate)
5. [Concept: `String` vs `&str`](#5-concept-string-vs-str)
6. [Concept: Reading Input — `std::io`](#6-concept-reading-input-stdio)
7. [Concept: `Result<T, E>` and `.parse()`](#7-concept-resultt-e-and-parse)
8. [Concept: The `?` Operator](#8-concept-the-operator)
9. [Putting It All Together — The Full Game](#9-putting-it-all-together-the-full-game)
10. [Exercises](#10-exercises)
11. [Summary](#11-summary)
12. [What's Next](#12-whats-next)

---

## 1. Project Overview

You'll build a small **Guess the Number** game. The computer picks a random integer, the player has a fixed number of attempts, and after each guess the program says "Too high", "Too low", or "Correct".

Along the way you'll learn **six new Rust concepts** the intro project deliberately skipped:

- `enum` (custom)
- `String` vs `&str`
- `std::io` and `read_line`
- `Result<T, E>`, `.parse()`, `.expect()`
- `match` (basic, for `Result` and enum)
- Adding an external crate (`rand`)

### What You'll Build

```text
I'm thinking of a number between 1 and 100.
You have 7 attempts. Good luck!
Attempt 1/7 > 50
  Too high!
Attempt 2/7 > 25
  Too low!
Attempt 3/7 > 37
  Too high!
...
You win! The secret was 33.
```

### Python → Rust

If you've written the same game in Python, the Rust version is longer on the screen but type-checked. The compiler refuses to let you forget to handle a parse error or pass the wrong kind of string.

---

## 2. Prerequisites

- Completed [01-Intro](../../01-Intro/README.md) — you should be comfortable with `let`, `let mut`, functions, `if/else`, `for` loops, tuples, and fixed-size arrays
- Rust installed (covered in 01-Intro §2)
- `cd workshop && cargo test` / `cargo run` is second nature

---

## 3. Concept: Custom `enum`

### What is an enum?

An `enum` (enumeration) is a type that says *"this value is exactly one of a fixed list of variants."* The classic example: a playing card is one of four suits. The compiler then tracks every variant for you.

```rust
enum GuessOutcome {
    Correct,
    TooHigh,
    TooLow,
}
```

A value of type `GuessOutcome` is **always exactly one of** those three things — never two, never zero. The variants are namespaced under the enum: `GuessOutcome::Correct`, not just `Correct`.

### Deriving common traits

In Python, when you define an Enum, you can print it and compare it with `==`, it just works. In Rust, basic types like `i32`, `bool`, and `String` come with printing and comparison built in. But for every **user-defined type** (like our `GuessOutcome` enum), you must explicitly grant these abilities. You can't print a value or compare two values unless you (or the compiler) implement the ability first. Some of these actions are so common that the compiler offers to do the work for you.

Above the enum, you'll see `#[derive(...)]` — that asks the compiler to implement such common functions for us via traits, a mechanism for defining the functions we need on data types:

```rust
#[derive(Debug, PartialEq)]
pub enum GuessOutcome {
    Correct,
    TooHigh,
    TooLow,
}
```

**What is a trait?** A trait is a named collection of method signatures — like an interface or ABC class in Python. You'll learn traits in depth in [02-Traits](../../02-Traits/README.md). For now, just know: `Debug` is a trait that defines how to print a value, and `PartialEq` defines how to compare with `==`.

When you write `#[derive(Debug)]`, you ask the compiler: "implement the `Debug` trait for my type." When you write `#[derive(PartialEq)]`, you ask it to implement `==`. You could write these implementations by hand, but `derive` auto-generates the boilerplate:

```rust
let outcome = GuessOutcome::TooHigh;
println!("{:?}", outcome);  // prints: TooHigh — Debug makes this work

let a = GuessOutcome::Correct;
let b = GuessOutcome::TooHigh;
assert!(a != b);  // PartialEq makes this work
```

Without `Debug`, `println!("{:?}", outcome)` is a compile error. Without `PartialEq`, `a != b` is a compile error. In Python these always work — in Rust, you must opt in.

**What is `{:?}`?** It's a format specifier that asks Rust to use the `Debug` trait's string representation. `{}` uses the `Display` trait (human-friendly); `{:?}` uses `Debug` (developer-friendly, shows variant names). You'll use `{:?}` constantly for logging and debugging.

### Python comparison

```python
# Python 3.10+
from enum import Enum

class GuessOutcome(Enum):
    Correct = "correct"
    TooHigh = "too_high"
    TooLow  = "too_low"
```

In Rust there's no runtime "what's the variant" string — the *type system* already knows. You just write `GuessOutcome::Correct`.

### Applying to our project

Look at `src/lib.rs` — `GuessOutcome` is already defined with `#[derive(Debug, PartialEq)]`. You'll use it in `check_guess` and `hint_for`.

```rust
pub fn check_guess(secret: u32, guess: u32) -> GuessOutcome {
    if guess == secret {
        GuessOutcome::Correct
    } else if guess > secret {
        GuessOutcome::TooHigh
    } else {
        GuessOutcome::TooLow
    }
}
```

A function returning an enum gives the compiler a *closed set* to check — there's no way to "return a value that's not one of these three."

### Exercise

After you read the next sections, the `check_guess` stub in `src/lib.rs` is one of the first you'll implement. Skip to [§9 Putting It All Together](#9-putting-it-all-together-the-full-game) when you're ready, or continue reading the concepts in order.

---

## 4. Concept: Adding an External Crate

### What is a crate?

A **crate** is a Rust library or program. The standard library is always available (`std`); **external crates** come from [crates.io](https://crates.io) — Rust's package registry, the moral equivalent of PyPI.

### `Cargo.toml` is the package file

You declared `Cargo.toml` in 01-Intro §2. It already combines the role of `pyproject.toml` and `requirements.txt`. To use an external crate, add it under `[dependencies]`:

```toml
[package]
name = "guess_game"
version = "0.1.0"
edition = "2024"

[dependencies]
rand = "0.10"
```

That's it. The next `cargo build` will download `rand` and compile it. No separate `pip install` step.

> **Versioning**: `"0.10"` means "any 0.10.x version" — Cargo picks the latest compatible release. You can pin exactly with `"=0.10.1"` if needed.

### `cargo add` (alternative)

You can also add dependencies from the command line:

```bash
cargo add rand
```

This appends the latest version to `Cargo.toml` for you. Both approaches produce the same result.

To pin a specific version from the command line:

```bash
cargo add rand@0.10.1
```

### Using the crate

Once it's in `Cargo.toml`, the crate is available in your code. For `rand 0.10+`, the random-number function you'll use is:

```rust
let secret: u32 = rand::random_range(1..=100);
```

`random_range` takes a **range**:
- `1..100` — exclusive on the right (1 to 99)
- `1..=100` — inclusive on the right (1 to 100) ← what we want

This is the Rust equivalent of Python's `random.randint(1, 100)`.

### Python comparison

```python
import random
secret = random.randint(1, 100)
```

```rust
let secret: u32 = rand::random_range(1..=100);
```

Same idea, but the dependency was declared once in `Cargo.toml` and is now a typed import.

### Applying to our project

This project's `Cargo.toml` already declares `rand = "0.10"`. The `generate_secret` function in `src/lib.rs` is where you'll use it.

---

### A quick word on Stack vs Heap

Before we dive into `String` vs `&str`, here's the memory distinction that matters:

- **Stack** — fast, fixed-size data. Primitive types like `i32`, `bool`, `f64` live here. The compiler knows their size at compile time, so it can allocate and free them automatically.
- **Heap** — flexible, dynamically-sized data. Types like `String` and `Vec` store their content on the heap because the size can change at runtime.

```
┌─────────────── Stack ───────────────┐
│  let x: i32 = 42;       (4 bytes)  │
│  let flag: bool = true;  (1 byte)   │
├─────────────────────────────────────┤
│  let s: String = ...;               │
│  ┌─── ptr ──┐     ┌─── Heap ─────┐ │
│  │ address  │────>│ "hello"      │ │
│  └──────────┘     │ (5 bytes)    │ │
│                   └──────────────┘ │
└─────────────────────────────────────┘
```

In Python, *everything* lives on the heap and the garbage collector handles cleanup. In Rust, the compiler knows exactly when each value goes out of scope and frees it — no garbage collector needed. This is what **ownership** is about, and you'll learn it in depth in [Section 02: Ownership](../../02-Ownership/README.md).

> **Ownership note:** In Rust, values like `String` and `Vec` live on the heap, while primitive values (e.g., `i32`, `bool`) live on the stack. Ownership rules govern when heap data is cleaned up.

---

## 5. Concept: `String` vs `&str`

This is the **single biggest adjustment** for Python developers. In Python, `"hello"` is just a `str`. In Rust, there are **two** string types you'll meet on day one:

```
┌──────────────────────┐         ┌──────────────────────┐
│       String         │         │        &str          │
│      (owned)         │         │     (borrowed)       │
├──────────────────────┤         ├──────────────────────┤
│  Lives on the heap   │         │  A view into someone │
│  Can grow & shrink   │         │  else's data         │
│  You own it          │         │  You just borrow it  │
│  Made with           │         │  String literals     │
│  String::from("...") │         │  are &str            │
│  or String::new()    │         │                      │
└──────────────────────┘         └──────────────────────┘
        ▲                                 ▲
        │  &str = &my_string[..]          │
        └─────── one can borrow ──────────┘
```

### When you see each one

| Context | Type you get |
|---|---|
| A literal: `"hello"` | `&str` (the compiler stores the bytes statically) |
| `String::new()` | `String` (empty, owned, growable) |
| `String::from("hi")` | `String` (owned, from a literal) |
| `read_line(&mut buf)` writes into | `&mut String` (a mutable borrowed `String`) |
| A function parameter that just reads: `fn f(s: &str)` | `&str` (accepts both `&str` and `&String`) |

### The Python intuition

| Python | Closest Rust |
|---|---|
| `"hello"` (literal) | `&str` |
| `s = "hi"; s += " world"` | `String` (must be `let mut s = String::from("hi")`) |
| `def f(s): ...` | `fn f(s: &str)` (we always pass borrowed views in) |
| `def f(s: str)` (a type hint) | `fn f(s: String)` (owns it — rarely what you want as a parameter) |

### Applying to our project

In the game, we need a *growable* buffer to hold what the user types — that's a `String`. We also need *borrowed views* into that buffer (or into string literals) — those are `&str`.

```rust
// Create an empty, growable buffer
let mut input = String::new();
io::stdin().read_line(&mut input)?;        // appends into `input`

// Now `&input` is a &String; `&input[..]` is a &str
// Passing `&input` to a function taking &str works because
// of an automatic coercion called "deref coercion".
parse_guess(&input);
```

The `parse_guess(input: &str)` function takes a `&str` so it can be called with **both** string literals (`parse_guess("42")`) and slices of owned strings (`parse_guess(&input)`).

---

## 6. Concept: Reading Input — `std::io`

### The two pieces

```rust
use std::io;

let mut input = String::new();
io::stdin().read_line(&mut input).expect("failed to read line");
```

1. `io::stdin()` returns a handle to the standard input stream.
2. `.read_line(&mut input)` **appends** a line of text (including the trailing `\n`) into `input`. It returns a `Result<usize, io::Error>` — the count of bytes read, or an I/O error.

### Why a mutable reference?

`read_line` *modifies* `input` — it appends to it. So it needs `&mut input`, not just `&input`. (This is the same `let mut` you met in 01-Intro — except now the mutability is on a *reference*, not a value. We'll go much deeper on this in the [Ownership section](../../02-Ownership/README.md).)

### Flushing the prompt

```rust
print!("Attempt {attempt}/{ATTEMPTS} > ");
io::stdout().flush().expect("failed to flush stdout");
```

By default, terminals are line-buffered — output doesn't appear until a newline is printed. The `print!` macro (no newline) might stay in the buffer while the program blocks on `read_line`, so the user sees a blank line. `io::stdout().flush()` forces the buffer out.

```rust
use std::io::Write;  // brings `flush` into scope
```

### Python comparison

```python
guess = input("Attempt 1/7 > ")   # Python: prompt + read in one call
```

Rust separates them: `print!` for the prompt, `read_line` for the input. The `.expect` is a "crash if it fails" — `read_line` can fail in pathological cases (broken pipe, etc.).

---

## 7. Concept: `Result<T, E>` and `.parse()`

### `Result` is Rust's error type

```rust
enum Result<T, E> {
    Ok(T),    // success — carries the value
    Err(E),   // failure — carries the error
}
```

A function that can fail returns a `Result`. The compiler then **forces** you to look at the `Err` arm.

### `parse` returns `Result`

```rust
let guess: u32 = "42".parse().unwrap();  // Ok(42)
let guess: u32 = "abc".parse().unwrap(); // panic!
```

`"42".parse::<u32>()` returns `Ok(42)`. `"abc".parse::<u32>()` returns `Err(...)`. The `::<u32>` is a **type annotation** on the method — "parse this as a `u32`."

### Handling the Result

Three options you'll see in this course:

| Style | Behaviour | When to use |
|---|---|---|
| `.unwrap()` | Panics on `Err` (with a default message) | Quick scripts, "this can't fail" |
| `.expect("msg")` | Panics on `Err` with your message | The same, with a better error |
| `match` / `?` | Handle the error properly | Anything user-facing |

```rust
let guess: u32 = input.trim().parse().expect("Please enter a valid number");
```

In the game, the `expect` form is fine — if the player types "abc", we want the program to print our custom message and re-prompt. That's what `match` is for in the full game.

### `match` on `Result`

```rust
match parse_guess(&input) {
    Ok(guess)  => println!("You guessed {guess}"),
    Err(msg)   => println!("Bad input: {msg}"),
}
```

This is the same `match` shape you'll see in 04-MasterMind (and is **exhaustive** — you must handle both arms). The compiler will tell you if you forget `Err`.

### Python comparison

```python
try:
    guess = int(input)
except ValueError as e:
    print(f"Bad input: {e}")
```

```rust
match input.trim().parse::<u32>() {
    Ok(guess) => println!("You guessed {guess}"),
    Err(e)    => println!("Bad input: {e}"),
}
```

Same shape, but the `Err` arm is required by the type system, not a discipline you have to remember.

---

## 8. Concept: The `?` Operator

A small but powerful shortcut. When you have a function that returns `Result`, you can **propagate** an error up with `?`:

```rust
pub fn play_round(secret: u32, input: &str) -> Result<GuessOutcome, String> {
    let guess = parse_guess(input)?;  // if Err, return early with that error
    Ok(check_guess(secret, guess))
}
```

The `?` after `parse_guess(input)` means:

> "If the result is `Ok(value)`, bind it to `guess` and continue. If it's `Err(e)`, **return** `Err(e)` from this function immediately."

It only works in functions that return `Result` (or `Option`). It saves a whole `match` block.

### Python comparison

There isn't a direct Python equivalent — the closest is the implicit "re-raise" inside a `try/except`:

```python
def play_round(secret, raw):
    try:
        guess = int(raw)
    except ValueError as e:
        raise MyError(str(e))
    return check_guess(secret, guess)
```

`?` is the same idea in one character.

### Why we use it here

`play_round` composes `parse_guess` and `check_guess`. Without `?`, you'd write:

```rust
pub fn play_round(secret: u32, input: &str) -> Result<GuessOutcome, String> {
    let guess = match parse_guess(input) {
        Ok(g)  => g,
        Err(e) => return Err(e),  // explicit early return
    };
    Ok(check_guess(secret, guess))
}
```

`?` makes it one line. We'll use it more in the [Ownership section](../../02-Ownership/README.md).

---

## 9. Putting It All Together — The Full Game

Here's the entire `main.rs`. The lib.rs functions are reused throughout:

```rust
use std::io;
use std::io::Write; // for `flush()`

use guess_game::{check_guess, generate_secret, hint_for, parse_guess};

const MIN: u32 = 1;
const MAX: u32 = 100;
const ATTEMPTS: u32 = 7;

fn main() {
    let secret = generate_secret(MIN, MAX);

    println!("I'm thinking of a number between {MIN} and {MAX}.");
    println!("You have {ATTEMPTS} attempts. Good luck!");

    for attempt in 1..=ATTEMPTS {
        print!("Attempt {attempt}/{ATTEMPTS} > ");
        io::stdout().flush().expect("failed to flush stdout");

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("failed to read line");

        match parse_guess(&input) {
            Err(msg) => {
                println!("  ⚠ {msg}");
                println!("  (attempt not counted — try again)");
                continue;  // try the same attempt again
            }
            Ok(guess) => {
                let outcome = check_guess(secret, guess);
                println!("  {}", hint_for(outcome));
                if outcome == guess_game::GuessOutcome::Correct {
                    println!("You win! The secret was {secret}.");
                    return;
                }
            }
        }
    }

    println!("Out of attempts! The secret was {secret}.");
}
```

### Step-by-step walkthrough

1. `generate_secret(MIN, MAX)` calls into `lib.rs` to get a random `u32`. This is the only place the game depends on `rand`.
2. The `for attempt in 1..=ATTEMPTS` loop (covered in 01-Intro §8) gives the player 7 turns.
3. `print!` + `flush()` make the prompt appear immediately. Without `flush`, the user would see a blank line and the program would block on `read_line`.
4. `let mut input = String::new();` creates an empty growable buffer. `String::new()` is the equivalent of Python's `""`.
5. `read_line(&mut input)` appends a line of text (including `\n`) into `input`. The `&mut` is required because the function modifies the buffer.
6. `match parse_guess(&input)` handles both the success and failure paths. `parse_guess` already trims the input and reports a friendly error message.
7. On `Err`, we print a warning and `continue` to the next iteration of the loop — the player gets to retry the same attempt.
8. On `Ok(guess)`, we call `check_guess` to get the outcome, print the hint, and check for victory.
9. If the player runs out of attempts, the loop ends and the secret is revealed.

### What each concept is doing here

| Concept | Where it appears |
|---|---|
| `enum` + `#[derive]` | `GuessOutcome` in `lib.rs` |
| External crate | `rand = "0.10"` in `Cargo.toml` |
| `String` vs `&str` | `String::new()` for the buffer; `&str` parameter for `parse_guess` |
| `read_line(&mut input)` | Reading the player's guess |
| `io::stdout().flush()` | Making the prompt visible |
| `Result<T, E>` | Return type of `parse_guess`, `read_line` |
| `.expect("msg")` | Crashing on I/O errors (acceptable in a small game) |
| `match` on `Result` | Handling parse success vs failure |
| `?` operator | Inside `play_round` in `lib.rs` |
| `match` on `enum` | `check_guess` in `lib.rs` |
| `continue` | Skipping the rest of the iteration on bad input |
| `for` loop with `..=` | The attempt counter |

---

## 10. Exercises

Try these in order. Each builds on the last.

### Easy — Add a "warm / cold" hint

Extend `hint_for` to print a different message when the guess is within 5 of the secret (we'll keep the secret visible to the hint function for now). This is a small change to one function.

### Medium — Limit minimum and maximum

Add a `valid_guess(guess: u32) -> bool` function that returns `false` if the guess is outside `[MIN, MAX]`. Use it in the main game to reject out-of-range guesses without counting the attempt.

### Hard — Difficulty levels

Add a difficulty enum (`Easy`, `Normal`, `Hard`) that sets the number of attempts:

```rust
pub enum Difficulty {
    Easy,    // 10 attempts, range 1..=50
    Normal,  // 7 attempts,  range 1..=100
    Hard,    // 5 attempts,  range 1..=500
}
```

Implement `Difficulty::attempts(&self) -> u32` and `Difficulty::range(&self) -> (u32, u32)`. Then call them from `main`.

---

## 11. Summary

| Concept | Rust | Python equivalent |
|---|---|---|
| External crate | `rand = "0.10"` in `Cargo.toml` | `pip install` + `import` |
| Owned string | `String::new()`, `String::from("...")` | `""`, `str(...)` |
| Borrowed string | `&str` | n/a (always a `str`) |
| Custom enum | `enum X { A, B, C }` | `class X(Enum): A = ...` |
| `derive` | `#[derive(Debug, PartialEq)]` | n/a |
| Read input | `io::stdin().read_line(&mut buf)` | `input()` |
| Flush output | `io::stdout().flush()` | n/a (auto-flushed) |
| `Result<T, E>` | `Ok(v)` / `Err(e)` | try/except |
| Parse | `.parse::<u32>()` | `int(...)` |
| `expect` | `.expect("msg")` | uncaught exception |
| `?` operator | `let x = func()?;` | re-raise |
| `match` on `Result` | `match r { Ok(v) => ..., Err(e) => ... }` | `try/except` |
| `match` on enum | `match e { A => ..., B => ... }` | `match`/`case` (3.10+) |
| `continue` | `continue;` | `continue` |

---

## 12. What's Next

You now know how to read user input, parse it safely, dispatch on outcomes, and pull in an external crate. That's the **core of every interactive CLI** you'll build in Rust.

The next project, [03-BasicCalculator](../../03-BasicCalculator/README.md), takes a different angle: **integer-specific Rust**. It deepens what you already know with:

- Integer types: `i32` vs `u32` vs `i64` vs `usize` (Python only has one int)
- Integer overflow and the `panic!` macro
- `while` and `for` loops in practice
- The `as` keyword for type conversion
- Built-in unit testing with `#[test]` and `#[should_panic]`

After that, [04-MasterMind](../../04-MasterMind/README.md) brings everything together: `struct`, `Vec`, `Option`, more `match` patterns, and the same I/O loop you built here — this time on a 4-digit code with bull/cow hints.

Topics that come even later:

- **Ownership and borrowing** ([Section 02: Ownership](../../02-Ownership/README.md)) — Rust's central idea. This is the biggest mindset shift from Python.
- **Slices** `&[T]` and **borrowed views** — covered alongside ownership.
- **Collections** ([Section 03: Collections](../../03-Collections/README.md)) — `Vec`, `HashMap`, `HashSet`, iterators.
- **File I/O** ([Section 04: File I/O](../../04-FileIO/README.md)) — reading CSVs and Parquet.
- **Concurrency** ([Section 05: Concurrency](../../05-Concurrency/README.md)) — threads, async, channels.
- **Pattern Matching: @ Bindings and Guards** ([root appendix](../README.md#pattern-matching--bindings-and-guards)) — advanced `match` patterns with `@` bindings and guards.

Make sure all **20 tests pass** in `workshop/`, then move on to [03-BasicCalculator](../../03-BasicCalculator/README.md).

---

*Next up — Project 1.3: Basic Calculator. You'll go deeper on integer types, arithmetic, control flow, and error handling.*

## Exercises

* **Easy** – modify the existing function to handle an extra edge case.
* **Medium** – extend the project with a new helper function that re‑uses the core logic.

