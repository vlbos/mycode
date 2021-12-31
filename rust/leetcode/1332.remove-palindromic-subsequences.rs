/*
 * @lc app=leetcode id=1332 lang=rust
 *
 * [1332] Remove Palindromic Subsequences
 */

// @lc code=start
impl Solution {
    pub fn remove_palindrome_sub(s: String) -> i32 {
        if s.is_empty() {
            return 0;
        }
        let ss = s.chars().collect::<Vec<char>>();
        for i in 0..ss.len() / 2 {
            if ss[i] != ss[ss.len() - 1 - i] {
                return 2;
            }
        }
        1
    }
}
// @lc code=end
