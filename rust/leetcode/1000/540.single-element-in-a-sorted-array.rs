/*
 * @lc app=leetcode id=540 lang=rust
 *
 * [540] Single Element in a Sorted Array
 */

// @lc code=start
impl Solution {
    pub fn single_non_duplicate(nums: Vec<i32>) -> i32 {
        let mut ans = 0;
        for n in &nums{
            ans^=*n;
        }
        ans
    }
}
// @lc code=end

