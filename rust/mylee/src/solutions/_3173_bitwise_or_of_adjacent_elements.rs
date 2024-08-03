// # [3173. Bitwise OR of Adjacent Elements 🔒](https://leetcode.com/problems/bitwise-or-of-adjacent-elements)

// ## Description

// Given an array nums of length n,
// return an array answer of length n - 1 such that answer[i] = nums[i] | nums[i + 1] where | is the bitwise OR operation.

//
// Example 1:

// Input: nums = [1,3,7,15]

// Output: [3,7,15]

// Example 2:

// Input: nums = [8,4,2]

// Output: [12,6]

// Example 3:

// Input: nums = [5,4,9,11]

// Output: [5,13,11]

//
// Constraints:

// 	2 <= nums.length <= 100
// 	0 <= nums[i] <= 100

//     vector<int> or_array(vector<int>& nums) {

#[allow(dead_code)]
pub struct Solution {}

impl Solution {
    pub fn or_array(nums: Vec<i32>) -> Vec<i32> {
        nums.windows(2).map(|w| w[0] | w[1]).collect()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn test_or_array_1() {
        assert_eq!(vec![3, 7, 15], Solution::or_array(vec![1, 3, 7, 15]));
    }
    #[test]
    pub fn test_or_array_2() {
        assert_eq!(vec![12, 6], Solution::or_array(vec![8, 4, 2]));
    }
    #[test]
    pub fn test_or_array_3() {
        assert_eq!(vec![5, 13, 11], Solution::or_array(vec![5, 4, 9, 11]));
    }
}
