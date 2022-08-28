// [2229\. Check if an Array Is Consecutive (Easy)](https://leetcode.com/problems/check-if-an-array-is-consecutive/)[](https://leetcode.ca/2022-05-01-2229-Check-if-an-Array-Is-Consecutive/#2229-check-if-an-array-is-consecutive-easy)
// =====================================================================================================================================================================================================================================

// Given an integer array `nums`, return `true` _if_ `nums` _is **consecutive**, otherwise return_ `false`_._

// An array is **consecutive** if it contains every number in the range `[x, x + n - 1]` (**inclusive**), where `x` is the minimum number in the array and `n` is the length of the array.

// **Example 1:**

// **Input:** nums = \[1,3,4,2\]
// **Output:** true
// **Explanation:**
// The minimum value is 1 and the length of nums is 4.
// All of the values in the range \[x, x + n - 1\] = \[1, 1 + 4 - 1\] = \[1, 4\] = (1, 2, 3, 4) occur in nums.
// Therefore, nums is consecutive.

// **Example 2:**

// **Input:** nums = \[1,3\]
// **Output:** false
// **Explanation:**
// The minimum value is 1 and the length of nums is 2.
// The value 2 in the range \[x, x + n - 1\] = \[1, 1 + 2 - 1\], = \[1, 2\] = (1, 2) does not occur in nums.
// Therefore, nums is not consecutive.

// **Example 3:**

// **Input:** nums = \[3,5,4\]
// **Output:** true
// **Explanation:**
// The minimum value is 3 and the length of nums is 3.
// All of the values in the range \[x, x + n - 1\] = \[3, 3 + 3 - 1\] = \[3, 5\] = (3, 4, 5) occur in nums.
// Therefore, nums is consecutive.

// **Constraints:**

// *   `1 <= nums.length <= 105`
// *   `0 <= nums[i] <= 105`

// **Companies**:
// [Turbot](https://leetcode.com/company/turbot)

// **Related Topics**:
// [Array](https://leetcode.com/tag/array/)

// **Similar Questions**:

// *   [Binary Tree Longest Consecutive Sequence (Medium)](https://leetcode.com/problems/binary-tree-longest-consecutive-sequence/)
// *   [Binary Tree Longest Consecutive Sequence II (Medium)](https://leetcode.com/problems/binary-tree-longest-consecutive-sequence-ii/)
// *   [Consecutive Characters (Easy)](https://leetcode.com/problems/consecutive-characters/)

// Solution 1. Sorting[](https://leetcode.ca/2022-05-01-2229-Check-if-an-Array-Is-Consecutive/#solution-1-sorting)
// ---------------------------------------------------------------------------------------------------------------

//     // OJ: https://leetcode.com/problems/check-if-an-array-is-consecutive/
//     // Time: O(N)
//     // Space: O(1)
//     class Solution {
//     public:
//         bool isConsecutive(vector<int>& A) {

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
