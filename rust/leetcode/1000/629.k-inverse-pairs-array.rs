/*
 * @lc app=leetcode id=629 lang=rust
 *
 * [629] K Inverse Pairs Array
 */

// @lc code=start
impl Solution {
    pub fn k_inverse_pairs(n: i32, k: i32) -> i32 {
       let (n, k) = (n as usize, k as usize);
        let mut f = vec![vec![0; k + 1]; 2];
        f[0][0] = 1;
        let p = 1000_000_007;
        for i in 1..=n {
            for j in 0..=k {
                let cur = i & 1;
                let pre=cur^1;
                f[cur][j] = if j  >= 1{ f[cur][j - 1] } else { 0 }
                    - if j  >= i { f[pre][j - i] } else { 0 }
                    + f[pre][j];
                if f[cur][j] >= p {
                    f[cur][j] -= p;
                } else if f[cur][j] < 0 {
                    f[cur][j] += p;
                }
            }
        }
        f[n & 1][k]
    }
}
// @lc code=end
