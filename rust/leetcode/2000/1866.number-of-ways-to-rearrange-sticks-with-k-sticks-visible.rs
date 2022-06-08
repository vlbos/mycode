/*
 * @lc app=leetcode id=1866 lang=rust
 *
 * [1866] Number of Ways to Rearrange Sticks With K Sticks Visible
 */

// @lc code=start
impl Solution {
    pub fn rearrange_sticks(n: i32, k: i32) -> i32 {
         let (n, k) = (n as usize, k as usize);
        let mut f = vec![0; k+1];
        f[0] = 1;
        let p = 1_000_000_007;
        for i in 1..=n {
            let mut g = vec![0; k+1];
            for j in 1..=k {
                g[j] = (f[j] * ((i - 1) as i64) % p + f[j - 1]) % p;
            }
            f = g;
        }
        f[k] as _
    }
}
// @lc code=end
