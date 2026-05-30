# Section 2: Ownership — Rust's Superpower over Python's GC

*The biggest mindset shift: how Rust manages memory without a garbage collector.*

## Prerequisites

- Completed [Section 1: Foundations](../01-Foundations/README.md)
- Understand basic Rust syntax and types

## Projects in This Section

| # | Project | Concepts | Format |
|---|---------|----------|--------|
| 3 | **TicketV1** — structs, ownership, stack vs heap | `struct`, validation, `mod`/`pub`, encapsulation, ownership, borrowing, stack/heap, destructors (`Drop`) | Tutorial |
| 4 | **Traits** — trait definitions, derive, bounds | `trait`, orphan rule, operator overloading, `derive`, trait bounds, `Deref`, `Sized`, `From` | Tutorial |
| 5 | **TicketV2** — enums, match, error handling | `enum`, `match`, `if let`, `Option`, `Result`, error enums, `Error` trait, `thiserror`, `TryFrom` | Tutorial |
| 37 | **OBRM** — ownership-based resource management | Ownership rules, `Drop` trait, RAII, borrowing, resource lifecycle | Project |
| 38 | **OwnershipLifetimes** — lifetimes & borrow checker | Move semantics, borrowing (`&T`/`&mut T`), lifetimes (`'a`), `Copy`/`Clone` | Project |

## Learning Path

1. **3-TicketV1** — the most important workshop. Master ownership with extensive diagrams
2. **4-Traits** — learn Rust's interface system (like Python protocols)
3. **5-TicketV2** — enums and Result-based error handling (essential for production)
4. **04-OBRM** — apply ownership in a resource management project
5. **05-OwnershipLifetimes** — deep dive into lifetime annotations

## Key Concepts Covered

| Rust Concept | Python Equivalent | Why It Matters |
|---|---|---|
| Ownership | N/A (GC) | Memory safety without GC pauses |
| Move semantics | N/A (everything is ref) | Predictable memory |
| Borrowing (`&`) | Pass-by-reference | Zero-cost data access |
| Mutable borrow (`&mut`) | N/A | Data race prevention |
| Lifetimes (`'a`) | N/A | Reference validity |
| `struct` | `class` / `dataclass` | Custom data types |
| `trait` | Protocol / ABC / interface | Polymorphism |
| `enum` | Enum / Union | Type-safe variants |
| `Result<T, E>` | Exceptions | Recoverable errors |
| `?` operator | `try` / except | Error propagation |
