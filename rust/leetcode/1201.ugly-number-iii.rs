/*
 * @lc app=leetcode id=1201 lang=rust
 *
 * [1201] Ugly Number III
 */

// @lc code=start
impl Solution {
    pub fn nth_ugly_number(n: i32, a: i32, b: i32, c: i32) -> i32 {
        let (n, a, b, c) = (n as i64, a as i64, b as i64, c as i64);
        let (mut l, mut r) = (0, a.min(b).min(c) * n);
        fn gcd(a: i64, b: i64) -> i64 {
            if a == 0 {
                return b;
            }
            gcd(b % a, a)
        }
        let lcm = |a: i64, b: i64| -> i64 { a * b / gcd(a, b) };
        let check = |m: i64| -> bool {
            m / a + m / b + m / c - m / lcm(a, b) - m / lcm(a, c) - m / lcm(b, c)
                + m / lcm(a, lcm(b, c))
                >= n
        };
        while l < r {
            let mid = (l + r) / 2;
            if check(mid) {
                r = mid;
            } else {
                l = mid + 1;
            }
        }
        l as _
    }
}
// @lc code=end
