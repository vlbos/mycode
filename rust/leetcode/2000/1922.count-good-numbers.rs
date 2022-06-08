/*
 * @lc app=leetcode id=1922 lang=rust
 *
 * [1922] Count Good Numbers
 */

// @lc code=start
impl Solution {
    pub fn count_good_numbers(n: i64) -> i32 {
        let quick_mul = |x: i64, mut y: i64| -> i64 {
            let mut ans = 1;
            let mut mul = x;
            while y > 0 {
                if y % 2 > 0 {
                    ans *= mul;
                    ans %= 1000_000_007;
                }
                mul *= mul;
                mul %= 1000_000_007;
                y /= 2;
            }
            ans
        };
        ((quick_mul(5, (n + 1) / 2) * quick_mul(4, n / 2)) % 1000_000_007) as _
    }
}
// @lc code=end
