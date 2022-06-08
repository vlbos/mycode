/*
 * @lc app=leetcode id=1723 lang=rust
 *
 * [1723] Find Minimum Time to Finish All Jobs
 */

// @lc code=start
impl Solution {
    pub fn minimum_time_required(jobs: Vec<i32>, k: i32) -> i32 {
        let n = jobs.len();
        let n1 = 1 << n;
        let mut sum = vec![0; n1];
        for i in 1..n1 {
            let (x, y) = (i.trailing_zeros() as usize, i - (1 << i.trailing_zeros()));
            sum[i] = sum[y] + jobs[x];
        }
        let k = k as usize;
        let mut dp = vec![vec![0; n1]; k];
        for i in 0..n1 {
            dp[0][i] = sum[i];
        }
        for i in 1..k {
            for j in 0..n1 {
                let mut min = i32::MAX;
                let mut x = j;
                while x > 0 {
                    min = min.min(dp[i - 1][j - x].max(sum[x]));
                    x = (x - 1) & j;
                }
                dp[i][j] = min;
            }
        }
        dp[k - 1][n1 - 1]
    }
}
// @lc code=end
