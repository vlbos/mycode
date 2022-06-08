/*
 * @lc app=leetcode id=231 lang=rust
 *
 * [231] Power of Two
 */

// @lc code=start
impl Solution {
    pub fn is_power_of_two(n: i32) -> bool {
                n > 0 && (n & (n - 1)) == 0
    }
}
// @lc code=end

