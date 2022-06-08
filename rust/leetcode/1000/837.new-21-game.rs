/*
 * @lc app=leetcode id=837 lang=rust
 *
 * [837] New 21 Game
 */

// @lc code=start
impl Solution {
    pub fn new21_game(n: i32, k: i32, max_pts: i32) -> f64 {
        if k == 0 {
            return 1.0;
        }
        let k = k as usize;
        let n = n as usize;
        let max_pts = max_pts as usize;
        let len = k + max_pts;
        let mut dp = vec![0f64; len];
        for i in k..=n {
            if i == len {
                break;
            }
            dp[i] = 1.0;
        }
        let max_ptsf64 = max_pts as f64;
        dp[k - 1] = max_ptsf64.min((n - k + 1) as f64) / max_ptsf64;
        for i in (0..k - 1).rev() {
            dp[i] = dp[i + 1] - (dp[i + max_pts + 1] - dp[i + 1]) / max_ptsf64;
        }
        dp[0]
    }
}
// @lc code=end
