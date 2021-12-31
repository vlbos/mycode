/*
 * @lc app=leetcode id=268 lang=rust
 *
 * [268] Missing Number
 */

// @lc code=start
impl Solution {
    pub fn missing_number(nums: Vec<i32>) -> i32 {
                (((nums.len() + 1) * nums.len() / 2) as i32) - nums.iter().sum::<i32>() as i32

    }
}
// @lc code=end

