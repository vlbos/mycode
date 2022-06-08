/*
 * @lc app=leetcode id=813 lang=rust
 *
 * [813] Largest Sum of Averages
 */

// @lc code=start
impl Solution {
    pub fn largest_sum_of_averages(nums: Vec<i32>, k: i32) -> f64 {
        let n = nums.len();
        let mut presum = vec![0; n + 1];
        for (i, v) in nums.iter().enumerate() {
            presum[i + 1] = presum[i] + nums[i];
        }
        let mut dp = vec![0f64; n + 1];
        for (i, v) in presum.iter().enumerate() {
            dp[i] = (presum[n] - presum[i]) as f64 / (n - i) as f64;
        }
        for _ in 0..k - 1 {
            for i in 0..n {
                for j in i + 1..n {
                    dp[i] = dp[i].max((presum[j] - presum[i]) as f64 / (j - i) as f64 + dp[j]);
                }
            }
        }
        dp[0]
    }
}
// @lc code=end
