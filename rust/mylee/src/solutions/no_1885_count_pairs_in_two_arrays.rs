// 1885\. Count Pairs in Two Arrays[](https://leetcode.ca/2021-07-23-1885-Count-Pairs-in-Two-Arrays/#1885-count-pairs-in-two-arrays)
// =================================================================================================================================

// Level[](https://leetcode.ca/2021-07-23-1885-Count-Pairs-in-Two-Arrays/#level)
// -----------------------------------------------------------------------------

// Medium

// Description[](https://leetcode.ca/2021-07-23-1885-Count-Pairs-in-Two-Arrays/#description)
// -----------------------------------------------------------------------------------------

// Given two integer arrays `nums1` and `nums2` of length `n`, count the pairs of indices `(i, j)` such that `i < j` and `nums1[i] + nums1[j] > nums2[i] + nums2[j]`.

// Return _the **number of pairs** satisfying the condition_.

// **Example 1:**

// **Input:** nums1 = \[2,1,2,1\], nums2 = \[1,2,1,2\]

// **Output:** 1

// **Explanation:** The pairs satisfying the condition are:

// *   (0, 2) where 2 + 2 > 1 + 1.

// **Example 2:**

// **Input:** nums1 = \[1,10,6,2\], nums2 = \[1,4,1,5\]

// **Output:** 5

// **Explanation:** The pairs satisfying the condition are:

// *   (0, 1) where 1 + 10 > 1 + 4.
// *   (0, 2) where 1 + 6 > 1 + 1.
// *   (1, 2) where 10 + 6 > 4 + 1.
// *   (1, 3) where 10 + 2 > 4 + 5.
// *   (2, 3) where 6 + 2 > 1 + 5.

// **Constraints:**

// *   `n == nums1.length == nums2.length`
// *   `1 <= n <= 10^5`
// *   `1 <= nums1[i], nums2[i] <= 10^5`

// Solution[](https://leetcode.ca/2021-07-23-1885-Count-Pairs-in-Two-Arrays/#solution)
// -----------------------------------------------------------------------------------

// Create an array `differences` with length `n` such that `differences[i] = nums1[i] - nums2[i]` for all `0 <= i < n`. Sort `differences`. For each `0 <= i < n - 1`, find the minimum `index` such that `differences[index] + differences[i] > 0`, or `index = n` if `differences[n - 1] + differences[i] <= 0`, and the number of `j`’s for the current `i` is `n - index`. In this way, the number of pairs of indices `(i, j)` can be calculated.

//     class Solution {
//         public long countPairs(int[] nums1, int[] nums2) {

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
