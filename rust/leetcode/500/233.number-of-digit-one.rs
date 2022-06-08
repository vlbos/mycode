/*
 * @lc app=leetcode id=233 lang=rust
 *
 * [233] Number of Digit One
 */

// @lc code=start
impl Solution {
    pub fn count_digit_one(n: i32) -> i32 {
        let mut mul_k = 1;
        let mut ans = 0;
        let n = n as i64;
        while n >= mul_k {
            ans += (n / (mul_k * 10)) * mul_k + 0i64.max((n % (mul_k * 10)) - mul_k + 1).min(mul_k);
            mul_k *= 10;
        }
        ans as _
    }
}
// @lc code=end
