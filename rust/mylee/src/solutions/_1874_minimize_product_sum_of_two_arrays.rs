// 1874\. Minimize Product Sum of Two Arrays[](https://leetcode.ca/2021-07-19-1874-Minimize-Product-Sum-of-Two-Arrays/#1874-minimize-product-sum-of-two-arrays)
// ============================================================================================================================================================

// Level[](https://leetcode.ca/2021-07-19-1874-Minimize-Product-Sum-of-Two-Arrays/#level)
// --------------------------------------------------------------------------------------

// Medium

// Description[](https://leetcode.ca/2021-07-19-1874-Minimize-Product-Sum-of-Two-Arrays/#description)
// --------------------------------------------------------------------------------------------------

// The **product sum** of two equal-length arrays `a` and `b` is equal to the sum of `a[i] * b[i]` for all `0 <= i < a.length` (**0-indexed**).

// *   For example, if `a = [1,2,3,4]` and `b = [5,2,3,1]`, the **product sum** would be `1*5 + 2*2 + 3*3 + 4*1 = 22`.

// Given two arrays `nums1` and `nums2` of length `n`, return _the **minimum product sum** if you are allowed to **rearrange** the **order** of the elements in `nums1`_.

// **Example 1:**

// **Input:** nums1 = \[5,3,4,2\], nums2 = \[4,2,2,5\]

// **Output:** 40

// **Explanation:** We can rearrange nums1 to become \[3,5,4,2\]. The product sum of \[3,5,4,2\] and \[4,2,2,5\] is 3_4 + 5_2 + 4_2 + 2_5 = 40.

// **Example 2:**

// **Input:** nums1 = \[2,1,4,5,7\], nums2 = \[3,2,4,8,6\]

// **Output:** 65

// **Explanation:** We can rearrange nums1 to become \[5,7,4,1,2\]. The product sum of \[5,7,4,1,2\] and \[3,2,4,8,6\] is 5_3 + 7_2 + 4_4 + 1_8 + 2\*6 = 65.

// **Constraints:**

// *   `n == nums1.length == nums2.length`
// *   `1 <= n <= 10^5`
// *   `1 <= nums1[i], nums2[i] <= 100`

// Solution[](https://leetcode.ca/2021-07-19-1874-Minimize-Product-Sum-of-Two-Arrays/#solution)
// --------------------------------------------------------------------------------------------

// Sort both `nums1` and `nums2`. Then for each `0 <= i < n`, `nums1[i]` is multiplied with `nums2[n - 1 - i]`. In this way, the product sum is minimized.

//     class Solution {
//         public int min_product_sum(int[] nums1, int[] nums2) {

#[allow(dead_code)]
pub struct Solution {}
impl Solution {
    pub fn min_product_sum(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        let (mut nums1, mut nums2) = (nums1, nums2);
        nums1.sort();
        nums2.sort();
        nums1
            .into_iter()
            .rev()
            .zip(nums2)
            .map(|(v1, v2)| v1 * v2)
            .sum::<i32>()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn test_min_product_sum_1() {
        assert_eq!(
            40,
            Solution::min_product_sum(vec![5, 3, 4, 2], vec![4, 2, 2, 5])
        );
    }
    #[test]
    pub fn test_min_product_sum_2() {
        assert_eq!(
            65,
            Solution::min_product_sum(vec![2, 1, 4, 5, 7], vec![3, 2, 4, 8, 6])
        );
    }
}
