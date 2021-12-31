/*
 * @lc app=leetcode id=858 lang=rust
 *
 * [858] Mirror Reflection
 */

// @lc code=start
impl Solution {
    pub fn mirror_reflection(p: i32, q: i32) -> i32 {
        fn gcd(p: i32, q: i32) -> i32 {
            if p == 0 {
                return q;
            }
            gcd(q % p, p)
        }
        let g = gcd(p, q);
        let mut p = p / g;
        let mut q = q / g;
        p %= 2;
        q %= 2;
        if p == 1 && q == 1 {
            return 1;
        }
        if p == 1 {
            0
        } else {
            2
        }
    }
}
// @lc code=end
