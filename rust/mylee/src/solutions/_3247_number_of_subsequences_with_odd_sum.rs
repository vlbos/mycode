// # [3247. Number of Subsequences with Odd Sum 🔒](https://leetcode.com/problems/number-of-subsequences-with-odd-sum)

// ## Description

// Given an array nums, return the number of subsequences with an odd sum of elements.

// Since the answer may be very large, return it modulo 109 + 7.

//
// Example 1:

//
// Input: nums = [1,1,1]

// Output: 4

// Explanation:

// The odd-sum subsequences are: [1, 1, 1], [1, 1, 1], [1, 1, 1], [1, 1, 1].
//

// Example 2:

//
// Input: nums = [1,2,2]

// Output: 4

// Explanation:

// The odd-sum subsequences are: [1, 2, 2], [1, 2, 2], [1, 2, 2], [1, 2, 2].
//

//
// Constraints:

//
// 	1  <= nums.lnegth  <= 105
// 	1  <= nums[i]  <= 109
//

//     int subsequence_count(vector<int>& nums) {
#[allow(dead_code)]
pub struct Solution;
impl Solution {
    pub fn subsequence_count(nums: Vec<i32>) -> i32 {
        0
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    pub fn test_subsequence_count_1() {
        assert_eq!(1, Solution::subsequence_count(vec![1, 1, 1]));
    }
    #[test]
    pub fn test_subsequence_count_2() {
        assert_eq!(4, Solution::subsequence_count(vec![1, 2, 2]));
    }
}
