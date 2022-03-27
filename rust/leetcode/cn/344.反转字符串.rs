/*
 * @lc app=leetcode.cn id=344 lang=rust
 *
 * [344] 反转字符串
 */

// @lc code=start
impl Solution {
    pub fn reverse_string(s: &mut Vec<char>) {
        let mut i = 0;
        let mut j = s.len() - 1;
        let mut c = 'a';
        while i < j {
            c = s[i];
            s[i] = s[j];
            s[j] = c;
            i += 1;
            j -= 1;
        }
    }
}
// @lc code=end
