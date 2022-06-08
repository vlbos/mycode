/*
 * @lc app=leetcode id=115 lang=rust
 *
 * [115] Distinct Subsequences
 */

// @lc code=start
impl Solution {
    pub fn num_distinct(s: String, t: String) -> i32 {
        let (m, n) = (s.len(), t.len());
        if m < n {
            return 0;
        }
        let mut dp = vec![vec![0; n + 1]; m + 1];
        for i in 0..=m {
            dp[i][n] = 1;
        }
        let (bs, bt) = (s.as_bytes(), t.as_bytes());
        for i in (0..m).rev() {
            let sc = bs[i];
            for j in (0..n).rev() {
                let tc = bt[j];
                if sc == tc {
                    dp[i][j] = dp[i + 1][j + 1] + dp[i + 1][j];
                } else {
                    dp[i][j] = dp[i + 1][j];
                }
            }
        }
        dp[0][0]
    }
}
// @lc code=end
