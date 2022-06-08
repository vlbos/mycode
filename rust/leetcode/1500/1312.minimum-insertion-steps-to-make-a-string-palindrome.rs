/*
 * @lc app=leetcode id=1312 lang=rust
 *
 * [1312] Minimum Insertion Steps to Make a String Palindrome
 */

// @lc code=start
impl Solution {
    pub fn min_insertions(s: String) -> i32 {
        let n = s.len();
        let bs = s.as_bytes();
        let mut dp = vec![vec![0; n]; n];
        for span in 2..=n {
            for i in 0..=n - span {
                let j = i + span - 1;
                dp[i][j] = dp[i + 1][j].min(dp[i][j - 1]) + 1;
                if bs[i] == bs[j] {
                    dp[i][j] = dp[i][j].min(dp[i + 1][j - 1]);
                }
            }
        }
        dp[0][n - 1]
    }
}
// @lc code=end
