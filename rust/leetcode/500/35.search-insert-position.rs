/*
 * @lc app=leetcode id=35 lang=rust
 *
 * [35] Search Insert Position
 */

// @lc code=start
impl Solution {
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
         for (i, c) in nums.iter().enumerate() {
            if (*c >= target) {
                return i as i32;
            }
        }
        nums.len() as i32
    }
}
// @lc code=end

