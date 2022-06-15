/*
 * @lc app=leetcode id=2160 lang=rust
 *
 * [2160] Minimum Sum of Four Digit Number After Splitting Digits
 */

// @lc code=start
impl Solution {
    pub fn minimum_sum(num: i32) -> i32 {
        let mut digit: Vec<i32> = num.to_string().bytes().map(|b| (b - b'0') as i32).collect();
        digit.sort();
        digit[0] * 10 + digit[2] + digit[1] * 10 + digit[3]
    }
}
// @lc code=end
