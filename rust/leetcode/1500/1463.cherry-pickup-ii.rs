/*
 * @lc app=leetcode id=1463 lang=rust
 *
 * [1463] Cherry Pickup II
 */

// @lc code=start
impl Solution {
    pub fn cherry_pickup(grid: Vec<Vec<i32>>) -> i32 {
        let (m, n) = (grid.len(), grid[0].len());
        let mut f = vec![vec![-1; n]; n];
        let mut g = f.clone();
        f[0][n - 1] = grid[0][0] + grid[0][n - 1];
        for i in 1..m {
            for j1 in 0..n {
                for j2 in 0..n {
                    let mut best = -1;
                    let (jj1, jj2) = (j1 as i32, j2 as i32);
                    for dj1 in jj1 - 1..=jj1 + 1 {
                        for dj2 in jj2 - 1..=jj2 + 1 {
                            if dj1 >= 0
                                && dj1 < n as i32
                                && dj2 >= 0
                                && dj2 < n as i32
                                && f[dj1 as usize][dj2 as usize] != -1
                            {
                                best = best.max(
                                    f[dj1 as usize][dj2 as usize]
                                        + if j1 == j2 {
                                            grid[i][j1]
                                        } else {
                                            grid[i][j1] + grid[i][j2]
                                        },
                                );
                            }
                        }
                    }
                    g[j1][j2] = best;
                }
            }
            std::mem::swap(&mut f, &mut g);
        }
        f.iter()
            .map(|x| *x.iter().max().unwrap())
            .max()
            .unwrap()
            .max(0)
    }
}
// @lc code=end
