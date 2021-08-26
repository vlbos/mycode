/*
 * @lc app=leetcode.cn id=557 lang=rust
 *
 * [557] 反转字符串中的单词 III
 */

// @lc code=start
impl Solution {
    pub fn reverse_words(s: String) -> String {
        let mut t = String::new();
        let mut r = String::new();
        for c in s.chars() {
            if c == ' ' {
                if !t.is_empty() {
                    if !r.is_empty() {
                        r.push(' ');
                    }
                    r.push_str(&t);
                    t = String::new();
                }
            } else {
                t.insert(0, c);
            }
        }
        if !t.is_empty() {
            if !r.is_empty() {
                r.push(' ');
            }
            r.push_str(&t);
            t = String::new();
        }
        r
    }
}
// @lc code=end
