/*
 * @lc app=leetcode id=27 lang=rust
 *
 * [27] Remove Element
 */

// @lc code=start
impl Solution {
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
         let mut p = nums.len();
        for i in 0..nums.len() {
            if (p == nums.len() && nums[i] == val) {
                p = i;
            } else if (p != nums.len() && nums[i] != val) {
                nums[p] = nums[i];
                p += 1;
            }
        }
        p as i32
    }
}
// @lc code=end

