/*
 * @lc app=leetcode id=1749 lang=rust
 *
 * [1749] Maximum Absolute Sum of Any Subarray
 */

// @lc code=start
impl Solution {
    pub fn max_absolute_sum(nums: Vec<i32>) -> i32 {
       let mut sum = vec![0; nums.len() + 1];
        let (mut min, mut max) = (nums[0], nums[0]);
        for i in 1..=nums.len() {
            sum[i] = sum[i - 1] + nums[i - 1];
            min = min.min(sum[i]);
            max = max.max(sum[i]);
        }
        (max - min).max(max.abs().max(min.abs()))
    }
}
// @lc code=end
impl Solution {
    pub fn max_absolute_sum(nums: Vec<i32>) -> i32 {
        let n=nums.len();
        let mut f=vec![0;n];
        let mut dp=vec![0;n];
        f[0]=nums[0];
        dp[0]=nums[0];
        let (mut mn,mut mx)=(f[0],dp[0]);
        for i in 1..n{
            f[i]=nums[i].min(f[i-1]+nums[i]);
            dp[i]=nums[i].max(dp[i-1]+nums[i]);
            mn=mn.min(f[i]);
            mx=mx.max(dp[i]);
        }
        mn.abs().max(mx.abs())
    }
}