
// ## [3641\. Longest Semi-Repeating Subarray 🔒](https://leetcode.com/problems/longest-semi-repeating-subarray)

// [![](https://img.shields.io/badge/Difficulty-Medium-4051B5?style=flat-square)](https://img.shields.io/badge/Difficulty-Medium-4051B5?style=flat-square)

// ## Description

// You are given an integer array `nums` of length `n` and an integer `k`.

// A **semi‑repeating** subarray is a contiguous subarray in which at most `k` elements repeat (i.e., appear more than once).

// Return the length of the longest **semi‑repeating** subarray in `nums`.

// **Example 1:**

// **Input:** nums = \[1,2,3,1,2,3,4\], k = 2

// **Output:** 6

// **Explanation:**

// The longest semi-repeating subarray is `[2, 3, 1, 2, 3, 4]`, which has two repeating elements (2 and 3).

// **Example 2:**

// **Input:** nums = \[1,1,1,1,1\], k = 4

// **Output:** 5

// **Explanation:**

// The longest semi-repeating subarray is `[1, 1, 1, 1, 1]`, which has only one repeating element (1).

// **Example 3:**

// **Constraints:**

// //    int longest_subarray(vector<int>& nums, int k) {

#[allow(dead_code)]
pub struct Solution;

impl Solution {
    pub fn longest_subarray(nums: Vec<i32>, k: i32) -> i32 {
       0
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    pub fn test_longest_subarray_1() {
        assert_eq!(6, Solution::longest_subarray(vec![1,2,3,1,2,3,4], 2));
    }
    #[test]
    pub fn test_longest_subarray_2() {
        assert_eq!(5, Solution::longest_subarray(vec![1,1,1,1,1], 4));
    }
}
