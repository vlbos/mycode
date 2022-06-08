/*
 * @lc app=leetcode id=1147 lang=rust
 *
 * [1147] Longest Chunked Palindrome Decomposition
 */

// @lc code=start
impl Solution {
    pub fn longest_decomposition(text: String) -> i32 {
        let n = text.len();
        for i in 1..=(n >> 1) {
            if text[0..i] == text[n - i..] {
                return 2 + Self::longest_decomposition(text[i..n - i].to_string());
            }
        }
        if n > 0 {
            1
        } else {
            0
        }
    }
}
// @lc code=end
