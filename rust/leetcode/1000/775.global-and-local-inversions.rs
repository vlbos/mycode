/*
 * @lc app=leetcode id=775 lang=rust
 *
 * [775] Global and Local Inversions
 */

// @lc code=start
impl Solution {
    pub fn is_ideal_permutation(nums: Vec<i32>) -> bool {
        nums.iter().enumerate().all(|(i,v)| (i as i32-v).abs()<2)
    }
}
// @lc code=end

