/*
 * @lc app=leetcode id=880 lang=rust
 *
 * [880] Decoded String at Index
 */

// @lc code=start
impl Solution {
    pub fn decode_at_index(s: String, k: i32) -> String {
        let mut size = s.chars().fold(0, |mut sum, c| {
            if c.is_ascii_digit() {
                sum *= (c as u8 - b'0') as i64
            } else {
                sum += 1;
            };
            sum
        });
        let mut k =  k as i64;

        for c in s.chars().rev() {
            k %= size;
            if k == 0 && c.is_ascii_alphabetic() {
                return c.to_string();
            }
            if c.is_ascii_digit() {
                size /= (c as u8 - b'0') as i64
            } else {
                size -= 1;
            }
        }
        String::new()
    }
}
// @lc code=end
