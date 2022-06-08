/*
 * @lc app=leetcode id=1537 lang=rust
 *
 * [1537] Get the Maximum Score
 */

// @lc code=start
impl Solution {
    pub fn max_sum(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        let (m, n) = (nums1.len(), nums2.len());
        let (mut best1, mut best2) = (0, 0);
        let (mut i, mut j) = (0, 0);
        while i < m || j < n {
            if i < m && j < n {
                if nums1[i] < nums2[j] {
                    best1 += nums1[i] as i64;
                    i += 1;
                } else if nums1[i] > nums2[j] {
                    best2 += nums2[j] as i64;
                    j += 1;
                } else {
                    let best = best1.max(best2) + nums1[i] as i64;
                    best1 = best;
                    best2 = best;
                    i += 1;
                    j += 1;
                }
            } else if i < m {
                best1 += nums1[i] as i64;
                i += 1;
            } else if j < n {
                best2 += nums2[j] as i64;
                j += 1;
            }
        }
        (best1.max(best2) % 1_000_000_007) as _
    }
}
// @lc code=end
