/*
 * @lc app=leetcode id=908 lang=rust
 *
 * [908] Smallest Range I
 */

// @lc code=start
impl Solution {
    pub fn smallest_range_i(nums: Vec<i32>, k: i32) -> i32 {
        let mut min = nums[0];
        let mut max = nums[0];
        for v in &nums {
            let vv = *v;
            min = min.min(vv);
            max = max.max(vv);
        }
        (max - min - 2 * k).max(0)
    }
}
// @lc code=end

