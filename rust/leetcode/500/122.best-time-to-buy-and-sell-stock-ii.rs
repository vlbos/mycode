/*
 * @lc app=leetcode id=122 lang=rust
 *
 * [122] Best Time to Buy and Sell Stock II
 */

// @lc code=start
impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
           let mut ans = 0;
        for i in 1..prices.len() {
            ans += (prices[i] - prices[i - 1]).max(0);
        }
        ans
    }
}
// @lc code=end

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let n=prices.len();
        let mut dp=vec![vec![0;2];n];
        dp[0][0]=0;
        dp[0][1]=-prices[0];
        for i in 1..n{
            dp[i][0]=dp[i-1][0].max(dp[i-1][1]+prices[i]);
            dp[i][1]=dp[i-1][1].max(dp[i-1][0]-prices[i]);
        }
        dp[n-1][0]
    }
}

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let n=prices.len();
        let mut dp=vec![0,-prices[0]];
        for i in 1..n{
            dp=vec![dp[0].max(dp[1]+prices[i]),dp[1].max(dp[0]-prices[i])];
        }
        dp[0]
    }
}