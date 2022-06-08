/*
 * @lc app=leetcode id=1392 lang=rust
 *
 * [1392] Longest Happy Prefix
 */

// @lc code=start
impl Solution {
    pub fn longest_prefix(s: String) -> String {
        let n = s.len();
        let bs = s.as_bytes();
        let mut ans = 0;
        let (mut pre, mut suf) = (0, 0);
        let (base, p) = (31, 1_000_000_007);
        let mut mul = 1;
        for i in 1..n {
            pre = (pre * base + (bs[i - 1] - b'a') as i64) % p;
            suf = (suf + mul * (bs[n - i] - b'a') as i64) % p;
            if pre == suf {
                ans = i;
            }
            mul = (mul * base) % p;
        }
        s[..ans].to_string()
    }
}
// @lc code=end
