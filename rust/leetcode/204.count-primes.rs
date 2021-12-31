/*
 * @lc app=leetcode id=204 lang=rust
 *
 * [204] Count Primes
 */

// @lc code=start
impl Solution {
    pub fn count_primes(n: i32) -> i32 {
        let mut is_primes = vec![true; n as usize].to_vec(); 

        let mut c = 0;
        let mut j = 0i64;
        let n = n as i64;
        let mut ii = 0i64;
        for i in 2..n {
            if is_primes[(i - 1) as usize] {
                c += 1;
                ii = i as i64;
                j = ii * ii;
                while j < n {
                    is_primes[(j - 1) as usize] = false;
                    j += ii;
                }
            }
        }
        c
    }
}
// @lc code=end

