// Leetcode Two Sum: 

// Given an array of integers nums and an integer target, return indices of the two numbers such that they add up to target.
// You may assume that each input would have exactly one solution, and you may not use the same element twice.
// You can return the answer in any order. 

use std::collections::HashMap;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        // Declares a mutable variable named hashmap
        let mut hashmap = HashMap::new();
        // In Rust, by default, when you assign a value to a variable, it takes ownership of that value
        // Only one owner is responsible for managing the memory of that value at a time
        // By using `&num`, we create references to the values in the nums vector, allowing us to work with the values indirectly
        // enumerate() is used to get both the index `i` and the value `num` of each element in the iteration
        for (i, &num) in nums.iter().enumerate() {
            let complement = target - num;
            // The `if let` construct is a shorthand for pattern matching in Rust
            // `Some` is an enum variant that is used to indicate the presence of a value
            if let Some(&index) = hashmap.get(&complement){
                // Return a new vector containing the indices of the two numbers that add up to the target
                return vec![index as i32, i as i32];
            }
            hashmap.insert(num, i);
        }
        // Return an empty vector if no pairs add up to target
        vec![]
    }
}