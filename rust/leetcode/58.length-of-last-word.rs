/*
 * @lc app=leetcode id=58 lang=rust
 *
 * [58] Length of Last Word
 */

// @lc code=start
impl Solution {
    pub fn length_of_last_word(s: String) -> i32 {
         let mut len = 0;
        for c in s.chars().rev() {
            if c == ' ' {
                if (len > 0) {
                    break;
                }
            } else {
                len += 1;
            }
        }
        len
    }
}
// @lc code=end

