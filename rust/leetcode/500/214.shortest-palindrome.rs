/*
 * @lc app=leetcode id=214 lang=rust
 *
 * [214] Shortest Palindrome
 */

// @lc code=start
impl Solution {
    pub fn shortest_palindrome(s: String) -> String {
        if s.len() < 2 {
            return s;
        }
        let base = 131;
        let p = 1000_000_007;
        let (mut l, mut r) = (0, 0);
        let mut mul = 1;
        let mut best = s.len();
        for (i, b) in s.bytes().enumerate() {
            l = (l * base + b as i64) % p;
            r = (r + mul * b as i64) % p;
            if l == r {
                best = i;
            }
            mul = (mul * base) % p;
        }
        let mut add = if best == s.len() - 1 {
            String::new()
        } else {
            s[best + 1..].to_string()
        };
        add = add.chars().rev().collect();
        add + s.as_str()
    }
}
// @lc code=end
