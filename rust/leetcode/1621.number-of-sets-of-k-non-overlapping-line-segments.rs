/*
 * @lc app=leetcode id=1621 lang=rust
 *
 * [1621] Number of Sets of K Non-Overlapping Line Segments
 */

// @lc code=start
impl Solution {
    pub fn number_of_sets(n: i32, k: i32) -> i32 {
        let p = 1000000007;
        let n = n as usize;
        let k = k as usize;
        let mut dp = vec![vec![vec![0; 2]; n]; n];
        dp[0][0][0] = 1;
        for i in 1..n {
            for j in 0..=k {
                dp[i][j][0] = (dp[i - 1][j][0] + dp[i - 1][j][1]) % p;
                dp[i][j][1] = dp[i - 1][j][1];
                if j > 0 {
                    dp[i][j][1] += dp[i - 1][j - 1][0];
                    dp[i][j][1] %= p;
                    dp[i][j][1] += dp[i - 1][j - 1][1];
                    dp[i][j][1] %= p;
                }
            }
        }
        (dp[n - 1][k][0] + dp[n - 1][k][1]) % p
    }
}
// @lc code=end
