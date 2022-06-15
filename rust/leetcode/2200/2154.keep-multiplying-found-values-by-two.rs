/*
 * @lc app=leetcode id=2154 lang=rust
 *
 * [2154] Keep Multiplying Found Values by Two
 */

// @lc code=start
impl Solution {
    pub fn find_final_value(nums: Vec<i32>, original: i32) -> i32 {
        let mut original = original;
        let mut nums = nums;
        nums.sort();
        for &num in &nums {
            if num == original {
                original *= 2;
            }
        }
        original
    }
}
// @lc code=end
