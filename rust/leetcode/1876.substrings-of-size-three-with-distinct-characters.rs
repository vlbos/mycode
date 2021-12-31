/*
 * @lc app=leetcode id=1876 lang=rust
 *
 * [1876] Substrings of Size Three with Distinct Characters
 */

// @lc code=start
impl Solution {
    pub fn count_good_substrings(s: String) -> i32 {
        if s.len() < 3 {
            return 0;
        }
        let s = s.chars().collect::<Vec<char>>();
        let mut ans = 0;
        for i in 0..s.len() - 2 {
            if s[i] != s[i + 1] && s[i] != s[i + 2] && s[i + 1] != s[i + 2] {
                ans += 1;
            }
        }
        ans
    }
}
// @lc code=end
