/*
 * @lc app=leetcode.cn id=405 lang=rust
 *
 * [405] 数字转换为十六进制数
 */

// @lc code=start
impl Solution {
    pub fn to_hex(num: i32) -> String {
        if num == 0 {
            return "0".to_string();
        }
        let mut r = String::new();
        let mut n = if num < 0 { num } else { num };

        let mut i = 0;
        while n != 0 && i < 8 {
            let b = n & 15;
            let c = if b < 10 {
                b.to_string()
            } else {
                (((b - 10) as u8 + 'a' as u8) as char).to_string()
            };
            r = c + &r;
            n >>= 4;
            i += 1;
        }
        r.to_string()
    }
}
// @lc code=end
