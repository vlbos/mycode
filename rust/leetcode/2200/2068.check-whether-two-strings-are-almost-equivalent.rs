/*
 * @lc app=leetcode id=2068 lang=rust
 *
 * [2068] Check Whether Two Strings are Almost Equivalent
 */

// @lc code=start
impl Solution {
    pub fn check_almost_equivalent(word1: String, word2: String) -> bool {
        let mut cnt = std::collections::HashMap::new();
        for c in word1.chars() {
            *cnt.entry(c).or_insert(0) += 1i32;
        }
        for c in word2.chars() {
            *cnt.entry(c).or_insert(0) -= 1;
        }
        cnt.iter().all(|x| x.1.abs() < 4)
    }
}
// @lc code=end
