/*
 * @lc app=leetcode id=1914 lang=rust
 *
 * [1914] Cyclically Rotating a Grid
 */

// @lc code=start
impl Solution {
    pub fn rotate_grid(grid: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
        let (m, n) = (grid.len(), grid[0].len());
        let nl = (m / 2).min(n / 2);
        let mut grid = grid;
        for l in 0..nl {
            let (mut r, mut c, mut v) = (Vec::new(), Vec::new(), Vec::new());
            for i in l..m - l - 1 {
                r.push(i);
                c.push(l);
                v.push(grid[i][l]);
            }
            for j in l..n - l - 1 {
                r.push(m - l - 1);
                c.push(j);
                v.push(grid[m - l - 1][j]);
            }
            for i in (l+1..=m - l - 1).rev() {
                r.push(i);
                c.push(n - l - 1);
                v.push(grid[i][n - l - 1]);
            }
            for j in (l+1..=n - l - 1).rev() {
                r.push(l);
                c.push(j);
                v.push(grid[l][j]);
            }
            let t = v.len();
            let kk = (k as usize) % t;
            for i in 0..t {
                let idx = (t + i - kk) % t;
                grid[r[i]][c[i]] = v[idx];
            }
        }
        grid
    }
}
// @lc code=end
