mod lib;

use lib::*;

fn main() {
    println!("Meet Safe and Unsafe (Lab)");

    let sum = safe_add(10, 20);
    println!("safe_add(10, 20) = {sum}");

    let val = 42;
    let ptr: *const i32 = &val;
    let deref = unsafe { unsafe_dereference(ptr) };
    println!("unsafe_dereference(&42) = {deref}");

    let mut target = 0;
    let mut_ptr: *mut i32 = &mut target;
    unsafe { unsafe_write(mut_ptr, 77) };
    println!("unsafe_write -> target = {target}");

    let mut nums = [1, 2, 3, 4, 5];
    let (left, right) = safe_split_sum(&mut nums);
    println!("safe_split_sum([1,2,3,4,5]) = ({left}, {right})");

    let idx = safe_index(&nums, 2);
    println!("safe_index([..], 2) = {idx:?}");

    let concepts = safety_concepts();
    println!("safety concepts: {}", concepts.join(", "));
}
