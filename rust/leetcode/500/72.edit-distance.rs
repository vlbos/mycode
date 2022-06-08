/*
 * @lc app=leetcode id=72 lang=rust
 *
 * [72] Edit Distance
 */

// @lc code=start
impl Solution {
    pub fn min_distance(word1: String, word2: String) -> i32 {
        let (m, n) = (word1.len(), word2.len());
        if m * n == 0 {
            return (m + n) as _;
        }
        let mut dp = vec![vec![0;n + 1]; m + 1];
        for i in 0..m + 1 {
            dp[i][0] = i;
        }
        for j in 0..n + 1 {
            dp[0][j] = j;
        }
        for i in 1..m + 1 {
            for j in 1..n + 1 {
                let left = dp[i - 1][j] + 1;
                let down = dp[i][j - 1] + 1;
                let mut left_down = dp[i - 1][j - 1];
                if word1.chars().nth(i - 1).unwrap() != word2.chars().nth(j - 1).unwrap() {
                    left_down += 1;
                }
                dp[i][j] = left.min(down.min(left_down));
            }
        }
        dp[m][n] as _
    }
}
// @lc code=end
