/*
 * @lc app=leetcode id=966 lang=rust
 *
 * [966] Vowel Spellchecker
 */

// @lc code=start
impl Solution {
    pub fn spellchecker(wordlist: Vec<String>, queries: Vec<String>) -> Vec<String> {
        use std::collections::HashMap;
        use std::collections::HashSet;
        let perfect = wordlist.iter().cloned().collect::<HashSet<String>>();
        let is_vowel = |c: char| -> bool { "aeiou".find(c).is_some() };
        let devowel = |s: &String| -> String {
            let mut ans = String::new();
            for c in s.chars() {
                ans.push(if is_vowel(c) { '*' } else { c });
            }
            ans
        };
        let mut lm = HashMap::new();
        let mut dm = HashMap::new();

        for w in &wordlist {
            lm.entry(w.to_ascii_lowercase()).or_insert(w.clone());
            dm.entry(devowel(&w.to_ascii_lowercase())).or_insert(w.clone());
        }
        let solve = |s: &String| -> String {
            if let Some(a) = perfect.get(s) {
                return a.clone();
            }
            if let Some(a) = lm.get(&s.to_ascii_lowercase()) {
                return a.clone();
            }
            if let Some(a) = dm.get(&devowel(&s.to_ascii_lowercase())) {
                return a.clone();
            }
            String::new()
        };
        let mut ans = Vec::new();
        for w in &queries {
            ans.push(solve(w));
        }
        ans
    }
}
// @lc code=end
