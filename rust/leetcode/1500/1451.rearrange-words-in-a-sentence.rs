/*
 * @lc app=leetcode id=1451 lang=rust
 *
 * [1451] Rearrange Words in a Sentence
 */

// @lc code=start
impl Solution {
    pub fn arrange_words(text: String) -> String {
        let mut v: Vec<(String, usize)> = text
            .split_ascii_whitespace()
            .enumerate()
            .map(|(i, v)| (v.to_string(), i))
            .collect();
        v.sort_by_key(|x| (x.0.len(), x.1));
        v.iter()
            .enumerate()
            .map(|(i, x)| {
                if i == 0 {
                    let mut xx = x.0.clone();
                    let mut c = xx.remove(0);
                    c.make_ascii_uppercase();
                    c.to_string() + xx.as_str()
                } else {
                    x.0.to_ascii_lowercase()
                }
            })
            .collect::<Vec<String>>()
            .join(" ")
    }
}
// @lc code=end
