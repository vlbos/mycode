/*
 * @lc app=leetcode id=1240 lang=rust
 *
 * [1240] Tiling a Rectangle with the Fewest Squares
 */

// @lc code=start
impl Solution {
    pub fn tiling_rectangle(n: i32, m: i32) -> i32 {
        let (n, m) = (n as usize, m as usize);
        let mut f = vec![vec![i32::MAX/3; m + 1]; n + 1];
        for i in 0..=n {
            f[i][0] = 0;
        }
        for i in 0..=m {
            f[0][i] = 0;
        }
        for i in 1..=n {
            for j in 1..=m {
                for k in 1..=i.min(j) {
                    let (i2, j1) = (i - k, j - k);
                    for i1 in 0..=i2 {
                        for j2 in j1..=j {
                            f[i][j] = f[i][j].min(
                                f[i1][j2] + f[i2][j - j2] + f[i - i1][j1] + f[i2 - i1][j2 - j1] + 1
                            );
                        }
                    }
                }
            }
        }
        f[n][m]
    }
}
// @lc code=end
