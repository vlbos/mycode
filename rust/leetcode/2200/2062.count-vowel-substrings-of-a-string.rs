/*
 * @lc app=leetcode id=2062 lang=rust
 *
 * [2062] Count Vowel Substrings of a String
 */

// @lc code=start
impl Solution {
    pub fn count_vowel_substrings(word: String) -> i32 {
        let mut ans = 0;
        use std::collections::HashSet;
        let mut vow = HashSet::from(['a', 'e', 'i', 'o', 'u']);
        for i in 0..word.len() {
            let mut cur = HashSet::new();
            for c in word[i..].chars() {
                cur.insert(c);
                if cur == vow {
                    ans += 1;
                }
            }
        }
        ans
    }
}
// @lc code=end
