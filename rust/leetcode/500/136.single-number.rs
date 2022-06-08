/*
 * @lc app=leetcode id=136 lang=rust
 *
 * [136] Single Number
 */

// @lc code=start
impl Solution {
    pub fn single_number(nums: Vec<i32>) -> i32 {
                let mut r: i32 = 0;
        for i in nums {
            r ^= i;
        }
        r
    }
}
// @lc code=end

