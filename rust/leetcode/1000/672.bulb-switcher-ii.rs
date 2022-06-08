/*
 * @lc app=leetcode id=672 lang=rust
 *
 * [672] Bulb Switcher II
 */

// @lc code=start
impl Solution {
    pub fn flip_lights(n: i32, presses: i32) -> i32 {
        let n = n.min(3) as usize;
        match presses{
            0=> 1,
            1=>[2,3,4][n-1],
            2=>[2,4,7][n-1],
            _=>[2,4,8][n-1],
        }
    }
}
// @lc code=end

