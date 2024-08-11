// # [3231. Minimum Number of Increasing Subsequence to Be Removed 🔒](https://leetcode.com/problems/minimum-number-of-increasing-subsequence-to-be-removed)

// ## Description

// Given an array of integers nums, you are allowed to perform the following operation any number of times:

//
// 	Remove a strictly increasing subsequence from the array.
//

// Your task is to find the minimum number of operations required to make the array empty.

//
// Example 1:

//
// Input: nums = [5,3,1,4,2]

// Output: 3

// Explanation:

// We remove subsequences [1, 2], [3, 4], [5].
//

// Example 2:

//
// Input: nums = [1,2,3,4,5]

// Output: 1
//

// Example 3:

//
// Input: nums = [5,4,3,2,1]

// Output: 5
//

//
// Constraints:

//
// 	1  <= nums.length  <= 105
// 	1  <= nums[i]  <= 105
//

// impl Solution {
//     pub fn min_operations(nums: Vec<i32>) -> i32 {

#[allow(dead_code)]
pub struct Solution;

impl Solution {
    pub fn min_operations(nums: Vec<i32>) -> i32 {
        0
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    pub fn test_min_operations_1() {
        assert_eq!(1, Solution::min_operations(vec![1, 2, 3, 4, 5]));
    }
    #[test]
    pub fn test_min_operations_2() {
        assert_eq!(5, Solution::min_operations(vec![5, 4, 3, 2, 1]));
    }
}
