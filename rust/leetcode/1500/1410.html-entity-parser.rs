/*
 * @lc app=leetcode id=1410 lang=rust
 *
 * [1410] HTML Entity Parser
 */

// @lc code=start
impl Solution {
    pub fn entity_parser(text: String) -> String {
        let mut text = text;
        let ra = vec![vec!["&quot;","\""],vec!["&apos;","'"],vec!["&gt;",">"],vec!["&lt;","<"],vec!["&frasl;","/"],vec!["&amp;","&"]];
        for r in &ra{
            text =text.replace(r[0],r[1]);
        }
        text
    }
}
// @lc code=end

