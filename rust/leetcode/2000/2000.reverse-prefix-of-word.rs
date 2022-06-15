/*
 * @lc app=leetcode id=2000 lang=rust
 *
 * [2000] Reverse Prefix of Word
 */

// @lc code=start
impl Solution {
    pub fn reverse_prefix(word: String, ch: char) -> String {
        if let Some(i) = word.chars().position(|x| x == ch) {
            word[..=i].chars().rev().collect::<String>() + &word[i + 1..]
        } else {
            word
        }
    }
}
// @lc code=end
