/*
 * @lc app=leetcode id=258 lang=rust
 *
 * [258] Add Digits
 */

// @lc code=start
impl Solution {
    pub fn add_digits(num: i32) -> i32 {
                (num - 1) % 9 + 1

    }
}
// @lc code=end

