/*
 * @lc app=leetcode id=633 lang=rust
 *
 * [633] Sum of Square Numbers
 */

// @lc code=start
impl Solution {
    pub fn judge_square_sum(c: i32) -> bool {
        let c = c as i64;
        let mut a = 0;
        while a * a <= c {
            let s = ((c - a * a) as f64).sqrt() as i64;
            if s * s == c - a * a {
                return true;
            }
            a += 1;
        }
        false
    }
}
// @lc code=end
