/*
 * @lc app=leetcode id=2114 lang=rust
 *
 * [2114] Maximum Number of Words Found in Sentences
 */

// @lc code=start
impl Solution {
    pub fn most_words_found(sentences: Vec<String>) -> i32 {
        sentences
            .into_iter()
            .map(|s| s.matches(char::is_whitespace).count() + 1)
            .max()
            .unwrap() as _
    }
}
// @lc code=end
