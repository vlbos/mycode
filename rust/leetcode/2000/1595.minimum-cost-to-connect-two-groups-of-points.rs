/*
 * @lc app=leetcode id=1595 lang=rust
 *
 * [1595] Minimum Cost to Connect Two Groups of Points
 */

// @lc code=start
impl Solution {
    pub fn connect_two_groups(cost: Vec<Vec<i32>>) -> i32 {
        let (m, n) = (cost.len(), cost[0].len());
        let n1 = 1 << n;
        let mut cost_matrix = vec![vec![0; n1]; m];
        for i in 0..m {
            for k in 0..n1 {
                let mut sum = 0;
                for j in 0..n {
                    if k & (1 << j) > 0 {
                        sum += cost[i][j];
                    }
                }
                cost_matrix[i][k] = sum;
            }
        }
        let mut dp = vec![vec![i32::MAX/2; n1]; m];
        dp[0] = cost_matrix[0].clone();
        for i in 1..m {
            for k in 1..n1 {
                for j in 0..n {
                    dp[i][(1 << j) | k] = dp[i][(1 << j) | k].min(dp[i - 1][k] + cost[i][j]);
                }
                let rest = n1 - 1 - k;
                let mut j = rest;
                while j >= 1 {
                    dp[i][j | k] = dp[i][j | k].min(dp[i - 1][k] + cost_matrix[i][j]);
                    j = rest & (j - 1);
                }
            }
        }
        dp[m - 1][n1 - 1]
    }
}
// @lc code=end
