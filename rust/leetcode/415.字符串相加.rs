/*
 * @lc app=leetcode.cn id=415 lang=rust
 *
 * [415] 字符串相加
 */

// @lc code=start
impl Solution {
    pub fn add_strings(num1: String, num2: String) -> String {
        let n1 = num1.into_bytes();
        let n2 = num2.into_bytes();
        let mut p = 0;
        let mut r = String::new();
        let mut i1 = (n1.len() - 1) as i64;
        let mut i2 = (n2.len() - 1) as i64;
        let mut m = 0;
        while i1 >= 0 && i2 >= 0 {
            m = (n1[i1 as usize] - '0' as u8) + (n2[i2 as usize] - '0' as u8) + p;
            p = m / 10;
            r.insert_str(0, &(m % 10).to_string());
            i1 -= 1;
            i2 -= 1;
        }
        while i1 >= 0 {
            m = (n1[i1 as usize] - '0' as u8) + p;
            p = m / 10;
            r.insert_str(0, &(m % 10).to_string());
            i1 -= 1;
        }

        while i2 >= 0 {
            m = (n2[i2 as usize] - '0' as u8) + p;
            p = m / 10;
            r.insert_str(0, &(m % 10).to_string());
            i2 -= 1;
        }
        if p > 0 {
            r.insert_str(0, &p.to_string());
        }

        r
    }
}
// @lc code=end
