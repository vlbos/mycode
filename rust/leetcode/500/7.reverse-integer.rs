/*
 * @lc app=leetcode id=7 lang=rust
 *
 * [7] Reverse Integer
 */

// @lc code=start
impl Solution {
    pub fn reverse(x: i32) -> i32 {
                let mut xx = x;
        let mut result = 0;
        while xx != 0 {
            if result > i32::MAX / 10 || result < i32::MIN / 10 {
                return 0;
            }
            result = result * 10 + xx % 10;
            xx /= 10;
        }

        result
    }
}
// @lc code=end

