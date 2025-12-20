// [3400\. Maximum Number of Matching Indices After Right Shifts 🔒](https://leetcode.com/problems/maximum-number-of-matching-indices-after-right-shifts)
// ======================================================================================================================================================
// [![](https://img.shields.io/badge/Difficulty-Medium-4051B5?style=flat-square)](https://img.shields.io/badge/Difficulty-Medium-4051B5?style=flat-square)

// Description
// -----------

// You are given two integer arrays, `nums1` and `nums2`, of the same length.

// An index `i` is considered **matching** if `nums1[i] == nums2[i]`.

// Return the **maximum** number of **matching** indices after performing any number of **right shifts** on `nums1`.

// A **right shift** is defined as shifting the element at index `i` to index `(i + 1) % n`, for all indices.

// **Example 1:**

// **Input:** nums1 = \[3,1,2,3,1,2\], nums2 = \[1,2,3,1,2,3\]

// **Output:** 6

// **Explanation:**

// If we right shift `nums1` 2 times, it becomes `[1, 2, 3, 1, 2, 3]`. Every index matches, so the output is 6.

// **Example 2:**

// **Input:** nums1 = \[1,4,2,5,3,1\], nums2 = \[2,3,1,2,4,6\]

// **Output:** 3

// **Explanation:**

// If we right shift `nums1` 3 times, it becomes `[5, 3, 1, 1, 4, 2]`. Indices 1, 2, and 4 match, so the output is 3.

// **Constraints:**

// *   `nums1.length == nums2.length`
// *   `1 <= nums1.length, nums2.length <= 3000`
// *   `1 <= nums1[i], nums2[i] <= 109`

//  int maximum_matching_indices(vector<int>& nums1, vector<int>& nums2) {

#[allow(dead_code)]
pub struct Solution;

impl Solution {
    pub fn maximum_matching_indices(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        let n = nums1.len();
        (0..n)
            .map(|k| {
                nums2
                    .iter()
                    .enumerate()
                    .filter(|&(i, &x)| x == nums1[(i + k) % n])
                    .count()
            })
            .max()
            .unwrap() as _
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn test_maximum_matching_indices_1() {
        assert_eq!(
            6,
            Solution::maximum_matching_indices(vec![3, 1, 2, 3, 1, 2], vec![1, 2, 3, 1, 2, 3])
        );
    }
    #[test]
    pub fn test_maximum_matching_indices_2() {
        assert_eq!(
            3,
            Solution::maximum_matching_indices(vec![1, 4, 2, 5, 3, 1], vec![2, 3, 1, 2, 4, 6])
        );
    }
}
