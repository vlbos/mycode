/*
 * @lc app=leetcode id=829 lang=rust
 *
 * [829] Consecutive Numbers Sum
 */

// @lc code=start
impl Solution {
    pub fn consecutive_numbers_sum(n: i32) -> i32 {
        let mut n = n;
        while n & 1 == 0 {
            n >>= 1;
        }
        let mut ans = 1;
        let mut d = 3;
        while d * d <= n {
            let mut e = 0;
            while n % d == 0 {
                n /= d;
                e += 1;
            }
            ans *= e + 1;
            d += 2;
        }
        if n > 1 {
            ans <<= 1;
        }
        ans
    }
}
// @lc code=end
