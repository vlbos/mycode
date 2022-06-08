/*
 * @lc app=leetcode id=953 lang=rust
 *
 * [953] Verifying an Alien Dictionary
 */

// @lc code=start
impl Solution {
    pub fn is_alien_sorted(words: Vec<String>, order: String) -> bool {
        let cmp = |str1: String, str2: String| -> bool {
            if str1 == str2 {
                return false;
            }
            let len = str1.len().min(str2.len());
            for i in 0..len {
                let s1 = str1.chars().nth(i).unwrap();
                let s2 = str2.chars().nth(i).unwrap();
                if s1 != s2 {
                    return order.find(s1).unwrap() > order.find(s2).unwrap();
                }
            }
            str1.len() > str2.len()
        };
        for i in 1..words.len() {
            if cmp(words[i - 1].clone(), words[i].clone()) {
                return false;
            }
        }
        true
    }
}
// @lc code=end

