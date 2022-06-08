/*
 * @lc app=leetcode id=2140 lang=rust
 *
 * [2140] Solving Questions With Brainpower
 */

// @lc code=start
impl Solution {
    pub fn most_points(questions: Vec<Vec<i32>>) -> i64 {
        let n = questions.len();
        let mut dp = vec![0i64; n + 1];
        for i in (0..n).rev() {
            dp[i] =
                dp[i + 1].max(dp[n.min(i + questions[i][1] as usize + 1)] + questions[i][0] as i64);
        }
        dp[0]
    }
}
// @lc code=end
