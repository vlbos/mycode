/*
 * @lc app=leetcode id=187 lang=rust
 *
 * [187] Repeated DNA Sequences
 */

// @lc code=start
impl Solution {
    pub fn find_repeated_dna_sequences(s: String) -> Vec<String> {
        if s.len() < 10 {
            return Vec::new();
        }
        let mut ss = std::collections::HashSet::new();
        let sv = s.chars().collect::<Vec<char>>();
        let mut ans = Vec::new();
        for i in 10..=sv.len() {
            let t = sv[i - 10..i].iter().collect();
            if ss.contains(&t) {
                if  !ans.contains(&t) {
                    ans.push(t);
                }
            } else {
                ss.insert(t);
            }
        }
        ans
    }
}
// @lc code=end
