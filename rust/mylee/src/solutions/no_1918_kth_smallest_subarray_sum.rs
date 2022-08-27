// 1918\. Kth Smallest Subarray Sum[](https://leetcode.ca/2021-08-01-1918-Kth-Smallest-Subarray-Sum/#1918-kth-smallest-subarray-sum)
// =================================================================================================================================

// Level[](https://leetcode.ca/2021-08-01-1918-Kth-Smallest-Subarray-Sum/#level)
// -----------------------------------------------------------------------------

// Medium

// Description[](https://leetcode.ca/2021-08-01-1918-Kth-Smallest-Subarray-Sum/#description)
// -----------------------------------------------------------------------------------------

// Given an integer array `nums` of length `n` and an integer `k`, return _the `k-th` **smallest subarray sum**_.

// A **subarray** is defined as a **non-empty** contiguous sequence of elements in an array. A **subarray sum** is the sum of all elements in the subarray.

// **Example 1:**

// **Input:** nums = \[2,1,3\], k = 4

// **Output:** 3

// **Explanation:** The subarrays of \[2,1,3\] are:

// *   \[2\] with sum 2
// *   \[1\] with sum 1
// *   \[3\] with sum 3
// *   \[2,1\] with sum 3
// *   \[1,3\] with sum 4
// *   \[2,1,3\] with sum 6

// Ordering the sums from smallest to largest gives 1, 2, 3, 3, 4, 6. The 4th smallest is 3.

// **Example 2:**

// **Input:** nums = \[3,3,5,5\], k = 7

// **Output:** 10

// **Explanation:** The subarrays of \[3,3,5,5\] are:

// *   \[3\] with sum 3
// *   \[3\] with sum 3
// *   \[5\] with sum 5
// *   \[5\] with sum 5
// *   \[3,3\] with sum 6
// *   \[3,5\] with sum 8
// *   \[5,5\] with sum 10
// *   \[3,3,5\], with sum 11
// *   \[3,5,5\] with sum 13
// *   \[3,3,5,5\] with sum 16

// Ordering the sums from smallest to largest gives 3, 3, 5, 5, 6, 8, 10, 11, 13, 16. The 7th smallest is 10.

// **Constraints:**

// *   `n == nums.length`
// *   `1 <= n <= 2 * 10^4`
// *   `1 <= nums[i] <= 5 * 10^4`
// *   `1 <= k <= n * (n + 1) / 2`

// Solution[](https://leetcode.ca/2021-08-01-1918-Kth-Smallest-Subarray-Sum/#solution)
// -----------------------------------------------------------------------------------

// Use binary search. The maximum subarray sum is the sum of all elements in `nums` and the minimum subarray sum is the minimum element in `nums`. Initialize `high` and `low` as the maximum subarray sum and the minimum subarray sum. Each time let `mid` be the mean of `high` and `low` and count the number of subarrays that have sum less than or equal to `mid`, and adjust `high` and `low` accordingly. Finally the `k`\-th smallest subarray sum can be obtained.

// To count the number of subarrays that have sum less than or equal to `mid`, use sliding window over `nums` and for each index, count the number of subarrays that end at the index with sum less than or equal to `mid`.

//     class Solution {
//         public int kthSmallestSubarraySum(int[] nums, int k) {

#[allow(dead_code)]
pub struct Solution {}
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn longest_word(
        words:Vec<String>,
    ) -> String {
       String::new()
    }
}

#[cfg(test)]
mod test {
    use super::*;
   
    #[test]
    pub fn test_longest_word_1() {
        assert_eq!("kiran".to_string(),Solution::longest_word(
            ["k","ki","kir","kira", "kiran"].into_iter().map(String::from).collect::<Vec<String>>()
        ));
    }
    #[test]
    pub fn test_longest_word_2() {
          assert_eq!("apple".to_string(),Solution::longest_word(
           ["a", "banana", "app", "appl", "ap", "apply", "apple"].into_iter().map(String::from).collect::<Vec<String>>()
        ));
    }
    #[test]
    pub fn test_longest_word_3() {
          assert_eq!(String::new(),Solution::longest_word(
            ["abc", "bc", "ab", "qwe"].into_iter().map(String::from).collect::<Vec<String>>(),
        ));
    }
}
