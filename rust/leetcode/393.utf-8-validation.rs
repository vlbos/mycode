/*
 * @lc app=leetcode id=393 lang=rust
 *
 * [393] UTF-8 Validation
 */

// @lc code=start
impl Solution {
    pub fn valid_utf8(data: Vec<i32>) -> bool {
        let mut n = 0;
        let mut bits = vec![0, 6, 14, 30];
        for d in &data {
            if n == 0 {
                n = bits.len();
                for (i, b) in bits.iter().enumerate() {
                    if d >> if i > 0 { 6 - i } else { 7 } == *b {
                        n = i;
                        break;
                    }
                }
                if n == bits.len() {
                    return false;
                }
            } else {
                if d >> 6 != 2 {
                    return false;
                }
                n -= 1;
            }
        }
        n == 0
    }
}
// @lc code=end
