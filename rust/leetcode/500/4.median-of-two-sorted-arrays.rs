/*
 * @lc app=leetcode id=4 lang=rust
 *
 * [4] Median of Two Sorted Arrays
 */

// @lc code=start
impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
         let (m, n) = (nums1.len(), nums2.len());
        if m > n {
            return Self::find_median_sorted_arrays(nums2, nums1);
        }
        let mn = m + n;
        let (mut left, mut right) = (0, m);
        let (mut median1, mut median2) = (0, 0);
        while left <= right {
            let  i = (left + right) / 2;
            let  j = ((mn + 1) / 2) - i;
            let num_im1 = if i == 0 { i32::MIN } else { nums1[i - 1] };
            let num_i = if i == m { i32::MAX } else { nums1[i] };
            let num_jm1 = if j == 0 { i32::MIN } else { nums2[j - 1] };
            let num_j = if j == n { i32::MAX } else { nums2[j] };
            if num_im1 <= num_j {
                median1 = num_im1.max(num_jm1);
                median2 = num_i.min(num_j);
                left = i + 1;
            } else {
                right = i - 1;
            }
        }

        if mn % 2 > 0 {
            median1 as f64
        } else {
            ((median1  + median2) as f64) / 2.0
        }

    }
}
// @lc code=end
