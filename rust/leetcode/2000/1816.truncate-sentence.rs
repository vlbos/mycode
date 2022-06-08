/*
 * @lc app=leetcode id=1816 lang=rust
 *
 * [1816] Truncate Sentence
 */

// @lc code=start
impl Solution {
    pub fn truncate_sentence(s: String, k: i32) -> String {
        let v = s.split_ascii_whitespace().collect::<Vec<&str>>();
        v[..k as usize].join(" ")
    }
}
// @lc code=end

