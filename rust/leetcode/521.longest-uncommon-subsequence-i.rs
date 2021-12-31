/*
 * @lc app=leetcode id=521 lang=rust
 *
 * [521] Longest Uncommon Subsequence I
 */

// @lc code=start
impl Solution {
    pub fn find_lu_slength(a: String, b: String) -> i32 {
         if a == b {
            return -1;
        }
        a.len().max(b.len()) as i32
    }
}
// @lc code=end

