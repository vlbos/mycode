/*
 * @lc app=leetcode id=1039 lang=rust
 *
 * [1039] Minimum Score Triangulation of Polygon
 */

// @lc code=start
impl Solution {
    pub fn min_score_triangulation(values: Vec<i32>) -> i32 {
        let n = values.len();
        let mut dp = vec![vec![0; n]; n];
        for i in (0..n - 2).rev() {
            for j in i + 2..n {
                for k in i + 1..j {
                    if dp[i][j] == 0 {
                        dp[i][j] = dp[i][k] + dp[k][j] + values[i] * values[k] * values[j];
                    } else {
                        dp[i][j] =
                            dp[i][j].min(dp[i][k] + dp[k][j] + values[i] * values[k] * values[j]);
                    }
                }
            }
        }
        dp[0][n - 1]
    }
}
// @lc code=end
