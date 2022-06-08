/*
 * @lc app=leetcode id=1745 lang=rust
 *
 * [1745] Palindrome Partitioning IV
 */

// @lc code=start
impl Solution {
    pub fn check_partitioning(s: String) -> bool {
        let n = s.len();
        let mut dp = vec![vec![0; n + 1]; n + 1];
        let bs = s.as_bytes();
        for i in 0..n {
            dp[i][i] = 1;
            if i < n - 1 && bs[i] == bs[i + 1] {
                dp[i][i + 1] = 1;
            }
        }
        for l in 3..=n {
            for i in 0..n + 1 - l {
                let j = i + l - 1;
                if bs[i] == bs[j] && dp[i + 1][j - 1] > 0 {
                    dp[i][j] = 1;
                }
            }
        }
        for s in 1..=n - 2 {
            for e in s..=n - 2 {
                if dp[0][s - 1] > 0 && dp[s][e] > 0 && dp[e + 1][n - 1] > 0 {
                    return true;
                }
            }
        }
        false
    }
}
// @lc code=end
