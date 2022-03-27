/*
 * @lc app=leetcode.cn id=283 lang=rust
 *
 * [283] 移动零
 */

// @lc code=start
impl Solution {
    pub fn move_zeroes(nums: &mut Vec<i32>) {
        let mut i = 0;
        for k in 0..nums.len() {
            if nums[k] != 0 {
                nums[i] = nums[k];
                i += 1;
            }
        }
        for j in i..nums.len() {
            nums[j] = 0;
        }
    }
}
// @lc code=end
