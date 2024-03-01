// [2036\. Maximum Alternating Subarray Sum](https://leetcode.com/problems/maximum-alternating-subarray-sum/)[](https://leetcode.ca/2021-10-20-2036-Maximum-Alternating-Subarray-Sum/#2036-maximum-alternating-subarray-sum)
// =========================================================================================================================================================================================================================

// Description:[](https://leetcode.ca/2021-10-20-2036-Maximum-Alternating-Subarray-Sum/#description)
// -------------------------------------------------------------------------------------------------

// A **subarray** of a **0-indexed** integer array is a contiguous **non-empty** sequence of elements within an array.

// The **alternating subarray sum** of a subarray that ranges from index `i` to `j` (**inclusive**, `0 <= i <= j < nums.length`) is `nums[i] - nums[i+1] + nums[i+2] - ... +/- nums[j]`.

// Given a **0-indexed** integer array nums, return _the **maximum alternating subarray sum** of any subarray of `nums`._

// Examples:[](https://leetcode.ca/2021-10-20-2036-Maximum-Alternating-Subarray-Sum/#examples)
// -------------------------------------------------------------------------------------------

// **Example 1:**

// **Input:** nums = \[3,-1,1,2\]
// **Output:** 5
// **Explanation:**
// The subarray \[3,-1,1\] has the largest alternating subarray sum.
// The alternating subarray sum is 3 - (-1) + 1 = 5.

// **Example 2:**

// **Input:** nums = \[2,2,2,2,2\]
// **Output:** 2
// **Explanation:**
// The subarrays \[2\], \[2,2,2\], and \[2,2,2,2,2\] have the largest alternating subarray sum.
// The alternating subarray sum of \[2\] is 2.
// The alternating subarray sum of \[2,2,2\] is 2 - 2 + 2 = 2.
// The alternating subarray sum of \[2,2,2,2,2\] is 2 - 2 + 2 - 2 + 2 = 2.

// **Example 3:**

// **Input:** nums = \[1\]
// **Output:** 1
// **Explanation:**
// There is only one non-empty subarray, which is \[1\].
// The alternating subarray sum is 1.

// Constraints:[](https://leetcode.ca/2021-10-20-2036-Maximum-Alternating-Subarray-Sum/#constraints)
// -------------------------------------------------------------------------------------------------

// *   `1 <= nums.length <= 105`
// *   `-105 <= nums[i] <= 105`

// Solution:[](https://leetcode.ca/2021-10-20-2036-Maximum-Alternating-Subarray-Sum/#solution)
// -------------------------------------------------------------------------------------------

// **Logical Thinking**

// We can divide this problem into two **dynamic programming** sub-problems: Firstly, if we negate the elements with odd indices, then the problem becomes to find the maximum subarray sum from `0` to `n`; Secondly, if we negate the elements with even indices, the problem becomes to find the maximum subarray sum from `1` to `n`. For each sub-problem, we can use **Kadane's Algorithm** to solve it, and the answer is the maximum of two sub-optimum.

// **C++**

//     //  Topic   ：2036. Maximum Alternating Subarray Sum (https://leetcode.com/problems/maximum-alternating-subarray-sum/)
//     //  Time    : O(N)
//     //  Space   : O(N)

//     class Solution {
//     public:
//         long long maximum_alternating_subarray_sum(vector<int>& nums) {

#[allow(dead_code)]
pub struct Solution {}
impl Solution {
    pub fn maximum_alternating_subarray_sum(nums: Vec<i32>) -> i64 {
        let (mut ans, mut sum1, mut sum2) = (nums[0] as i64, nums[0] as i64, 0);
        for (i, &v) in nums[1..].iter().enumerate() {
            let num = v as i64;
            if i % 2 > 0 {
                if sum1 + num > num {
                    sum1 += num;
                } else {
                    sum1 = num;
                }
                sum2 -= num;
            } else {
                sum1 -= num;
                if sum2 + num > num {
                    sum2 += num;
                } else {
                    sum2 = num;
                }
            }
            ans = ans.max(sum1);
            ans = ans.max(sum2);
        }
        ans
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn test_maximum_alternating_subarray_sum_1() {
        assert_eq!(
            5,
            Solution::maximum_alternating_subarray_sum(vec![3, -1, 1, 2])
        );
    }
    #[test]
    pub fn test_maximum_alternating_subarray_sum_2() {
        assert_eq!(
            2,
            Solution::maximum_alternating_subarray_sum(vec![2, 2, 2, 2, 2])
        );
    }
    #[test]
    pub fn test_maximum_alternating_subarray_sum_3() {
        assert_eq!(1, Solution::maximum_alternating_subarray_sum(vec![1]));
    }
}
