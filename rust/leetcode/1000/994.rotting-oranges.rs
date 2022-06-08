/*
 * @lc app=leetcode id=994 lang=rust
 *
 * [994] Rotting Oranges
 */

// @lc code=start
impl Solution {
    pub fn oranges_rotting(grid: Vec<Vec<i32>>) -> i32 {
        let mut grid = grid;
        let mut ans = 0;
        let (n,m) = (grid.len(),grid[0].len());
        let mut q = std::collections::VecDeque::new();
        for (r, row) in grid.iter().enumerate() {
            for (c, val) in row.iter().enumerate() {
                if grid[r][c] == 2 {
                    q.push_back((r, c, 0));
                }
            }
        }
        let neighbours = |x: usize, y: usize| -> Vec<(usize, usize)> {
            let dirs = [[0, 1], [0, -1], [1, 0], [-1, 0]];
            let mut ret = Vec::new();
            for d in &dirs {
                let (tx,ty) = (x as i32 + d[0],y as i32 + d[1]);
                if tx < 0 || tx >= n as i32 || ty < 0 || ty >= m as i32 {
                    continue;
                }
                ret.push((tx as usize, ty as usize));

            }
            ret
        };

        while let Some((x, y, d)) = q.pop_front() {
            for (tx, ty) in neighbours(x, y) {
                if grid[tx][ty] == 1 {
                    grid[tx][ty] = 2;
                    ans = d + 1;
                    q.push_back((tx, ty, d + 1));
                }
            }
        }
        if grid.iter().any(|x| x.iter().any(|&a| a == 1)) {
            return -1;
        }
        ans
    }
}
// @lc code=end
