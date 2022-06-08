/*
 * @lc app=leetcode id=1568 lang=rust
 *
 * [1568] Minimum Number of Days to Disconnect Island
 */

// @lc code=start
impl Solution {
    pub fn min_days(grid: Vec<Vec<i32>>) -> i32 {
        let (m, n) = (grid.len(), grid[0].len());
        fn dfs(x: i32, y: i32, grid: &mut Vec<Vec<i32>>) {
            let (m, n) = (grid.len() as i32, grid[0].len() as i32);
            grid[x as usize][y as usize] = 2;
            let dirs = [0, 1, 0, -1, 0];
            for i in 0..dirs.len() - 1 {
                let (dx, dy) = (x + dirs[i], y + dirs[i + 1]);
                if dx < 0 || dx >= m || dy < 0 || dy >= n || grid[dx as usize][dy as usize] != 1 {
                    continue;
                }
                dfs(dx, dy, grid);
            }
        }
        let count = |grid: &mut Vec<Vec<i32>>| {
            let mut ans = 0;
            for i in 0..m {
                for j in 0..n {
                    if grid[i][j] == 1 {
                        ans += 1;
                        dfs(i as i32, j as i32, grid);
                    }
                }
            }
            for i in 0..m {
                for j in 0..n {
                    if grid[i][j] == 2 {
                        grid[i][j] = 1;
                    }
                }
            }
            ans
        };
        let mut grid = grid;
        if count(&mut grid) != 1 {
            return 0;
        }
        for i in 0..m {
            for j in 0..n {
                if grid[i][j] == 1 {
                    grid[i][j] = 0;
                    if count(&mut grid) != 1 {
                        return 1;
                    }
                    grid[i][j] = 1;
                }
            }
        }
        2
    }
}
// @lc code=end
