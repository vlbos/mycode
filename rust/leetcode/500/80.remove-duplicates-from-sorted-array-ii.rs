/*
 * @lc app=leetcode id=80 lang=rust
 *
 * [80] Remove Duplicates from Sorted Array II
 */

// @lc code=start
impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        if nums.len() < 3 {
            return nums.len() as i32;
        }
        let mut i = 2;
        let mut j = 2;
        while i < nums.len() {
            if nums[i] != nums[j - 2] {
                nums[j] = nums[i];
                j += 1;
            }
            i += 1;
        }
        j as i32
    }
}
// @lc code=end
