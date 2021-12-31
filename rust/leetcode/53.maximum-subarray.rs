/*
 * @lc app=leetcode id=53 lang=rust
 *
 * [53] Maximum Subarray
 */

// @lc code=start
impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        let mut pre = 0;
        let mut max = nums[0];
        for n in &nums {
            pre = (pre + *n).max(*n);
            max = max.max(pre);
        }

        max
    }
}
// @lc code=end
