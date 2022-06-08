/*
 * @lc app=leetcode id=1685 lang=rust
 *
 * [1685] Sum of Absolute Differences in a Sorted Array
 */

// @lc code=start
impl Solution {
    pub fn get_sum_absolute_differences(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        let mut ans = vec![0; n];
        ans[0] = nums.iter().sum::<i32>() - nums[0] * n as i32;
        for i in 1..n {
            let d = nums[i] - nums[i - 1];
            ans[i] = ans[i - 1] - (n - i * 2) as i32 * d;
        }
        ans
    }
}
// @lc code=end
