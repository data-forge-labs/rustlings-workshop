# Section 1: Foundations — From Python Loops to Rust Safety

*Getting started: syntax, types, control flow, and your first Rust programs.*

## Prerequisites

- Rust installed (see [0-Intro](./0-Intro/README.md))
- Basic Python knowledge
- Familiarity with terminal/command line

## Projects in This Section

| # | Project | Concepts | Format |
|---|---------|----------|--------|
| 0 | **Intro** — Rust primer, syntax, `fn main` | `fn main()`, `let`, `mut`, `println!`, basic types, `&str`, arithmetic | Reference |
| 1 | **BasicCalculator** — integers, branching, loops, overflow | `i32`/`u32`, `if`/`else`, `while`/`for`, panics, overflow, saturating arithmetic, `as` casting | Tutorial |
| 2 | **MasterMind** — guess a 4-digit secret code | `struct`, `impl`, `Vec<T>`, `Option<T>`, `match`, `String`/`&str`, `rand`, iterators, console I/O | Project |
| 32 | **Week1FinalReflection** — data structures mindset | Memory safety, zero-cost abstractions, Rust vs Python | Reflection |

## Learning Path

1. Start with **0-Intro** to get Rust installed and write your first program
2. Move to **1-BasicCalculator** to learn integers, control flow, and loops
3. Build **2-MasterMind** to apply everything in a real game
4. Finish with **32-Week1FinalReflection** to solidify the concepts

## Key Concepts Covered

| Rust Concept | Python Equivalent | Why It Matters |
|---|---|---|
| `let` / `let mut` | Variable assignment | Control mutability |
| `i32`, `u32`, `f64` | `int`, `float` | Type safety, memory efficiency |
| `if` / `else` as expression | `if` / `else` (statement) | Functional style |
| `while` / `for` loops | `while` / `for` | Iteration |
| `panic!` | `raise Exception` | Unrecoverable errors |
| Saturating arithmetic | N/A (unbounded ints) | Safe overflow handling |
| `struct` / `impl` | `class` | Organize data + behavior |
| `Vec<T>` | `list` | Dynamic collections |
| `Option<T>` | `None` / `Optional` | Safe null handling |
