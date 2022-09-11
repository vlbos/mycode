// # [2393. Count Strictly Increasing Subarrays](https://leetcode.com/problems/count-strictly-increasing-subarrays)

// ## Description

// You are given an array nums consisting of positive integers.

// Return the number of subarrays of nums that are in strictly increasing order.

// A subarray is a contiguous part of an array.

// Example 1:

// Input: nums = [1,3,5,4,4,6]
// Output: 10
// Explanation: The strictly increasing subarrays are the following:
// - Subarrays of length 1: [1], [3], [5], [4], [4], [6].
// - Subarrays of length 2: [1,3], [3,5], [4,6].
// - Subarrays of length 3: [1,3,5].
// The total number of subarrays is 6 + 3 + 1 = 10.

// Example 2:

// Input: nums = [1,2,3,4,5]
// Output: 15
// Explanation: Every subarray is strictly increasing. There are 15 possible subarrays that we can take.

// Constraints:

// 	1 <= nums.length <= 10^5
// 	1 <= nums[i] <= 10^6

// ## Solutions

// <!-- tabs:start -->

// ### **Python3**

// ```python
// class Solution:
//     def count_subarrays(self, nums: List[int]) -> int:

#[allow(dead_code)]
pub struct Solution {}

impl Solution {
    pub fn count_subarrays(nums: Vec<i32>) -> i32 {
        let (mut count, mut pre) = (1, i32::MAX / 2);
        let mut ans = 0;

        for &num in &nums {
            if num > pre {
                count += 1;
            } else {
                count = 1;
            }
            pre = num;
            ans += count;
        }
        ans
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn test_count_subarrays_1() {
        assert_eq!(10, Solution::count_subarrays(vec![1, 3, 5, 4, 4, 6]));
    }
    #[test]
    pub fn test_count_subarrays_2() {
        assert_eq!(15, Solution::count_subarrays(vec![1, 2, 3, 4, 5]));
    }
}
