/*
 * @lc app=leetcode id=1594 lang=rust
 *
 * [1594] Maximum Non Negative Product in a Matrix
 */

// @lc code=start
impl Solution {
    pub fn max_product_path(grid: Vec<Vec<i32>>) -> i32 {
        let (m, n) = (grid.len(), grid[0].len());
        let (mut maxgt, mut minlt) = (vec![vec![0i64; n]; m], vec![vec![0i64; n]; m]);
        maxgt[0][0] = grid[0][0] as i64;
        minlt[0][0] = grid[0][0] as i64;
        for i in 1..m {
            maxgt[i][0] = maxgt[i - 1][0] * grid[i][0] as i64;
            minlt[i][0] = minlt[i - 1][0] * grid[i][0] as i64;
        }
        for i in 1..n {
            maxgt[0][i] = maxgt[0][i - 1] * grid[0][i] as i64;
            minlt[0][i] = minlt[0][i - 1] * grid[0][i] as i64;
        }
        for i in 1..m {
            for j in 1..n {
                if grid[i][j] >= 0 {
                    maxgt[i][j] = maxgt[i][j - 1].max(maxgt[i - 1][j]) * grid[i][j] as i64;
                    minlt[i][j] = minlt[i][j - 1].min(minlt[i - 1][j]) * grid[i][j] as i64;
                } else {
                    maxgt[i][j] = minlt[i][j - 1].min(minlt[i - 1][j]) * grid[i][j] as i64;
                    minlt[i][j] = maxgt[i][j - 1].max(maxgt[i - 1][j]) * grid[i][j] as i64;
                }
            }
        }
        if maxgt[m - 1][n - 1] < 0 {
            -1
        } else {
            (maxgt[m - 1][n - 1] %1000_000_007) as _
        }
    }
}
// @lc code=end
