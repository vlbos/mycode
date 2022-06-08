/*
 * @lc app=leetcode id=714 lang=rust
 *
 * [714] Best Time to Buy and Sell Stock with Transaction Fee
 */

// @lc code=start
impl Solution {
    pub fn max_profit(prices: Vec<i32>, fee: i32) -> i32 {
        let mut buy = prices[0]+fee;
        let mut profit=0;
        for i in 1..prices.len(){
                if prices[i]+fee<buy{
                    buy=prices[i]+fee;

                }else if prices[i] > buy{
                    profit+=prices[i]-buy;
                    buy=prices[i];
                }
        }
        profit
    }
}
// @lc code=end

