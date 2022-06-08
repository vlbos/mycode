/*
 * @lc app=leetcode id=174 lang=rust
 *
 * [174] Dungeon Game
 */

// @lc code=start
impl Solution {
    pub fn calculate_minimum_hp(dungeon: Vec<Vec<i32>>) -> i32 {
        let (m, n) = (dungeon.len(), dungeon[0].len());
        let mut dp = vec![vec![i32::MAX; n + 1]; m + 1];
        dp[m - 1][n] = 1;
        dp[m][n - 1] = 1;
        for i in (0..m).rev() {
            for j in (0..n).rev() {
                dp[i][j] = 1i32.max(dp[i + 1][j].min(dp[i][j + 1]) - dungeon[i][j]);
            }
        }
        dp[0][0]
    }
}
// @lc code=end
