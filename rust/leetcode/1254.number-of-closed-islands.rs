/*
 * @lc app=leetcode id=1254 lang=rust
 *
 * [1254] Number of Closed Islands
 */

// @lc code=start
impl Solution {
    pub fn closed_island(grid: Vec<Vec<i32>>) -> i32 {
        fn dfs(grid: &mut Vec<Vec<i32>>, i: i32, j: i32) -> i32 {
            let (m, n) = (grid.len() as i32, grid[0].len() as i32);
            if i < 0 || i >= m || j < 0 || j >= n {
                return 0;
            }
            if grid[i as usize][j as usize] == 1 {
                return 1;
            }
            grid[i as usize][j as usize] = 1;
              dfs(grid,i + 1,j) & dfs(grid,i - 1,j) & dfs(grid,i,j + 1) & dfs(grid,i,j - 1)
        }
        let (m, n) = (grid.len(), grid[0].len());
        let mut ans = 0;
        let mut grid = grid;
        for i in 0..m {
            for j in 0..n {
                if grid[i][j] == 0 {
                    ans += dfs(&mut grid, i as i32, j as i32);
                }
            }
        }
        ans
    }
}
// @lc code=end
