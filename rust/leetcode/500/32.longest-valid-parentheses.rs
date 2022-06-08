/*
 * @lc app=leetcode id=32 lang=rust
 *
 * [32] Longest Valid Parentheses
 */

// @lc code=start
impl Solution {
    pub fn longest_valid_parentheses(s: String) -> i32 {
        let (mut l, mut r) = (0, 0);
        let mut ans = 0;
        for c in s.chars() {
            if c == '(' {
                l += 1;
            } else {
                r += 1;
                if l == r {
                    ans = ans.max(2 * l);
                } else if r > l {
                    l = 0;
                    r = 0;
                }
            }
        }
        l = 0;
        r = 0;
        for c in s.chars().rev() {
            if c == ')' {
                r += 1;
            } else {
                l += 1;
                if l == r {
                    ans = ans.max(2 * l);
                } else if l > r {
                    l = 0;
                    r = 0;
                }
            }
        }
        ans
    }
}
// @lc code=end
