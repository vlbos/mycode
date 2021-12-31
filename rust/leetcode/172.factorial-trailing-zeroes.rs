/*
 * @lc app=leetcode id=172 lang=rust
 *
 * [172] Factorial Trailing Zeroes
 */

// @lc code=start
impl Solution {
    pub fn trailing_zeroes(n: i32) -> i32 {
        if n < 5 {
            return 0;
        }
        let mut c = 0;
        let mut x = 5;
        for i in (5..=n).step_by(5) {
            x = 5;
            let mut ii = i;
            while ii % x == 0 {
                c += 1;
                ii /= 5;
            }
        }
        c
    }
}
// @lc code=end

