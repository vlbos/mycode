// 1891\. Cutting Ribbons[](https://leetcode.ca/2021-07-24-1891-Cutting-Ribbons/#1891-cutting-ribbons)
// ===================================================================================================

// Level[](https://leetcode.ca/2021-07-24-1891-Cutting-Ribbons/#level)
// -------------------------------------------------------------------

// Medium

// Description[](https://leetcode.ca/2021-07-24-1891-Cutting-Ribbons/#description)
// -------------------------------------------------------------------------------

// You are given an integer array `ribbons`, where `ribbons[i]` represents the length of the `i-th` ribbon, and an integer `k`. You may cut any of the ribbons into any number of segments of **positive integer** lengths, or perform no cuts at all.

// *   For example, if you have a ribbon of length `4`, you can:
//     *   Keep the ribbon of length `4`,
//     *   Cut it into one ribbon of length `3` and one ribbon of length `1`,
//     *   Cut it into two ribbons of length `2`,
//     *   Cut it into one ribbon of length `2` and two ribbons of length `1`, or
//     *   Cut it into four ribbons of length `1`.

// Your goal is to obtain `k` ribbons of all the **same positive integer length**. You are allowed to throw away any excess ribbon as a result of cutting.

// Return _the **maximum** possible positive integer length that you can obtain `k` ribbons of, or `0` if you cannot obtain `k` ribbons of the same length_.

// **Example 1:**

// **Input:** ribbons = \[9,7,5\], k = 3

// **Output:** 5

// **Explanation:**

// *   Cut the first ribbon to two ribbons, one of length 5 and one of length 4.
// *   Cut the second ribbon to two ribbons, one of length 5 and one of length 2.
// *   Keep the third ribbon as it is.

// Now you have 3 ribbons of length 5.

// **Example 2:**

// **Input:** ribbons = \[7,5,9\], k = 4

// **Output:** 4

// **Explanation:**

// *   Cut the first ribbon to two ribbons, one of length 4 and one of length 3.
// *   Cut the second ribbon to two ribbons, one of length 4 and one of length 1.
// *   Cut the third ribbon to three ribbons, two of length 4 and one of length 1.

// Now you have 4 ribbons of length 4.

// **Example 3:**

// **Input:** ribbons = \[5,7,9\], k = 22

// **Output:** 0

// **Explanation:** You cannot obtain k ribbons of the same positive integer length.

// **Constraints:**

// *   `1 <= ribbons.length <= 10^5`
// *   `1 <= ribbons[i] <= 10^5`
// *   `1 <= k <= 10^9`

// Solution[](https://leetcode.ca/2021-07-24-1891-Cutting-Ribbons/#solution)
// -------------------------------------------------------------------------

// Use binary search. Initialize `low` as `0` and `high` as the maximum in `ribbons`. Each time let `mid = (high - low + 1) / 2 + low` and calculate the number of ribbons that can be obtained with length `mid`. If the number of ribbons is greater than or equal to `k`, then the length is at least `mid`, so let `low = mid`. Otherwise, the length is at most `mid - 1`, so let `high = mid - 1`. Finally, return `low` as the maximum possible length.

//     class Solution {
//         public int maxLength(int[] ribbons, int k) {



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
