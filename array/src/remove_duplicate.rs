//! Given an integer array nums sorted in non-decreasing order, 
//! remove the duplicates in-place such that each unique element 
//! appears only once. The relative order of the elements should 
//! be kept the same.
//!
//! Return k after placing the final result in the first k slots of nums.
//!
//! Do not allocate extra space for another array. 
//! You must do this by modifying the input array in-place with O(1) extra memory.

/// Remove duplicates from vector
///
/// Note: uses O(1) memory
fn remove_duplicates(mut nums: Vec<i32>) -> i32 {
    let mut prev = 0;
    for idx in 1..nums.len() {
        if nums[prev] != nums[idx] {
            prev += 1;
            nums[prev] = nums[idx];
        }
    }

    (prev + 1) as i32
}

fn main() {
    let mut nums = vec![10, 2, 3, 2, 5];
    nums.sort();

    let num_unique = remove_duplicates(nums);
    println!("{}", num_unique);
}
