# AGENTS.md — Workshop Designer Instructions

This file describes how to systematically design new Rust workshops for the **Rust Tutorial — Learn by Doing** series. It ensures consistency, completeness, and a smooth learner experience.

---

## 1. Overall System

The repository contains:

- `README.md` – the main index, project table, and **Rust Concepts Coverage** checklist.
- `AGENTS.md` – this file.
- One folder per workshop, named `N-ProjectName/`, containing:
  - `project.py` – the original Python implementation the learner can run (present only in 100-exercises projects).
  - `README.md` – the full step‑by‑step Rust tutorial (the main learning file).

The workshops are ordered; later workshops may assume concepts from earlier ones.

---

## 2. Agent Workflow (for each new workshop)

1. **Read `README.md`** and extract the current coverage table.
2. **Identify gaps** – find Rust concepts marked `❌` that have not been introduced yet.
3. **Select a group of 3–6 concepts** that:
   - Form a natural teaching cluster (e.g., `Result` + `?`, `enum` + `match`, `HashMap` + `entry`).
   - Do not depend on other untaught concepts (or can be explained with minimal forward references).
   - Can be demonstrated in a practical, small Python‑to‑Rust project.
4. **Design a project** (name it `N-ProjectName` where N is the next number) that uses those concepts. The project must:
   - Be a real, self‑contained Python script that works and is educational.
   - Be convertible to Rust in a step‑by‑step manner.
   - Follow the workshop structure defined in Section 5.
5. **Create the folder** `N-ProjectName/` and populate:
   - `project.py` – the Python version (with comments explaining the logic, if applicable).
   - `README.md` – the Rust tutorial.
6. **Update `README.md`**:
   - Add a new row to the **Projects** table with number, name, and a concise list of the new Rust concepts.
   - Update the **Rust Concepts Coverage** table: mark each introduced concept as ✅ and set “First Project” to `N`.
7. **Verify** that all newly covered concepts are listed in the project row and the coverage table.

---

## 3. How to Choose Concepts

When selecting concepts, consider the following dependencies:

| Concept                | Prerequisites                          |
|------------------------|----------------------------------------|
| `Result<T, E>`, `?`    | `Option<T>`, basic error handling      |
| `enum` (custom)        | basic types, `match`                   |
| `match` advanced       | basic `match`                          |
| `HashMap`              | `Vec`, ownership, borrowing            |
| `HashSet`              | `HashMap` / `Vec`                      |
| File I/O               | `Result`, `String`, `Vec`              |
| `derive` macros        | `struct`, traits understanding         |
| Generics & traits      | `struct`, `impl`                       |
| Lifetimes              | ownership, references                  |
| Serde                  | `derive`, `Result`, `File`             |
| Concurrency            | ownership, `Arc`, `Mutex`              |
| `async`/`.await`       | `Future`, I/O, `tokio`                 |

Always pick concepts that build on already‑covered ones, unless you plan to explain the prerequisite inline (which is acceptable for simple ones).

---

## 4. Project Idea Inspiration

A good workshop project:

- Is a tiny, self‑contained application (CLI tool, simple game, utility).
- Can be written in ~50–200 lines of Python.
- Has clear I/O (console, file, network) and logic separation.

Examples for future workshops:

- `Result` + `?` + `enum` → a simple command‑line calculator that parses expressions.
- `HashMap` + `Iterator` adapters → word frequency counter.
- File I/O + `Result` + `serde` → JSON config reader/writer.
- `HashSet` + `enum` → duplicate detector.
- `derive` + `Display` + `From` → a small logging library.
- Concurrency + `Arc` + `Mutex` → a multi‑threaded counter or chat.

---

## 5. Workshop File Structure (`README.md`)

The workshop must be a Markdown file that follows this template:

```markdown
# 🦀 Project Name — Python to Rust Workshop

## Table of Contents
1. [Introduction](#1-introduction)
2. [Prerequisites](#2-prerequisites)
3. [Running the Python Version](#3-running-the-python-version)
4. [Concept: ...](#4-concept-...)   ← one section per concept
5. [Concept: ...](#5-concept-...)
...
X. [Putting It All Together](#X-putting-it-all-together)
Y. [Complete Code Reference](#Y-complete-code-reference)
Z. [Summary](#Z-summary)

## 1. Introduction
Briefly describe the project, what the Python script does, and which new Rust concepts will be learned.

## 2. Prerequisites
List previously covered concepts and the required tools.

## 3. Running the Python Version
Show how to run the provided `project.py` and explain its behaviour.

## 4. Concept: [Name]
### Explanation
Explain the Rust concept in plain language, with small code snippets, diagrams (ASCII art or tables), and comparisons to Python.

### Example (stand‑alone)
A tiny Rust program that illustrates the concept, completely independent of the workshop project.

### Applying to Our Project
Show exactly how the concept will be used in the upcoming Rust code. Provide the relevant code excerpt from the project.

## 5. Concept: [Name]
(same structure)

...

## X. Putting It All Together
Now walk through building the Rust project file‑by‑file (or section‑by‑section), integrating all concepts. Do not repeat full concept explanations; instead, refer back to the dedicated sections. Provide the complete code as the learner writes it, with comments linking to the concepts.

## Y. Complete Code Reference
Include the full final Rust source code (e.g., `src/main.rs` or `src/lib.rs` + `src/main.rs`) so the learner can verify.

## Z. Summary
Table listing the new concepts covered, with short descriptions and where they were used.
```

**Important:** The “Concept” sections must come **before** the final assembly, so the learner understands each tool before using it.

---

## 6. Python Sample Requirements

The `project.py` file must:

- Be a working, runnable Python 3 script.
- Use only the standard library (or a very common library if absolutely necessary).
- Be well‑commented to explain the logic.
- Serve as the specification for the Rust version.

---

## 7. Updating README.md

When the workshop is ready, update `README.md`:

- In the **Projects** table, add a new row like:

  ```markdown
  | 2 | **Word Counter** — count word frequencies from a file | `HashMap`, `File`, `BufReader`, `Result`, `?` |
  ```

- In the **Rust Concepts Coverage** table, change each newly covered concept from `❌` to `✅` and set “First Project” to the workshop number.

- If a concept was touched briefly but will be deepened later, keep it as ❌ and note it in the workshop’s summary.

---

## 8. Final Check

Before finalising, verify:

- The workshop runs end‑to‑end (i.e., someone can follow it and compile the Rust code).
- The Python script is correct and matches the described behaviour.
- The coverage table is accurate.
- No other sections of the README are broken.

---
