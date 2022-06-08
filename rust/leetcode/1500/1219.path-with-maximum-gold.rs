/*
 * @lc app=leetcode id=1219 lang=rust
 *
 * [1219] Path with Maximum Gold
 */

// @lc code=start
impl Solution {
    pub fn get_maximum_gold(grid: Vec<Vec<i32>>) -> i32 {
        let (m, n) = (grid.len(), grid[0].len());
        fn dfs(grid: &mut Vec<Vec<i32>>, i: i32, j: i32) -> i32 {
             let (m, n) = (grid.len() as i32, grid[0].len() as i32);
            if i < 0 || i >= m || j < 0 || j >= n || grid[i as usize][j as usize] == 0 {
                return 0;
            }
            let dirs = [1, 0, -1, 0, 1];
            let v = grid[i as usize][j as usize];
            grid[i as usize][j as usize] = 0;
            let mut ans = 0;
            for k in 0..dirs.len() - 1 {
                ans = ans.max(dfs(grid, i + dirs[k], j + dirs[k + 1]));
            }
            grid[i as usize][j as usize] = v;
            ans+v
        }
        let mut grid = grid;
        let mut ans = 0;
        for i in 0..m {
            for j in 0..n {
                if grid[i][j] == 0 {
                    continue;
                }
                ans = ans.max(dfs(&mut grid, i as i32, j as i32));
            }
        }
        ans
    }
}
// @lc code=end
