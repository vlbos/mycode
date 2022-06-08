/*
 * @lc app=leetcode id=639 lang=rust
 *
 * [639] Decode Ways II
 */

// @lc code=start
impl Solution {
    pub fn num_decodings(s: String) -> i32 {
       let p = 1000_000_007;
        let check_1digit = |ch: u8| -> i64 {
            if ch == b'0' {
                return 0;
            }
            if ch == b'*' {
                9
            } else {
                1
            }
        };
        let check_2digit = |c0: u8, c1: u8| -> i64 {
            if c0 == b'*' && c1 == b'*' {
                return 15;
            }
            if c0 == b'*' {
                return if c1 <= b'6' { 2 } else { 1 };
            }
            if c1 == b'*' {
                if c0 == b'1' {
                    return 9;
                }
                if c0 == b'2' {
                    return 6;
                }
                return 0;
            }
            if c0 != b'0' && ((c0 - b'0') * 10 + (c1 - b'0')) <= 26 {
                1
            } else {
                0
            }
        };
        let bs = s.as_bytes();
        let (mut a, mut b, mut c) = (0, 1, 0);
        for i in 1..=bs.len() {
            c = (b * check_1digit(bs[i - 1])) % p;
            if i > 1 {
                c = (c + a * check_2digit(bs[i - 2], bs[i - 1])) % p;
            }
            a = b;
            b = c;
        }
        c as _
    }
}
// @lc code=end
