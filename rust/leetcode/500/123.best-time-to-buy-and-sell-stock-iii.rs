/*
 * @lc app=leetcode id=123 lang=rust
 *
 * [123] Best Time to Buy and Sell Stock III
 */

// @lc code=start
impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let (mut buy1, mut sell1) = (-prices[0], 0);
        let (mut buy2, mut sell2) = (-prices[0], 0);
        for i in 1..prices.len() {
            buy1 = buy1.max(-prices[i]);
            sell1 = sell1.max(buy1 + prices[i]);
            buy2 = buy2.max(sell1 - prices[i]);
            sell2 = sell2.max(buy2 + prices[i]);
        }
        sell2
    }
}
// @lc code=end
impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let n=prices.len();
        let (mut buy1,mut sell1,mut buy2,mut sell2)=(-prices[0],0,-prices[0],0);
        for i in 1..n{
            buy1=buy1.max(-prices[i]);
            sell1=sell1.max(buy1+prices[i]);
            buy2=buy2.max(sell1-prices[i]);
            sell2=sell2.max(buy2+prices[i]);
        }
        sell2
    }
}