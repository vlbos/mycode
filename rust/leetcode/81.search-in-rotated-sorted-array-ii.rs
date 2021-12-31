/*
 * @lc app=leetcode id=81 lang=rust
 *
 * [81] Search in Rotated Sorted Array II
 */

// @lc code=start
impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> bool {
        for n in &nums {
            if *n == target {
                return true;
            }
        }
        false
    }
}
// @lc code=end
