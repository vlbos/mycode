/*
 * @lc app=leetcode id=476 lang=rust
 *
 * [476] Number Complement
 */

// @lc code=start
impl Solution {
    pub fn find_complement(num: i32) -> i32 {
        let mut n = num;
        let mut r = 0;
        while n > 0 {
            r += 1;
            n >>= 1;
        }
        !num & ((1 << (r - 1)) - 1)
    }
}
// @lc code=end

