/*
 * @lc app=leetcode id=70 lang=rust
 *
 * [70] Climbing Stairs
 */

// @lc code=start
impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
         let mut x = 0;
        let mut x1 = 0;
        let mut r = 1;
        for i in 1..=n {
            x1 = x;
            x = r;
            r = x + x1;
        }
        r
    }
}
// @lc code=end

