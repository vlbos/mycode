/*
 * @lc app=leetcode id=709 lang=rust
 *
 * [709] To Lower Case
 */

// @lc code=start
impl Solution {
    pub fn to_lower_case(s: String) -> String {
         let mut b = s.into_bytes();
        for i in 0..b.len() {
            if b[i] >= 'A' as u8 && b[i] <= 'Z' as u8 {
                b[i] = 'a' as u8 + b[i] - 'A' as u8;
            }
        }
        String::from_utf8(b).unwrap()
    }
}
// @lc code=end

