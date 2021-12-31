/*
 * @lc app=leetcode id=819 lang=rust
 *
 * [819] Most Common Word
 */

// @lc code=start
impl Solution {
    pub fn most_common_word(paragraph: String, banned: Vec<String>) -> String {
        let p = paragraph
            .split(&[',', ' ', '.', '!', '\'', ';', '?'][..])
            .into_iter()
            .filter(|s| !s.is_empty())
            .map(|s| s.to_ascii_lowercase())
            .collect::<Vec<String>>();
        let mut m = std::collections::HashMap::<String, i32>::new();
        for w in &p {
            if let Some(c) = m.get_mut(w) {
                *c += 1;
            } else {
                m.insert((*w).clone(), 1);
            }
        }
        let mut max = 0;
        let mut result = String::new();
        for (k, v) in m {
            if !banned.contains(&k) && v > max {
                result = k.to_string();
                max = v;
            }
        }
        result
    }
}
// @lc code=end

