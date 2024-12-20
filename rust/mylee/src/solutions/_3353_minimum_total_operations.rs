// [3353\. Minimum Total Operations 🔒](https://leetcode.com/problems/minimum-total-operations)
// ============================================================================================

// [![](https://img.shields.io/badge/Difficulty-Easy-4051B5?style=flat-square)](https://img.shields.io/badge/Difficulty-Easy-4051B5?style=flat-square)

// Description
// -----------

// Given an array of integers `nums`, you can perform _any_ number of operations on this array.

// In each **operation**, you can:

// *   Choose a **prefix** of the array.
// *   Choose an integer `k` (which can be negative) and add `k` to each element in the chosen prefix.

// A **prefix** of an array is a subarray that starts from the beginning of the array and extends to any point within it.

// Return the **minimum** number of operations required to make all elements in `arr` equal.

// **Example 1:**

// **Input:** nums = \[1,4,2\]

// **Output:** 2

// **Explanation:**

// *   **Operation 1**: Choose the prefix `[1, 4]` of length 2 and add -2 to each element of the prefix. The array becomes `[-1, 2, 2]`.
// *   **Operation 2**: Choose the prefix `[-1]` of length 1 and add 3 to it. The array becomes `[2, 2, 2]`.
// *   Thus, the minimum number of required operations is 2.

// **Example 2:**

// **Input:** nums = \[10,10,10\]

// **Output:** 0

// **Explanation:**

// *   All elements are already equal, so no operations are needed.

// **Constraints:**

// *   `1 <= nums.length <= 105`
// *   `-109 <= nums[i] <= 109`

//  int min_operations(vector<int>& nums)

#[allow(dead_code)]
pub struct Solution;

impl Solution {
    pub fn min_operations(nums: Vec<i32>) -> i32 {
        nums.windows(2).filter(|w| w[0] != w[1]).count() as _
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    pub fn test_min_operations_1() {
        assert_eq!(2, Solution::min_operations(vec![1, 4, 2]));
    }
    #[test]
    pub fn test_min_operations_2() {
        assert_eq!(0, Solution::min_operations(vec![10, 10, 10]));
    }
}
