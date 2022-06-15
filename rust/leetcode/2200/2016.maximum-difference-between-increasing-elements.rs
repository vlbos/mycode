/*
 * @lc app=leetcode id=2016 lang=rust
 *
 * [2016] Maximum Difference Between Increasing Elements
 */

// @lc code=start
impl Solution {
    pub fn maximum_difference(nums: Vec<i32>) -> i32 {
        let mut ans = -1;
        let mut premin = nums[0];
        for &num in &nums[1..] {
            if num > premin {
                ans = ans.max(num - premin);
            } else {
                premin = num;
            }
        }
        ans
    }
}
// @lc code=end
