/*
 * @lc app=leetcode id=242 lang=rust
 *
 * [242] Valid Anagram
 */

// @lc code=start
impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        let mut s = s.into_bytes();
        let mut t = t.into_bytes();
        s.sort();
        t.sort();
        s == t
    }
}
// @lc code=end

