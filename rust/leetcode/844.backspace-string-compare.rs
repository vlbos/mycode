/*
 * @lc app=leetcode id=844 lang=rust
 *
 * [844] Backspace String Compare
 */

// @lc code=start
impl Solution {
    pub fn backspace_compare(s: String, t: String) -> bool {
         let h = |s: String| -> String {
            let mut r = String::new();
            for c in s.chars() {
                if c == '#' {
                    if r.len() > 0 {
                        r = (&r[0..r.len() - 1]).to_string();
                    }
                } else {
                    r += &c.to_string();
                }
            }
            r
        };
        h(s) == h(t)
    }
}
// @lc code=end

