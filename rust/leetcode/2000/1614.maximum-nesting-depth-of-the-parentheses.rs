/*
 * @lc app=leetcode id=1614 lang=rust
 *
 * [1614] Maximum Nesting Depth of the Parentheses
 */

// @lc code=start
impl Solution {
    pub fn max_depth(s: String) -> i32 {
        let mut stack = 0;
        let mut max = 0;
        for c in s.chars() {
            if c == '(' {
                stack += 1;
            } else if c == ')' {
                if stack > 0 {
                    stack -= 1;
                    max = max.max(stack + 1);
                }
            }
        }
        max
    }
}
// @lc code=end
