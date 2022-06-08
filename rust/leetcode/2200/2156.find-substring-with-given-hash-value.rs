/*
 * @lc app=leetcode id=2156 lang=rust
 *
 * [2156] Find Substring With Given Hash Value
 */

// @lc code=start
impl Solution {
    pub fn sub_str_hash(s: String, power: i32, modulo: i32, k: i32, hash_value: i32) -> String {
        let (n, k) = (s.len(), k as usize);
        let (power, modulo, hash_value) = (power as i64, modulo as i64, hash_value as i64);
        let mut pos = 0;
        let mut h = 0;
        let mut mul = 1;
        let bs = s.as_bytes();
        for i in (n - k..n).rev() {
            h = (h * power + (bs[i] - b'a' + 1) as i64) % modulo;
            if i != n - k {
                mul = mul * power % modulo;
            }
        }
        if h == hash_value {
            pos = n - k;
        }
        for i in (0..n - k).rev() {
            h = ((h - (bs[i + k] - b'a' + 1) as i64 * mul % modulo + modulo) * power
                + (bs[i] - b'a' + 1) as i64)
                % modulo;
            if h == hash_value {
                pos = i;
            }
        }
        s[pos..pos + k].to_string()
    }
}
// @lc code=end
