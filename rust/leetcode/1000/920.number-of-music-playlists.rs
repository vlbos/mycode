/*
 * @lc app=leetcode id=920 lang=rust
 *
 * [920] Number of Music Playlists
 */

// @lc code=start
impl Solution {
    pub fn num_music_playlists(n: i32, goal: i32, k: i32) -> i32 {
        let (n, m, k) = (n as usize, goal as usize, k as usize);
        let mut dp = vec![vec![0; n + 1]; m + 1];
        dp[0][0] = 1;
        for i in 1..=m {
            for j in 1..=n {
                dp[i][j] += dp[i - 1][j - 1] * (n - j + 1) as i64;
                dp[i][j] += dp[i - 1][j] * (if j > k { j - k } else { 0 }) as i64;
                dp[i][j] %= 1_000_000_007;
            }
        }
        dp[m][n] as _
    }
}
// @lc code=end
