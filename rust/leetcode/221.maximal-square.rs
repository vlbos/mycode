/*
 * @lc app=leetcode id=221 lang=rust
 *
 * [221] Maximal Square
 */

// @lc code=start
impl Solution {
    pub fn maximal_square(matrix: Vec<Vec<char>>) -> i32 {
        if matrix.is_empty() || matrix[0].is_empty() {
            return 0;
        }
        let m = matrix.len();
        let n = matrix[0].len();
        let mut dp = vec![vec![0; n]; m];
        let mut size = i32::MIN;
        for i in 0..m {
            for j in 0..n {
                if matrix[i][j] == '1' {
                    if i == 0 || j == 0 {
                        dp[i][j] = 1;
                    } else {
                        dp[i][j] = 1 + dp[i - 1][j - 1].min(dp[i - 1][j].min(dp[i][j - 1]));
                    }
                    size = size.max(dp[i][j]);
                }
            }
        }
        size * size
    }
}
// @lc code=end
