/*
 * @lc app=leetcode id=890 lang=rust
 *
 * [890] Find and Replace Pattern
 */

// @lc code=start
impl Solution {
    pub fn find_and_replace_pattern(words: Vec<String>, pattern: String) -> Vec<String> {
        let matches = |word: &String, pattern: &String| -> bool {
            let mut m = std::collections::HashMap::new();
            let mut m2 = std::collections::HashMap::new();
            for (i, c) in word.chars().enumerate() {
                let p = pattern.chars().nth(i).unwrap();
                if !m.contains_key(&c) {
                    m.insert(c, p);
                }
                if !m2.contains_key(&p) {
                    m2.insert(p, c);
                }
                if *m.get(&c).unwrap() != p || *m2.get(&p).unwrap() != c {
                    return false;
                }
            }
            true
        };
        let mut ans = Vec::new();
        for w in &words {
            if matches(w, &pattern) {
                ans.push(w.clone());
            }
        }
        ans
    }
}
// @lc code=end
