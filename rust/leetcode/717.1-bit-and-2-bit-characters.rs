/*
 * @lc app=leetcode id=717 lang=rust
 *
 * [717] 1-bit and 2-bit Characters
 */

// @lc code=start
impl Solution {
    pub fn is_one_bit_character(bits: Vec<i32>) -> bool {
         let mut i = 0;
        while i < bits.len() - 1 {
            i += bits[i] as usize + 1;
        }
        i == bits.len() - 1
    }
}
// @lc code=end

