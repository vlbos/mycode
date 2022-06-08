/*
 * @lc app=leetcode id=434 lang=rust
 *
 * [434] Number of Segments in a String
 */

// @lc code=start
impl Solution {
    pub fn count_segments(s: String) -> i32 {
         let mut n = 0;
        let mut r = 0;
        for c in s.chars() {
            if c == ' ' {
                if n > 0 {
                    r += 1;
                    n = 0;
                }
            } else {
                n += 1;
            }
        }
        if n > 0 {
            r += 1;
        }
        r
    }
}
// @lc code=end

