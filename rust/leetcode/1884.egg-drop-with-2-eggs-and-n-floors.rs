/*
 * @lc app=leetcode id=1884 lang=rust
 *
 * [1884] Egg Drop With 2 Eggs and N Floors
 */

// @lc code=start
impl Solution {
    pub fn two_egg_drop(n: i32) -> i32 {
        ((-1.0 + ((n as f64)* 8.0 + 1.0).sqrt()) / 2.0).ceil() as _
    }
}
// @lc code=end

