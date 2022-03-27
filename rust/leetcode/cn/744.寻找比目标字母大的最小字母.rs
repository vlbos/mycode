/*
 * @lc app=leetcode.cn id=744 lang=rust
 *
 * [744] 寻找比目标字母大的最小字母
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
