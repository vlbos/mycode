/*
 * @lc app=leetcode id=878 lang=rust
 *
 * [878] Nth Magical Number
 */

// @lc code=start
impl Solution {
    pub fn nth_magical_number(n: i32, a: i32, b: i32) -> i32 {
        let (n, a, b) = (n as i64, a as i64, b as i64);
        fn gcd(a: i64, b: i64) -> i64 {
            if b == 0 {
                a
            } else {
                gcd(b, a % b)
            }
        }
        let l = a / gcd(a, b) * b;

        let p = 10i64.pow(9) + 7;
        let mut low = 0;
        let mut high = 10i64.pow(15);
        while low < high {
            let mid = (low + high) / 2;
            if mid / a + mid / b - mid / l < n {
                low = mid + 1;
            } else {
                high = mid;
            }
        }
        (low % p) as _
    }
}
// @lc code=end
