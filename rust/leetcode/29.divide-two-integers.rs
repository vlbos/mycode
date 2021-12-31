/*
 * @lc app=leetcode id=29 lang=rust
 *
 * [29] Divide Two Integers
 */

// @lc code=start
impl Solution {
    pub fn divide(dividend: i32, divisor: i32) -> i32 {
        if divisor == 1 {
            return dividend;
        }
        if divisor == i32::MIN {
            if dividend == i32::MIN {
                return 1;
            } else {
                return 0;
            }
        }
        if divisor == -1 {
            if dividend < 0 && dividend < -i32::MAX {
                return i32::MAX;
            }

            return -dividend;
        }
        let mut ans = 0;
        let mut d = dividend;
        if dividend == i32::MIN {
            if dividend > 0 && divisor < 0 || dividend < 0 && divisor > 0 {
                d = dividend + divisor;
                if (dividend < 0 && d < 0) || (dividend > 0 && d > 0) {
                    ans += 1;
                }
            } else {
                d = dividend - divisor;
                if (dividend < 0 && d < 0) || (dividend > 0 && d > 0) {
                    ans += 1;
                }
            }
        }
        let mut signed = if dividend > 0 && divisor < 0 || dividend < 0 && divisor > 0 {
            -1
        } else {
            1
        };
        let mut d = d.abs();
        let mut dd = divisor.abs();
        if d < dd {
            return ans * signed;
        }

        while d >= dd {
            d -= dd;
            ans += 1;
        }
        ans * signed
    }
}
// @lc code=end
