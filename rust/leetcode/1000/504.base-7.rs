/*
 * @lc app=leetcode id=504 lang=rust
 *
 * [504] Base 7
 */

// @lc code=start
impl Solution {
    pub fn convert_to_base7(num: i32) -> String {
         if num == 0 {
            return "0".to_string();
        }
        let mut n = num.abs();
        let mut r = String::new();
        while n > 0 {
            r.insert(0, ('0' as u8 + (n % 7) as u8) as char);
            n /= 7;
        }
        if num < 0 {
            r.insert(0, '-');
        }
        r
    }
}
// @lc code=end

