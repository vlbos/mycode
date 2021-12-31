/*
 * @lc app=leetcode id=678 lang=rust
 *
 * [678] Valid Parenthesis String
 */

// @lc code=start
impl Solution {
    pub fn check_valid_string(s: String) -> bool {
        let mut maxcount = 0;
        let mut mincount = 0;
        for c in s.chars() {
            if c == '(' {
                mincount += 1;
                maxcount += 1;
            } else if c == ')' {
                mincount = (mincount - 1).max(0);
                maxcount -= 1;
                if maxcount < 0 {
                    return false;
                }
            } else {
                mincount = (mincount - 1).max(0);
                maxcount += 1;
            }
        }
        mincount == 0
    }
}
// @lc code=end
