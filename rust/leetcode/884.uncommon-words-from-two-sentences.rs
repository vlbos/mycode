/*
 * @lc app=leetcode id=884 lang=rust
 *
 * [884] Uncommon Words from Two Sentences
 */

// @lc code=start
impl Solution {
    pub fn uncommon_from_sentences(s1: String, s2: String) -> Vec<String> {
         let mut s = Vec::new();
        let vs1 = s1
            .split_ascii_whitespace()
            .into_iter()
            .collect::<Vec<&str>>();
        let vs2 = s2
            .split_ascii_whitespace()
            .into_iter()
            .collect::<Vec<&str>>();
        let mut m = std::collections::HashMap::new();
        for v in &vs1 {
            let c = m.entry(*v).or_insert(0);
            *c += 1;
        }
        for v in &vs2 {
            let c = m.entry(*v).or_insert(0);
            *c += 1;
        }
        for (k, v) in m {
            if v == 1 {
                s.push(k.to_string());
            }
        }
        s
    }
}
// @lc code=end

