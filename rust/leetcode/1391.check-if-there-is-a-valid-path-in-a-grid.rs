/*
 * @lc app=leetcode id=1391 lang=rust
 *
 * [1391] Check if There is a Valid Path in a Grid
 */

// @lc code=start
impl Solution {
    pub fn has_valid_path(grid: Vec<Vec<i32>>) -> bool {
         let (m, n) = (grid.len(), grid[0].len());
        if m == 1 && n == 1 {
            return true;
        }
        let mut visited = vec![vec![false;n];m];
        fn dfs(grid: &Vec<Vec<i32>>, visited: &mut Vec<Vec<bool>>,i: usize, j: usize) -> bool {
            let (m, n) = (grid.len(), grid[0].len());
            if i == m - 1 && j == n - 1 {
                return true;
            }
            visited[i][j]=true;
            let (mi, ni) = (m as i32, n as i32);
            let patterns = [0b0101, 0b1010, 0b0011, 0b0110, 0b1001, 0b1100];
            let dirs = [[0, -1], [1, 0], [0, 1], [-1, 0]];
            for (k, d) in dirs.iter().enumerate() {
                let p = patterns[grid[i][j] as usize - 1];
                if p & (1 << k) > 0 {
                    let (ii, jj) = (i as i32 + d[0], j as i32 + d[1]);
                    if ii >= 0
                        && ii < mi
                        && jj >= 0
                        && jj < ni
                        && !visited[ii as usize][jj as usize] && patterns[grid[ii as usize][jj as usize] as usize - 1] & (1 << ((k + 2) % 4)) > 0
                    {
                        if dfs(grid, visited,ii as usize, jj as usize) {
                            return true;
                        }
                    }
                }
            }
            false
        }
        dfs(&grid,&mut visited,0, 0)
    }
}
// @lc code=end
