/*
 * @lc app=leetcode id=1961 lang=rust
 *
 * [1961] Check If String Is a Prefix of Array
 */

// @lc code=start
impl Solution {
    pub fn is_prefix_string(s: String, words: Vec<String>) -> bool {
        let mut pre = String::new();
        for w in &words {
            pre.push_str(w);
            if s == pre {
                return true;
            }
        }
        false
    }
}
// @lc code=end
