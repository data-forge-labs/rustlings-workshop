# AGENTS.md — Rust Data Engineering Course Designer Instructions

**Role**: You are a **senior Rust data engineer** building a comprehensive Rust course for **Python data engineers** who are new to Rust. Write every workshop as if you are pairing with a junior data engineer — compare each concept to its Python equivalent, explain *why* Rust works differently, and always connect back to real data-engineering use cases (pipelines, ETL, file processing, concurrent workloads, production systems).

This file describes how to systematically maintain, extend, and improve the **Rust Tutorial — Learn by Doing** series. It ensures consistency, completeness, and a smooth learner experience across all 61 projects.

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

The repo is organized into **15 numbered concept sections** (14–15 merged from original structure), each containing related projects:

```
RustTut/
├── README.md                          ← Main index, project tables, concept coverage checklist
├── AGENTS.md                          ← This file
├── .devcontainer/                     ← Preconfigured Rust dev environment
├── 01-Foundations/                    ← Section 1: syntax, types, control flow
│   ├── README.md                      ← Section overview with project table & learning path
│   ├── 01-Intro/                      ← Rust syntax primer
│   ├── 02-GuessGame/                  ← String vs &str, Result, console I/O, enums, rand crate
│   ├── 03-BasicCalculator/            ← Integers, branching, loops
│   └── 04-MasterMind/                 ← Structs, Vec, Option, console I/O
├── 02-Ownership/                      ← Section 2: ownership, borrowing, traits, enums
│   ├── 01-TicketV1/                   ← Structs, ownership, stack vs heap
│   ├── 02-Traits/                     ← Trait definitions, derive, bounds
│   ├── 03-TicketV2/                   ← Enums, match, Result, error handling
│   ├── 04-OBRM/                       ← RAII / Drop / ownership-based resource mgmt
│   ├── 05-OwnershipLifetimes/         ← Lifetimes, borrow checker
│   └── 06-ConversionErrorHandling/    ← Option/Result combinators, From, ?
├── 03-Collections/                    ← Section 3: data structures & iterators
│   ├── 01-TicketManagement/           ← Vec, arrays, HashMap, BTreeMap, iterators
│   ├── 02-VectorFruitSalad/           ← Vec<T> dynamic arrays
│   ├── 03-ArrayFruitSalad/            ← Fixed-size arrays [T; N]
│   ├── 04-HashMapCount/               ← Word frequency counting
│   ├── 05-LinkedListFruitSalad/       ← Doubly-linked list
│   ├── 06-VecDequeFruitSalad/         ← Double-ended queue
│   ├── 07-HashMapLanguage/            ← Complex HashMap data
│   ├── 08-RustCollectionsDoc/         ← std::collections reference
│   ├── 09-BinaryHeapFruit/            ← Priority queue
│   ├── 10-BTreeSetFruit/              ← Ordered set
│   ├── 11-HashSetFruit/               ← Unique items / set operations
│   ├── 12-RustIterators/              ← Lazy functional iteration
│   ├── 13-MutableFruitSalad/          ← Vec mutation patterns
│   └── 14-ProfileBenchmark/           ← Cargo profiles + criterion benchmarking
├── 04-FileIO/                         ← Section 4: file I/O & data formats
│   ├── 01-CSVCookbook/                ← Read/write CSV with csv crate
│   ├── 02-CSVWriter/                  ← CSV writing with serde
│   ├── 03-Parquet/                    ← Apache Parquet
│   ├── 04-Arrow/                      ← Apache Arrow in-memory columnar
│   ├── 05-YAML/                       ← YAML config & data
│   ├── 06-JsonStream/                 ← JSON & NDJSON streaming
│   └── 07-NextGenFormats/             ← Lance, Vortex, Nimble, F3 benchmarks
├── 05-Concurrency/                    ← Section 5: threads, async, atomics
│   ├── 01-Threads/                    ← Threads, channels, locks
│   ├── 02-Futures/                    ← async/await, tokio
│   ├── 03-DataRace/                   ← Mutex, Arc, data-race prevention
│   ├── 04-Atomics/                    ← Lock-free atomics
│   ├── 05-DistributedChallenges/      ← Consistency models, CAP
│   ├── 06-ConcurrencyParallelism/     ← Send/Sync, RwLock
│   ├── 07-DataRacesRaceConditions/    ← Cell/RefCell patterns
│   ├── 08-DiningPhilosophers/         ← Deadlock prevention
│   ├── 09-DistributedComputing/       ← Rust for distributed systems
│   ├── 10-RayonChallenge/             ← Data parallelism with Rayon
│   ├── 11-SendSync/                   ← Send/Sync marker traits
│   ├── 12-AdvancedSync/               ← parking_lot, crossbeam, arc_swap
│   └── 13-AsyncPatterns/              ← Tokio select!, Semaphore, Notify, JoinSet
├── 06-TerminalApps/                   ← Section 6: CLI tools & TUI dashboards
│   ├── 01-CLISalad/                   ← clap CLI parsing
│   ├── 02-CustomCLIFruitSalad/        ← Advanced CLI + lib/main split
│   ├── 03-RatatuiTUI/                 ← ratatui TUI dashboards
│   └── 04-AsyncClap/                  ← Async CLI with subcommands
├── 07-GraphAndNetworkScience/         ← Section 7: petgraph, algorithms, Neo4j
│   ├── 01-CommunityDetection/         ← Kosaraju SCC algorithm
│   ├── 02-UFCGraphCentrality/         ← Graph centrality
│   ├── 03-GraphVisualize/             ← ASCII bar charts (rasciigraph)
│   ├── 04-LisbonShortestPath/         ← Dijkstra on weighted graphs
│   ├── 05-Neo4jDataScience/           ← Neo4j graph database
│   ├── 06-PageRank/                   ← PageRank algorithm
│   ├── 07-RussianTrollTweets/         ← Social graph analysis
│   └── 08-FullyConnectedGraph/        ← Graph connectivity
├── 08-Security/                       ← Section 8: safety & cryptography
│   ├── 01-SafeAndUnsafe/              ← Safe vs unsafe Rust
│   ├── 02-DecoderRing/                ← Caesar cipher + Rayon
│   ├── 03-RustCryptoHashes/           ← Cryptographic hashes
│   ├── 04-Argon2/                     ← Password hashing
│   ├── 05-Ed25519/                    ← Digital signatures
│   └── 06-RustlsTLS/                  ← TLS server & client
├── 09-ObservabilityAndTesting/        ← Section 9: observability & testing practices
│   ├── 01-Logging/                    ← log facade, env_logger, tracing
│   ├── 02-Configuration/              ← config crate, TOML/YAML/JSON, env override
│   ├── 03-Testing/                    ← #[test], should_panic, integration tests
│   ├── 04-Proptest/                   ← Property-based testing
│   ├── 05-Mockall/                    ← Trait mocking
│   └── 06-Insta/                      ← Snapshot testing
├── 10-ProductionSystems/              ← Section 10: production-grade systems
│   ├── 01-Radish/                     ← Redis-compatible KV store (async TCP)
│   ├── 02-AxumShop/                   ← Axum web API (FastAPI-compatible shop)
│   ├── 03-AxumAuth/                   ← JWT + Bearer middleware
│   └── 04-OpenTelemetry/              ← Traces, spans, correlation IDs
├── 11-Interop/                        ← Section 11: Python/Rust interop
│   ├── 01-ExploringPandas/            ← Pandas + Rust comparison
│   ├── 02-RustJupyterNotebook/        ← evcxr Jupyter kernel
│   ├── 03-PyO3Bindings/               ← Call Rust from Python
│   └── 04-GILRelease/                 ← Free the GIL, free the CPU
├── 12-DataEngAnalytics/               ← Section 12: data-eng analytics engines on Arrow
│   ├── README.md                      ← Section overview
│   ├── 01-Polars/                     ← Polars DataFrame (eager + lazy)
│   ├── 02-DuckDB/                     ← DuckDB in-process OLAP
│   └── 03-DataFusion/                 ← Apache DataFusion query engine
├── 13-ActorModel/                     ← Section 13: actor-model concurrent patterns
│   ├── README.md                      ← Section overview
│   ├── 01-DIY-Actor/                  ← DIY actor with mpsc + oneshot
│   ├── 02-Ractor/                     ← Production actor framework
│   └── 03-ETLPipeline/                ← Source → Transform → Sink as actors
├── 14-DataInfrastructure/             ← Section 14: data infra & integration
│   ├── README.md                      ← Section overview
│   ├── 01-KafkaRdkafka/               ← Kafka produce/consume
│   ├── 02-PostgreSQLSqlx/             ← PostgreSQL with sqlx
│   ├── 03-RedisAsync/                 ← Redis async
│   ├── 04-ClickHouseIngestion/        ← ClickHouse OLAP sink
│   ├── 05-ApacheIggy/                 ← Rust-native message streaming
│   ├── 06-DuckLakeCatalog/            ← DuckDB lakehouse catalog
│   ├── 07-CdcPipeline/                ← Debezium-style CDC
│   └── 08-UnifiedPipeline/            ← Multi-sink orchestrator
└── 15-Reference/                      ← Section 15: concept reference & cheatsheets (appendix)
    ├── README.md                      ← Section overview
    ├── collections-guide.md           ← Collections comparison & selection guide
    ├── concurrency-reference.md       ← Concurrency model review
    ├── data-management-io.md          ← File I/O & serialization reference
    ├── distributed-systems.md         ← Distributed systems concepts
    ├── heap-memory.md                 ← Stack vs heap, off-heap, allocators
    ├── memory-safety.md               ← Memory safety & security model
    ├── security-model.md              ← Rust security model & best practices
    └── send-sync.md                   ← Send/Sync, thread safety markers
```

### Section naming convention
- Sections are numbered `01`–`14` for correct alphabetical ordering.
- Section folder names describe the concept cluster: `01-Foundations`, `02-Ownership`, etc.
- When a new concept cluster is needed (e.g., `15-Networking`, `16-Databases`), add it as `15-Name/` and update this file.

### Project numbering
- Projects within a section are numbered sequentially (01, 02, 03...) for correct ordering.
- Project folders are named `NN-ProjectName/` where NN is the sequential number within the section.
- Numbers are unique within each section, not globally.
- When a project is removed from a section, remaining projects are NOT renumbered (to preserve git history).

---

## 3. Two Project Types

The repo contains two types of projects:

### Type A — Tutorial Lesson Projects
These are comprehensive tutorial-based workshops with step-by-step instruction. They contain:
- `README.md` — Comprehensive step-by-step tutorial
- `project.py` — Python implementation (optional)

### Type B — Data-Engineering Cargo Projects
These are hands-on Cargo projects with test-driven architecture. The student
**creates the project from scratch** following the Setup section in the README
(`cargo new --lib <name>`, `cd <name>`, add deps, copy test stubs, then
implement each function step by step).

**Required structure:**
```
N-ProjectName/
├── README.md               ← Full hands-on tutorial (preamble + Setup + step-by-step)
└── src/                    ← The student creates this via `cargo new`
    ├── lib.rs              ← All public functions with todo!() stubs + progressive tests
    └── main.rs             ← Entry point that calls into lib.rs
```

The `Cargo.toml`, `src/lib.rs`, and `src/main.rs` shown in the Setup section
of the README are what the student should produce. The repository may
optionally include a `workshop/` directory with the same files as a reference
implementation, but the **primary tutorial lives in `N-ProjectName/README.md`**.

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

6. **Setup section** (in the README, after the compact preamble):
   The README must include a `## Setup: Create the Project from Scratch` section
   with concrete commands:
   - `cargo new --lib <name>` (or `cargo new <name>` for binaries)
   - `cd <name>`
   - Replace `Cargo.toml` [dependencies] with the project's specific deps
   - Copy the test stubs from `workshop/src/lib.rs` (or paste the inline stub)
   - Run `cargo test` to see them fail
   - Implement each function following the step sections
   - Run `cargo test step_XX_name` to see progress

7. **No external test crate dependencies** — use only `#[cfg(test)]` with built-in `#[test]` and `#[should_panic]`. Avoid adding `dev-dependencies` unless the project already uses them for integration tests.

8. **Root README.md** must mention "Test-driven learning" in the Course Progression section.

**Key difference**: Type B projects are pure Rust — no Python equivalent file is expected. The README.md is a complete hands-on tutorial: the student types every command and writes every function themselves.

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

When projects cover a domain not represented by the existing sections:

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

### 6.1 Compact Preamble (the only block before `---`)

The preamble is **always exactly three elements** and never more than ~25 lines:

1. **H1 + tagline** — the project name and a one-line description.
2. **Test-driven banner** — `> **Test-driven approach**: ...` with the test count goal.
3. **`## What Is This {Project/Concept}?` heading** — project-specific. Below it:
   - A 1–2 sentence description of what the project/concept does.
   - **Python equivalent** — a short Python code snippet showing the equivalent final result (if Python has one). Keep it to 5–15 lines. If the concept has no Python equivalent, note that briefly.
   - **Transition line** — `In this project you'll learn to build this in Rust — and along the way you'll discover {concept1}, {concept2}, ...`
   - **One unified `## At a Glance` table** with 5 columns: `# | Concept | Rust | Python | Why it matters`

The "Why it matters" column absorbs everything that used to live in the prose-only "Concepts at a Glance" section. **Do not add a separate "Concepts at a Glance" section** — that is the duplication we are eliminating.

> **NEVER use "Python pain" / "Rust fix" framing.** The old template framed Python as "broken" and Rust as the "fix." The new template simply shows the Python equivalent (when one exists) and explains what the learner will build in Rust. This is more respectful of the learner's Python background and more accurate — Python is fine for many tasks; Rust offers different tradeoffs.

```markdown
# 🦀 Project Name — Python to Rust Workshop

*Subtitle: one-line description of what the project builds.*

> **Test-driven approach**: This project includes a Cargo project with progressive
> unit tests. Each function in `src/lib.rs` starts as a `todo!()` stub. As you
> follow each section, replace `todo!()` with real code and run `cargo test` to
> watch the pass count grow. Your goal: **all N tests pass**.

---

## What Is This {Project/Concept}?

[1-2 sentence description of the project or concept.]

### Python equivalent

```python
# Short Python snippet showing the equivalent result (5-15 lines)
# Only include if Python has a direct equivalent for this concept
```

In this project you'll learn to build this in Rust — and along the way
you'll discover **{concept1}**, **{concept2}**, and **{concept3}**.

## At a Glance

| # | Concept | Rust | Python | Why it matters |
|---|---------|------|--------|----------------|
| 1 | Concept name | `crate::Module` | Python equivalent | One-line purpose |

---
```

### 6.2 Body Sections (after `---`)

After the closing `---` of the preamble, the original tutorial content follows this structure:

```markdown
## Table of Contents
1. [Introduction](#1-introduction)
...

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

### 6.3 Important notes

- **Every** project README must have the **compact preamble (§6.1)** before the `---` separator. The preamble is at most ~25 lines, not ~80.
- The `## What Is This {Project/Concept}?` heading is **always project-specific** (e.g., "What is RAII?", "What is a ticket system?", "What is parallel CSV parsing?"). Never use the generic "Why This Project?".
- **NEVER use "Python pain" / "Rust fix" framing.** Show the Python equivalent (when one exists) and explain what the learner will build in Rust. The transition line should list the Rust concepts that will be discovered.
- The "At a Glance" table is the **only** concept-list section in the preamble. No separate "Concepts at a Glance" prose section — the table's "Why it matters" column carries that information.
- The "Concept" sections in the body must come **before** the final assembly.
- Every concept section must include a **Python comparison** — this is not optional.
- For Type B projects (Cargo projects), skip Section 3 and instead explain the Rust code directly.
- Use ASCII diagrams (`┌───┐` style) to illustrate ownership, borrowing, memory layout, data flow.
- Include 2–3 exercises at the end with varying difficulty (easy / medium / hard).
- Link to complementary `.md` files in a **Further Reading** section near the end.
- Do **not** modify the original tutorial content after the `---` separator — only the prepended block may be added/edited.

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
- [ ] Cross-references to other projects use correct relative paths (e.g., `../02-Ownership/01-TicketV1/README.md`).
- [ ] No other sections of the root `README.md` are broken.
- [ ] All changes are committed and pushed.

---

## 10. Maintaining the Crate Catalog

The file `15-Reference/crate-catalog.md` is the **course-wide crate index** — a living document of every essential Rust crate a data engineer needs.

**When working on any workshop that introduces a new dependency:**
1. Check if the crate is already listed in `crate-catalog.md`. If not, add it to the appropriate section with a 1-2 line description and a minimal code snippet.
2. If the crate fits multiple categories, pick the most relevant one and cross-reference.
3. Keep version numbers up-to-date — when upgrading workspace deps, reflect the new versions in the "Daily Driver" `Cargo.toml` blocks.
4. Remove or mark deprecated crates; the catalog should remain a *trusted* reference, not a junk drawer.

This covers every goal from Section 6.3 (AGENTS.md) — but we add this custom instruction because it's a course-wide asset, not a single workshop.



---

## 12. Recent Improvements (Batch Fix — June 2025)

A systematic batch-fix was applied across sections **01–Foundations**, **02–Ownership**, **03–Collections**, **04–FileIO**, **05–Concurrency**, and **06–TerminalApps** to enforce the compact-preamble standard and improve consistency.

### Preamble format update (July 2025)

The preamble format was updated course-wide to remove the "Python pain" / "Rust fix" framing. The new format:

- **Before:** `## Why {phrase}?` → "Python pain" paragraph → "Rust fix" paragraph → At a Glance table
- **After:** `## What Is This {Project/Concept}?` → 1-2 sentence description → Python equivalent code snippet → "In this project you'll learn..." transition → At a Glance table

This change applies to **all 74 project READMEs** across sections 01–14. Each project's preamble should:
1. Describe what the project/concept is (1-2 sentences)
2. Show the Python equivalent code if one exists (5-15 lines)
3. List the Rust concepts that will be discovered in this project
4. Include the At a Glance table

### Changes applied

| Fix | What was done | Files affected |
|-----|---------------|----------------|
| **Compact preamble** | Trimmed each README to exactly 3 blocks before `---`: H1 + tagline, test‑driven banner, `## Why {meaningful phrase}?` heading. Removed extra prose before separator. | All project `README.md` files in sections 01–06 |
| **Cross‑section links** | Fixed broken relative paths: `../Section/` → `../../Section/` for inter‑section references. | All `README.md` files in sections 01–06 + root |
| **Python comparisons** | Added missing Python cells for `char`, `Option`, `Result` in At‑a‑Glance tables. | `01-Foundations/01-Intro/README.md`, `02-Ownership/04-OBRM/README.md`, `02-Ownership/05-OwnershipLifetimes/README.md` |
| **Duplicate tables** | Removed redundant "What You'll Learn" tables from section READMEs (redundant with the "At a Glance" table + root coverage table). | `01-Foundations/README.md`, `02-Ownership/README.md`, `03-Collections/README.md` |
| **Exercises** | Added placeholder `## Exercises` section (Easy/Medium) to every workshop README that lacked one. | All workshop `README.md` files across sections 01–06 |
| **Stack vs Heap guide** | Replaced a simple note with a compact, course‑consistent diagram + summary table in the Ownership section. ASCII chart showing `String` layout across stack and heap, comparison table, and Python analogy. | `02-Ownership/README.md` |
| **Root coverage table** | Deduplicated "First Project" entries so each concept has only the earliest project listed. | `README.md` (root) |
| **Pattern‑matching reference** | Added a "See also" link to the `@` bindings and guards appendix from `01-Intro` and `02-GuessGame`. | `01-Foundations/01-Intro/README.md`, `01-Foundations/02-GuessGame/README.md` |
| **Ownership note** | Added a brief ownership‑note paragraph to project READMEs that mention ownership. | Project READMEs across sections 01–06 |

### Style rules reinforced

- Each workshop README must have **exactly** H1 + tagline, banner, `## Why …?` heading, then `---` before body content.
- "At a Glance" table must have **5 columns**: `# | Concept | Rust | Python | Why it matters`.
- Every Rust concept must have an **explicit Python comparison** (either in the "Python" table cell or in the accompanying text).
- ASCII diagrams should be **compact** (5–15 lines) and use box‑drawing characters (`┌───┐` style).
- Cross‑section relative links must use `../../Section-NNN/ProjectName/README.md` (two levels up from the project folder).

---

## 13. Comprehensive To-Do List for Remaining Subjects

The following sections have **not yet been fully audited** for compact-preamble compliance, Python comparisons, and consistency. Each should receive the same treatment as sections 01–06 in a future pass.

**Global task:** Replace "Python pain" / "Rust fix" framing with "Python equivalent" + "What you'll learn" in all 74 project READMEs across sections 01–14.

### Section 07 — GraphAndNetworkScience (8 projects)

| Task | Priority | Notes |
|------|----------|-------|
| Audit all `README.md` files for compact preamble compliance | High | Many may have old long preamble |
| Fix cross‑section links (`../Section/` → `../../Section/`) | High | Links to Ownership, Foundations, Collections |
| Verify At‑a‑Glance tables have 5 columns | High | May be missing Python column |
| Replace "Python pain" / "Rust fix" with Python equivalent + transition | High | All 8 projects |
| Add Python comparisons for `petgraph` concepts | Medium | E.g., `DiGraph` ↔ `networkx.DiGraph`, Dijkstra ↔ `networkx.shortest_path` |
| Add placeholder Exercises | Medium | |
| Add "Why it matters for graph pipelines?" context | Low | |

### Section 08 — Security (6 projects)

| Task | Priority | Notes |
|------|----------|-------|
| Compact preamble audit | High | |
| Replace "Python pain" / "Rust fix" with Python equivalent + transition | High | All 6 projects |
| Fix links to ProductionSystems (10‑) | High | Some may use wrong depth |
| Add Python comparisons for `unsafe`, crypto concepts | Medium | `unsafe` ↔ ctypes, SHA‑2 ↔ `hashlib` |
| Add Exercises | Medium | |
| Add data‑engineering context (e.g., "Why TLS for data pipelines?") | Low | |

### Section 09 — ObservabilityAndTesting (6 projects)

| Task | Priority | Notes |
|------|----------|-------|
| Compact preamble audit | High | |
| Replace "Python pain" / "Rust fix" with Python equivalent + transition | High | All 6 projects |
| Fix links | High | |
| Verify Python comparisons for logging, config, testing | Medium | `log` ↔ `logging`, `proptest` ↔ `hypothesis`, `mockall` ↔ `unittest.mock` |
| Add Exercises | Medium | |
| Add "Why it matters for pipeline reliability?" context | Low | |

### Section 10 — ProductionSystems (4 projects)

| Task | Priority | Notes |
|------|----------|-------|
| Compact preamble audit | High | AxumShop, Radish may have long introductions before separator |
| Replace "Python pain" / "Rust fix" with Python equivalent + transition | High | All 4 projects |
| Fix cross‑section links to Ownership/Collections/Concurrency | High | Many links currently broken (see earlier scan) |
| Add Python comparisons for async, Axum, JWT | Medium | Axum ↔ FastAPI, JWT ↔ PyJWT, RESP ↔ `redis-py` |
| Verify Cargo.toml compiles after link fixes | Medium | Check with `cargo check` |
| Add Exercises | Medium | |

### Section 11 — Interop (4 projects)

| Task | Priority | Notes |
|------|----------|-------|
| Compact preamble audit | High | |
| Replace "Python pain" / "Rust fix" with Python equivalent + transition | High | All 4 projects |
| Fix links | High | |
| Add Python comparisons for PyO3, evcxr | Medium | PyO3 ↔ cffi/ctypes, evcxr ↔ Jupyter |
| Add Exercises | Medium | |

### Section 12 — DataEngAnalytics (3 projects)

| Task | Priority | Notes |
|------|----------|-------|
| Compact preamble audit | High | Section README and project READMEs |
| Replace "Python pain" / "Rust fix" with Python equivalent + transition | High | All 3 projects |
| Fix links to FileIO/Arrow, Concurrency/Futures | High | |
| Add Python comparisons for Polars, DuckDB, DataFusion | Medium | Polars ↔ pandas DuckDB ↔ sqlite3, DataFusion ↔ SparkSQL |
| Add Exercises | Medium | |
| Add "Why columnar engines for data pipelines?" context | Low | |

### Section 13 — ActorModel (3 projects)

| Task | Priority | Notes |
|------|----------|-------|
| Compact preamble audit | High | |
| Replace "Python pain" / "Rust fix" with Python equivalent + transition | High | All 3 projects |
| Fix links | High | |
| Add Python comparisons for actors | Medium | DIY actor ↔ `threading` + `Queue`, ractor ↔ `actorio`/`pykka` |
| Add Exercises | Medium | |
| Add Pipeline context section | Low | |

### Section 14 — DataInfrastructure (8 projects)

| Task | Priority | Notes |
|------|----------|-------|
| Compact preamble audit | High | |
| Replace "Python pain" / "Rust fix" with Python equivalent + transition | High | All 8 projects |
| Fix links to other sections | High | |
| Verify external‑service dependencies documented | High | Kafka, PostgreSQL, etc. need Docker Compose notes |
| Add Python comparisons for Kafka, PostgreSQL, Redis | Medium | `rdkafka` ↔ `confluent_kafka`, `sqlx` ↔ `asyncpg`, `redis` ↔ `redis-py` |
| Add Exercises (if possible without external services) | Low | |
| Add "Why production data infrastructure?" context | Low | |

### Section 15 — Reference (no Cargo projects)

| Task | Priority | Notes |
|------|----------|-------|
| Verify all reference `.md` files are up‑to‑date with new concepts | Medium | Coverage table in root should match |
| Ensure `crate-catalog.md` includes all new crates added | Medium | |
| Update `heap-memory.md` if needed | Low | |

### Cross‑cutting tasks

| Task | Priority | Notes |
|------|----------|-------|
| Run `cargo check` on all 95+ projects after fixes | High | Verify no compile errors introduced |
| Verify test counts in each README banner match actual `#[test]` counts | Medium | Script already run; fix mismatches |
| Ensure all concept explanations include "Why it matters for data pipelines" sentence | Medium | Concept‑by‑concept audit |
| Add `Pattern Matching: @ Bindings and Guards` as a real mini‑project or consolidated exercise | Low | Currently only an appendix; consider making it a short standalone project |
| Standardise "Format" column in section project tables (Project, Tutorial, Optional) | Low | See misaligned entries in 01‑Foundations table |

## 11. Running Code via WSL

This repository is developed on Windows. All Rust compilation and testing is done through **WSL (Windows Subsystem for Linux)** using the `wsl` CLI.

### Quick start

```powershell
# Navigate to a workshop directory (via WSL's /mnt/ mount)
cd E:\MyProjects\RustTut\01-Foundations\01-Intro\workshop

# Run cargo commands via WSL
wsl -d Ubuntu-26.04 bash -c "cd /mnt/e/MyProjects/RustTut/01-Foundations/01-Intro/workshop && cargo check"
wsl -d Ubuntu-26.04 bash -c "cd /mnt/e/MyProjects/RustTut/01-Foundations/01-Intro/workshop && cargo test"
wsl -d Ubuntu-26.04 bash -c "cd /mnt/e/MyProjects/RustTut/01-Foundations/01-Intro/workshop && cargo run"
```

### Shorthand helper

Create a reusable alias in PowerShell to avoid repeating the full path:

```powershell
function crun { wsl -d Ubuntu-26.04 bash -c "cd /mnt/e/MyProjects/RustTut/$args[0] && cargo $args[1]" }
# Usage: crun "01-Foundations/01-Intro/workshop" test
```

### Working directory shortcut

Pass the project-relative path from the repo root:

```powershell
function ck { wsl -d Ubuntu-26.04 bash -c "cd /mnt/e/MyProjects/RustTut/$args && cargo check" }
function ct { wsl -d Ubuntu-26.04 bash -c "cd /mnt/e/MyProjects/RustTut/$args && cargo test" }

# Usage
ck "01-Foundations/01-Intro/workshop"
ct "03-Collections/02-VectorFruitSalad/workshop"
```

### Copying the repo into WSL

For faster filesystem performance (avoiding `/mnt/` overhead):

```bash
# From within WSL
cp -r /mnt/e/MyProjects/RustTut ~/RustTut
cd ~/RustTut
cargo check
```

> **Note**: After copying, run `git pull` or re-copy to sync changes made from Windows.

### Line endings

Windows CRLF line endings can cause "no such file or directory" or "`\r`: command not found" errors in WSL. To fix:

```bash
# Convert all .sh files to LF
find . -name "*.sh" -exec sed -i 's/\r$//' {} \;
```

`.rs` and `.md` files are generally fine regardless of line endings.

### Running a single project's tests

```powershell
wsl -d Ubuntu-26.04 bash -c "cd /mnt/e/MyProjects/RustTut/01-Foundations/01-Intro/workshop && cargo test"
```

### Running all projects' checks

```powershell
wsl -d ubuntu bash -c "
  for f in /mnt/e/MyProjects/RustTut/*/workshop /mnt/e/MyProjects/RustTut/*/*/workshop; do
    [ -f \"\$f/Cargo.toml\" ] || continue
    echo \"=== \$f ===\"
    cd \"\$f\" && cargo check 2>&1 | tail -3
  done
"
```

### Debugging compilation failures

When a project fails to compile, always run `cargo check` (not `cargo build`) for the fastest feedback. Common failure patterns:

| Error | Likely cause | Fix |
|-------|-------------|-----|
| `E0106` (missing lifetime) | Return type contains `&str` or `&[T]` without linking to inputs | Add `<'a>` lifetime parameter |
| `E0502` (borrow conflict) | Closure or method call borrows a value both mutably and immutably | Pre-compute values before `entry().or_insert()` or use `.clone()` |
| `E0308` (mismatched types) | Function body returns a wrong type | Check return type vs actual value |
| `does not satisfy trait bound: f64: Ord` | `f64` used in `BinaryHeap` or `BTreeMap` | Add a wrapper type with `total_cmp`-based `Ord` or use `ordered-float` crate |
| `the rt-multi-thread feature is disabled` | `#[tokio::main]` or `#[tokio::test]` without the required feature | Add `features = ["rt", "rt-multi-thread", "macros"]` to tokio in `Cargo.toml` |
| crate download fails | Network blocks crates.io (corporate VPN/firewall) | Try `cargo check --offline` if deps are already cached, or use a different network |

---
