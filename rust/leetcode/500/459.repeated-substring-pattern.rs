/*
 * @lc app=leetcode id=459 lang=rust
 *
 * [459] Repeated Substring Pattern
 */

// @lc code=start
impl Solution {
    pub fn repeated_substring_pattern(s: String) -> bool {
         let mut ss = s.clone();
        ss = ss.repeat(2);
        ss.remove(0);
        ss.remove(ss.len() - 1);
        ss.contains(&s)
    }
}
// @lc code=end

