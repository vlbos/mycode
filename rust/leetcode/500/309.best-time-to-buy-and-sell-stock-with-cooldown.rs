/*
 * @lc app=leetcode id=309 lang=rust
 *
 * [309] Best Time to Buy and Sell Stock with Cooldown
 */

// @lc code=start
impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        if prices.is_empty() {
            return 0;
        }
        let n = prices.len();
        let mut f0 = -prices[0];
        let mut f1 = 0;
        let mut f2 = 0;
        for i in 1..n {
            let t0 = f0;
            let t1 = f1;
            let t2 = f2;
            f0 = t0.max(t2 - prices[i]);
            f1 = t0 + prices[i];
            f2 = t2.max(t1);
        }
        f1.max(f2)
    }
}
// @lc code=end
