/*
 * @lc app=leetcode id=2116 lang=rust
 *
 * [2116] Check if a Parentheses String Can Be Valid
 */

// @lc code=start
impl Solution {
    pub fn can_be_valid(s: String, locked: String) -> bool {
        let (mut mx, mut mn) = (0, 0);
        let blocked = locked.as_bytes();
        for (i, c) in s.chars().enumerate() {
            if blocked[i] == b'1' {
                let d = if c == '(' { 1 } else { -1 };
                mx += d;
                mn = (mn + d).max(((i + 1) % 2) as i32);
            } else {
                mx += 1;
                mn = (mn - 1).max(((i + 1) % 2) as i32);
            }
            if mx < mn {
                return false;
            }
        }
        mn == 0
    }
}
// @lc code=end
