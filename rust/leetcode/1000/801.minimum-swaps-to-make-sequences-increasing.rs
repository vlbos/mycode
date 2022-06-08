/*
 * @lc app=leetcode id=801 lang=rust
 *
 * [801] Minimum Swaps To Make Sequences Increasing
 */

// @lc code=start
impl Solution {
    pub fn min_swap(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        let n = nums1.len();
        let (mut n1, mut s1) = (0, 1);
        for i in 1..n {
            let (mut n2, mut s2) = (i32::MAX, i32::MAX);
            if nums1[i - 1] < nums1[i] && nums2[i - 1] < nums2[i] {
                n2 = n2.min(n1);
                s2 = s2.min(s1 + 1);
            }
            if nums1[i - 1] < nums2[i] && nums2[i - 1] < nums1[i] {
                n2 = n2.min(s1);
                s2 = s2.min(n1 + 1);
            }
            n1 = n2;
            s1 = s2;
        }
        s1.min(n1)
    }
}
// @lc code=end
