/*
 * @lc app=leetcode id=2108 lang=rust
 *
 * [2108] Find First Palindromic String in the Array
 */

// @lc code=start
impl Solution {
    pub fn first_palindrome(words: Vec<String>) -> String {
        for w in &words {
            if w.clone() == w.chars().rev().collect::<String>() {
                return w.clone();
            }
        }
        String::new()
    }
}
// @lc code=end
