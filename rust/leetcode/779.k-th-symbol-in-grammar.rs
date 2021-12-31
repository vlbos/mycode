/*
 * @lc app=leetcode id=779 lang=rust
 *
 * [779] K-th Symbol in Grammar
 */

// @lc code=start
impl Solution {
    pub fn kth_grammar(n: i32, k: i32) -> i32 {
        (k-1).count_ones() as i32 %2
    }
}
// @lc code=end

