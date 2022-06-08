/*
 * @lc app=leetcode id=1223 lang=rust
 *
 * [1223] Dice Roll Simulation
 */

// @lc code=start
impl Solution {
    pub fn die_simulator(n: i32, roll_max: Vec<i32>) -> i32 {
        let n = n as usize;
        let mut dp = vec![vec![vec![0;16 ]; 7]; n + 1];
        for i in 1..n + 1 {
            for j in 1..7 {
                if i == 1 {
                    dp[i][j][1] = 1;
                    continue;
                }
                for k in 2..roll_max[j - 1] as usize + 1 {
                    dp[i][j][k] = dp[i - 1][j][k - 1];
                }
                let mut s = 0;
                for l in 1..7 {
                    if l == j {
                        continue;
                    }
                    for k in 1..16 {
                        s += dp[i - 1][l][k];
                        s %= 1_000_000_007;
                    }
                }
                dp[i][j][1] = s;
            }
        }
        let mut ans = 0;
        for j in 1..7 {
            for k in 1..16 {
                ans += dp[n][j][k];
                ans %= 1_000_000_007;
            }
        }
        ans
    }
}
// @lc code=end
