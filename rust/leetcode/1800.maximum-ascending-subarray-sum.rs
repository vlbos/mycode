/*
 * @lc app=leetcode id=1800 lang=rust
 *
 * [1800] Maximum Ascending Subarray Sum
 */

// @lc code=start
impl Solution {
    pub fn max_ascending_sum(nums: Vec<i32>) -> i32 {
        let mut sum = 0;
        let mut pre = 0;
        let mut max = 0;
        for n in &nums {
            if *n > pre {
                sum += *n;
            } else {
                max = max.max(sum);
                sum = *n;
            }
            pre = *n;
        }
        max.max(sum)
    }
}
// @lc code=end
