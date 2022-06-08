/*
 * @lc app=leetcode id=1626 lang=rust
 *
 * [1626] Best Team With No Conflicts
 */

// @lc code=start
impl Solution {
    pub fn best_team_score(scores: Vec<i32>, ages: Vec<i32>) -> i32 {
        let n = scores.len();
        let mut order = (0..n).collect::<Vec<usize>>();
        order.sort_by_key(|&x| (ages[x], scores[x]));
        let mut dp = vec![0; n];
        let mut ans = 0;
        for (i, &idx) in order.iter().enumerate() {
            let score = scores[idx];
            dp[i] = score;
            for j in 0..i {
                if scores[order[j]] <= score {
                    dp[i] = dp[i].max(dp[j] + score);
                }
            }
            ans = ans.max(dp[i]);
        }
        ans
    }
}
// @lc code=end
