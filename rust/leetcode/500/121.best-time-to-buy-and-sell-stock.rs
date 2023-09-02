/*
 * @lc app=leetcode id=121 lang=rust
 *
 * [121] Best Time to Buy and Sell Stock
 */

// @lc code=start
impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
              let mut max = 0;
        let mut min = i32::MAX;

        for i in 0..prices.len() {
            let p = prices[i];
            if (p < min) {
                min = p;
            } else if (p - min) > max {
                max = p - min;
            }
        }
        max
    }
}
// @lc code=end

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let n=prices.len();
        let mut dp=vec![0;n];
        let mut min=prices[0];
        for (i,&price) in prices[1..].iter().enumerate(){
            min=min.min(price);
            dp[i+1]=dp[i].max(price-min);
        }
        dp[n-1]
    }
}