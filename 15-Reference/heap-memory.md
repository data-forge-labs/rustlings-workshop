# Heap Memory Architecture — Rust vs Python

## Stack vs Heap — Step-by-Step Execution Trace

The following trace shows how Rust's ownership model manages stack and heap memory line by line.

```rust
fn main() {
    let x: i32 = 42;           // Step 1
    let name = String::from("Rust");  // Step 2
    let v = vec![1, 2, 3];     // Step 3
    let b = Box::new(99);      // Step 4
    let y = x;                 // Step 5: i32: Copy
    let z = name;              // Step 6: String: Move!
    // name is now invalid
    println!("{}", z);         // Step 7
} // Everything dropped here    // Step 8
```

### Execution Trace — Three-Column View

| Step | Code | Stack (before end of function) | Heap |
|------|------|--------------------------------|------|
| 1 | `let x: i32 = 42;` | `x: i32 = 42` (4B) | — |
| 2 | `let name = String::from("Rust");` | `x: i32 = 42` (4B)<br>`name: String { ptr, len=4, cap=4 }` (24B) | `"Rust"` (4B) ← `name.ptr` |
| 3 | `let v = vec![1, 2, 3];` | `x: i32 = 42` (4B)<br>`name: String { ptr, len=4, cap=4 }` (24B)<br>`v: Vec { ptr, len=3, cap=3 }` (24B) | `"Rust"` (4B) ← `name.ptr`<br>`[1, 2, 3]` (12B) ← `v.ptr` |
| 4 | `let b = Box::new(99);` | `x: i32 = 42` (4B)<br>`name: String { ptr, len=4, cap=4 }` (24B)<br>`v: Vec { ptr, len=3, cap=3 }` (24B)<br>`b: Box { ptr }` (8B) | `"Rust"` (4B) ← `name.ptr`<br>`[1, 2, 3]` (12B) ← `v.ptr`<br>`99` (4B) ← `b.ptr` |
| 5 | `let y = x;` | `x: i32 = 42` (4B)<br>`name: String { ptr, len=4, cap=4 }` (24B)<br>`v: Vec { ptr, len=3, cap=3 }` (24B)<br>`b: Box { ptr }` (8B)<br>`y: i32 = 42` (4B) | (unchanged — `i32` is `Copy`) |
| 6 | `let z = name;` | `x: i32 = 42` (4B)<br>`name: INVALID (moved)` (24B)<br>`v: Vec { ptr, len=3, cap=3 }` (24B)<br>`b: Box { ptr }` (8B)<br>`y: i32 = 42` (4B)<br>`z: String { ptr, len=4, cap=4 }` (24B) | `"Rust"` (4B) ← `z.ptr`<br>`[1, 2, 3]` (12B) ← `v.ptr`<br>`99` (4B) ← `b.ptr` |
| 7 | `println!("{}", z);` | (unchanged) | (unchanged — prints "Rust") |
| 8 | `}` (scope end) | All stack vars dropped | `"Rust"` freed (via `z`)<br>`[1, 2, 3]` freed (via `v`)<br>`99` freed (via `b`) |

**Key observations:**
- **Step 5** (`y = x`): `i32` implements `Copy` — bitwise copy, both `x` and `y` valid
- **Step 6** (`z = name`): `String` does **not** implement `Copy` — **move** transfers ownership, `name` becomes invalid
- **Step 8**: RAII — `z`, `v`, `b` all implement `Drop`, heap memory freed automatically in reverse order

### Stack vs Heap — The Basics

```
  ┌───────────────────┐      ┌──────────────────────┐
  │   STACK (LIFO)    │      │   HEAP (dynamic)      │
  ├───────────────────┤      ├──────────────────────┤
  │ i32:      4 bytes │      │ String data:          │
  │ f64:      8 bytes │      │  "hello world..."     │
  │ [i32;5]: 20 bytes │      │ Vec elements:         │
  │ &T:       8 bytes │      │  [1, 2, 3, 4, ...]   │
  │ String:  24 bytes │      │ Box<T> value          │
  │ Vec<i32>: 24 bytes│      │ HashMap entries       │
  ├───────────────────┤      ├──────────────────────┤
  │ Fixed size        │      │ Variable size         │
  │ Compiler known    │      │ Runtime known         │
  │ No alloc cost     │      │ Alloc + free cost     │
  │ Thread-local      │      │ Shared across threads │
  └───────────────────┘      └──────────────────────┘
```

### Rust — Stack by Default, Explicit Heap

```rust
fn main() {
    let x = 42;             // stack: 4 bytes (i32)
    let y = 3.14;           // stack: 8 bytes (f64)
    let arr = [1, 2, 3];    // stack: 12 bytes ([i32; 3])
    
    let s = String::from("hello");  // stack: 24 bytes (ptr + len + cap)
                                     // heap: 5 bytes for "hello"
    let v = vec![1, 2, 3];          // stack: 24 bytes (ptr + len + cap)
                                     // heap: 12 bytes for [1, 2, 3]
    let b = Box::new(42);           // heap: 4 bytes, stack: 8 byte pointer
}
```

### Python — Everything on the Heap

```python
x = 42          # PyObject on heap (~28 bytes)
y = 3.14        # PyObject on heap (~24 bytes)
arr = [1, 2, 3] # PyListObject + 3 PyObject pointers on heap
s = "hello"     # PyUnicodeObject on heap (~70 bytes)
```

```
  Python Memory Layout:
  ┌─────────────────────────────────────────────────┐
  │  Python Heap (PyMem_Malloc / pymalloc)          │
  │                                                  │
  │  ┌──────────┐  ┌──────────┐  ┌──────────┐      │
  │  │ PyObject │  │ PyObject │  │ PyObject │      │
  │  │ ob_refcnt│  │ ob_refcnt│  │ ob_refcnt│      │
  │  │ ob_type  │  │ ob_type  │  │ ob_type  │      │
  │  │ value=42 │  │ value=___│  │ items=[ ]│      │
  │  └──────────┘  └──────────┘  └──────────┘      │
  │                                                  │
  │  Every Python value = PyObject header (16-56 B)  │
  │  + value bytes. No stack allocation for user data │
  └─────────────────────────────────────────────────┘
```

## Rust Heap Internals

### How Box, Vec, String Use the Heap

```
  String on stack (24 bytes)          Heap
  ──────────────────────────          ────
  ┌──────────┬──────┬──────┐
  │  ptr     │ len  │ cap  │──────┐  ┌────────────┐
  │ 0x7f...  │  5   │  5   │      └──│ h │ e │ l │ l │ o │
  └──────────┴──────┴──────┘          └────────────┘

  Vec<i32> on stack (24 bytes)        Heap
  ───────────────────────────         ────
  ┌──────────┬──────┬──────┐
  │  ptr     │ len  │ cap  │──────┐  ┌─────┬─────┬─────┐
  │ 0x7f...  │  3   │  3   │      └──│  1  │  2  │  3  │
  └──────────┴──────┴──────┘          └─────┴─────┴─────┘

  Box<i32> on stack (8 bytes)         Heap
  ────────────────────────            ────
  ┌──────────┐
  │  ptr     │──────────────────────┐  ┌─────┐
  │ 0x7f...  │                      └──│ 42  │
  └──────────┘                          └─────┘
```

### The Global Allocator

Rust uses a configurable global allocator. By default it's the **system allocator** on most platforms (Windows: `HeapAlloc`, Linux: `glibc malloc`), but you can swap it:

```rust
// Use jemalloc instead of the system allocator
use jemallocator::Jemalloc;
#[global_allocator]
static GLOBAL: Jemalloc = Jemalloc;

// Or mimalloc (fast, modern allocator)
use mimalloc::MiMalloc;
#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;
```

Key allocator properties:

| Allocator | Fragmentation | Thread Scaling | Speed | Use Case |
|-----------|--------------|----------------|-------|----------|
| System (`malloc`) | Medium | Poor | Fast | Default, simple |
| **jemalloc** | Low | Excellent | Fast | High-concurrency, Firefox, Rust (historical default) |
| **mimalloc** | Low | Excellent | Very fast | Modern alternative, Microsoft |
| **tcmalloc** | Low | Good | Fast | Google's allocator, gperftools |

## Off-Heap Memory — What Is It?

"Off-heap" means **memory allocated outside the language runtime's managed heap**. The concept matters in Python/Java where a GC heap exists; in Rust there's no managed heap — all allocations are "off-heap" by comparison.

### Python's Memory Hierarchy

```
  Python Process Memory
  ┌──────────────────────────────────────────────────┐
  │ Stack (C call frames)                            │
  ├──────────────────────────────────────────────────┤
  │ Python Heap (managed by pymalloc / PyMem)        │  ← The "heap"
  │ ┌──────────────────────────────────────────────┐ │
  │ │  PyObject arena (256 KB blocks)              │ │
  │ │  [int] [str] [dict] [list] [object] ...      │ │
  │ │  Every Python value lives here                │ │
  │ └──────────────────────────────────────────────┘ │
  ├──────────────────────────────────────────────────┤
  │ Off-Heap (NOT managed by Python's GC)            │  ← "Off-heap"
  │ ┌──────────────────────────────────────────────┐ │
  │ │  numpy arrays (.npy data buffer)             │ │
  │ │  PyArrow buffers (shared memory)             │ │
  │ │  memory-mapped files (mmap)                  │ │
  │ │  C extensions (malloc/free directly)         │ │
  │ │  DMA buffers, GPU memory                     │ │
  │ └──────────────────────────────────────────────┘ │
  └──────────────────────────────────────────────────┘
```

**In Python:** "Off-heap" = data allocated via C-level `malloc`/`mmap` instead of Python's `PyMem_Malloc`. The GC doesn't track it — you must free it manually (or via a wrapper object's `__del__`).

Examples:
```python
import numpy as np
arr = np.zeros((1000, 1000))  # arr.data is a ~8 MB buffer off-heap
                                # arr is a small PyObject on the Python heap
                                # but the float64 data lives in a C malloc buffer

import mmap
m = mmap.mmap(-1, 1024*1024)   # 1 MB off-heap (OS-managed)
```

### Java's Memory Hierarchy

```
  JVM Process Memory
  ┌──────────────────────────────────────────────────┐
  │ Stack (thread stacks)                            │
  ├──────────────────────────────────────────────────┤
  │ Managed Heap (Young + Old Gen)                   │  ← The "heap"
  │ ┌──────────────────────────────────────────────┐ │
  │ │  Java objects, arrays                        │ │
  │ │  GC-managed, compacted, swept               │ │
  │ └──────────────────────────────────────────────┘ │
  ├──────────────────────────────────────────────────┤
  │ Off-Heap (NOT managed by GC)                     │  ← "Off-heap"
  │ ┌──────────────────────────────────────────────┐ │
  │ │  Direct ByteBuffer (malloc'd)               │ │
  │ │  Memory-mapped files (NIO FileChannel)       │ │
  │ │  sun.misc.Unsafe.allocateMemory()            │ │
  │ │  JNI native allocations                      │ │
  │ └──────────────────────────────────────────────┘ │
  └──────────────────────────────────────────────────┘
```

### Rust — No "Heap vs Off-Heap" Distinction

Since Rust has no GC-managed heap, **all allocations are equivalent**. But the concept maps to:

| Concept | Python / Java | Rust Equivalent |
|---------|--------------|-----------------|
| Managed heap | Python PyMem / Java GC | `#[global_allocator]` (just the allocator) |
| Off-heap data | numpy buffers, DirectByteBuffer | All heap data is "off-heap" in that sense |
| GC-managed objects | All Python objects, Java objects | N/A — RAII handles everything |
| Custom allocator | `PyMem_SetAllocator`, custom allocator | `#[global_allocator]` or per-type `Allocator` |
| Memory-mapped I/O | `mmap.mmap`, `FileChannel.map` | `memmap2` crate, `mmap` syscall |

```rust
use memmap2::MmapMut;
use std::fs::OpenOptions;

// Memory-mapped file — zero-copy I/O, OS-managed pages
let file = OpenOptions::new()
    .read(true).write(true).create(true)
    .open("data.bin")?;
file.set_len(1024 * 1024)?;
let mut mmap = unsafe { MmapMut::map_mut(&file)? };

// Direct write to mapped memory (no syscall, no heap alloc)
mmap[0..4].copy_from_slice(&42u32.to_ne_bytes());
```

## Rust Heap vs Python Heap — Key Differences

```
  Comparison:
  ┌─────────────────────────────────────────────────────────────┐
  │  Aspect              │  Python                     │  Rust │
  ├──────────────────────┼─────────────────────────────┼───────┤
  │ Per-value overhead   │  ~28-56 bytes (PyObject)    │  0    │
  │ Integer size         │  28 bytes (PyLongObject)    │  4 B  │
  │ List/Vec of 1M ints  │  ~28 MB + 8 MB for pointers│  4 MB │
  │ Allocation pattern   │  Frequent, GC-managed       │  Rare │
  │ Free pattern         │  GC sweep (unpredictable)   │  RAII │
  │ GC pauses?           │  Yes (stop-the-world)       │  No   │
  │ Memory fragmentation │  High (GC compaction helps) │  Low  │
  │ Thread-local alloc?  │  Per-interpreter arenas     │  Yes  │
  │ Custom allocator     │  PyMem_SetAllocator         │  Yes  │
  └─────────────────────────────────────────────────────────────┘
```

### Python Object Overhead

```python
import sys
print(sys.getsizeof(42))          # 28 bytes (on 64-bit)
print(sys.getsizeof("hello"))     # 54 bytes + string data
print(sys.getsizeof([1,2,3]))     # 104 bytes (list) + 3 * 8 bytes (pointers)
```

Every Python value has a `PyObject` header (`ob_refcnt` + `ob_type` = 16 bytes minimum on 64-bit). For integers, floats, and small strings, the overhead **exceeds the data itself**.

### Rust Zero Overhead

```rust
println!("{}", std::mem::size_of::<i32>());           // 4
println!("{}", std::mem::size_of::<String>());        // 24 (stack) + heap data
println!("{}", std::mem::size_of::<Vec<i32>>());      // 24 (stack) + heap data
```

Rust types have **zero overhead** — no header, no refcount, no vtable (unless you opt in via `dyn Trait` or `Rc`).

## Why This Matters for Data Engineering

### Memory Budget

```python
# 10 million floats
import array
a = array.array('d', [0.0]) * 10_000_000  # 80 MB (dense, off-heap via C buffer)
python_list = [0.0] * 10_000_000           # ~280 MB (80 MB data + 200 MB PyObject overhead)
```

```rust
// Rust — same data
let v: Vec<f64> = vec![0.0; 10_000_000];  // 80 MB (exactly 8 bytes per element)
```

### GC Pause Impact

```
  Latency Profile:
  
  Python GC:            ▁▂▃▅▇██▇▅▃▂▁   pauses of 50-200ms
  Rust alloc/free:      ▁▁▁▁▄▃▁▁▁▁▁   deterministic, microseconds
  
  Green = work, Red = GC/alloc pause
```

### When Off-Heap Matters in Rust

In Rust, you explicitly control where data lives:

- **Stack** — small, fixed-size data that doesn't outlive the function
- **Heap (Box/Vec/String)** — dynamic data with RAII cleanup
- **Mmap (memmap2)** — file-backed, OS-paged, zero-copy access for large datasets
- **Custom allocators** — arena allocators for game engines, slab allocators for network packets
- **Huge pages** — 2 MB pages for large allocations (improves TLB cache hit rate)

```rust
use std::alloc::{alloc, Layout};

// Direct low-level allocation — truly "off-heap" (bypasses normal Rust allocator)
let layout = Layout::from_size_align(4096, 4096).unwrap();
let ptr = unsafe { alloc(layout) };
// ... use ptr ...
// Must free manually:
// unsafe { dealloc(ptr, layout); }
```

## Summary

| Concept | Python | Rust |
|---------|--------|------|
| All user data | On GC heap (PyObject) | Stack (primitives) + Heap (dynamic) |
| Object overhead | ~28-56 bytes per value | 0 bytes (exact data only) |
| Memory management | Reference counting + GC | RAII / ownership (compile-time) |
| Off-heap data | numpy, Arrow, mmap, C buffers | All heap data; explicit mmap/alloc |
| Custom allocator | `PyMem_SetAllocator` | `#[global_allocator]` or per-type |
| GC pauses | Yes (stop-the-world sweep) | No (deterministic drop) |
| Thread-local caching | Per-interpreter arena | Per-thread (jemalloc/mimalloc) |
