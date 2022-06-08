/*
 * @lc app=leetcode id=1863 lang=rust
 *
 * [1863] Sum of All Subset XOR Totals
 */

// @lc code=start
impl Solution {
    pub fn subset_xor_sum(nums: Vec<i32>) -> i32 {
        let mut res = 0;
        for n in &nums {
            res |= *n;
        }
        res << nums.len() - 1
    }
}
// @lc code=end
