//! # Reflection Questions:
//!
//! ## What is a LinkedList in Rust and how it different from a Vector or a VecDeque?
//!
//! A `LinkedList` in Rust is a collection data structure belonging to the standard library's
//! `std::collections` module. It represents a doubly-linked list, where each element is
//! connected to the next and previous element, allowing for efficient insertion and removal
//! of elements at any position in the sequence. This structure is particularly useful when
//! the application requires frequent insertion and removal operations over random access.
//!
//! Comparatively, a `Vector` (Vec) is a resizable array that provides efficient access to
//! elements by index, making it ideal for scenarios where random access and space efficiency
//! are priorities. However, inserting or removing elements from anywhere but the end of a Vec
//! can be costly due to the need to shift elements.
//!
//! On the other hand, a `VecDeque` is a double-ended queue that supports efficient insertion
//! and removal of elements from both ends. It combines some of the best features of Vecs and
//! LinkedLists, offering more flexibility than a Vec for certain use cases without the
//! overhead of a LinkedList's per-element pointers. However, like Vecs, VecDeque also
//! struggles with efficiency when it comes to inserting or removing elements from the middle.
//!
//! In summary, the choice between using a LinkedList, Vec, or VecDeque depends on the specific
//! requirements of the use case, such as the need for efficient random access, insertion and
//! removal speed, and memory overhead considerations.
//!
//! ## In what situations might you prefer to use a LinkedList over other data structures?
//!
//! You might prefer to use a `LinkedList` over other data structures in several specific
//! situations, taking into account both its advantages and certain limitations, such as cache
//! locality concerns. Here are scenarios where a LinkedList might be preferred:
//!
//! 1. **Frequent Insertions/Removals**: LinkedLists excel when the workload involves frequent
//! insertions or removals of elements, especially at non-end positions. Unlike array-based
//! structures (e.g., `Vec`), LinkedLists don't require shifting elements, making these operations
//! more efficient.
//!
//! 2. **Memory Allocation Concerns**: They allocate memory individually for each element, which
//! can be advantageous in scenarios where avoiding memory fragmentation is important or when
//! collection sizes vary significantly.
//!
//! 3. **Element-wise Processing**: For tasks that involve traversing the list to process or remove
//! elements based on certain conditions, the nature of LinkedLists facilitates efficient
//! next-element access and removal without the overhead of reorganizing the entire structure.
//!
//! 4. **Specialized Algorithms/Data Structures**: Certain algorithms or complex data structures
//! that benefit from the dynamic, node-based storage of LinkedLists might perform better or be
//! easier to implement using this type of collection.
//!
//! **Cache Locality Concerns**: One significant downside of LinkedLists compared to contiguous
//! storage options (like `Vec` or `VecDeque`) is poorer cache locality. Modern CPUs are designed
//! to efficiently access data that is stored contiguously in memory. When accessing elements in a
//! LinkedList, the non-contiguous nature of its storage means that the CPU cache is utilized less
//! effectively. This can lead to slower overall performance, particularly for operations that
//! involve iterating over the elements of the list. The additional memory overhead for storing
//! pointers to the next and previous elements also contributes to increased memory usage.
//!
//! In summary, while LinkedLists offer certain operational benefits, it's crucial to consider
//! their impact on cache locality and overall performance. For many use cases, especially those
//! involving large datasets or where iteration speed is critical, the advantages of contiguous
//! storage might outweigh the benefits of a LinkedList's dynamic insertion and removal capabilities.
//!
//! ## Why is there a need to convert the LinkedList to a Vec and then back to LinkedList in this program?
//!
//! The need to convert a `LinkedList` to a `Vec` and then back to a `LinkedList` in this program
//! arises primarily due to the operation being performed - shuffling the elements. The `rand`
//! crate's `shuffle` method operates on slices, which requires contiguous memory storage, unlike
//! the non-contiguous nature of `LinkedList`. Here's a breakdown of the process and its rationale:
//!
//! 1. **Shuffling**: To shuffle the elements of the `LinkedList`, they first need to be in a form
//! that supports random access efficiently. A `Vec` provides this by laying out its elements
//! contiguously in memory, allowing the `shuffle` method to swap elements at random indices
//! without significant overhead.
//!
//! 2. **Conversion to Vec**: The `into_iter().collect()` pattern is used to convert the
//! `LinkedList` into a `Vec`. This step is necessary because the `LinkedList` does not support
//! direct indexing or efficient random access, which are required for the shuffling operation.
//!
//! 3. **Performing the Shuffle**: Once in `Vec` form, the `shuffle` method can efficiently
//! randomize the order of the elements. This operation leverages the `Vec`'s ability to quickly
//! access and modify elements at any position.
//!
//! 4. **Conversion back to LinkedList**: After shuffling, the collection is converted back into a
//! `LinkedList`. This step is taken because the subsequent operations or the overall program logic
//! may benefit from the `LinkedList`'s characteristics, such as efficient insertions and removals
//! at any point in the list, which might not be as efficient with a `Vec`.
//!
//! 5. **Why not VecDeque?**: While a `VecDeque` could also support efficient push and pop
//! operations at both ends (like a `LinkedList`), it does not offer a built-in `shuffle` method,
//! necessitating a similar conversion to `Vec` for shuffling. Thus, the choice between using a
//! `LinkedList` or a `VecDeque` might depend on other factors specific to the program's requirements.
//!
//! In essence, this conversion process leverages the strengths of both `Vec` (efficient random
//! access) and `LinkedList` (efficient insertions/removals at any point), demonstrating a
//! pragmatic approach to achieve a specific functionality (shuffling) that is not directly
//! supported by the `LinkedList`.


/*
As with the VecDeque example, this code starts by creating a LinkedList of fruits,
converts it to a Vec for shuffling, and then converts it back to a LinkedList.
After the shuffling, it adds "Pomegranate", "Fig", and "Cherry" to the end of the list.
Finally, it prints out the final fruit salad.

This example shows how to use a LinkedList, but remember that LinkedList
has a higher memory overhead and worse cache locality than Vec or VecDeque,
so it's typically not the best choice unless you have a specific need for the properties
of a linked list. In Rust, it's usually better to use a Vec or VecDeque.

A LinkedList is a doubly-linked list, which means that each element in the list
has a pointer to the next element and the previous element.
A great example of when to use a LinkedList is when you need to insert or remove elements
from the middle of the list.
*/

use rand::seq::SliceRandom; // rand is a random number generation library in Rust
use rand::thread_rng;
use std::collections::LinkedList;

fn main() {
    let mut fruit: LinkedList<&str> = LinkedList::new();
    fruit.push_back("Arbutus");
    fruit.push_back("Loquat");
    fruit.push_back("Strawberry Tree Berry");

    /*
    Please note that converting a LinkedList to a Vec and back to a LinkedList
    isn't a common operation in practice. I included
    it in this example to keep the code as similar as possible
    to the original VecDeque example.
     */

    // Scramble (shuffle) the fruit
    let mut rng = thread_rng();
    let mut fruit: Vec<_> = fruit.into_iter().collect();
    fruit.shuffle(&mut rng);

    // Convert it back to LinkedList
    let mut fruit: LinkedList<_> = fruit.into_iter().collect();

    // Add fruits to the both ends of the list after shuffling
    fruit.push_front("Pomegranate");
    fruit.push_back("Fig");
    fruit.push_back("Cherry");

    // Print out the fruit salad
    println!("Fruit Salad:");
    for (i, item) in fruit.iter().enumerate() {
        if i != fruit.len() - 1 {
            print!("{}, ", item);
        } else {
            println!("{}", item);
        }
    }
}
