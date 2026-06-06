# Rust Memory Safety — Reference

## Ownership Rules

```
┌───────────────────────────────────────────┐
│ 1. Each value has exactly one owner       │
│ 2. When the owner goes out of scope,      │
│    the value is dropped (RAII)            │
│ 3. At any time, you may have either:      │
│    - one mutable reference (&mut T)       │
│    - or any number of immutable refs (&T) │
└───────────────────────────────────────────┘
```

```rust
let s = String::from("hello");  // s is owner
let t = s;                      // MOVED: s is now invalid
// println!("{s}");             // compile error: use of moved value

let mut u = String::from("world");
let r1 = &u;                    // immutable borrow
let r2 = &u;                    // OK — multiple immutable borrows
// let r3 = &mut u;             // ERROR — can't borrow as mutable
```

## Stack vs Heap

```
Stack                    Heap
─────                    ────
  i32 (4 bytes)          String (ptr, len, cap on stack)
  f64 (8 bytes)           │
  [i32; 5] (20 bytes)     └──→ "hello" on heap
  &T (8 byte pointer)
  Vec<i32> (24 bytes)
```

- **Stack**: LIFO, fast, fixed size at compile time, no deallocation cost.
- **Heap**: Dynamic size, allocation cost, freed when owner drops (RAII).
- Rust decides automatically — no `malloc`/`free` in user code.
- For a deeper dive (Rust vs Python memory architecture, off-heap concept, allocator comparison), see [heap-memory.md](./heap-memory.md).

## RAII & Drop

```rust
struct Connection { handle: u32 }
impl Drop for Connection {
    fn drop(&mut self) {
        println!("Closing connection {}", self.handle);
    }
}
{
    let c = Connection { handle: 1 };
    // use c
} // drop() called automatically here
```

- **Resource Acquisition Is Initialization**: Resources are acquired in the constructor and released in `Drop`.
- No need for `try`/`finally` — the destructor runs when the variable goes out of scope.
- File handles, locks, network sockets all use RAII.

## Lifetimes

```rust
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}
```

- `'a` is a lifetime annotation — the reference lives **at most as long as `'a`**
- The borrow checker ensures no reference outlives its referent
- Most lifetimes are elided (inferred) — you only write them for complex cases

## Safe vs Unsafe Rust

| Safe Rust | Unsafe Rust |
|-----------|-------------|
| All memory safety guaranteed by compiler | Manual memory management |
| No undefined behavior possible | Can cause UB if wrong |
| 99% of code in practice | Used for FFI, inline asm, performance |
| `unsafe` block is not needed | Wrapped in `unsafe { }` blocks |

Allowed in `unsafe`:
- Dereference a raw pointer (`*const T`, `*mut T`)
- Call an `unsafe` function (e.g., FFI)
- Access/modify a mutable static variable
- Implement an unsafe trait (e.g., `Send`, `Sync`)

## Five Bugs Rust Prevents at Compile Time

| Bug | How Rust Prevents It | Python/C Counterpart |
|-----|---------------------|---------------------|
| **Use-after-free** | Ownership + borrow checker prevents dangling references | Python: GC handles it; C: use-after-free is a common bug |
| **Double-free** | RAII — each value dropped exactly once | Python: GC; C: `free()` called twice |
| **Buffer overflow** | Bounds-checked indexing (`[]` panics on OOB) | Python: bounds-checked at runtime; C: unchecked |
| **Null pointer deref** | No null — `Option<T>` forces handling | Python: None may raise AttributeError; C: null ptr segfault |
| **Data races** | Borrow checker + Send/Sync traits | Python: GIL helps; Java: may race unless synchronized |

## Python/Java vs Rust Memory Model

| Aspect | Python / Java | Rust |
|--------|---------------|------|
| Memory management | GC (tracing) | RAII (compile-time) |
| Object ownership | Shared references | Single owner + borrowing |
| Null safety | None / null | `Option<T>` |
| Thread safety | GIL (Python) or locks (Java) | Send + Sync traits |
| Performance cost | GC pauses, allocation | Zero-cost abstractions |
| Learning curve | Lower | Steeper (borrow checker) |
