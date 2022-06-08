/*
 * @lc app=leetcode id=1163 lang=rust
 *
 * [1163] Last Substring in Lexicographical Order
 */

// @lc code=start
impl Solution {
    pub fn last_substring(s: String) -> String {
        let n = s.len();
        let bs = s.as_bytes();
        let (mut l, mut r, mut k) = (0, 1, 0);
        while r + k < n {
            if bs[l + k] == bs[r + k] {
                k += 1;
            } else if bs[l] < bs[r + k] {
                l = r + k;
                r = l + 1;
                k = 0;
            } else if bs[l + k] < bs[r + k] {
                l = r;
                r += 1;
                k = 0;
            } else {
                r += 1;
                k = 0;
            }
        }
        s[l..].to_string()
    }
}
// @lc code=end
