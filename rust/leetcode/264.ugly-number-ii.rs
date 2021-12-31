/*
 * @lc app=leetcode id=264 lang=rust
 *
 * [264] Ugly Number II
 */

// @lc code=start
impl Solution {
    pub fn nth_ugly_number(n: i32) -> i32 {
        let n = n as usize;
        let mut dp = vec![1; n];
        let mut p = vec![0, 0, 0];
        for i in 1..n {
            dp[i] = (dp[p[0]]*2).min(dp[p[1]]*3).min(dp[p[2]]*5);
            if dp[i] == dp[p[0]]*2 {
                p[0] += 1;
            }
            if dp[i] == dp[p[1]]*3 {
                p[1] += 1;
            }
            if dp[i] == dp[p[2]]*5 {
                p[2] += 1;
            }
        }
        dp[n - 1]
    }
}
// @lc code=end
