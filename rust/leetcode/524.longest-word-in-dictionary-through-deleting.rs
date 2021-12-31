/*
 * @lc app=leetcode id=524 lang=rust
 *
 * [524] Longest Word in Dictionary through Deleting
 */

// @lc code=start
impl Solution {
    pub fn find_longest_word(s: String, dictionary: Vec<String>) -> String {
        let mut d = dictionary;
        d.sort_by(|a, b| {
            if b.len() < a.len() {
                std::cmp::Ordering::Less
            } else if b.len() == a.len() {
                if a < b {
                    std::cmp::Ordering::Less
                } else {
                    std::cmp::Ordering::Greater
                }
            } else {
                std::cmp::Ordering::Greater
            }
        });
        for dd in d {
            let ddd = dd.chars().collect::<Vec<char>>();
            let mut i = 0;
            for c in s.chars() {
                if i == ddd.len() {
                    break;
                }
                if ddd[i] == c {
                    i += 1;
                }
            }
            if i == dd.len() {
                return dd;
            }
        }
        String::new()
    }
}
// @lc code=end
