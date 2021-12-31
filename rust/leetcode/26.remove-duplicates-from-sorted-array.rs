/*
 * @lc app=leetcode id=26 lang=rust
 *
 * [26] Remove Duplicates from Sorted Array
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

