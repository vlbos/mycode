// [2021\. Brightest Position on Street (Medium)](https://leetcode.com/problems/brightest-position-on-street/)[](https://leetcode.ca/2021-10-05-2021-Brightest-Position-on-Street/#2021-brightest-position-on-street-medium)
// =========================================================================================================================================================================================================================

// A perfectly straight street is represented by a number line. The street has street lamp(s) on it and is represented by a 2D integer array `lights`. Each `lights[i] = [positioni, rangei]` indicates that there is a street lamp at position `positioni` that lights up the area from `[positioni - rangei, positioni + rangei]` (**inclusive**).

// The **brightness** of a position `p` is defined as the number of street lamp that light up the position `p`.

// Given `lights`, return _the **brightest** position on the_ _street. If there are multiple brightest positions, return the **smallest** one._

// **Example 1:**

// ![](https://assets.leetcode.com/uploads/2021/09/28/image-20210928155140-1.png)

// **Input:** lights = \[\[-3,2\],\[1,2\],\[3,3\]\]
// **Output:** -1
// **Explanation:**
// The first street lamp lights up the area from \[(-3) - 2, (-3) + 2\] = \[-5, -1\].
// The second street lamp lights up the area from \[1 - 2, 1 + 2\] = \[-1, 3\].
// The third street lamp lights up the area from \[3 - 3, 3 + 3\] = \[0, 6\].

// Position -1 has a brightness of 2, illuminated by the first and second street light.
// Positions 0, 1, 2, and 3 have a brightness of 2, illuminated by the second and third street light.
// Out of all these positions, -1 is the smallest, so return it.

// **Example 2:**

// **Input:** lights = \[\[1,0\],\[0,1\]\]
// **Output:** 1
// **Explanation:**
// The first street lamp lights up the area from \[1 - 0, 1 + 0\] = \[1, 1\].
// The second street lamp lights up the area from \[0 - 1, 0 + 1\] = \[-1, 1\].

// Position 1 has a brightness of 2, illuminated by the first and second street light.
// Return 1 because it is the brightest position on the street.

// **Example 3:**

// **Input:** lights = \[\[1,2\]\]
// **Output:** -1
// **Explanation:**
// The first street lamp lights up the area from \[1 - 2, 1 + 2\] = \[-1, 3\].

// Positions -1, 0, 1, 2, and 3 have a brightness of 1, illuminated by the first street light.
// Out of all these positions, -1 is the smallest, so return it.

// **Constraints:**

// *   `1 <= lights.length <= 105`
// *   `lights[i].length == 2`
// *   `-108 <= positioni <= 108`
// *   `0 <= rangei <= 108`

// **Companies**:
// [Amazon](https://leetcode.com/company/amazon)

// **Related Topics**:
// [Array](https://leetcode.com/tag/array/), [Prefix Sum](https://leetcode.com/tag/prefix-sum/), [Ordered Set](https://leetcode.com/tag/ordered-set/)

// **Similar Questions**:

// *   [Minimum Number of Buckets Required to Collect Rainwater from Houses (Medium)](https://leetcode.com/problems/minimum-number-of-buckets-required-to-collect-rainwater-from-houses/)

// Solution 1. Heap[](https://leetcode.ca/2021-10-05-2021-Brightest-Position-on-Street/#solution-1-heap)
// -----------------------------------------------------------------------------------------------------

//     // OJ: https://leetcode.com/problems/brightest-position-on-street/
//     // Time: O(NlogN)
//     // Space: O(N)
//     class Solution {
//     public:
//         int brightestPosition(vector<vector<int>>& A) {

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
