/*
 * @lc app=leetcode id=744 lang=rust
 *
 * [744] Find Smallest Letter Greater Than Target
 */

// @lc code=start
impl Solution {
    pub fn next_greatest_letter(letters: Vec<char>, target: char) -> char {
        let t = target as u8;
        for c in &letters {
            let n = *c as u8;
            if n > t {
                return n as char;
            }
        }
        letters[0]
    }
}
// @lc code=end

