//! Given an integer array nums, return true if any value appears 
//! at least twice in the array, and return false if every element is distinct.
use std::collections::HashSet;

/// Check if an array contains duplicates
///
/// We can create a set of elements and compare whether
/// that set has fewer elements than the original array
///
/// # Note
///
/// Uses O(N) memory
fn contains_duplicate(nums: Vec<i32>) -> bool {
    nums.iter().collect::<HashSet<_>>().len() < nums.len()
}

fn main() {
    let nums = vec![0, 1, 2, 10, 20, 3, 2];
    let any_duplicates = contains_duplicate(nums);
    println!("Contains duplicates? {}", any_duplicates);
}
