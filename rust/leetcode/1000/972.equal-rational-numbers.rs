/*
 * @lc app=leetcode id=972 lang=rust
 *
 * [972] Equal Rational Numbers
 */

// @lc code=start
impl Solution {
    pub fn is_rational_equal(s: String, t: String) -> bool {
       fn gcd(a: i64, b: i64) -> i64 {
            if b == 0 {
                a
            } else {
                gcd(b, a % b)
            }
        }
        let iadd = |a: (i64, i64), b: (i64, i64)| -> (i64, i64) {
            let numerator = a.0 * b.1 + a.1 * b.0;
            let denominator = a.1 * b.1;
            let g = gcd(numerator, denominator);
            (numerator / g, denominator / g)
        };
        let convert = |s: &String| -> (i64, i64) {
            let mut state = 0;
            let mut ans = (0, 1);
            let mut decimal_size = 0;
            for part in s.split(&['.', '(', ')'][..]) {
                state += 1;
                if part.is_empty() {
                    continue;
                }
                let x = part.parse::<i64>().unwrap();
                let sz = part.len();
                if state == 1 {
                    ans = iadd(ans, (x, 1));
                } else if state == 2 {
                    ans = iadd(ans, (x, 10i64.pow(sz as u32)));
                    decimal_size = sz;
                } else {
                    let mut denom = 10i64.pow(decimal_size as u32);
                    denom *= 10i64.pow(sz as u32) - 1;
                    ans = iadd(ans, (x, denom));
                }
            }
            ans
        };
        convert(&s) == convert(&t)
    }
}
// @lc code=end
