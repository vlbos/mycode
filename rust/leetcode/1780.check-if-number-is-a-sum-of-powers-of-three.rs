/*
 * @lc app=leetcode id=1780 lang=rust
 *
 * [1780] Check if Number is a Sum of Powers of Three
 */

// @lc code=start
impl Solution {
    pub fn check_powers_of_three(n: i32) -> bool {
        let mut n = n;
        while n > 0 {
            let d = n % 3;
            if d == 2 {
                return false;
            }
            n /= 3;
        }
        true
    }
}
// @lc code=end
