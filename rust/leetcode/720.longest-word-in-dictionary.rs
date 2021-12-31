/*
 * @lc app=leetcode id=720 lang=rust
 *
 * [720] Longest Word in Dictionary
 */

// @lc code=start
impl Solution {
    pub fn longest_word(words: Vec<String>) -> String {
        let mut w = Vec::<String>::new();
        let mut lw = String::new();
        let mut i = 0;
        let mut len = 0;
        while i <= 30 {
            len = w.len();
            for s in &words {
                if (*s).len() == 1
                    || (*s).len() == i && w.contains(&(s[..(*s).len() - 1].to_string()))
                {
                    w.push((*s).to_string());
                    if (*s).len() > lw.len() {
                        lw = (*s).to_string();
                    } else if (*s).len() == lw.len() && (*s) < lw {
                        lw = (*s).to_string();
                    }
                }
            }
            if len == w.len() {
                break;
            }
            i += 1;
        }
        lw
    }
}
// @lc code=end

