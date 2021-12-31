/*
 * @lc app=leetcode id=372 lang=rust
 *
 * [372] Super Pow
 */

// @lc code=start
impl Solution {
    pub fn super_pow(a: i32, b: Vec<i32>) -> i32 {
        if a == 1 || b.is_empty()||b[0] == 0 {
            return 1;
        }
        let base = 1337;
        let fast_pow = |p: i32, q: i32| -> i32 {
            let mut ans = 1;
            let mut p = p % base;
            let mut q = q;
            while q > 0 {
                if q % 2 != 0 {
                    ans = ((ans % base) * (p % base)) % base;
                }
                p = p.pow(2) % base;
                q >>= 1;
            }
            ans
        };
        let n = b.len();
        let a = a % base;
        let p1 = fast_pow(a, *b.last().unwrap());
        let p2 = fast_pow(Self::super_pow(a, b[..n - 1].to_vec()), 10) % base;
        ((p1 % base) * (p2 % base)) % base
    }
}
// @lc code=end
