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
//         long long maximumAlternatingSubarraySum(vector<int>& nums) {

#[allow(dead_code)]
pub struct Solution {}
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn longest_word(words: Vec<String>) -> String {
        String::new()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn test_longest_word_1() {
        assert_eq!(
            "kiran".to_string(),
            Solution::longest_word(
                ["k", "ki", "kir", "kira", "kiran"]
                    .into_iter()
                    .map(String::from)
                    .collect::<Vec<String>>()
            )
        );
    }
    #[test]
    pub fn test_longest_word_2() {
        assert_eq!(
            "apple".to_string(),
            Solution::longest_word(
                ["a", "banana", "app", "appl", "ap", "apply", "apple"]
                    .into_iter()
                    .map(String::from)
                    .collect::<Vec<String>>()
            )
        );
    }
    #[test]
    pub fn test_longest_word_3() {
        assert_eq!(
            String::new(),
            Solution::longest_word(
                ["abc", "bc", "ab", "qwe"]
                    .into_iter()
                    .map(String::from)
                    .collect::<Vec<String>>(),
            )
        );
    }
}
