/*
 * @lc app=leetcode id=1929 lang=rust
 *
 * [1929] Concatenation of Array
 */

// @lc code=start
impl Solution {
    pub fn get_concatenation(nums: Vec<i32>) -> Vec<i32> {
        let mut n = nums.clone();
        n.extend(nums);
        n
    }
}
// @lc code=end

