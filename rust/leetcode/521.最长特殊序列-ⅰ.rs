/*
 * @lc app=leetcode.cn id=521 lang=rust
 *
 * [521] 最长特殊序列 Ⅰ
 */

// @lc code=start
impl Solution {
    pub fn find_lu_slength(a: String, b: String) -> i32 {
        if a==b{
        return -1;
        }
        a.len().max(b.len()) as i32
    }
}
// @lc code=end

