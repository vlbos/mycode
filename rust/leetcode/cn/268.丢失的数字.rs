/*
 * @lc app=leetcode.cn id=268 lang=rust
 *
 * [268] 丢失的数字
 */

// @lc code=start
impl Solution {
    pub fn missing_number(nums: Vec<i32>) -> i32 {
        (((nums.len() + 1) * nums.len() / 2) as i32) - nums.iter().sum::<i32>() as i32
    }
}
// @lc code=end
