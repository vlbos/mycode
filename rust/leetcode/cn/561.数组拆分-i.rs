/*
 * @lc app=leetcode.cn id=561 lang=rust
 *
 * [561] 数组拆分 I
 */

// @lc code=start
impl Solution {
    pub fn array_pair_sum(nums: Vec<i32>) -> i32 {
        let mut n = nums;
        n.sort();
        let mut r = 0;
        for i in (0..n.len()).step_by(2) {
            r += n[i];
        }
        r
    }
}
// @lc code=end
