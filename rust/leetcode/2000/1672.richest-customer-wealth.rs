/*
 * @lc app=leetcode id=1672 lang=rust
 *
 * [1672] Richest Customer Wealth
 */

// @lc code=start
impl Solution {
    pub fn maximum_wealth(accounts: Vec<Vec<i32>>) -> i32 {
        accounts.iter().map(|x|x.iter().sum::<i32>()).max().unwrap()
    }
}
// @lc code=end

