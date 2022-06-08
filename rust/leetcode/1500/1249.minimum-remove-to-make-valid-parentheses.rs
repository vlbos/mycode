/*
 * @lc app=leetcode id=1249 lang=rust
 *
 * [1249] Minimum Remove to Make Valid Parentheses
 */

// @lc code=start
impl Solution {
    pub fn min_remove_to_make_valid(s: String) -> String {
        let mut open_seen = 0;
        let mut balance = 0;
        let mut tmp = String::new();
        for c in s.chars() {
            if c == '(' {
                open_seen += 1;
                balance += 1;
            } else if c == ')' {
                if balance == 0 {
                    continue;
                }
                balance -= 1;
            }
            tmp.push(c);
        }
        let mut open_to_keep = open_seen - balance;
        let mut ans = String::new();
        for c in tmp.chars() {
            if c == '(' {
                open_to_keep -= 1;
                if open_to_keep < 0 {
                    continue;
                }
            }
            ans.push(c);
        }
        ans
    }
}
// @lc code=end
