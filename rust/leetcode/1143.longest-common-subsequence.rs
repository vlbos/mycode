/*
 * @lc app=leetcode id=1143 lang=rust
 *
 * [1143] Longest Common Subsequence
 */

// @lc code=start
impl Solution {
    pub fn longest_common_subsequence(text1: String, text2: String) -> i32 {
        let (n1, n2) = (text1.len(), text2.len());
        let mut dp = vec![vec![0; n2+1]; n1+1];
        for i in 1..=n1 {
            let c1 = text1.chars().nth(i - 1).unwrap();
            for j in 1..=n2 {
                let c2 = text2.chars().nth(j - 1).unwrap();
                if c1 == c2 {
                    dp[i][j] = dp[i - 1][j - 1] + 1;
                } else {
                    dp[i][j] = dp[i][j - 1].max(dp[i - 1][j]);
                }
            }
        }
        dp[n1][n2]
    }
}
// @lc code=end
