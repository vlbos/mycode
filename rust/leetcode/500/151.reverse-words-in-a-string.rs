/*
 * @lc app=leetcode id=151 lang=rust
 *
 * [151] Reverse Words in a String
 */

// @lc code=start
impl Solution {
    pub fn reverse_words(s: String) -> String {
        let mut sv = s.split_whitespace().collect::<Vec<&str>>();
        sv.reverse();
        sv.join(" ")
    }
}
// @lc code=end

