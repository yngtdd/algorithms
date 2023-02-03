use std::collections::HashSet;

fn contains_duplicate(nums: Vec<i32>) -> bool {
    nums.iter().collect::<HashSet<_>>().len() < nums.len()
}

fn main() {
    let nums = vec![0, 1, 2, 10, 20, 3, 2];
    let any_duplicates = contains_duplicate(nums);
    println!("Contains duplicates? {}", any_duplicates);
}
