/*
 * @lc app=leetcode id=2110 lang=rust
 *
 * [2110] Number of Smooth Descent Periods of a Stock
 */

// @lc code=start
impl Solution {
    pub fn get_descent_periods(prices: Vec<i32>) -> i64 {
        let n = prices.len();
        let mut prev = 1;
        let mut ans = 1;
        for i in 1..n {
            if prices[i] == prices[i - 1] - 1 {
                prev += 1;
            } else {
                prev = 1;
            }
            ans += prev as i64;
        }
        ans
    }
}
// @lc code=end
