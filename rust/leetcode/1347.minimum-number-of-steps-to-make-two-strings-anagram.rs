/*
 * @lc app=leetcode id=1347 lang=rust
 *
 * [1347] Minimum Number of Steps to Make Two Strings Anagram
 */

// @lc code=start
impl Solution {
    pub fn min_steps(s: String, t: String) -> i32 {
        let mut m = std::collections::HashMap::new();
        for c in s.chars() {
            *m.entry(c).or_insert(0) += 1;
        }
        let mut ans = 0;
        for c in t.chars() {
            if let Some(mut i) = m.get_mut(&c) {
                if *i > 0 {
                    *i -= 1;
                } else {
                    ans += 1;
                }
            } else {
                ans += 1;
            }
        }
        ans
    }
}
// @lc code=end
