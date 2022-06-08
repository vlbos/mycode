/*
 * @lc app=leetcode id=1342 lang=rust
 *
 * [1342] Number of Steps to Reduce a Number to Zero
 */

// @lc code=start
impl Solution {
    pub fn number_of_steps(num: i32) -> i32 {
        let mut n = num;
        let mut ans = 0;
        while n > 0 {
            if n % 2 == 0 {
                n /= 2;
            } else {
                n -= 1;
            }
            ans += 1;
        }
        ans
    }
}
// @lc code=end
