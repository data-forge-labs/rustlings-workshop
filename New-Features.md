Here is the in-depth research on new Rust features from **v1.80.0 to v1.86.0**, including sample code and best practices for each. 

> **Note on scope**: As of today (June 21, 2026), the stable channel includes versions **1.80.0 through 1.86.0**. Versions **1.87.0 to 1.96.0** have not yet been released. This research covers all currently stable features up to the latest release (1.86.0).

---

### 🦀 Rust 1.80.0 (2024-07-25)

#### 1. Stabilization of `LazyCell` and `LazyLock`
These types encapsulate lazy initialization logic. Data is initialized only on first access.

- `LazyLock`: Thread-safe (implements `Sync`). Ideal for `static` variables.
- `LazyCell`: Not thread-safe (does not implement `Sync`). Use in single-threaded contexts or `thread_local!`.

```rust
use std::sync::LazyLock;
use std::time::Instant;

// Global static variable, initialized only once at first access
static LAZY_TIME: LazyLock<Instant> = LazyLock::new(Instant::now);

fn main() {
    let start = Instant::now();
    std::thread::scope(|s| {
        s.spawn(|| {
            // First access by any thread triggers initialization
            println!("Thread lazy time is {:?}", LAZY_TIME.duration_since(start));
        });
        println!("Main lazy time is {:?}", LAZY_TIME.duration_since(start));
    });
}
```

**Best Practices**:
- For complex or expensive global initialization (e.g., DB connection pools, config loading), prefer `LazyLock` over third-party crates like `lazy_static` or `once_cell`.
- In single-threaded or `#![no_std]` environments, use `LazyCell`.

#### 2. Exclusive Range Patterns (`a..b`)
You can now use `a..b` (excluding the right endpoint) directly in `match` expressions.

```rust
fn main() {
    let score = 85;
    match score {
        0..=59 => println!("Fail"),
        60..=69 => println!("Pass"),
        70..=79 => println!("Average"),
        80..=89 => println!("Good"),
        90..=100 => println!("Excellent"),
        _ => println!("Invalid score"),
    }
}
```

**Best Practices**:
- Use `a..b` when dealing with half-open interval semantics (e.g., array slicing indices). It improves readability and avoids potential integer overflow bugs from using `a..=b-1`.

#### 3. Enhanced `cfg` Checking
Cargo now checks all `cfg` attributes against the features defined in your `Cargo.toml`. The `unexpected_cfgs` lint warns about typos or unexpected configurations.

```rust
// Cargo.toml defines feature = "rayon"
fn main() {
    // Warning: feature "crayon" is not defined; likely a typo for "rayon"
    #[cfg(feature = "crayon")]
    rayon::join(|| {}, || {});
}
```

**Best Practices**:
- Add `#![deny(unexpected_cfgs)]` to your `lib.rs` or `main.rs` to catch conditional compilation typos early in your CI pipeline.

---

### 🦀 Rust 1.81.0 (2024-09-05)

#### 1. `core::error::Error` Stabilization
The `Error` trait has been moved to `core`. This means `#![no_std]` libraries can now use the standard error handling trait.

```rust
#![no_std]

use core::error::Error;
use core::fmt;

#[derive(Debug)]
pub struct MyError;

impl fmt::Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "An error occurred")
    }
}

impl Error for MyError {}
```

**Best Practices**:
- When writing `no_std` libraries, make your custom error types implement `core::error::Error`. This ensures compatibility with the broader Rust ecosystem.

#### 2. `#[expect(lint)]` Attribute
This new lint level allows you to explicitly **expect** a lint. If the lint **does not** occur, the compiler issues a warning. This solves the problem of stale `#[allow]` attributes masking outdated warnings.

```rust
fn main() {
    // We expect an unused variable warning here.
    #[expect(unused_variables)]
    let x = 42;
    
    // If `x` is used in the future, this `expect` will trigger a warning,
    // reminding us to remove it.
}
```

**Best Practices**:
- Use `#[expect]` instead of `#[allow]` to temporarily suppress lints, especially during refactoring or transitions. It ensures you don't leave useless `allow` attributes behind.
- Pair this with Clippy's `clippy::allow_attributes` and `clippy::allow_attributes_without_reason` lints to enforce this practice.

#### 3. New Sorting Algorithms
The standard library's `sort` and `sort_unstable` algorithms have been updated for better performance and faster compilation. Critically, they now detect buggy `Ord` implementations and panic if the sort order is invalid.

```rust
fn main() {
    let mut numbers = vec![1, 2, 3];
    // If your custom PartialOrd/Ord has a bug, this panics instead of returning garbage.
    numbers.sort(); 
}
```

**Best Practices**:
- Ensure your custom types' `PartialOrd` and `Ord` implementations satisfy transitivity, antisymmetry, and totality. Otherwise, your program may panic during sorting.

---

### 🦀 Rust 1.82.0 (2024-10-17)

#### 1. `cargo info` Subcommand
This long-awaited command (over a decade in the making) is now built into Cargo, allowing you to query crate information directly from the terminal.

```bash
# View information about the `cc` crate from your Cargo.lock
$ cargo info cc

# View information for a specific version of `cc`
$ cargo info cc@1.1.30
```

**Best Practices**:
- Before adding a new dependency, use `cargo info <crate>` to quickly check its license, documentation, latest version, and maintenance status. This is a great engineering hygiene habit.

#### 2. Apple Silicon Raised to Tier 1
The `aarch64-apple-darwin` target platform is now a Tier 1 platform, meaning the Rust core team guarantees it is fully tested and works reliably.

---

### 🦀 Rust 1.83.0 (2024-11-28)

#### 1. Massive `const` Context Expansions
This version massively expanded what can be executed at compile time.

- **Referencing `static` in `const`**: You can now take the address of a `static` variable in const contexts (but cannot read its value).

```rust
static S: i32 = 25;
// Allowed: taking the address of a static variable.
const C: &i32 = &S; 
```

- **Mutable references in `const`**: You can now use mutable references inside `const fn` for computations.

```rust
// Allowed: using mutable references inside const computations.
const C: i32 = {
    let mut x = 10;
    let y = &mut x;
    *y += 5;
    x
}; // C = 15
```

- **Important limitation**: The final value of a `const` cannot contain mutable references.

```rust
// Error: the final value of a const cannot be a mutable reference.
// const BAD: &mut i32 = &mut 4; 
```

**Best Practices**:
- Leverage these new `const` capabilities to shift more runtime computations to compile time (e.g., building complex lookup tables, parsing compile-time known configurations). This boosts performance and catches errors earlier.

---

### 🦀 Rust 1.84.0 (2025-01-09)

#### 1. Cargo MSRV-Aware Resolver
Cargo now respects your project's MSRV (Minimum Supported Rust Version) and automatically selects dependency versions compatible with it.

Enable it in `.cargo/config.toml`:
```toml
[resolver]
incompatible-rust-versions = "fallback" # or "error"
```

Then, when adding dependencies:
```bash
$ cargo add clap
# If the latest clap requires Rust 1.74, but your project MSRV is 1.60,
# Cargo automatically selects an older, compatible version.
# Updating crates.io index
# warning: ignoring clap@4.5.23 (which requires rustc 1.74) to maintain demo's rust-version of 1.60
# Adding clap v4.0.32 to dependencies
```

**Best Practices**:
- Set the `package.rust-version` field in your `Cargo.toml` to declare your project's MSRV.
- For library authors, enabling the MSRV-aware resolver greatly reduces the maintenance burden of supporting older toolchains.
- In CI, you can override this behavior via the `CARGO_RESOLVER_INCOMPATIBLE_RUST_VERSIONS=allow` environment variable to test with the latest dependencies.

---

### 🦀 Rust 1.85.0 (2025-02-20) — Rust 2024 Edition

This release stabilizes the **Rust 2024 Edition**—a major, opt-in edition containing significant changes.

Key changes include:
- **Language**:
  - **RPIT Lifetime Capture Rules**: `impl Trait` in return types (RPIT) now captures all generic parameters by default, making behavior more intuitive.
  - **`if let` Temporary Scopes**: Temporaries in `if let` now live for the entire `if let` expression, resolving surprising compilation errors from earlier editions.
  - **`unsafe extern` Blocks**: `extern` blocks must now be declared with the `unsafe` keyword.
  - **`unsafe` Attributes**: Attributes like `#[export_name]`, `#[link_section]`, and `#[no_mangle]` must now be marked `unsafe`.
  - **`unsafe_op_in_unsafe_fn` Warn-by-Default**: Performing unsafe operations (e.g., dereferencing raw pointers) inside `unsafe fn` now triggers a warning by default, encouraging explicit `unsafe {}` blocks.

**Best Practices**:
- Use `cargo fix --edition` and follow the [Edition Guide](https://doc.rust-lang.org/edition-guide/rust-2024/index.html) to migrate your projects step-by-step.
- This is the largest edition update to date; schedule dedicated time for migration in your project roadmap.

---

### 🦀 Rust 1.86.0 (2025-04-03)

#### 1. Trait Upcasting (Trait Object Upcasting)
You can now automatically convert a reference to a trait object (e.g., `&dyn SubTrait`) to its super-trait's trait object (e.g., `&dyn SuperTrait`).

```rust
trait SuperTrait {
    fn super_method(&self);
}

trait SubTrait: SuperTrait {
    fn sub_method(&self);
}

struct MyStruct;

impl SuperTrait for MyStruct {
    fn super_method(&self) { println!("Super method"); }
}

impl SubTrait for MyStruct {
    fn sub_method(&self) { println!("Sub method"); }
}

fn main() {
    let sub: &dyn SubTrait = &MyStruct;
    
    // Previously: required manually implementing an `as_super` method.
    // Now: direct upcasting is allowed!
    let super_trait: &dyn SuperTrait = sub; 
    super_trait.super_method();
}
```

**Best Practices**:
- Use this feature to simplify code when dealing with type-erased trait objects or when you need to handle a hierarchy of traits uniformly.
- This feature also works with smart pointers like `Arc<dyn Trait>` and `*const dyn Trait`.

#### 2. `get_disjoint_mut`: Safely Mutable-Borrow Multiple Indices
The standard library adds `get_disjoint_mut` methods to `[T]` and `HashMap`, allowing you to safely obtain mutable references to multiple elements simultaneously.

```rust
fn main() {
    let mut data = vec![1, 2, 3, 4, 5];
    
    // Safely mutably borrow indices 0, 2, and 4 at the same time.
    let [a, b, c] = data.get_disjoint_mut([0, 2, 4]).unwrap();
    *a += 10;
    *b += 10;
    *c += 10;
    
    assert_eq!(data, vec![11, 2, 13, 4, 15]);
}
```

**Best Practices**:
- Use `get_disjoint_mut` to replace unsafe code patterns involving `split_at_mut` or raw pointer manipulation when you need to modify non-overlapping parts of a slice or map simultaneously.
- Always handle the `Result` (or `Option` for slices) returned, as invalid indices (or missing keys for `HashMap`) will cause it to return `None`/`Err`.

Here is the continuation of the in-depth research, covering Rust versions **1.87.0 through 1.96.0**.

---

### 🦀 Rust 1.87.0 (2025-05-15)

#### 1. Anonymous Pipes
The standard library now provides access to anonymous pipes, integrated with `std::process::Command`'s input/output methods.

```rust
use std::process::Command;
use std::io::Read;

fn main() -> std::io::Result<()> {
    let (mut recv, send) = std::io::pipe()?;
    let mut command = Command::new("path/to/bin")
        // Both stdout and stderr write to the same pipe, combining the two.
        .stdout(send.try_clone()?)
        .stderr(send)
        .spawn()?;

    let mut output = Vec::new();
    recv.read_to_end(&mut output)?;
    // Important: read from the pipe before the process exits to avoid
    // filling OS buffers if the program emits too much output.

    assert!(command.wait()?.success());
    Ok(())
}
```


**Best Practices**:
- Use anonymous pipes for efficient inter-process communication without platform-specific code.
- Always read from the pipe before waiting for the child process to avoid deadlocks due to filled OS buffers.

#### 2. Safe Architecture Intrinsics
Most `std::arch` intrinsics that were previously unsafe only due to requiring target features can now be called in safe code when those features are enabled.

```rust
#![forbid(unsafe_op_in_unsafe_fn)]

use std::arch::x86_64::*;

fn sum(slice: &[u32]) -> u32 {
    #[cfg(target_arch = "x86_64")]
    {
        if is_x86_feature_detected!("avx2") {
            // SAFETY: We have detected the feature is enabled at runtime,
            // so it's safe to call this function.
            return unsafe { sum_avx2(slice) };
        }
    }
    slice.iter().sum()
}

#[target_feature(enable = "avx2")]
#[cfg(target_arch = "x86_64")]
fn sum_avx2(slice: &[u32]) -> u32 {
    // Safe AVX2 implementation
    // ...
}
```


**Best Practices**:
- Use `is_x86_feature_detected!()` at runtime before calling target-feature-specific functions.
- Keep the `unsafe` block narrowly scoped around the intrinsic call, with clear safety comments.

---

### 🦀 Rust 1.88.0 (2025-06-26)

#### 1. Let Chains
This feature allows `&&`-chaining of `let` statements inside `if` and `while` conditions, even intermingling with boolean expressions. **Only available in Rust 2024 Edition**.

```rust
// Before: required nested if let and if blocks
// Now:
if let Channel::Stable(v) = release_info()
    && let Semver { major, minor, .. } = v
    && major == 1
    && minor == 88
{
    println!("`let_chains` was stabilized in this version");
}
```


**Best Practices**:
- Migrate your crate to Rust 2024 Edition (`cargo fix --edition`) to use this feature.
- Use let chains to flatten deeply nested conditional logic for better readability.

#### 2. Naked Functions
Rust now supports writing naked functions with no compiler-generated epilogue or prologue, giving full control over the generated assembly.

```rust
#[unsafe(naked)]
pub unsafe extern "sysv64" fn wrapping_add(a: u64, b: u64) -> u64 {
    // Equivalent to `a.wrapping_add(b)`.
    core::arch::naked_asm!(
        "lea rax, [rdi + rsi]",
        "ret"
    );
}
```


**Best Practices**:
- Use naked functions only in low-level settings like operating systems, embedded applications, or compiler builtins.
- The `#[unsafe(naked)]` attribute signals that the compiler will not add any special handling for arguments or return values.
- Always mark naked functions as `unsafe`—they bypass Rust's usual safety guarantees.

#### 3. Boolean Configuration
The `cfg` predicate language now supports `true` and `false` boolean literals.

```rust
// Always enabled
#[cfg(true)]
fn always_available() {}

// Always disabled
#[cfg(false)]
fn never_available() {}
```

**Best Practices**:
- Use boolean `cfg` for debugging or temporarily toggling code without modifying the actual configuration.

---

### 🦀 Rust 1.89.0 (2025-08-07)

#### 1. Explicitly Inferred Arguments to Const Generics
Rust now supports `_` as an argument to const generic parameters, inferring the value from context.

```rust
pub fn all_false<const LEN: usize>() -> [bool; LEN] {
    [false; _]  // LEN is inferred from the return type
}

// Not allowed: `_` in a signature
// pub const fn all_false<const LEN: usize>() -> [bool; _] { [false; LEN] }
```


**Best Practices**:
- Use `_` for const generics to reduce redundancy when the value is obvious from context.
- Do not use `_` in function signatures where it could make the API ambiguous.

#### 2. Mismatched Lifetime Syntaxes Lint
A new warn-by-default lint detects inconsistent lifetime elision that can confuse readers.

```rust
// Warning: hiding a lifetime that's elided elsewhere is confusing
fn items(scores: &[u8]) -> std::slice::Iter<u8> {
    scores.iter()
}
// Help: use `'_` for type paths
// fn items(scores: &[u8]) -> std::slice::Iter<'_, u8>
```


**Best Practices**:
- When returning types with hidden lifetimes (like `Iter`), explicitly write `'_` to make the lifetime visible.
- This lint improves code clarity for both newcomers and experts.

---

### 🦀 Rust 1.90.0 (2025-09-18)

#### 1. LLD is Now the Default Linker on `x86_64-unknown-linux-gnu`
Rust now uses the LLD linker by default on this target, improving linking performance for large binaries and incremental rebuilds.

```toml
# .cargo/config.toml - To opt out of LLD if needed:
[target.x86_64-unknown-linux-gnu]
rustflags = ["-Clinker-features=-lld"]
```


**Best Practices**:
- In most cases, LLD is backwards compatible with BFD and you won't notice any difference other than faster compiles.
- If you encounter linker issues, opt out using the `-Clinker-features=-lld` flag.

#### 2. Cargo Workspace Publishing
`cargo publish --workspace` is now natively supported, publishing all crates in a workspace in dependency order.

```bash
cargo publish --workspace
```

**Best Practices**:
- Use this for workspaces with multiple crates to automate the publishing process.
- Note that publishes are **not atomic**—network or server errors can still result in a partially published workspace.

#### 3. Platform Changes
The `x86_64-apple-darwin` target was demoted from Tier 1 with host tools to Tier 2 with host tools.

---

### 🦀 Rust 1.91.0 (2025-10-30)

#### 1. `aarch64-pc-windows-msvc` Promoted to Tier 1
Windows on 64-bit ARM systems now receives the highest level of support.

**Best Practices**:
- Developers targeting Windows on ARM can now rely on full test coverage and prebuilt binaries.

#### 2. Lint Against Dangling Raw Pointers from Local Variables
A new warn-by-default lint catches raw pointers to local variables being returned from functions.

```rust
fn f() -> *const u8 {
    let x = 0;
    &x  // warning: a dangling pointer will be produced because `x` will be dropped
}
```


**Best Practices**:
- This lint catches a common footgun—raw pointers don't have lifetimes, so returning a pointer to a local is dangerous.
- The code itself isn't unsafe, but dereferencing the returned pointer would be.
- Future releases will add more tooling for safe raw pointer interactions.

---

### 🦀 Rust 1.92.0 (2025-12-11)

#### 1. Never Type Lints Now Deny-by-Default
Two future-compatibility lints related to the never type (`!`) are now deny-by-default, meaning they cause compilation errors when detected.

```rust
// Code affected by `never_type_fallback_flowing_into_unsafe`
// or `dependency_on_unit_never_type_fallback` will now error.
```


**Best Practices**:
- If these lints are reported in your crate graph, fix them immediately.
- You can temporarily `#[allow]` them, but they detect code likely to be broken by future never type stabilization.
- These lints only fire when building the affected crates directly, not as dependencies.

#### 2. `unused_must_use` No Longer Warns on `Result<(), UninhabitedType>`
The lint no longer warns on `Result<(), Infallible>` or similar types where the error can never occur.

```rust
// No warning: error type is uninhabited
fn always_ok() -> Result<(), Infallible> { Ok(()) }
```

**Best Practices**:
- This avoids forcing developers to handle errors that can never happen.

#### 3. Linux Backtraces with `-C panic=abort`
Unwind tables are now generated by default on Linux, fixing backtraces even when using `-C panic=abort`.

---

### 🦀 Rust 1.93.0 (2026-01-22)

#### 1. Bundled musl Updated to 1.2.5
All `*-linux-musl` targets now ship with musl 1.2.5, bringing major improvements to the DNS resolver for static Linux binaries.

**Best Practices**:
- For portable Linux binaries that do networking, this makes them more reliable, especially with large DNS records and recursive nameservers.
- This update includes a breaking change: removal of legacy compatibility symbols. Ensure your dependencies use `libc >= 0.2.146` (released June 2023).

#### 2. Global Allocator Can Use Thread-Local Storage
The standard library now permits global allocators written in Rust to use `thread_local!` and `std::thread::current` without re-entrancy concerns.

#### 3. `cfg` Attributes on `asm!` Lines
`cfg` can now be applied to individual statements within `asm!` blocks, avoiding repetition.

```rust
asm!(
    "nop",
    #[cfg(target_feature = "sse2")] "nop",
    // ...
);
```


**Best Practices**:
- Use per-line `cfg` in inline assembly to keep code DRY and maintainable.

---

### 🦀 Rust 1.94.0 (2026-03-05)

#### 1. Array Windows (`array_windows`)
A new iterating method for slices that returns fixed-length array references (`&[T; N]`) instead of dynamically-sized slices.

```rust
fn has_abba(s: &str) -> bool {
    s.as_bytes()
        .array_windows()
        .any(|[a1, b1, b2, a2]| (a1 != b1) && (a1 == a2) && (b1 == b2))
}
// The destructuring pattern lets the compiler infer we want windows of 4!
```


**Best Practices**:
- Use `array_windows` when you know the window size at compile time—the compiler can infer the length from patterns.
- Unlike `.windows(4)`, you avoid manual indexing and runtime bounds checks.

#### 2. Cargo Config Inclusion
Cargo now supports the `include` key in configuration files for better organization and sharing.

```toml
# .cargo/config.toml
include = [
    "frodo.toml",
    "samwise.toml",
    { path = "optional.toml", optional = true },
]
```


**Best Practices**:
- Use `include` to split large Cargo configurations into manageable pieces.
- Mark optional includes for developer-specific configurations that might not exist.

#### 3. TOML 1.1 Support
Cargo now parses TOML v1.1 for manifests and configuration files, supporting:
- Inline tables across multiple lines with trailing commas
- `\xHH` and `\e` string escape characters
- Optional seconds in times

---

### 🦀 Rust 1.95.0 (2026-04-16)

#### 1. `cfg_select!` Macro
A compile-time conditional macro similar to the popular `cfg-if` crate.

```rust
cfg_select! {
    unix => {
        fn foo() { /* unix specific functionality */ }
    }
    target_pointer_width = "32" => {
        fn foo() { /* non-unix, 32-bit functionality */ }
    }
    _ => {
        fn foo() { /* fallback implementation */ }
    }
}

let is_windows_str = cfg_select! {
    windows => "windows",
    _ => "not windows",
};
```


**Best Practices**:
- Use `cfg_select!` as a built-in alternative to the `cfg-if` crate for compile-time conditional compilation.

#### 2. `if-let` Guards in `match` Expressions
Building on let chains from 1.88, this brings pattern-matching conditionals into `match` expressions.

```rust
match value {
    Some(x) if let Ok(y) = compute(x) => {
        // Both `x` and `y` are available here
        println!("{}, {}", x, y);
    }
    _ => {}
}
```


**Best Practices**:
- Use `if let` guards for complex matching conditions that depend on nested pattern matches.
- Note that patterns in `if let` guards are not considered in exhaustiveness evaluation.

#### 3. Removal of Custom Target Specs on Stable
Support for passing custom target specifications to `rustc` on stable was removed.

**Best Practices**:
- Custom target specifications now require nightly Rust.
- The team is gathering use cases for potential future stabilization.

---

### 🦀 Rust 1.96.0 (2026-05-28)

#### 1. New `core::range::Range*` Types
New range types that implement `IntoIterator` rather than `Iterator`, making them `Copy`-able.

```rust
use core::range::Range;

#[derive(Clone, Copy)]
pub struct Span(Range<usize>);

impl Span {
    pub fn of(self, s: &str) -> &str {
        &s[self.0]  // Now works because Range is Copy!
    }
}
```


**Best Practices**:
- Use `core::range::Range` and friends in public APIs where you need `Copy` range types.
- The `Range` syntax (`0..1`) still produces legacy types for now, but will switch to `core::range` types in a future edition.
- Library authors should use `impl RangeBounds` in public APIs to accept both legacy and new range types.

#### 2. `assert_matches!` and `debug_assert_matches!`
New macros that check if a value matches a pattern, panicking with a debug representation otherwise.

```rust
use core::assert_matches;

fn get_random_number() -> u32 {
    4  // chosen by fair dice roll, guaranteed to be random
}

fn main() {
    assert_matches!(get_random_number(), 1..=6);
}
```


**Best Practices**:
- These macros are not in the prelude—import them manually from `core` or `std`.
- They provide better diagnostics than `assert!(matches!(..))` by printing the value on failure.

#### 3. WebAssembly Breaking Change
WebAssembly targets no longer pass `--allow-undefined` to the linker. Undefined symbols are now linker errors.

**Best Practices**:
- This change catches bugs earlier by preventing modules from linking with missing symbols.
- If the old behavior is intended, re-enable with `RUSTFLAGS=-Clink-arg=--allow-undefined`.
- Or use `#[link(wasm_import_module = "env")]` on the block defining the symbol.

#### 4. Security Advisories
Two CVEs were fixed for users of third-party registries:
- **CVE-2026-5223** (medium): symlink extraction in crate tarballs
- **CVE-2026-5222** (low): authentication with normalized URLs

**Best Practices**:
- Users of `crates.io` are not affected by either vulnerability.

---

