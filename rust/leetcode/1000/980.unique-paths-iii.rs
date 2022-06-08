/*
 * @lc app=leetcode id=980 lang=rust
 *
 * [980] Unique Paths III
 */

// @lc code=start
impl Solution {
    pub fn unique_paths_iii(grid: Vec<Vec<i32>>) -> i32 {
        let (m, n) = (grid.len(), grid[0].len());
        let (mut sr, mut sc) = (0, 0);
        let (mut tr, mut tc) = (0, 0);
        let mut todo=0;
        for i in 0..m {
            for j in 0..n {
                if grid[i][j] != -1 {
                    todo += 1;
                }
                if grid[i][j] == 1 {
                    sr = i;
                    sc = j;
                } else if grid[i][j] == 2 {
                    tr = i;
                    tc = j;
                }
            }
        }
        let mut ans = 0;
        fn dfs(
            i: usize,
            j: usize,
            mut todo: i32,
            ans: &mut i32,
            grid: &mut Vec<Vec<i32>>,
            tr: usize,
            tc: usize,
        ) {
            todo -= 1;
            if todo < 0 {
                return;
            }
            if i == tr && j == tc {
                if todo == 0 {
                    *ans += 1;
                }
                return;
            }
            grid[i][j] = 3;
            let dirs = [0, 1, 0, -1, 0];
            let (m, n) = (grid.len() as i32, grid[0].len() as i32);
            for k in 0..dirs.len() - 1 {
                let (nr, nc) = (i as i32 + dirs[k], j as i32 + dirs[k + 1]);
                if nr < 0 || nr >= m || nc < 0 || nc >= n {
                    continue;
                }
                let (nr, nc) = (nr as usize, nc as usize);
                if grid[nr][nc] % 2 == 0 {
                    dfs(nr, nc, todo, ans, grid, tr, tc);
                }
            }
            grid[i][j] = 0;
        }
        let mut grid = grid;
        dfs(sr, sc, todo, &mut ans, &mut grid,tr,tc);
        ans
    }
}
// @lc code=end
