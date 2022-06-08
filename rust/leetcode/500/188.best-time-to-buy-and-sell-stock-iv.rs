/*
 * @lc app=leetcode id=188 lang=rust
 *
 * [188] Best Time to Buy and Sell Stock IV
 */

// @lc code=start
impl Solution {
    pub fn max_profit(k: i32, prices: Vec<i32>) -> i32 {
        if prices.is_empty() {
            return 0;
        }
        let n = prices.len();
        let k = (k as usize).min(n / 2) + 1;
        let (mut buy, mut sell) = (vec![0; k], vec![0; k]);
        buy[0] = -prices[0];
        sell[0] = 0;
        for i in 1..k {
            buy[i] = i32::MIN / 2;
            sell[i] = i32::MIN / 2;
        }
        for i in 1..n {
            buy[0] = buy[0].max(sell[0] - prices[i]);
            for j in 1..k {
                buy[j] = buy[j].max(sell[j] - prices[i]);
                sell[j] = sell[j].max(buy[j - 1] + prices[i]);
            }
        }
        *sell.iter().max().unwrap()
    }
}
// @lc code=end
