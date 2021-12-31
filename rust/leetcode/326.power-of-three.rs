/*
 * @lc app=leetcode id=326 lang=rust
 *
 * [326] Power of Three
 */

// @lc code=start
impl Solution {
    pub fn is_power_of_three(n: i32) -> bool {
                n > 0 && 1162261467 % n == 0

    }
}
// @lc code=end

