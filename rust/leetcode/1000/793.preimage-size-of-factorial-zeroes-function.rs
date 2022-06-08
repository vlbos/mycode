/*
 * @lc app=leetcode id=793 lang=rust
 *
 * [793] Preimage Size of Factorial Zeroes Function
 */

// @lc code=start
impl Solution {
    pub fn preimage_size_fzf(k: i32) -> i32 {
        let k = k as i64;
        let (mut lo, mut hi) = (k, k * 10 + 1);
        while lo < hi {
            let mi = (lo + hi) / 2;
            let zmi = zeta(mi);
            if zmi == k {
                return 5;
            }
            if zmi < k {
                lo = mi + 1;
            } else {
                hi = mi;
            }
        }
        fn zeta(x: i64) -> i64 {
            if x > 0 {
                x / 5 + zeta(x / 5)
            } else {
                0
            }
        }
        0
    }
}
// @lc code=end
