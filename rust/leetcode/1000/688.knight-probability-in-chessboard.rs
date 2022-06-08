/*
 * @lc app=leetcode id=688 lang=rust
 *
 * [688] Knight Probability in Chessboard
 */

// @lc code=start
impl Solution {
    pub fn knight_probability(n: i32, k: i32, row: i32, column: i32) -> f64 {
        let ones = vec![-1, 1];
        let twos = vec![-2, 2];
        let mut d = Vec::new();
        for o in &ones {
            for t in &twos {
                let mut e = vec![*o, *t];
                d.push(e.clone());
                e.reverse();
                d.push(e);
            }
        }
        let nn = n as usize;
        let mut dp = vec![vec![0f64; nn]; nn];
        dp[row as usize][column as usize] = 1f64;
        for _ in (0..k) {
            let mut dp2 = vec![vec![0f64; nn]; nn];
            for r in 0..nn {
                for c in 0..nn {
                    for dd in &d {
                        let nr = r as i32 + dd[0];
                        let nc = c as i32 + dd[1];
                        if nr >= 0 && nr < n && nc >= 0 && nc < n {
                            dp2[nr as usize][nc as usize] += dp[r][c] / 8f64;
                        }
                    }
                }
            }
            dp = dp2;
        }
        let mut ans = 0f64;
        for r in &dp {
            for c in r {
                ans += *c;
            }
        }
        ans
    }
}
// @lc code=end
