/*
 * @lc app=leetcode id=1221 lang=rust
 *
 * [1221] Split a String in Balanced Strings
 */

// @lc code=start
impl Solution {
    pub fn balanced_string_split(s: String) -> i32 {
        let mut r = 0;
        let mut l = 0;
        let mut ans = 0;
        for c in s.chars() {
            if c == 'L' {
                l += 1;
            }
            if c == 'R' {
                r += 1;
            }
            if l > 0 && r > 0 && l == r {
                ans += 1;
                l = 0;
                r = 0;
            }
        }
        ans
    }
}
// @lc code=end
