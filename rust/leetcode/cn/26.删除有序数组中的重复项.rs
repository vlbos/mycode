/*
 * @lc app=leetcode.cn id=26 lang=rust
 *
 * [26] 删除有序数组中的重复项
 */

// @lc code=start
impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let mut len = 0;
        for i in 0..nums.len() {
            if len > 0 {
                if (nums[len - 1] != nums[i]) {
                    if (len != i) {
                        nums[len] = nums[i];
                    }
                    len += 1;
                }
            } else {
                len += 1;
            }
        }
        len as i32
    }
}
// @lc code=end
