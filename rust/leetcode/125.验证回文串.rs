/*
 * @lc app=leetcode.cn id=125 lang=rust
 *
 * [125] 验证回文串
 */

// @lc code=start
impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        let v = s.into_bytes();
        let mut i = 0;
        let mut j = v.len() - 1;
        let mut ci = '0';
        let mut cj = '0';
        while i < j {
            ci = (v[i] as char);
            cj = (v[j] as char);
            while i < j && !ci.is_ascii_alphanumeric() {
                i += 1;
                ci = (v[i] as char);
            }
            while i < j && !cj.is_ascii_alphanumeric() {
                j -= 1;
                cj = (v[j] as char);
            }
            if i < j {
                if ci.to_ascii_lowercase() != cj.to_ascii_lowercase() {
                    return false;
                }
                i += 1;
                j -= 1;
            }
        }
        true
    }
}
// @lc code=end
