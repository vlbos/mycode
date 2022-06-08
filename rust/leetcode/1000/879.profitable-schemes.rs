/*
 * @lc app=leetcode id=879 lang=rust
 *
 * [879] Profitable Schemes
 */

// @lc code=start
impl Solution {
    pub fn profitable_schemes(n: i32, min_profit: i32, group: Vec<i32>, profit: Vec<i32>) -> i32 {
        let (n, m) = (n as usize, min_profit as usize);
        let mut dp = vec![vec![0i64; m + 1]; n + 1];
        for i in 0..n + 1 {
            dp[i][0] = 1;
        }
        for (&earn, members) in profit.iter().zip(group) {
            let (e, me) = (earn as usize, members as usize);
            for j in (me..=n).rev() {
                for k in (0..=m).rev() {
                    dp[j][k] =
                        (dp[j][k] + dp[j - me][if k > e { k - e } else { 0 }]) % 1_000_000_007;
                }
            }
        }
        dp[n][m] as _
    }
}
// @lc code=end
