/*
 * @lc app=leetcode id=903 lang=rust
 *
 * [903] Valid Permutations for DI Sequence
 */

// @lc code=start
impl Solution {
    pub fn num_perms_di_sequence(s: String) -> i32 {
        let bs = s.as_bytes();

        let n = s.len();
        let mut dp = vec![vec![0i64; n + 2]; n + 1];
        dp[0][0] = 1;
        for i in 1..=n {
            if bs[i - 1] == b'D' {
                for j in (0..i).rev() {
                    dp[i][j] = (dp[i][j + 1] + dp[i - 1][j]) % 1_000_000_007;
                }
            } else {
                for j in 1..=i {
                    dp[i][j] = (dp[i][j - 1] + dp[i - 1][j - 1]) % 1_000_000_007;
                }
            }
        }
        let mut ans = 0;
        for j in 0..=n {
            ans = (ans + dp[n][j]) % 1_000_000_007;
        }
        ans as _
    }
}
// @lc code=end
