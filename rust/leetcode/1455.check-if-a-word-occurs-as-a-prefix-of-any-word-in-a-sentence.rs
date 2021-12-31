/*
 * @lc app=leetcode id=1455 lang=rust
 *
 * [1455] Check If a Word Occurs As a Prefix of Any Word in a Sentence
 */

// @lc code=start
impl Solution {
    pub fn is_prefix_of_word(sentence: String, search_word: String) -> i32 {
        let s = sentence.split_ascii_whitespace().collect::<Vec<&str>>();
        for (i,c) in s.iter().enumerate(){
            if c.len()>=search_word.len() && (&c[0..search_word.len()]).to_string()==search_word{
                return i as i32 +1;
            }
        }
        -1
    }
}
// @lc code=end

