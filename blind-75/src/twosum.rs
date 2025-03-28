/*
Given an array of integers nums and an integer target, return indices of the two numbers such that they add up to target.
You may assume that each input would have exactly one solution, and you may not use the same element twice.
You can return the answer in any order.
--------------------------------------------------------------
Example 1:
Input: nums = [2,7,11,15], target = 9
Output: [0,1]
Explanation: Because nums[0] + nums[1] == 9, we return [0, 1].

Example 2:
Input: nums = [3,2,4], target = 6
Output: [1,2]

Example 3:
Input: nums = [3,3], target = 6
Output: [0,1]
--------------------------------------------------------------
Constraints:
    2 <= nums.length <= 104
    -109 <= nums[i] <= 109
    -109 <= target <= 109
    Only one valid answer exists.
*/

// naive, On^2

pub fn two_sum(arr: Vec<i32>, tar: i32) -> Vec<i32> {
    for i in 0..arr.len() {
        for j in (i + 1)..arr.len() {
            if arr[i] + arr[j] == tar {
                return vec![i as i32, j as i32];
            }
        }
    }
    vec![]
}

// hashmap

use std::collections::HashMap;

pub fn two_sum_hash(arr: Vec<i32>, tar: i32) -> Vec<i32> {
    let mut seen = HashMap::new();
    for (i, num) in arr.iter().enumerate() {
        let complement = tar - num;
        if let Some(&j) = seen.get(&complement) {
            return vec![j as i32, i as i32];
        }
        seen.insert(num, i);
    }
    vec![]
}
