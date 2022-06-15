/*
 * @lc app=leetcode id=2129 lang=rust
 *
 * [2129] Capitalize the Title
 */

// @lc code=start
impl Solution {
    pub fn capitalize_title(title: String) -> String {
            title
            .split_ascii_whitespace()
            .map(|s| {
                let mut s=s.to_string();
                s.make_ascii_lowercase();
                if s.len() > 2 {
                    s[..1].make_ascii_uppercase();s[..1].to_string() + &s[1..]
                } else {
                    s.to_string()
                }
            })
            .collect::<Vec<String>>()
            .join(" ")
    }
}
// @lc code=end
