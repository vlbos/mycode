/*
 * @lc app=leetcode id=1544 lang=rust
 *
 * [1544] Make The String Great
 */

// @lc code=start
impl Solution {
    pub fn make_good(s: String) -> String {
        let mut ans = Vec::new();
        for c in s.chars() {
            if ans.is_empty() {
                ans.push(c);
            } else {
                let l = ans[ans.len() - 1];
                if l.to_ascii_lowercase() == c.to_ascii_lowercase()
                    && (l.is_ascii_lowercase() && c.is_ascii_uppercase()
                        || c.is_ascii_lowercase() && l.is_ascii_uppercase())
                {
                    ans.pop();
                }else{
                    ans.push(c);
                }
            }
        }
        ans.iter().collect::<String>()
    }
}
// @lc code=end
