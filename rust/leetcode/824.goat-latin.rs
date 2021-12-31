/*
 * @lc app=leetcode id=824 lang=rust
 *
 * [824] Goat Latin
 */

// @lc code=start
impl Solution {
    pub fn to_goat_latin(sentence: String) -> String {
        let s = sentence.split_ascii_whitespace().collect::<Vec<&str>>();
        let v = vec!['a', 'e', 'i', 'o', 'u', 'A', 'E', 'I', 'O', 'U'];
        let mut r = String::new();
        for (i, w) in s.iter().enumerate() {
            let p = (*w).chars().next().unwrap();
            if !r.is_empty() {
                r += " ";
            }
            r += &(if v.contains(&p) {
                (*w).to_owned()
            } else {
                w[1..].to_owned() + &w[0..1]
            } + "ma"
                + &"a".repeat(i + 1));
        }
        r
    }
}
// @lc code=end

