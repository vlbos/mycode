/*
 * @lc app=leetcode id=961 lang=rust
 *
 * [961] N-Repeated Element in Size 2N Array
 */

// @lc code=start
impl Solution {
    pub fn repeated_n_times(nums: Vec<i32>) -> i32 {
         for i in 1..=3 {
            for j in 0..nums.len() - i {
                if nums[j] == nums[j + i] {
                    return nums[j];
                }
            }
        }
        0
    }
}
// @lc code=end

