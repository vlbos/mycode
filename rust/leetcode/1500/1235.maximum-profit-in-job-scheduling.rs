/*
 * @lc app=leetcode id=1235 lang=rust
 *
 * [1235] Maximum Profit in Job Scheduling
 */

// @lc code=start
impl Solution {
    pub fn job_scheduling(start_time: Vec<i32>, end_time: Vec<i32>, profit: Vec<i32>) -> i32 {
        let n = start_time.len();
        let mut jobs: Vec<Vec<i32>> = start_time
            .into_iter()
            .zip(end_time)
            .zip(profit)
            .map(|x| vec![x.0 .0, x.0 .1, x.1])
            .collect();
        jobs.sort_by_key(|x| x[1]);
        jobs.insert(0, vec![0, 0, 0]);
        let mut dp = vec![0; n + 1];
        for i in 1..n + 1 {
            let mut cur = i;
            for j in (0..i).rev() {
                if jobs[j][1] <= jobs[i][0] {
                    cur = j;
                    break;
                }
            }
            dp[i] = dp[i - 1].max(jobs[i][2] + dp[cur]);
        }
        dp[dp.len() - 1]
    }
}
// @lc code=end
