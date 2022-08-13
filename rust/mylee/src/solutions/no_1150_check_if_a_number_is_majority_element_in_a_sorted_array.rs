// 1150\. Check If a Number Is Majority Element in a Sorted Array
// ==============================================================

// Given an array `nums` sorted in **non-decreasing** order, and a number `target`, return `True` if and only if `target` is a majority element.

// A _majority element_ is an element that appears **more than `N/2`** times in an array of length `N`.

// **Example 1:**

// **Input:** nums = \[2,4,5,5,5,5,5,6,6\], target = 5
// **Output:** true
// **Explanation:**
// The value 5 appears 5 times and the length of the array is 9.
// Thus, 5 is a majority element because 5 > 9/2 is true.

// **Example 2:**

// **Input:** nums = \[10,100,101,101\], target = 101
// **Output:** false
// **Explanation:**
// The value 101 appears 2 times and the length of the array is 4.
// Thus, 101 is not a majority element because 2 > 4/2 is false.

// **Note:**

// 1.  `1 <= nums.length <= 1000`
// 2.  `1 <= nums[i] <= 10^9`
// 3.  `1 <= target <= 10^9`

// ### Difficulty:

// Easy

// ### Lock:

// Prime

// ### Company:

// [Salesforce](https://leetcode.ca/tags/#Salesforce)

#[allow(dead_code)]
pub struct Solution {}
impl Solution {
    pub fn is_majority_element(nums: Vec<i32>, target: i32) -> bool {
        let mut ans = 0;
        for &num in &nums {
            ans += if num == target { 1 } else { -1 };
        }
        ans > 0
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn test_is_majority_element_1() {
        assert!(Solution::is_majority_element(
            vec![2, 4, 5, 5, 5, 5, 5, 6, 6],
            5
        ));
    }
    #[test]
    pub fn test_is_majority_element_2() {
        assert!(!Solution::is_majority_element(vec![10, 100, 101, 101], 101));
    }
}
