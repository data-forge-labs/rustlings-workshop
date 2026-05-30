# AGENTS.md — Rust Data Engineering Course Designer Instructions

**Role**: You are a **senior Rust data engineer** building a comprehensive Rust course for **Python data engineers** who are new to Rust. Write every workshop as if you are pairing with a junior data engineer — compare each concept to its Python equivalent, explain *why* Rust works differently, and always connect back to real data-engineering use cases (pipelines, ETL, file processing, concurrent workloads, production systems).

This file describes how to systematically maintain, extend, and improve the **Rust Tutorial — Learn by Doing** series. It ensures consistency, completeness, and a smooth learner experience across all 60 projects.

---

## 1. Course Mission

This repository is a **full Rust data engineering course** — not just a language tutorial. Every project, every explanation, every exercise must serve the goal of turning a Python data engineer into a productive Rust data engineer.

**Core principles:**
- **Python comparisons are mandatory** — every new Rust concept must be compared to its Python equivalent (e.g., `Vec` = list, `HashMap` = dict, `Result` = try/except, `Option` = None, `struct` = dataclass, `match` = match/case).
- **Data-engineering context** — whenever possible, use data-engineering examples: CSV/Parquet processing, ETL pipelines, concurrent data loading, graph analytics, file I/O, network services.
- **Progressive disclosure** — start simple, layer on complexity. Never introduce a concept before its prerequisites.
- **Production mindset** — teach error handling, testing, documentation, and safe concurrency from the start.

---

## 2. Repository Architecture

The repo is organized into **9 numbered concept sections**, each containing related projects:

```
RustTut/
├── README.md                       ← Main index, project tables, concept coverage checklist
├── AGENTS.md                       ← This file
├── .devcontainer/                  ← Preconfigured Rust dev environment
├── 01-Foundations/                 ← Section 1: syntax, types, control flow
│   ├── README.md                   ← Section overview with project table & learning path
│   ├── 0-Intro/                    ← Rust syntax primer
│   ├── 1-BasicCalculator/          ← Integers, branching, loops
│   ├── 2-MasterMind/               ← Structs, Vec, Option, console I/O
│   └── 32-Week1FinalReflection/    ← Reflection / review
├── 02-Ownership/                   ← Section 2: ownership, borrowing, traits, enums
│   ├── 3-TicketV1/                 ← Structs, ownership, stack vs heap
│   ├── 4-Traits/                   ← Trait definitions, derive, bounds
│   ├── 5-TicketV2/                 ← Enums, match, Result, error handling
│   ├── 37-OBRM/                    ← RAII / Drop / ownership-based resource mgmt
│   └── 38-OwnershipLifetimes/      ← Lifetimes, borrow checker
├── 03-Collections/                 ← Section 3: data structures & iterators
│   ├── 6-TicketManagement/         ← Vec, arrays, HashMap, BTreeMap, iterators
│   ├── 9-VectorFruitSalad/         ← Vec<T> dynamic arrays
│   ├── 10-ArrayFruitSalad/         ← Fixed-size arrays [T; N]
│   ├── 11-HashMapCount/            ← Word frequency counting
│   ├── 12-LinkedListFruitSalad/    ← Doubly-linked list
│   ├── 13-VecDequeFruitSalad/      ← Double-ended queue
│   ├── 15-HashMapLanguage/         ← Complex HashMap data
│   ├── 16-CollectionsLessonReflection/
│   ├── 17-RustCollectionsDoc/      ← std::collections reference
│   ├── 18-BinaryHeapFruit/         ← Priority queue
│   ├── 19-BTreeSetFruit/           ← Ordered set
│   ├── 23-HashSetFruit/            ← Unique items / set operations
│   ├── 28-RustIterators/           ← Lazy functional iteration
│   ├── 30-WhenToUseRustSet/        ← Collection selection guide
│   └── 36-MutableFruitSalad/       ← Vec mutation patterns
├── 04-FileIO/                      ← Section 4: file I/O & data formats
│   ├── 53-CSVCookbook/             ← Read/write CSV with csv crate
│   ├── 54-CSVWriter/               ← CSV writing with serde
│   ├── 55-Parquet/                 ← Apache Parquet / Arrow
│   └── 56-DataManagementLessonReflection/
├── 05-Concurrency/                 ← Section 5: threads, async, atomics
│   ├── 7-Threads/                  ← Threads, channels, locks (100-exercises)
│   ├── 8-Futures/                  ← async/await, tokio (100-exercises)
│   ├── 34-DataRace/                ← Mutex, Arc, data-race prevention
│   ├── 44-Atomics/                 ← Lock-free atomics
│   ├── 45-DistributedChallenges/   ← Consistency models, CAP
│   ├── 46-ConcurrencyParallelism/  ← Send/Sync, RwLock
│   ├── 47-DataRacesRaceConditions/ ← Cell/RefCell patterns
│   ├── 48-DiningPhilosophers/      ← Deadlock prevention
│   ├── 49-DistributedComputing/    ← Rust for distributed systems
│   ├── 50-RayonChallenge/          ← Data parallelism with Rayon
│   ├── 51-SendSync/                ← Send/Sync marker traits
│   └── 52-ConcurrencyLessonReflection/
├── 06-CLIAndTools/                 ← Section 6: CLI tools & graph algorithms
│   ├── 14-CLISalad/                ← clap CLI parsing
│   ├── 20-CommunityDetection/      ← Kosaraju SCC algorithm
│   ├── 21-UFCGraphCentrality/      ← Graph centrality
│   ├── 22-GraphVisualize/          ← ASCII bar charts
│   ├── 24-LisbonShortestPath/      ← Dijkstra on weighted graphs
│   ├── 25-Neo4jDataScience/        ← Neo4j graph database
│   ├── 26-PageRank/                ← PageRank algorithm
│   ├── 27-RussianTrollTweets/      ← Social graph analysis
│   ├── 29-DataStructuresLessonReflection/
│   ├── 31-FullyConnectedGraph/     ← Graph connectivity
│   └── 33-CustomCLIFruitSalad/     ← Advanced CLI + modules
├── 07-Security/                    ← Section 7: safety & cryptography
│   ├── 35-SafeAndUnsafe/           ← Safe vs unsafe Rust
│   ├── 39-SafetyLessonReflection/  ← Memory safety guarantees
│   ├── 40-DecoderRing/             ← Caesar cipher + Rayon
│   ├── 41-RustCryptoHashes/        ← Cryptographic hashes
│   ├── 42-RustSoftwareSecurity/    ← Rust vs C/C++/Java security
│   └── 43-SecurityLessonReflection/
├── 08-Interop/                     ← Section 8: Python/Rust interop
│   ├── 57-ExploringPandas/         ← Pandas + Rust comparison
│   └── 58-RustJupyterNotebook/     ← evcxr Jupyter kernel
└── 09-ProductionSystems/          ← Section 9: production-grade systems
    └── 59-Radish/                  ← Redis-compatible KV store (async TCP)
```

### Section naming convention
- Sections are numbered `01`–`09` for correct alphabetical ordering.
- Section folder names describe the concept cluster: `01-Foundations`, `02-Ownership`, etc.
- When a new concept cluster is needed (e.g., `10-Networking`, `11-Databases`), add it as `10-Name/` and update this file.

### Project numbering
- Projects within a section keep their original number (0–59) for cross-referencing.
- Project folders are named `N-ProjectName/` where N is the original number.
- Numbers may not be contiguous within a section (e.g., Section 1 has 0, 1, 2, 32).
- Numbers are NOT reused — even if a project is removed, the number stays retired.

---

## 3. Two Project Types

The repo contains two types of projects, each with a different "complete structure":

### Type A — 100-Exercises Lesson Projects (projects 0–8)
These come from the [100-exercises-to-learn-rust](https://github.com/mainmatter/100-exercises-to-learn-rust) source. They are tutorial-based with progressive exercises.

**Required structure:**
```
N-ProjectName/
├── README.md               ← Comprehensive step-by-step Rust tutorial (the workshop)
├── 00_intro.md             ← Complementary lesson files (numbered 00–NN)
├── 01_concept.md
├── ...
├── NN_outro.md
└── project.py              ← Python implementation (optional, present in some)
```

**Reference model: `01-Foundations/2-MasterMind/`** contains:
- `README.md` — full tutorial with Python comparisons, diagrams, and exercises
- `master_mind.md` + `master-mind-advanced.md` — complementary lesson files
- `master-mind.py` + `master_mind2.py` — Python implementations

### Type B — Data-Engineering Cargo Projects (projects 9–59)
These come from the [data-engineering-rust](https://github.com/jolisper/data-engineering-rust) source. They are hands-on Cargo projects.

**Required structure:**
```
N-ProjectName/
├── README.md               ← Tutorial or introduction pointing to workshop content
├── Cargo.toml              ← Rust project manifest
└── src/
    ├── lib.rs              ← All public functions with todo!() stubs + progressive tests
    └── main.rs             ← Entry point that calls into lib.rs
```

### Test-Driven Architecture

Every project must follow the **progressive test** pattern:

1. **`src/lib.rs`**: Place all public functions here. Each function starts as `pub fn foo() { todo!() }`. The user replaces `todo!()` with real code as they progress through the README.

2. **`src/main.rs`**: Minimal CLI entry point that calls functions from `lib.rs`. Tests should NOT depend on main.rs.

3. **Test organization**: Group tests by tutorial step using nested modules:
   ```rust
   #[cfg(test)]
   mod tests {
       mod step_01_concept_name {
           // Tests that pass when the user implements the first concept
       }
       mod step_02_next_concept {
           // Additional tests for the next concept
       }
   }
   ```

4. **Test coverage**: Every function in `lib.rs` must have at least 2–3 tests covering normal cases, edge cases, and error conditions. For functions that panic, include a `#[should_panic]` test.

5. **README banner** (at top, after the subtitle):
   ```
   > **Test-driven approach**: This project includes a Cargo project with progressive
   > unit tests. Each function in `src/lib.rs` starts as a `todo!()` stub. As you
   > follow each section, replace `todo!()` with real code and run `cargo test` to
   > watch the pass count grow. Your goal: **all N tests pass**.
   ```

6. **No external test crate dependencies** — use only `#[cfg(test)]` with built-in `#[test]` and `#[should_panic]`. Avoid adding `dev-dependencies` unless the project already uses them for integration tests.

7. **Root README.md** must mention "Test-driven learning" in the Course Progression section.

**Key difference**: Type B projects are pure Rust — no Python equivalent file is expected. The README.md focuses on explaining the Rust code directly with Python comparisons.

---

## 4. Agent Workflow

### 4.1 Creating a new workshop (filling a gap)

1. **Read `README.md`** and extract the current **Rust Concepts Coverage** table.
2. **Identify gaps** — find Rust concepts marked `❌` that have not been introduced yet.
3. **Select a group of 1–5 concepts** that:
   - Form a natural teaching cluster (e.g., `Result` + `?`, `enum` + `match`, `HashMap` + `entry`).
   - Do not depend on other untaught concepts (or can be explained with minimal forward references).
   - Can be demonstrated in a practical, small data-engineering project.
4. **Find the target project folder** — look at the project's current README.md stub and the main README.md Projects table to understand what the project is about.
5. **Design the workshop content**:
   - Write the full step-by-step tutorial into `README.md` (overwriting the stub).
   - Follow the workshop template in Section 6.
   - For Type A projects: write as an extension of the complementary `.md` lesson files.
   - For Type B projects: write as a guide that walks through the `src/` code.
6. **Update `README.md`** (the root one):
   - Add any new Rust concepts to the **Rust Concepts Coverage** table.
   - Update the **Projects** table row if needed (add concepts covered).
7. **Verify** that all newly covered concepts are listed in both the project row and the coverage table.

### 4.2 Improving an existing section or project

When a user asks to improve a section or folder:

1. **Read the section's README.md** and all project README.md files in that section.
2. **Assess the gap**: What is missing? Common improvement types:
   - ✏️ **Polish**: Fix typos, clarify wording, add missing Python comparisons.
   - 📊 **Visuals**: Add ASCII diagrams, flow charts, tables comparing Rust to Python.
   - 🏋️ **Exercises**: Add practice exercises with solutions at the end of each workshop.
   - 📚 **Depth**: Cover a concept that was only mentioned briefly.
   - 🔗 **Cross-references**: Link to prerequisite or follow-up projects.
   - 🐍 **Python equivalents**: Ensure every Rust concept has a Python comparison.
   - 🛠️ **Code**: Fix bugs in Cargo.toml, src/, or add missing Cargo projects.

3. **Make changes** to the project's `README.md` and/or source files.
4. **Update the concept coverage table** in root `README.md` if new concepts are introduced.
5. **Verify** the project still compiles (for Type B) or the tutorial still makes sense.

### 4.3 Adding a new section

When projects cover a domain not represented by the 9 existing sections:

1. Choose the next section number (e.g., `10-Databases`, `11-Networking`).
2. Create the folder and `README.md` with the section overview.
3. Move or add projects under it.
4. Update this file (Section 2 — Repository Architecture) and the root `README.md` directory tree.
5. Update the **Projects** table in root `README.md` with new rows.

---

## 5. How to Choose Concepts

When selecting concepts for a workshop, consider the following dependencies:

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

Always pick concepts that build on already‑covered ones. If a prerequisite hasn't been introduced in a prior project, explain it inline (with a forward reference to where it will be covered in depth).

---

## 6. Workshop Template (`README.md`)

The workshop must be a Markdown file that follows this template. Every workshop serves as the **primary learning file** for that project — the learner reads this, not the complementary files.

```markdown
# 🦀 Project Name — Python to Rust Workshop

## Table of Contents
1. [Introduction](#1-introduction)
2. [Prerequisites](#2-prerequisites)
3. [Running the Python Version](#3-running-the-python-version)  ← skip for Type B
4. [Concept: ...](#4-concept-...)   ← one section per concept
5. [Concept: ...](#5-concept-...)
...
X. [Putting It All Together](#X-putting-it-all-together)
Y. [Complete Code Reference](#Y-complete-code-reference)
Z. [Summary](#Z-summary)

## 1. Introduction
Briefly describe the project, what it does (or what the Python script does for Type A), and which new Rust concepts will be learned. Include a data-engineering motivation.

**Python → Rust**: If this project has a Python version, explain how the Rust version differs and why.

## 2. Prerequisites
List previously covered projects/concepts and the required tools. Link to prerequisite project README.md files.

## 3. Running the Python Version (Type A only)
Show how to run the provided `project.py` and explain its behaviour. Skip this section for Type B projects.

## 4. Concept: [Name]
### Explanation
Explain the Rust concept in plain language, with small code snippets, ASCII diagrams or tables, and **comparisons to Python**. Always start with "In Python, this is..."

### Example (stand‑alone)
A tiny Rust program that illustrates the concept, completely independent of the workshop project.

### Applying to Our Project
Show exactly how the concept will be used in the upcoming Rust code. Provide the relevant code excerpt from the project.

## 5. Concept: [Name]
(same structure)

...

## X. Putting It All Together
Walk through building the Rust project file‑by‑file (or section‑by‑section), integrating all concepts. Do not repeat full concept explanations; instead, refer back to the dedicated sections. Provide the complete code the learner writes.

## Y. Complete Code Reference
Include the full final Rust source code so the learner can verify their work.

## Z. Summary
Table listing the new concepts covered, with short descriptions and where they were used.
```

**Important notes:**
- The "Concept" sections must come **before** the final assembly.
- Every concept section must include a **Python comparison** — this is not optional.
- For Type B projects (Cargo projects), skip Section 3 and instead explain the Rust code directly.
- Use ASCII diagrams (`┌───┐` style) to illustrate ownership, borrowing, memory layout, data flow.
- Include 2–3 exercises at the end with varying difficulty (easy / medium / hard).
- Link to complementary `.md` files in a **Further Reading** section near the end.

---

## 7. Updating the Root `README.md`

When a workshop is created or improved:

1. **Projects table**: Add or update the project's row with the project number, name, description, and a concise list of the new Rust concepts:

   ```markdown
   | 2 | **MasterMind** — code‑breaking game with structs and Vec | `struct`, `Vec`, `Option`, `match`, `loop` |
   ```

2. **Rust Concepts Coverage table**: For each newly covered concept, change `❌` to `✅` and set "First Project" to the project number. If a concept was already `✅`, add the new project number to the "First Project" column.

3. If a concept was touched briefly but will be deepened later, keep it as `❌` and note it in the workshop's summary.

---

## 8. Concept Coverage Checklist

The root `README.md` contains a **Rust Concepts Coverage** table that tracks every Rust concept the course should teach. When adding a new workshop, check this table first:

- [ ] Are the concepts you're teaching already covered? → Improve existing coverage, don't duplicate.
- [ ] Are there concepts marked `❌` that fit naturally into this project? → Cover them and update the table.
- [ ] Are any concepts you need not yet taught in a prerequisite project? → Add a brief inline explanation with a forward reference.

Current status: ~65 concepts covered (✅), 0 concepts remaining uncovered (❌). The course's goal is to maintain 100% concept coverage.

---

## 9. Final Check

Before finalising any change:

- [ ] The workshop runs end‑to‑end (for Type B: `cargo run` compiles; for Type A: the tutorial steps are verifiable).
- [ ] Every new Rust concept has a Python comparison.
- [ ] The concept coverage table in root `README.md` is accurate.
- [ ] Cross-references to other projects use correct relative paths (e.g., `../02-Ownership/3-TicketV1/README.md`).
- [ ] No other sections of the root `README.md` are broken.
- [ ] All changes are committed and pushed.

---
