/*
 * @lc app=leetcode id=2085 lang=rust
 *
 * [2085] Count Common Words With One Occurrence
 */

// @lc code=start
impl Solution {
    pub fn count_words(words1: Vec<String>, words2: Vec<String>) -> i32 {
        use std::collections::HashMap;
        let (mut cnt1, mut cnt2) = (HashMap::new(), HashMap::new());
        for w in &words1 {
            *cnt1.entry(w.clone()).or_insert(0) += 1;
        }
        for w in &words2 {
            *cnt2.entry(w.clone()).or_insert(0) += 1;
        }
        let mut ans = 0;
        for w in &words1 {
            if *cnt1.get(w).unwrap_or(&0) == 1 && *cnt2.get(w).unwrap_or(&0) == 1 {
                ans += 1;
            }
        }
        ans
    }
}
// @lc code=end
