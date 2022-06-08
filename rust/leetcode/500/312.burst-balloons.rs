/*
 * @lc app=leetcode id=312 lang=rust
 *
 * [312] Burst Balloons
 */

// @lc code=start
impl Solution {
    pub fn max_coins(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut dp = vec![vec![0; n + 2]; n + 2];
        let mut val = vec![1];
        val.extend(nums);
        val.push(1);
        for i in (0..n).rev() {
            for j in i + 2..=n + 1 {
                for k in i + 1..j {
                    let mut sum = val[i] * val[k] * val[j];
                    sum += dp[i][k] + dp[k][j];
                    dp[i][j] = dp[i][j].max(sum);
                }
            }
        }
        dp[0][n + 1]
    }
}
// @lc code=end
