/*
 * @lc app=leetcode id=2145 lang=rust
 *
 * [2145] Count the Hidden Sequences
 */

// @lc code=start
impl Solution {
    pub fn number_of_arrays(differences: Vec<i32>, lower: i32, upper: i32) -> i32 {
        let (mut x, mut y, mut cur) = (0, 0, 0);
        for &d in &differences {
            cur += d;
            x = x.min(cur);
            y = y.max(cur);
            if y - x > upper - lower {
                return 0;
            }
        }
        upper - lower - (y - x) + 1
    }
}
// @lc code=end
