/*
 * @lc app=leetcode id=42 lang=rust
 *
 * [42] Trapping Rain Water
 */

// @lc code=start
impl Solution {
    pub fn trap(height: Vec<i32>) -> i32 {
        let mut ans = 0;
        let (mut l, mut r) = (0, height.len() - 1);
        let (mut lm, mut rm) = (0, 0);
        while l < r {
            lm = lm.max(height[l]);
            rm = rm.max(height[r]);
            if height[l] < height[r] {
                ans += lm - height[l];
                l += 1;
            } else {
                ans += rm - height[r];
                r -= 1;
            }
        }
        ans
    }
}
// @lc code=end
