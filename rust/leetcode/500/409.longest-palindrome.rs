/*
 * @lc app=leetcode id=409 lang=rust
 *
 * [409] Longest Palindrome
 */

// @lc code=start
impl Solution {
    pub fn longest_palindrome(s: String) -> i32 {
        let mut m = std::collections::HashMap::<char, i32>::new();
        let mut r = 0;
        let mut k = 0;
        for c in s.chars() {
            if let Some(_m) = m.get_mut(&c) {
                *_m += 1;
            } else {
                m.insert(c, 1);
            }
        }
        for (_, n) in m {
            r += n / 2;
            if n % 2 > 0 {
                k = 1;
            }
        }
        r * 2 + k
    }
}
// @lc code=end

