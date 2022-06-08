/*
 * @lc app=leetcode id=1930 lang=rust
 *
 * [1930] Unique Length-3 Palindromic Subsequences
 */

// @lc code=start
impl Solution {
    pub fn count_palindromic_subsequence(s: String) -> i32 {
        let n = s.len();
        let mut ans = 0;
        use std::collections::HashSet;
        let bs = s.as_bytes();
        for b in b'a'..=b'z' {
            let (mut l, mut r) = (0, n as i32 - 1);
            while l < n && bs[l] != b {
                l += 1;
            }
            while r >= 0 && bs[r as usize] != b {
                r -= 1;
            }
            if r - (l as i32) < 2 {
                continue;
            }
            let c = bs[l + 1..r as usize].iter().cloned().collect::<HashSet<u8>>();
            ans += c.len() as i32;
        }
        ans
    }
}
// @lc code=end
