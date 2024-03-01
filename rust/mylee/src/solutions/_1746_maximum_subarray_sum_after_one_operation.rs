// 1746\. Maximum Subarray Sum After One Operation
// ===============================================

// You are given an integer array `nums`. You must perform **exactly one** operation where you can **replace** one element `nums[i]` with `nums[i] * nums[i]`.

// Return _the **maximum** possible subarray sum after **exactly one** operation_. The subarray must be non-empty.

// **Example 1:**

// **Input:** nums = \[2,-1,-4,-3\]
// **Output:** 17
// **Explanation:** You can perform the operation on index 2 (0-indexed) to make nums = \[2,-1,**16**,-3\]. Now, the maximum subarray sum is 2 + -1 + 16 = 17.

// **Example 2:**

// **Input:** nums = \[1,-1,1,1,-1,-1,1\]
// **Output:** 4
// **Explanation:** You can perform the operation on index 1 (0-indexed) to make nums = \[1,**1**,1,1,-1,-1,1\]. Now, the maximum subarray sum is 1 + 1 + 1 + 1 = 4.

// **Constraints:**

// *   `1 <= nums.length <= 105`
// *   `-104 <= nums[i] <= 104`

// ### Difficulty:

// Medium

// ### Lock:

// Prime

// ### Company:

// [Sprinklr](https://leetcode.ca/tags/#Sprinklr)

// int max_sum_after_operation(int[] nums)

#[allow(dead_code)]
pub struct Solution {}

impl Solution {
    pub fn max_sum_after_operation(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut dp = vec![vec![0; 2]; n];
        dp[0][0] = nums[0];
        dp[0][1] = nums[0] * nums[0];
        let mut ans = dp[0][1];
        for i in 1..n {
            dp[i][0] = nums[i].max(dp[i - 1][0] + nums[i]);
            dp[i][1] = [
                nums[i] * nums[i],
                nums[i] * nums[i] + dp[i - 1][0],
                dp[i - 1][1] + nums[i],
            ]
            .into_iter()
            .max()
            .unwrap();
            ans = ans.max(dp[i][1]);
        }
        ans
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn test_max_sum_after_operation_1() {
        assert_eq!(17, Solution::max_sum_after_operation(vec![2, -1, -4, -3],));
    }
    #[test]
    pub fn test_max_sum_after_operation_2() {
        assert_eq!(17, Solution::max_sum_after_operation(vec![2, -1, -4, -3],));
    }
}
