/*
 * @lc app=leetcode id=1855 lang=rust
 *
 * [1855] Maximum Distance Between a Pair of Values
 */

// @lc code=start
impl Solution {
    pub fn max_distance(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        let mut ans = 0;
        let mut i = 0;
        let n1 = nums1.len();
        for (j, &v) in nums2.iter().enumerate() {
            while i < n1 && nums1[i] > v {
                i += 1;
            }
            if i < n1 {
                ans = ans.max((j - i) as i32);
            }
        }
        ans
    }
}
// @lc code=end
