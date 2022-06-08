/*
 * @lc app=leetcode id=1510 lang=rust
 *
 * [1510] Stone Game IV
 */

// @lc code=start
impl Solution {
    pub fn winner_square_game(n: i32) -> bool {
        let n = n as usize;
        let mut f = vec![false; n + 1];
        for i in 1..=n {
            let mut k = 1;
            while k * k <= i {
                if !f[i - k * k] {
                    f[i] = true;
                    break;
                }
                k += 1;
            }
        }
        f[n]
    }
}
// @lc code=end
