/*
 * @lc app=leetcode id=552 lang=rust
 *
 * [552] Student Attendance Record II
 */

// @lc code=start
impl Solution {
    pub fn check_record(n: i32) -> i32 {
       let mut dp = vec![vec![0i64; 3]; 2];
        let p = 1000_000_007;
        dp[0][0] = 1;
        for _ in 0..n {
            let mut new_dp = vec![vec![0; 3]; 2];
            for j in 0..2 {
                for k in 0..3 {
                    new_dp[j][0] = (new_dp[j][0] + dp[j][k]) % p;
                }
            }
            for k in 0..3 {
                new_dp[1][0] = (new_dp[1][0] + dp[0][k]) % p;
            }
            for j in 0..2 {
                for k in 1..3 {
                    new_dp[j][k] = (new_dp[j][k] + dp[j][k - 1]) % p;
                }
            }
            dp = new_dp;
        }
        let mut ans = 0;
        for j in 0..2 {
            for k in 0..3 {
                ans += dp[j][k];
            }
        }
        (ans%p) as _
    }
}
// @lc code=end
