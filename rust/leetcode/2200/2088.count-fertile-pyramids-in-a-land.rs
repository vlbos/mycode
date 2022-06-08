/*
 * @lc app=leetcode id=2088 lang=rust
 *
 * [2088] Count Fertile Pyramids in a Land
 */

// @lc code=start
impl Solution {
    pub fn count_pyramids(grid: Vec<Vec<i32>>) -> i32 {
        let (m, n) = (grid.len(), grid[0].len());
        let mut f = vec![0; n];
        let mut ans = 0;
        for (i, row) in grid.iter().enumerate().rev() {
            let mut g = vec![0; n];
            for (j, &v) in row.iter().enumerate() {
                if v == 0 {
                    g[j] = -1;
                } else if i != m - 1 && j != 0 && j != n - 1 {
                    g[j] = *f[j - 1..=j + 1].iter().min().unwrap() + 1;
                    ans += g[j];
                }
            }
            f = g;
        }
        f = vec![0; n];
        for (i, row) in grid.iter().enumerate() {
            let mut g = vec![0; n];
            for (j, &v) in row.iter().enumerate() {
                if v == 0 {
                    g[j] = -1;
                } else if i != 0 && j != 0 && j != n - 1 {
                    g[j] = *f[j - 1..=j + 1].iter().min().unwrap() + 1;
                    ans += g[j];
                }
            }
            f = g;
        }
        ans
    }
}
// @lc code=end
