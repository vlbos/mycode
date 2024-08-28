// # [2519. Count the Number of K-Big Indices 🔒](https://leetcode.com/problems/count-the-number-of-k-big-indices)

// ## Description

// You are given a 0-indexed integer array nums and a positive integer k.

// We call an index i k-big if the following conditions are satisfied:

// 	There exist at least k different indices idx1 such that idx1  < i and nums[idx1]  < nums[i].
// 	There exist at least k different indices idx2 such that idx2 > i and nums[idx2]  < nums[i].

// Return the number of k-big indices.

// Example 1:

// Input: nums = [2,3,6,5,2,3], k = 2
// Output: 2
// Explanation: There are only two 2-big indices in nums:
// - i = 2 --> There are two valid idx1: 0 and 1. There are three valid idx2: 2, 3, and 4.
// - i = 3 --> There are two valid idx1: 0 and 1. There are two valid idx2: 3 and 4.

// Example 2:

// Input: nums = [1,1,1], k = 3
// Output: 0
// Explanation: There are no 3-big indices in nums.

// Constraints:

// 	1  <= nums.length  <= 105
// 	1  <= nums[i], k  <= nums.length

//  pub fnk_big_indices(nums: Vec<i32>, k: i32) -> i32 {

#[allow(dead_code)]
pub struct Solution;

impl Solution {
    pub fn k_big_indices(nums: Vec<i32>, k: i32) -> i32 {
        let (n, k) = (nums.len(), k as usize);
        let mut prefix = vec![false; n];
        let mut bh = std::collections::BinaryHeap::new();
        for i in 0..n {
            if bh.len() == k && *bh.peek().unwrap() < nums[i] {
                prefix[i] = true;
            }
            bh.push(nums[i]);
            if bh.len() > k {
                bh.pop();
            }
        }

        let mut ans = 0;
        bh.clear();
        for i in (0..n).rev() {
            if bh.len() == k && *bh.peek().unwrap() < nums[i] && prefix[i] {
                ans += 1;
            }
            bh.push(nums[i]);
            if bh.len() > k {
                bh.pop();
            }
        }
        ans
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    pub fn test_k_big_indices_1() {
        assert_eq!(2, Solution::k_big_indices(vec![2, 3, 6, 5, 2, 3], 2));
    }
    #[test]
    pub fn test_k_big_indices_2() {
        assert_eq!(0, Solution::k_big_indices(vec![1, 1, 1], 3));
    }
}
