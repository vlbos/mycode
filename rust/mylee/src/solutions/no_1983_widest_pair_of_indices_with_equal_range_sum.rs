// 1983\. Widest Pair of Indices With Equal Range Sum[](https://leetcode.ca/2021-08-28-1983-Widest-Pair-of-Indices-With-Equal-Range-Sum/#1983-widest-pair-of-indices-with-equal-range-sum)
// =======================================================================================================================================================================================

// Level[](https://leetcode.ca/2021-08-28-1983-Widest-Pair-of-Indices-With-Equal-Range-Sum/#level)
// -----------------------------------------------------------------------------------------------

// Medium

// Description[](https://leetcode.ca/2021-08-28-1983-Widest-Pair-of-Indices-With-Equal-Range-Sum/#description)
// -----------------------------------------------------------------------------------------------------------

// You are given two **0-indexed** binary arrays `nums1` and `nums2`. Find the **widest** pair of indices `(i, j)` such that `i <= j` and `nums1[i] + nums1[i+1] + ... + nums1[j] == nums2[i] + nums2[i+1] + ... + nums2[j]`.

// The **widest** pair of indices is the pair with the **largest distance** between `i` and `j`. The **distance** between a pair of indices is defined as `j - i + 1`.

// Return _the **distance** of the **widest** pair of indices. If no pair of indices meets the conditions, return `0`_.

// **Example 1:**

// **Input:** nums1 = \[1,1,0,1\], nums2 = \[0,1,1,0\]

// **Output:** 3

// **Explanation:**

// If i = 1 and j = 3:

// nums1\[1\] + nums1\[2\] + nums1\[3\] = 1 + 0 + 1 = 2.

// nums2\[1\] + nums2\[2\] + nums2\[3\] = 1 + 1 + 0 = 2.

// The distance between i and j is j - i + 1 = 3 - 1 + 1 = 3.

// **Example 2:**

// **Input:** nums1 = \[0,1\], nums2 = \[1,1\]

// **Output:** 1

// **Explanation:**

// If i = 1 and j = 1:

// nums1\[1\] = 1.

// nums2\[1\] = 1.

// The distance between i and j is j - i + 1 = 1 - 1 + 1 = 1.

// **Example 3:**

// **Input:** nums1 = \[0\], nums2 = \[1\]

// **Output:** 0

// **Explanation:**

// There are no pairs of indices that meet the requirements.

// **Constraints:**

// *   `n == nums1.length == nums2.length`
// *   `1 <= n <= 10^5`
// *   `nums1[i]` is either `0` or `1`.
// *   `nums2[i]` is either `0` or `1`.

// Solution[](https://leetcode.ca/2021-08-28-1983-Widest-Pair-of-Indices-With-Equal-Range-Sum/#solution)
// -----------------------------------------------------------------------------------------------------

// Let `differences[i] = nums1[i] - nums2[j]`. Then find the widest pair of indices `(i, j)` such that `i <= j` and `differences[i] + differences[i + 1] + ... + differences[j] == 0`. Calculate the prefix sum array of `differences` and use hash map to store each prefix sum and the smallest index with the prefix sum. For each index `j`, if there already exists an index `i` such that the prefix sums at indices `i` and `j` are the same, then the width is calculated as `j - i`. Find the maximum width and return.

//     class Solution {
//         public int widestPairOfIndices(int[] nums1, int[] nums2) {

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
