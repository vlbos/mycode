/*
 * @lc app=leetcode.cn id=459 lang=rust
 *
 * [459] 重复的子字符串
 */

// @lc code=start
impl Solution {
    pub fn repeated_substring_pattern(s: String) -> bool {
               let mut ss = s.clone();
                ss=ss.repeat(2);
                ss.remove(0);
                ss.remove(ss.len()-1);
                ss.contains(&s)
    }
}
// @lc code=end

