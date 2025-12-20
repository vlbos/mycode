// ## [3595\. Once Twice 🔒](https://leetcode.com/problems/once-twice)

// [![](https://img.shields.io/badge/Difficulty-Medium-4051B5?style=flat-square)](https://img.shields.io/badge/Difficulty-Medium-4051B5?style=flat-square)

// ## Description

// You are given an integer array `nums`. In this array:

// +   Exactly one element appears **once**.

// +   Exactly one element appears **twice**.

// +   All other elements appear **exactly three times**.

// Return an integer array of length 2, where the first element is the one that appears **once**,
// and the second is the one that appears **twice**.

// Your solution must run in **O(n)** time and **O(1)** space.

// **Example 1:**

// **Input:** nums = \[2,2,3,2,5,5,5,7,7\]

// **Output:** \[3,7\]

// **Explanation:**

// The element 3 appears **once**, and the element 7 appears **twice**. The remaining elements each appear **three times**.

// **Example 2:**

// **Input:** nums = \[4,4,6,4,9,9,9,6,8\]

// **Output:** \[8,6\]

// **Explanation:**

// The element 8 appears **once**, and the element 6 appears **twice**. The remaining elements each appear **three times**.

// **Constraints:**

// +   `3 <= nums.length <= 105`
// +   `-231 <= nums[i] <= 231 - 1`
// +   `nums.length` is a multiple of 3.
// +   Exactly one element appears once, one element appears twice, and all other elements appear three times.

//  vector<int> once_twice(vector<int>& nums) {

#[allow(dead_code)]
pub struct Solution {}

impl Solution {
    pub fn once_twice(nums: Vec<i32>) -> Vec<i32> {
        let mut dp = vec![0; 3];
        dp[0] = !0;
        for &x in &nums {
            dp = (0..3)
                .map(|i| (x & dp[(i + 2) % 3]) | (!x & dp[i]))
                .collect();
        }
        let mut dp2 = vec![0; 3];
        dp2[0] = !0;
        for &x in &nums {
            if !x & dp[1] == 0 && x & dp[2] == 0 {
                dp2 = (0..3)
                    .map(|i| (x & dp2[(i + 2) % 3]) | (!x & dp2[i]))
                    .collect();
            }
        }
        vec![dp2[1], (dp2[1] ^ dp[1]) | dp[2]]
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn test_once_twice_1() {
        assert_eq!(
            vec![3, 7],
            Solution::once_twice(vec![2, 2, 3, 2, 5, 5, 5, 7, 7])
        );
    }
    #[test]
    pub fn test_once_twice_2() {
        assert_eq!(
            vec![8, 6],
            Solution::once_twice(vec![4, 4, 6, 4, 9, 9, 9, 6, 8])
        );
    }
}
