/*
 * @lc app=leetcode id=1009 lang=rust
 *
 * [1009] Complement of Base 10 Integer
 */

// @lc code=start
impl Solution {
    pub fn bitwise_complement(n: i32) -> i32 {
        if n == 0 {
            return 1;
        }
        let mut n = n;
        let mut an = 0;
        let mut i = 0;
        while n > 0 {
            an = ((n % 2 + 1) % 2) * 2i32.pow(i) + an;
            n /= 2;
            i += 1;
        }
        an
    }
}
// @lc code=end
