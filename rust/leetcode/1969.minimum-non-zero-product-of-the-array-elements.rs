/*
 * @lc app=leetcode id=1969 lang=rust
 *
 * [1969] Minimum Non-Zero Product of the Array Elements
 */

// @lc code=start
impl Solution {
    pub fn min_non_zero_product(p: i32) -> i32 {
        let p21 = (1i64 << p);
        let inf = 1000_000_007;
        let mut ans = (p21 - 1) % inf;
        let mut k = (1i64 << (p - 1) )- 1;
        let mut m = (p21 - 2) % inf;
        while k > 0 {
            if (k & 1) > 0 {
                ans = (ans * m) % inf;
            }
            k >>= 1;
            m = (m * m) % inf;
        }
        ans as _
    }
}
// @lc code=end
