/*
 * @lc app=leetcode.cn id=696 lang=rust
 *
 * [696] 计数二进制子串
 */

// @lc code=start
impl Solution {
    pub fn count_binary_substrings(s: String) -> i32 {
        let mut r = 0;
        let sb = s.clone().into_bytes();
        let mut i = 0;
        while i < sb.len() - 1 {
            if sb[i] != sb[i + 1] {
                r += 1;
                let mut j = 1;
                while i >= j && i + 1 + j < sb.len() {
                    if sb[i - j] == sb[i] && sb[i + 1] == sb[i + 1 + j] {
                        r += 1;
                    } else {
                        break;
                    }
                    j += 1;
                }
            }
            i += 1;
        }
        r
    }
}
// @lc code=end
