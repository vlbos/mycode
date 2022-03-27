/*
 * @lc app=leetcode id=1140 lang=rust
 *
 * [1140] Stone Game II
 */

// @lc code=start
impl Solution {
    pub fn stone_game_ii(piles: Vec<i32>) -> i32 {
        let n = piles.len();
        let mut sum = 0;
        let mut dp = vec![vec![0; n + 1]; n];
        for i in (0..n).rev() {
            sum += piles[i];
            for m in 1..=n {
                if i + 2 * m >= n {
                    dp[i][m] = sum;
                } else {
                    for x in 1..=2 * m {
                        dp[i][m] = dp[i][m].max(sum - dp[i + x][m.max(x)]);
                    }
                }
            }
        }
        dp[0][1]
    }
}
// @lc code=end
