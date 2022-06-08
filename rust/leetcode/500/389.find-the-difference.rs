/*
 * @lc app=leetcode id=389 lang=rust
 *
 * [389] Find the Difference
 */

// @lc code=start
impl Solution {
    pub fn find_the_difference(s: String, t: String) -> char {
         let mut r = 0;
        for c in s.chars() {
            r ^= c as u8;
        }
        for c in t.chars() {
            r ^= c as u8;
        }
        r as char
    }
}
// @lc code=end

