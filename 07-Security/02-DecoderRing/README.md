# Decoder Ring — Python to Rust Workshop

> **Test-driven approach**: This project includes a Cargo project with progressive
> unit tests. Each function in `workshop/src/lib.rs` starts as a `todo!()` stub. As you
> follow each section, replace `todo!()` with real code and run `cd workshop && cargo test` to
> watch the pass count grow. Your goal: **all 7 tests pass**.

## Why This Project?

### The Problem

Python frequency analysis for Caesar ciphers is straightforward but slow on large corpora — each shift decrypts and scores sequentially while CPU cores sit idle:

```python
from collections import Counter
import concurrent.futures

def crack(text, depth=26):
    results = []
    for shift in range(depth):
        decrypted = decrypt(text, shift)
        score = score_text(decrypted)
        results.append((shift, decrypted, score))
    return max(results, key=lambda x: x[2])
```

```
Python sequential brute-force:
  Shift 0 -> decrypt -> score
  Shift 1 -> decrypt -> score     <- one at a time
  Shift 2 -> decrypt -> score
  ... up to 25                    <- CPU cores idle!
```

Parallelising with `ProcessPoolExecutor` adds boilerplate, pickling overhead, and manual thread management.

### The Rust Solution

Rust's Rayon library transforms sequential iteration into parallel execution with one method call:

```rust
use rayon::prelude::*;

pub fn guess_shift_parallel(text: &str, depth: usize) -> (usize, String, f32) {
    (0..depth)
        .into_par_iter()  // <- automatic work-stealing across CPU cores
        .map(|shift| {
            let decrypted = decrypt(text, shift);
            let score = score_text(&decrypted, &freqs);
            (shift, decrypted, score)
        })
        .max_by(|a, b| a.2.partial_cmp(&b.2).unwrap())
        .unwrap()
}
```

`into_par_iter()` replaces `into_iter()`. Rayon handles work distribution and load balancing automatically — no thread pools, no futures, no manual scheduling.

## What You'll Learn

| # | Concept | Rust Type / Module | Python Equivalent | Purpose |
|---|---------|--------------------|------------------|---------|
| 1 | HashMap<char, f32> | std::collections::HashMap | `dict` | Store English letter frequency table |
| 2 | Pattern matching on char ranges | `match` on `'a'..='z'` | `if/elif` chains with `ord()`/`chr()` | Classify letters for Caesar shifting |
| 3 | chars().map().collect() | Iterator adapters | List comprehension + `str.join()` | Transform strings functionally |
| 4 | Functional iteration with max_by | `Iterator::max_by` | `max(key=...)` | Find the best-scoring shift |
| 5 | Rayon parallel iterators | `rayon::prelude::*` | `concurrent.futures.ProcessPoolExecutor` | Parallel brute-force across all 26 shifts |
| 6 | Filter-map accumulator | `filter` / `map` / `sum` | `collections.Counter` | Frequency scoring against English baseline |
| 7 | ASCII byte arithmetic | `u8` wrapping with `% 26` | `ord()` / `chr()` modulo | Caesar cipher letter shifting |

## Concepts at a Glance

**1. HashMap<char, f32>** — In Python you use a `dict` for frequency tables; Rust's `HashMap` is the same concept but with explicit type parameters `HashMap<char, f32>`. The `entry()` API replaces Python's `defaultdict` pattern.

**2. Pattern matching on char ranges** — Rust's `match` with range patterns (`'a'..='z'`) is more concise than Python's `if 'a' <= ch <= 'z'` chains and is verified by the compiler for exhaustiveness.

**3. chars().map().collect()** — Python uses list comprehensions with `str.join()`; Rust chains iterator adapters lazily. Nothing executes until `.collect()` materialises the result.

**4. Functional iteration with max_by** — Python's `max(results, key=lambda x: x[2])` becomes Rust's `.max_by(|a, b| a.2.partial_cmp(&b.2).unwrap())`. `partial_cmp` is needed because `f32` has NaN which breaks total ordering.

**5. Rayon parallel iterators** — Python requires `ProcessPoolExecutor` with manual `submit()` and `result()` gathering. Rust's `into_par_iter()` is a single method change. Rayon uses work-stealing for automatic load balancing across CPU cores.

**6. Filter-map accumulator** — Python's `Counter` counts automatically; Rust builds counts explicitly with `entry().or_insert()` then divides for percentages. Both follow the same accumulator pattern.

**7. ASCII byte arithmetic** — Python's `ord()`/`chr()` work on Unicode code points; Rust's `u8` byte arithmetic is faster and explicit about ASCII-only scope. The `% 26` wrap-around is identical in both languages.

---

## Table of Contents

1. [Introduction](#1-introduction)
2. [Prerequisites](#2-prerequisites)
3. [Concept: English Letter Frequencies](#3-concept-english-letter-frequencies)
4. [Concept: Caesar Cipher Decryption](#4-concept-caesar-cipher-decryption)
5. [Concept: Frequency Scoring](#5-concept-frequency-scoring)
6. [Concept: Brute-Force Shift Detection](#6-concept-brute-force-shift-detection)
7. [Concept: Parallel Cracking with Rayon](#7-concept-parallel-cracking-with-rayon)
8. [Putting It All Together](#8-putting-it-all-together)
9. [Complete Code Reference](#9-complete-code-reference)
10. [Summary](#10-summary)

## 1. Introduction

In this workshop you will build a tool that cracks Caesar ciphers using
statistical frequency analysis -- and then parallelises it with Rayon. A
Caesar cipher shifts each letter by a fixed number of positions. By comparing
letter frequencies in the encrypted text against known English frequencies,
you can automatically discover the shift without knowing it in advance.

In Python, you would use `collections.Counter` to count letters and score them
against a frequency table. Rust's `HashMap` and iterators accomplish the same
goal, and its Rayon library lets you parallelise the search across all
available CPU cores with minimal code changes.

**Data-engineering motivation**: Frequency analysis is a foundational
technique for data profiling and anomaly detection in text pipelines.
Parallel search over a parameter space (like trying all 26 shifts) is a
pattern that appears in hyperparameter tuning, log parsing, and pattern
matching at scale.

## 2. Prerequisites

- Completed [Section 3: Collections](../../03-Collections/README.md) --
  `HashMap`, iterators
- Understanding of closures and functional iteration
- Basic familiarity with `cd workshop && cargo run` and CLI arguments

## 3. Concept: English Letter Frequencies

### Explanation

The first step is building a `HashMap<char, f32>` that maps each English
letter to its expected frequency percentage. In English text, 'e' is the most
common letter (~12.7%), followed by 't' (~9.1%), 'a' (~8.2%), and so on.

In Python, you would write:

```python
from collections import Counter
freqs = Counter("the quick brown fox jumps over the lazy dog")
# then normalise to percentages
```

In Rust, we construct the map directly:

```rust
pub fn gen_counts() -> HashMap<char, f32> {
    let mut freqs = HashMap::new();
    freqs.insert('e', 12.70);
    freqs.insert('t', 9.06);
    // ... and the rest
    freqs
}
```

The Python `Counter` automatically counts occurrences. In Rust we use known
standard frequencies because they give better statistical accuracy than any
single sample text.

### Applying to Our Project

Implement `gen_counts()` to return a `HashMap` with all 26 letters and their
standard English frequency percentages. The test confirms the map contains 'e',
't', and 'a'.

## 4. Concept: Caesar Cipher Decryption

### Explanation

A Caesar cipher shifts each letter forward by a fixed amount. To decrypt, you
shift backward. For example, with shift 3: "hello" encrypts to "khoor", and
decrypting "khoor" with shift 3 recovers "hello". Letters wrap around the
alphabet: shifting 'a' back by 1 gives 'z'.

In Python:

```python
def decrypt(text: str, shift: int) -> str:
    result = []
    for ch in text:
        if 'a' <= ch <= 'z':
            shifted = chr(((ord(ch) - ord('a') - shift) % 26) + ord('a'))
            result.append(shifted)
        elif 'A' <= ch <= 'Z':
            shifted = chr(((ord(ch) - ord('A') - shift) % 26) + ord('A'))
            result.append(shifted)
        else:
            result.append(ch)
    return "".join(result)
```

In Rust, we use `chars()` to iterate and pattern matching to classify
characters:

```rust
pub fn decrypt(text: &str, shift: usize) -> String {
    text.chars().map(|c| match c {
        'a'..='z' => (((c as u8 - b'a' - shift as u8) % 26) + b'a') as char,
        'A'..='Z' => (((c as u8 - b'A' - shift as u8) % 26) + b'A') as char,
        _ => c,
    }).collect()
}
```

Key differences from Python:
- Rust uses `u8` arithmetic on byte values, wrapping with `% 26`
- `chars().map().collect()` is the idiomatic functional pipeline
- Non-alphabetic characters (spaces, punctuation) pass through unchanged

### Applying to Our Project

Implement `decrypt()` that applies a reverse Caesar shift. The test checks
basic shift, wrap-around (shift 3 on "abc" produces "xyz"), empty strings,
and that non-alpha characters are preserved.

## 5. Concept: Frequency Scoring

### Explanation

To score how "English-like" a decrypted text is, count its letter frequencies
and compare them to the known English frequencies. The score is the sum of
expected frequency percentages for each letter that appears.

In Python:

```python
def score_text(text, freqs):
    counts = Counter(c.lower() for c in text if c.isalpha())
    total = sum(counts.values())
    if total == 0:
        return 0.0
    return sum(freqs.get(ch, 0.0) * (count / total) for ch, count in counts.items()) / 100.0
```

In Rust, we iterate, filter, and accumulate:

```rust
pub fn score_text(text: &str, freqs: &HashMap<char, f32>) -> f32 {
    let letters: Vec<char> = text.chars()
        .filter(|c| c.is_ascii_alphabetic())
        .map(|c| c.to_ascii_lowercase())
        .collect();
    let total = letters.len() as f32;
    if total == 0.0 { return 0.0; }
    let mut counts = HashMap::new();
    for ch in &letters { *counts.entry(*ch).or_insert(0.0) += 1.0; }
    counts.iter()
        .map(|(ch, &cnt)| freqs.get(ch).copied().unwrap_or(0.0) * cnt / total)
        .sum::<f32>()
}
```

### Applying to Our Project

Implement `score_text()`. The test verifies that English-like text scores
higher than random gibberish.

## 6. Concept: Brute-Force Shift Detection

### Explanation

Now we try every possible shift (0 through 25), decrypt the text with each,
score it, and return the best result.

In Python:

```python
def guess_shift(text, depth=26):
    best_shift, best_text, best_score = 0, "", -1.0
    for shift in range(depth):
        decrypted = decrypt(text, shift)
        score = score_text(decrypted, freqs)
        if score > best_score:
            best_shift, best_text, best_score = shift, decrypted, score
    return best_shift, best_text, best_score
```

In Rust, we use functional iteration to find the maximum:

```rust
pub fn guess_shift(text: &str, depth: usize) -> (usize, String, f32) {
    let freqs = gen_counts();
    (0..depth)
        .map(|shift| {
            let decrypted = decrypt(text, shift);
            let score = score_text(&decrypted, &freqs);
            (shift, decrypted, score)
        })
        .max_by(|a, b| a.2.partial_cmp(&b.2).unwrap())
        .unwrap()
}
```

### Applying to Our Project

Implement `guess_shift()`. The test encrypts "hello" with shift 23, then
verifies the function detects shift 3 (26 - 23 = 3) as correct.

## 7. Concept: Parallel Cracking with Rayon

### Explanation

Rayon is a data-parallelism library for Rust. It converts sequential iterators
into parallel ones with a single method call: `into_par_iter()` replaces
`into_iter()`.

```rust
use rayon::prelude::*;

pub fn guess_shift_parallel(text: &str, depth: usize) -> (usize, String, f32) {
    let freqs = gen_counts();
    (0..depth)
        .into_par_iter()  // <-- the only change
        .map(|shift| {
            let decrypted = decrypt(text, shift);
            let score = score_text(&decrypted, &freqs);
            (shift, decrypted, score)
        })
        .max_by(|a, b| a.2.partial_cmp(&b.2).unwrap())
        .unwrap()
}
```

In Python, you would use `concurrent.futures.ProcessPoolExecutor`:

```python
with ProcessPoolExecutor() as executor:
    futures = [executor.submit(try_shift, text, shift, freqs)
               for shift in range(depth)]
    best = max((f.result() for f in futures), key=lambda x: x[2])
```

Rayon is simpler because it handles work-stealing, thread pools, and data
distribution automatically. All 26 shifts are evaluated across all CPU cores.

### Applying to Our Project

Implement `guess_shift_parallel()` by adding `into_par_iter()` to the
`guess_shift` implementation. The function signature is identical so callers
swap between sequential and parallel freely. Run `cargo bench` to compare
performance.

## 8. Putting It All Together

Open `workshop/src/lib.rs` and replace each `todo!()`:

**Step 1 (Frequencies):** Implement `gen_counts()` with all 26 letter
frequencies. Tests: 1 passes.

**Step 2 (Decryption):** Implement `decrypt()` with ASCII letter shifting
and wrap-around. Tests: 4 more pass (total 5).

**Step 3 (Scoring):** Implement `score_text()` with frequency comparison.
Tests: 1 more pass (total 6).

**Step 4 (Guess):** Implement `guess_shift()` using brute-force search.
Tests: 1 more pass (total 7).

Run `cd workshop && cargo test` after each step. For extra credit, implement
`guess_shift_parallel()` using Rayon and compare with `cargo bench`.

To use the CLI:

```bash
cd workshop && cargo run -- --message "Ypp dy dro lexuob. Ofobi zobcyx pyb drowcovfoc" --guess
```

## 9. Complete Code Reference

```rust
use std::collections::HashMap;

pub fn gen_counts() -> HashMap<char, f32> {
    let mut freqs = HashMap::new();
    freqs.insert('a', 8.17);  freqs.insert('b', 1.49);
    freqs.insert('c', 2.78);  freqs.insert('d', 4.25);
    freqs.insert('e', 12.70); freqs.insert('f', 2.23);
    freqs.insert('g', 2.02);  freqs.insert('h', 6.09);
    freqs.insert('i', 6.97);  freqs.insert('j', 0.15);
    freqs.insert('k', 0.77);  freqs.insert('l', 4.03);
    freqs.insert('m', 2.41);  freqs.insert('n', 6.75);
    freqs.insert('o', 7.51);  freqs.insert('p', 1.93);
    freqs.insert('q', 0.10);  freqs.insert('r', 5.99);
    freqs.insert('s', 6.33);  freqs.insert('t', 9.06);
    freqs.insert('u', 2.76);  freqs.insert('v', 0.98);
    freqs.insert('w', 2.36);  freqs.insert('x', 0.15);
    freqs.insert('y', 1.97);  freqs.insert('z', 0.07);
    freqs
}

pub fn decrypt(text: &str, shift: usize) -> String {
    text.chars().map(|c| match c {
        'a'..='z' => (((c as u8 - b'a' - shift as u8) % 26) + b'a') as char,
        'A'..='Z' => (((c as u8 - b'A' - shift as u8) % 26) + b'A') as char,
        _ => c,
    }).collect()
}

pub fn score_text(text: &str, freqs: &HashMap<char, f32>) -> f32 {
    let letters: Vec<char> = text.chars()
        .filter(|c| c.is_ascii_alphabetic())
        .map(|c| c.to_ascii_lowercase())
        .collect();
    let total = letters.len() as f32;
    if total == 0.0 { return 0.0; }
    let mut counts = HashMap::new();
    for ch in &letters { *counts.entry(*ch).or_insert(0.0) += 1.0; }
    counts.iter()
        .map(|(ch, &cnt)| freqs.get(ch).copied().unwrap_or(0.0) * cnt / total)
        .sum::<f32>()
}

pub fn guess_shift(text: &str, depth: usize) -> (usize, String, f32) {
    let freqs = gen_counts();
    (0..depth)
        .map(|shift| {
            let decrypted = decrypt(text, shift);
            let score = score_text(&decrypted, &freqs);
            (shift, decrypted, score)
        })
        .max_by(|a, b| a.2.partial_cmp(&b.2).unwrap())
        .unwrap()
}

pub fn guess_shift_parallel(text: &str, depth: usize) -> (usize, String, f32) {
    use rayon::prelude::*;
    let freqs = gen_counts();
    (0..depth)
        .into_par_iter()
        .map(|shift| {
            let decrypted = decrypt(text, shift);
            let score = score_text(&decrypted, &freqs);
            (shift, decrypted, score)
        })
        .max_by(|a, b| a.2.partial_cmp(&b.2).unwrap())
        .unwrap()
}
```

## 10. Summary

| Concept | Python Equivalent | Where Used |
|---|---|---|
| `HashMap<char, f32>` | `dict` for frequency table | `gen_counts` |
| `chars().map().collect()` | `for` loop over string | `decrypt` |
| Functional iteration | `max(key=...)` | `guess_shift` |
| Rayon parallel iterators | `concurrent.futures.ProcessPoolExecutor` | `guess_shift_parallel` |
| Letter frequency scoring | `collections.Counter` + manual score | `score_text` |
