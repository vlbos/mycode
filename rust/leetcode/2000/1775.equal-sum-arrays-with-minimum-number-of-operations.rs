/*
 * @lc app=leetcode id=1775 lang=rust
 *
 * [1775] Equal Sum Arrays With Minimum Number of Operations
 */

// @lc code=start
impl Solution {
    pub fn min_operations(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        let (mut s1, mut s2) = (nums1.iter().sum::<i32>(), nums2.iter().sum::<i32>());
        if s1 == s2 {
            return 0;
        }
        if s1 > s2 {
            return Self::min_operations(nums2, nums1);
        }
        let mut d = s2 - s1;
        let mut ans = 0;
        let mut t = -1;
        let mut f = vec![0; 6];
        for &n in &nums1 {
            f[(6 - n) as usize] += 1;
        }
        for &n in &nums2 {
            f[(n - 1) as usize] += 1;
        }
        for i in (1..6).rev() {
            if d == 0 {
                break;
            }
            while f[i] > 0 && d > 0 {
                ans += 1;
                f[i] -= 1;
                d -= i as i32;
            }
        }

        if d > 0 {
            -1
        } else {
            ans
        }
    }
}
// @lc code=end
