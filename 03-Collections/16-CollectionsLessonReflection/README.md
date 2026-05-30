# Lesson 1 Reflection Project

> **Test-driven approach**: This project includes a Cargo project with progressive unit tests. Each function in `src/lib.rs` starts as a `todo!()` stub. As you follow each section, replace `todo!()` with real code and run `cargo test` to watch the pass count grow. Your goal: **all 7 tests pass**.

## Objective
This project serves as a comprehensive reflection on the key concepts learned in Lesson 1 about Rust collections. Through this project, you'll consolidate your understanding of:
- Different collection types and their use cases
- Performance characteristics of various collections
- Best practices for collection usage
- Trade-offs between different data structures
- Real-world applications of collections

## Key Concepts Covered

### 1. Vector (Vec<T>)
- Dynamic array implementation
- Similar to Python lists
- Efficient random access
- Contiguous memory allocation
- Best for:
  - Random access operations
  - Sequential iteration
  - Memory efficiency

### 2. LinkedList
- Doubly-linked list implementation
- Efficient insertions/deletions
- Non-contiguous memory
- Best for:
  - Frequent insertions/deletions
  - Dynamic size changes
  - When random access isn't needed

### 3. VecDeque
- Double-ended queue
- Ring buffer implementation
- Fast push/pop at both ends
- Best for:
  - Queue implementations
  - Sliding window algorithms
  - Breadth-first search

### 4. HashMap
- Key-value storage
- O(1) average operations
- Hash-based implementation
- Best for:
  - Frequency counting
  - Lookup tables
  - Associative arrays

## Performance Considerations

### 1. Memory Efficiency
- Vec: Most memory efficient (contiguous)
- LinkedList: Less efficient (pointers)
- VecDeque: Moderate (ring buffer)
- HashMap: Moderate (hash table overhead)

### 2. Operation Complexity
- Random Access:
  - Vec: O(1)
  - LinkedList: O(n)
  - VecDeque: O(1)
  - HashMap: O(1)

- Insertion/Deletion:
  - Vec: O(n) at middle, O(1) at end
  - LinkedList: O(1)
  - VecDeque: O(1) at ends
  - HashMap: O(1)

## Best Practices

### 1. Collection Selection
- Use Vec for general-purpose lists
- Use LinkedList for frequent middle insertions
- Use VecDeque for queue-like operations
- Use HashMap for key-value lookups

### 2. Mutability
- Prefer immutable collections when possible
- Use mut only when necessary
- Consider ownership and borrowing rules

### 3. Performance Optimization
- Consider cache locality
- Choose appropriate initial capacity
- Use appropriate collection for operation patterns

## Real-World Applications

### 1. Vec Use Cases
- Data processing pipelines
- Image processing
- Game state management

### 2. LinkedList Use Cases
- Undo/redo operations
- Text editors
- Music playlists

### 3. VecDeque Use Cases
- Task scheduling
- Network packet buffering
- Breadth-first search

### 4. HashMap Use Cases
- Word frequency counting
- Cache implementations
- Configuration storage

## Rust Fundamentals

### 1. Collection Types Overview
```rust
// Vector (Vec<T>)
let mut vec = Vec::new();
vec.push(1);  // O(1) amortized
vec.insert(0, 2);  // O(n)

// VecDeque
use std::collections::VecDeque;
let mut deque = VecDeque::new();
deque.push_front(1);  // O(1)
deque.push_back(2);   // O(1)

// LinkedList
use std::collections::LinkedList;
let mut list = LinkedList::new();
list.push_front(1);  // O(1)
list.push_back(2);   // O(1)

// HashMap
use std::collections::HashMap;
let mut map = HashMap::new();
map.insert("key", "value");  // O(1) average
```

### 2. Performance Characteristics
```rust
// Vector performance
let mut vec = Vec::with_capacity(1000);
for i in 0..1000 {
    vec.push(i);  // Fast, O(1) amortized
}

// VecDeque performance
let mut deque = VecDeque::with_capacity(1000);
for i in 0..1000 {
    deque.push_back(i);  // Fast, O(1)
    deque.push_front(i); // Fast, O(1)
}

// LinkedList performance
let mut list = LinkedList::new();
for i in 0..1000 {
    list.push_back(i);  // Fast, O(1)
    list.push_front(i); // Fast, O(1)
}
```

### 3. Memory Layout
```rust
// Vector memory layout
let vec = vec![1, 2, 3];
// Contiguous memory block
// [1][2][3]

// LinkedList memory layout
let mut list = LinkedList::new();
list.push_back(1);
list.push_back(2);
// Non-contiguous memory
// [1] -> [2]

// VecDeque memory layout
let mut deque = VecDeque::new();
deque.push_back(1);
deque.push_front(2);
// Ring buffer
// [2][ ][1]
```

### 4. Use Cases and Trade-offs
```rust
// When to use Vec
let mut vec = Vec::new();
// - Need random access
// - Mostly appending elements
// - Memory efficiency important

// When to use VecDeque
let mut deque = VecDeque::new();
// - Need fast operations at both ends
// - Implementing queues
// - Moderate memory overhead acceptable

// When to use LinkedList
let mut list = LinkedList::new();
// - Frequent insertions/deletions in middle
// - Don't need random access
// - Memory overhead acceptable

// When to use HashMap
let mut map = HashMap::new();
// - Need key-value pairs
// - Fast lookups needed
// - Order not important
```

### 5. Iterator Patterns
```rust
// Vector iteration
let vec = vec![1, 2, 3];
for item in &vec {
    println!("{}", item);
}

// HashMap iteration
let map = HashMap::from([("a", 1), ("b", 2)]);
for (key, value) in &map {
    println!("{}: {}", key, value);
}

// Iterator methods
let sum: i32 = vec.iter().sum();
let doubled: Vec<i32> = vec.iter().map(|x| x * 2).collect();
```

## Project Implementation

### 1. Collection Type Selection
```rust
// Choosing the right collection
fn select_collection(use_case: &str) -> Box<dyn Iterator<Item=i32>> {
    match use_case {
        "random_access" => {
            let vec = vec![1, 2, 3];
            Box::new(vec.into_iter())
        },
        "frequent_insertions" => {
            let mut list = LinkedList::new();
            list.push_back(1);
            list.push_back(2);
            Box::new(list.into_iter())
        },
        _ => {
            let deque = VecDeque::from(vec![1, 2, 3]);
            Box::new(deque.into_iter())
        }
    }
}
```

### 2. Performance Analysis
```rust
fn analyze_performance(collection: &[i32]) {
    let start = std::time::Instant::now();
    // Perform operations
    let _sum: i32 = collection.iter().sum();
    let duration = start.elapsed();
    println!("Operation took: {:?}", duration);
}
```

### 3. Memory Usage
```rust
fn analyze_memory<T>(collection: &[T]) {
    let size = std::mem::size_of_val(collection);
    println!("Memory usage: {} bytes", size);
}
```

### 4. Use Case Examples
```rust
// Vector use case
fn process_vector() {
    let mut vec = Vec::new();
    for i in 0..1000 {
        vec.push(i);  // Fast append
    }
    let _ = vec[500];  // Fast random access
}

// LinkedList use case
fn process_linked_list() {
    let mut list = LinkedList::new();
    for i in 0..1000 {
        list.push_back(i);  // Fast append
    }
    // Fast insertion in middle
    let mut cursor = list.cursor_front_mut();
    cursor.insert_after(999);
}
```

## Advanced Collection Concepts

### 1. Collection Traits and Implementations
```rust
// Common traits implemented by collections
use std::iter::Iterator;
use std::ops::Index;
use std::fmt::Debug;

// Custom collection implementing common traits
struct CustomVec<T> {
    data: Vec<T>,
}

impl<T> CustomVec<T> {
    fn new() -> Self {
        CustomVec { data: Vec::new() }
    }
}

impl<T> Iterator for CustomVec<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.data.pop()
    }
}

impl<T> Index<usize> for CustomVec<T> {
    type Output = T;
    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}
```

### 2. Memory Management and Ownership
```rust
// Ownership patterns with collections
fn ownership_examples() {
    // Moving ownership
    let vec1 = vec![1, 2, 3];
    let vec2 = vec1;  // vec1 is moved
    // println!("{:?}", vec1);  // Error: vec1 is no longer valid

    // Borrowing
    let vec3 = vec![4, 5, 6];
    let sum = calculate_sum(&vec3);  // vec3 is borrowed
    println!("Sum: {}", sum);  // vec3 is still valid

    // Mutable borrowing
    let mut vec4 = vec![7, 8, 9];
    modify_vec(&mut vec4);
    println!("Modified: {:?}", vec4);
}

fn calculate_sum(vec: &Vec<i32>) -> i32 {
    vec.iter().sum()
}

fn modify_vec(vec: &mut Vec<i32>) {
    vec.push(10);
}
```

### 3. Advanced Iterator Patterns
```rust
// Complex iterator chains
fn advanced_iterators() {
    let numbers = vec![1, 2, 3, 4, 5];
    
    // Filter and map
    let evens_squared: Vec<_> = numbers.iter()
        .filter(|&x| x % 2 == 0)
        .map(|x| x * x)
        .collect();
    
    // Fold and reduce
    let sum = numbers.iter().fold(0, |acc, &x| acc + x);
    let product = numbers.iter().reduce(|acc, &x| acc * x);
    
    // Zip and enumerate
    let pairs: Vec<_> = numbers.iter()
        .enumerate()
        .zip(numbers.iter().rev())
        .collect();
}
```

### 4. Collection Performance Optimization
```rust
// Performance optimization techniques
fn optimize_collections() {
    // Pre-allocate capacity
    let mut vec = Vec::with_capacity(1000);
    for i in 0..1000 {
        vec.push(i);
    }

    // Use appropriate collection for access pattern
    let mut deque = VecDeque::with_capacity(1000);
    for i in 0..1000 {
        deque.push_back(i);
        deque.push_front(i);
    }

    // Optimize HashMap with good hash function
    let mut map = HashMap::with_capacity_and_hasher(
        1000,
        std::collections::hash_map::RandomState::new()
    );
}
```

### 5. Error Handling with Collections
```rust
// Error handling patterns
fn collection_error_handling() {
    let vec = vec![1, 2, 3];
    
    // Safe access with get
    match vec.get(5) {
        Some(value) => println!("Value: {}", value),
        None => println!("Index out of bounds"),
    }
    
    // Using unwrap_or
    let value = vec.get(5).unwrap_or(&0);
    
    // Custom error types
    #[derive(Debug)]
    enum CollectionError {
        OutOfBounds(usize),
        Empty,
    }
    
    fn safe_access<T>(vec: &Vec<T>, index: usize) -> Result<&T, CollectionError> {
        if vec.is_empty() {
            return Err(CollectionError::Empty);
        }
        vec.get(index).ok_or(CollectionError::OutOfBounds(index))
    }
}
```

## Practical Applications

### 1. Data Processing Pipeline
```rust
// Efficient data processing
fn process_data() {
    let data = vec![1, 2, 3, 4, 5];
    
    // Pipeline processing
    let result: Vec<_> = data.into_iter()
        .filter(|&x| x % 2 == 0)
        .map(|x| x * 2)
        .collect();
    
    // Parallel processing
    use rayon::prelude::*;
    let parallel_result: Vec<_> = data.par_iter()
        .filter(|&x| x % 2 == 0)
        .map(|x| x * 2)
        .collect();
}
```

### 2. Cache Implementation
```rust
// LRU Cache using collections
use std::collections::HashMap;
use std::collections::VecDeque;

struct LRUCache<K, V> {
    capacity: usize,
    map: HashMap<K, V>,
    order: VecDeque<K>,
}

impl<K: Eq + std::hash::Hash + Clone, V> LRUCache<K, V> {
    fn new(capacity: usize) -> Self {
        LRUCache {
            capacity,
            map: HashMap::new(),
            order: VecDeque::new(),
        }
    }
    
    fn get(&mut self, key: &K) -> Option<&V> {
        if let Some(value) = self.map.get(key) {
            self.order.retain(|k| k != key);
            self.order.push_back(key.clone());
            Some(value)
        } else {
            None
        }
    }
}
```

### 3. Graph Representation
```rust
// Graph using collections
struct Graph {
    adjacency_list: HashMap<usize, Vec<usize>>,
}

impl Graph {
    fn new() -> Self {
        Graph {
            adjacency_list: HashMap::new(),
        }
    }
    
    fn add_edge(&mut self, from: usize, to: usize) {
        self.adjacency_list.entry(from)
            .or_insert_with(Vec::new)
            .push(to);
    }
    
    fn bfs(&self, start: usize) -> Vec<usize> {
        let mut visited = Vec::new();
        let mut queue = VecDeque::new();
        queue.push_back(start);
        
        while let Some(node) = queue.pop_front() {
            if !visited.contains(&node) {
                visited.push(node);
                if let Some(neighbors) = self.adjacency_list.get(&node) {
                    for &neighbor in neighbors {
                        queue.push_back(neighbor);
                    }
                }
            }
        }
        visited
    }
}
```

## Best Practices and Patterns

### 1. Collection Selection Guidelines
- Use `Vec` for:
  - Random access
  - Sequential iteration
  - Memory efficiency
  - When size is unknown
- Use `VecDeque` for:
  - Queue operations
  - Sliding windows
  - When both ends need access
- Use `LinkedList` for:
  - Frequent insertions/deletions
  - When order matters
  - When size changes often
- Use `HashMap` for:
  - Key-value lookups
  - Set operations
  - When order doesn't matter

### 2. Performance Optimization
- Pre-allocate capacity when size is known
- Use appropriate collection for access pattern
- Consider cache locality
- Use iterator methods for efficiency
- Implement custom collections when needed

### 3. Memory Management
- Understand ownership and borrowing
- Use references when possible
- Consider memory layout
- Implement Drop trait when needed
- Handle memory leaks

## Running the Project
1. Ensure you have Rust and Cargo installed
2. Navigate to the project directory
3. Run `cargo build` to compile
4. Run `cargo run` to execute the program

## Learning Outcomes
- Understanding advanced collection concepts
- Implementing custom collections
- Optimizing collection performance
- Handling errors with collections
- Applying collections to real-world problems
- Following Rust best practices
- Understanding memory management 