/*
 * @lc app=leetcode id=479 lang=rust
 *
 * [479] Largest Palindrome Product
 */

// @lc code=start
impl Solution {
    pub fn largest_palindrome(n: i32) -> i32 {
        if n == 1 {
            return 9;
        }
        let max = 10i64.pow(n as u32) - 1;
        let min = 10i64.pow(n as u32 - 1);
        for i in (min..=max).rev() {
            let mut temp = i;
            let mut sum = i;
            while temp > 0 {
                sum = sum * 10 + temp % 10;
                temp /= 10;
            }
            for j in (min..=max).rev() {
                if j * j < sum {
                    break;
                }
                if sum%j == 0 {
                    return (sum % 1337) as _;
                }
            }
        }
        n
    }
}
// @lc code=end
