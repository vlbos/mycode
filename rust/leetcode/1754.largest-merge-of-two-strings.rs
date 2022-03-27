/*
 * @lc app=leetcode id=1754 lang=rust
 *
 * [1754] Largest Merge Of Two Strings
 */

// @lc code=start
impl Solution {
    pub fn largest_merge(word1: String, word2: String) -> String {
        let mut ans = Vec::new();
        let (w1, w2) = (word1.as_bytes(),word2.as_bytes());
        let (mut i, mut j) = (0, 0);
        while i < w1.len() && j < w2.len() {
            if w1[i..] > w2[j..] {
                ans.push(w1[i]);
                i += 1;
            } else {
                ans.push(w2[j]);
                j += 1;
            }
        }
        if i < w1.len() {
            ans.extend(&w1[i..]);
        } else {
            ans.extend(&w2[j..]);
        }
        String::from_utf8(ans).unwrap()
    }
}
// @lc code=end
