/*
 * @lc app=leetcode id=319 lang=rust
 *
 * [319] Bulb Switcher
 */

// @lc code=start
impl Solution {
    pub fn bulb_switch(n: i32) -> i32 {
        let n = n as f64;
        n.sqrt() as i32
    }
}
// @lc code=end
