/*
 * @lc app=leetcode id=1335 lang=rust
 *
 * [1335] Minimum Difficulty of a Job Schedule
 */

// @lc code=start
impl Solution {
    pub fn min_difficulty(job_difficulty: Vec<i32>, d: i32) -> i32 {
        let (n, d) = (job_difficulty.len(), d as usize);
        if n < d {
            return -1;
        }
        let mut dp = vec![vec![i32::MAX / 2; n + 1]; d + 1];
        dp[0][0] = 0;
        for i in 1..=d {
            for j in 1..=n {
                for k in 0..j {
                    dp[i][j] = dp[i][j].min(
                        dp[i - 1][j - k - 1] + *job_difficulty[j - k - 1..j].iter().max().unwrap(),
                    );
                }
            }
        }
        if dp[d][n] == i32::MAX {
            -1
        } else {
            dp[d][n]
        }
    }
}
// @lc code=end
