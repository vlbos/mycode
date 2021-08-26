/*
 * @lc app=leetcode.cn id=896 lang=rust
 *
 * [896] 单调数列
 */

// @lc code=start
impl Solution {
    pub fn is_monotonic(nums: Vec<i32>) -> bool {
        if nums.len() < 2 {
            return true;
        }
        let mut b = nums[0] - nums[1];
        for i in 2..nums.len() {
            if b != 0 {
                break;
            }
            b = nums[i - 1] - nums[i];
        }
        if b == 0 {
            return true;
        }

        for i in 1..nums.len() {
            if nums[i - 1] != nums[i] && (b > 0) != (nums[i - 1] > nums[i]) {
                return false;
            }
        }
        true
    }
}
// @lc code=end
