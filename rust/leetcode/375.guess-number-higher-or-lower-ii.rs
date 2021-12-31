/*
 * @lc app=leetcode id=375 lang=rust
 *
 * [375] Guess Number Higher or Lower II
 */

// @lc code=start
impl Solution {
    pub fn get_money_amount(n: i32) -> i32 {
        let  n = n as usize;
        let mut dp =vec![vec![0;n+1];n+1];
        for l in 2..=n{
            for s in 1..=n-l+1{
                let mut min = i32::MAX;
                for p in s+(l-1)/2..s+l-1{
                      min = min.min( p as i32+dp[s][p-1].max(dp[p+1][s+l-1] ));
                }
                dp[s][s+l-1]=min;
            }
        }
        dp[1][n]
    }
}
// @lc code=end

