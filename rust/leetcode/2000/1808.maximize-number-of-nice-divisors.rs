/*
 * @lc app=leetcode id=1808 lang=rust
 *
 * [1808] Maximize Number of Nice Divisors
 */

// @lc code=start
impl Solution {
    pub fn max_nice_divisors(prime_factors: i32) -> i32 {
       let n = prime_factors;
        if n <= 3 {
            return n;
        }
        let mut ans = 0;
        let (quotient, remainder) = (n / 3, n % 3);
        let mypow = |mut x: i64, n: i32| {
            let mut ans = 1;
            let mut n = n.abs();
            while n > 0 {
                if n % 2 > 0 {
                    ans *= x;
                    ans %= 1_000_000_007;
                }
                x *= x;
                x %= 1_000_000_007;
                n /= 2;
            }
            ans
        };
        ((if remainder == 0 {
            mypow(3, quotient)
        } else if remainder == 1 {
            mypow(3, quotient - 1) * 4
        } else {
            mypow(3, quotient) * 2
        }) % 1_000_000_007) as _
    }
}
// @lc code=end
