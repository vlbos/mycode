/*
 * @lc app=leetcode id=330 lang=rust
 *
 * [330] Patching Array
 */

// @lc code=start
impl Solution {
    pub fn min_patches(nums: Vec<i32>, n: i32) -> i32 {
        let mut ans = 0;
        let mut x = 1;
        let len = nums.len();
        let mut i = 0;
        while x <= n as i64 {
            if i < len &&  (nums[i] as i64)<= x {
                x += nums[i] as i64;
                i += 1;
            } else {
                x *= 2;
                ans += 1;
            }
        }
        ans
    }
}
// @lc code=end
