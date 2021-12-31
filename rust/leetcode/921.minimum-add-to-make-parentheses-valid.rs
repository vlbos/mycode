/*
 * @lc app=leetcode id=921 lang=rust
 *
 * [921] Minimum Add to Make Parentheses Valid
 */

// @lc code=start
impl Solution {
    pub fn min_add_to_make_valid(s: String) -> i32 {
        let mut l = 0;
        let mut ans = 0;
        for c in s.chars() {
            if c == ')' {
                if l == 0 {
                    ans += 1;
                } else {
                    l -= 1;
                }
            } else {
                l += 1;
            }
        }
        ans + l
    }
}
// @lc code=end
