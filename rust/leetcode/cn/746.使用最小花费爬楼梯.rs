/*
 * @lc app=leetcode.cn id=746 lang=rust
 *
 * [746] 使用最小花费爬楼梯
 */

// @lc code=start
impl Solution {
    pub fn min_cost_climbing_stairs(cost: Vec<i32>) -> i32 {
        let mut dp = vec![0; cost.len() + 1].to_vec();

        for i in 2..dp.len() {
            dp[i] = (dp[i - 1] + cost[i - 1] as usize).min(dp[i - 2] + cost[i - 2] as usize);
        }
        dp[cost.len()] as i32
    }
}
// @lc code=end
