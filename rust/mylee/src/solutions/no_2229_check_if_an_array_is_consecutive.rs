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
//         bool is_consecutive(vector<int>& A) {

#[allow(dead_code)]
pub struct Solution {}
impl Solution {
    pub fn is_consecutive(nums: Vec<i32>) -> bool {
        let (min, max) = (*nums.iter().min().unwrap(), *nums.iter().max().unwrap());
        if max - min >= nums.len() as i32 {
            return false;
        }
        let mut nums = nums;
        nums.sort();
        nums.windows(2).all(|x| x[0] + 1 == x[1])
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn test_is_consecutive_1() {
        assert!(Solution::is_consecutive(vec![1, 3, 4, 2]));
    }
    #[test]
    pub fn test_is_consecutive_2() {
        assert!(!Solution::is_consecutive(vec![1, 3]));
    }
    #[test]
    pub fn test_is_consecutive_3() {
        assert!(Solution::is_consecutive(vec![3, 5, 4]));
    }
}
