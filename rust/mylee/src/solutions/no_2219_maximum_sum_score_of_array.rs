// # [2219. Maximum Sum Score of Array](https://leetcode.com/problems/maximum-sum-score-of-array)

// ## Description

// You are given a 0-indexed integer array nums of length n.

// The sum score of nums at an index i where 0 <= i < n is the maximum of:

//
// 	The sum of the first i + 1 elements of nums.
// 	The sum of the last n - i elements of nums.
//

// Return the maximum sum score of nums at any index.

// Example 1:

//
// Input: nums = [4,3,-2,5]
// Output: 10
// Explanation:
// The sum score at index 0 is max(4, 4 + 3 + -2 + 5) = max(4, 10) = 10.
// The sum score at index 1 is max(4 + 3, 3 + -2 + 5) = max(7, 6) = 7.
// The sum score at index 2 is max(4 + 3 + -2, -2 + 5) = max(5, 3) = 5.
// The sum score at index 3 is max(4 + 3 + -2 + 5, 5) = max(10, 5) = 10.
// The maximum sum score of nums is 10.
//

// Example 2:

//
// Input: nums = [-3,-5]
// Output: -3
// Explanation:
// The sum score at index 0 is max(-3, -3 + -5) = max(-3, -8) = -3.
// The sum score at index 1 is max(-3 + -5, -5) = max(-8, -5) = -5.
// The maximum sum score of nums is -3.
//

// Constraints:

//
// 	n == nums.length
// 	1 <= n <= 10^5
// 	-10^5 <= nums[i] <= 10^5
//

// ## Solutions

// <!-- tabs:start -->

// ### **Python3**

// ```python
// class Solution:
//     def maximum_sum_score(self, nums: List[int]) -> int:

#[allow(dead_code)]
pub struct Solution {}

impl Solution {
    pub fn maximum_sum_score(nums: Vec<i32>) -> i64 {
        let sum = nums.iter().map(|&x| x as i64).sum::<i64>();
        let mut pre = 0;
        let mut ans = i64::MIN;
        for &num in &nums {
            let suf = sum - pre;
            pre += num as i64;
            ans = ans.max(pre.max(suf));
        }
        ans
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn test_maximum_sum_score_1() {
        assert_eq!(10, Solution::maximum_sum_score(vec![4, 3, -2, 5]));
    }
    #[test]
    pub fn test_maximum_sum_score_2() {
        assert_eq!(-3, Solution::maximum_sum_score(vec![-3, -5]));
    }
}
