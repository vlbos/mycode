/*
 * @lc app=leetcode id=2144 lang=rust
 *
 * [2144] Minimum Cost of Buying Candies With Discount
 */

// @lc code=start
impl Solution {
    pub fn minimum_cost(cost: Vec<i32>) -> i32 {
        let mut cost = cost;
        cost.sort_by(|a, b| b.cmp(&a));
        let mut ans = 0;
        for v in cost.chunks(3) {
            ans += v[0] + if v.len() > 1 { v[1] } else { 0 };
        }
        ans
    }
}
// @lc code=end
