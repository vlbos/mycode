/*
 * @lc app=leetcode id=9 lang=rust
 *
 * [9] Palindrome Number
 */

// @lc code=start
impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        if x < 0 || (x % 10 == 0 && x != 0) {
            return false;
        }
        let mut xx = x;
        let mut result = 0;
        while xx > result {
            result = result * 10 + xx % 10;
            xx /= 10;
        }

        result == xx || xx == result / 10
    }
}
// @lc code=end

