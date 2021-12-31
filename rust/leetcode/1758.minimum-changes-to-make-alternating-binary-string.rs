/*
 * @lc app=leetcode id=1758 lang=rust
 *
 * [1758] Minimum Changes To Make Alternating Binary String
 */

// @lc code=start
impl Solution {
    pub fn min_operations(s: String) -> i32 {
        let mut one = 0;
        for (i, c) in s.chars().enumerate() {
            let j = c as u8 - '0' as u8;
            let i2 = (i % 2) as u8;
            if j != i2 {
               one += 1;
            }
        }
        (s.len() as i32 -one).min(one)
    }
}
// @lc code=end
