/*
 * @lc app=leetcode id=62 lang=rust
 *
 * [62] Unique Paths
 */

// @lc code=start
impl Solution {
    pub fn unique_paths(m: i32, n: i32) -> i32 {
        let mut m = m as usize;
        let mut n = n as usize;

        let mut f = vec![vec![0; m]; n];
        for i in 0..n {
            f[i][0] = 1;
        }
        for i in 0..m {
            f[0][i] = 1;
        }
        for i in 1..n {
            for j in 1..m {
                f[i][j] = f[i - 1][j] + f[i][j - 1];
            }
        }
        f[n - 1][m - 1]
    }
}
// @lc code=end
