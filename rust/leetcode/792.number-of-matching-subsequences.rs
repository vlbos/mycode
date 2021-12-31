/*
 * @lc app=leetcode id=792 lang=rust
 *
 * [792] Number of Matching Subsequences
 */

// @lc code=start
impl Solution {
    pub fn num_matching_subseq(s: String, words: Vec<String>) -> i32 {
        let mut heads = std::collections::HashMap::new();
        for w in &words {
            heads
                .entry(w.bytes().nth(0).unwrap())
                .or_insert(Vec::new())
                .push((w.clone(), 0));
        }
        let mut ans = 0;
        for c in s.bytes() {
            if let Some(h) = heads.get_mut(&c) {
                let old = h.clone();
                *h = Vec::new();
                for n in &old {
                    let index = n.1 + 1;
                    if index == n.0.len() {
                        ans += 1;
                    } else {
                        let w = n.0.clone();
                        heads
                            .entry(w.bytes().nth(index).unwrap())
                            .or_insert(Vec::new())
                            .push((w, index));
                    }
                }
            }
        }
        ans
    }
}
// @lc code=end
