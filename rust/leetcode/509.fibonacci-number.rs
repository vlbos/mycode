/*
 * @lc app=leetcode id=509 lang=rust
 *
 * [509] Fibonacci Number
 */

// @lc code=start
impl Solution {
    pub fn fib(n: i32) -> i32 {
        if n < 2 {
            return n;
        }
        Self::fib(n - 1) + Self::fib(n - 2)
    }
}
// @lc code=end

