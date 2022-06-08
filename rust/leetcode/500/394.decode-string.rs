/*
 * @lc app=leetcode id=394 lang=rust
 *
 * [394] Decode String
 */

// @lc code=start
impl Solution {
    pub fn decode_string(s: String) -> String {
        fn dfs(s: &[char], idx: &mut usize) -> String {
            if *idx == s.len() || s[*idx] == ']' {
                return String::new();
            }
            let mut n = 0;
            let mut ss = String::new();
            let mut ans = String::new();
            let mut i = *idx;
            if s[i].is_ascii_digit() {
                while i < s.len() && s[i].is_ascii_digit() {
                    n *= 10;
                    n += (s[i] as u8 - b'0') as usize;
                    i += 1;
                }
                i += 1;
                *idx = i;
                ss = dfs(s, idx);
                i = *idx;
                i += 1;
                ans.push_str(ss.repeat(n).as_str());
            } else if s[i].is_ascii_alphabetic() {
                ans.push(s[i]);
                i += 1;
            }
            *idx = i;
            ans + dfs(s, idx).as_str()
        }
        let sv = s.chars().collect::<Vec<char>>();
        let mut idx = 0;
        dfs(&sv, &mut idx)
    }
}
// @lc code=end
