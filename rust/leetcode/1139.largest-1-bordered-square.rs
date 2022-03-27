/*
 * @lc app=leetcode id=1139 lang=rust
 *
 * [1139] Largest 1-Bordered Square
 */

// @lc code=start
impl Solution {
    pub fn largest1_bordered_square(grid: Vec<Vec<i32>>) -> i32 {
        let (n, m) = (grid.len(), grid[0].len());
        let mut dp = vec![vec![vec![0; m + 1]; n + 1]; 2];
        let mut ans = 0;
        for i in 1..=n {
            for j in 1..=m {
                if grid[i-1][j-1] == 0 {
                    continue;
                }
                dp[0][i][j] += dp[0][i][j - 1] + 1;
                dp[1][i][j] += dp[1][i - 1][j] + 1;
                let min = dp[0][i][j].min(dp[1][i][j]);
                for k in (1..=min).rev() {
                    if dp[0][i - k + 1][j] >= k && dp[1][i][j - k + 1] >= k {
                        ans = ans.max(k);
                        break;
                    }
                }
            }
        }
        (ans * ans) as _
    }
}
// @lc code=end
