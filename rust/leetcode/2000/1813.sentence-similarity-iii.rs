/*
 * @lc app=leetcode id=1813 lang=rust
 *
 * [1813] Sentence Similarity III
 */

// @lc code=start
impl Solution {
    pub fn are_sentences_similar(sentence1: String, sentence2: String) -> bool {
        if sentence1 == sentence2 {
            return true;
        }
        let s1: Vec<&str> = sentence1.split_ascii_whitespace().collect();
        let s2: Vec<&str> = sentence2.split_ascii_whitespace().collect();
        let (mut i, mut j) = (0, 0);
        let len = s1.len().min(s2.len());
        while i < len && s1[i] == s2[i] {
            i += 1;
        }
        while j < len && s1[s1.len() - j - 1] == s2[s2.len() - j - 1] {
            j += 1;
        }
        i + j >= len
    }
}
// @lc code=end
