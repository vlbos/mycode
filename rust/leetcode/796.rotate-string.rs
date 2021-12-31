/*
 * @lc app=leetcode id=796 lang=rust
 *
 * [796] Rotate String
 */

// @lc code=start
impl Solution {
    pub fn rotate_string(s: String, goal: String) -> bool {
         for i in 1..s.len() {
            if goal == String::from(&s[i..]) + &s[0..i] {
                return true;
            }
        }
        false
    }
}
// @lc code=end

