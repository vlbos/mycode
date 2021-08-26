/*
 * @lc app=leetcode.cn id=205 lang=rust
 *
 * [205] 同构字符串
 */

// @lc code=start
impl Solution {
    pub fn is_isomorphic(s: String, t: String) -> bool {
        let mut h: std::collections::HashMap<char, char> = std::collections::HashMap::new();
        let mut hr: std::collections::HashMap<char, char> = std::collections::HashMap::new();
        let mut tc = t.chars();
        for c in s.chars() {
            let ct = tc.next();
            match (h.get(&c), hr.get(&ct.unwrap())) {
                (Some(_c), Some(_ct)) => {
                    if (*_c) != ct.unwrap() || (*_ct) != c {
                        return false;
                    }
                }
                (None, None) => {
                    h.insert(c, ct.unwrap());
                    hr.insert(ct.unwrap(), c);
                }
                _ => return false,
            }
        }
        true
    }
}
// @lc code=end
