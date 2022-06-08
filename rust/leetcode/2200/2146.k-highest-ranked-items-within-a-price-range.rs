/*
 * @lc app=leetcode id=2146 lang=rust
 *
 * [2146] K Highest Ranked Items Within a Price Range
 */

// @lc code=start
impl Solution {
    pub fn highest_ranked_k_items(
        grid: Vec<Vec<i32>>,
        pricing: Vec<i32>,
        start: Vec<i32>,
        k: i32,
    ) -> Vec<Vec<i32>> {
        let (m, n) = (grid.len() as i32, grid[0].len() as i32);
        let mut grid = grid;
        let mut items = Vec::new();
        let mut q = std::collections::VecDeque::new();
        let (nxx, nyy) = (start[0], start[1]);
        let (nx, ny) = (nxx as usize, nyy as usize);
        if grid[nx][ny] >= pricing[0] && grid[nx][ny] <= pricing[1] {
            items.push(vec![0, grid[nx][ny], nxx, nyy]);
        }
        q.push_back(vec![nxx, nyy, 0]);
        grid[nx][ny] = 0;
        let dirs = [0, 1, 0, -1, 0];
        while let Some(curr) = q.pop_front() {
            let (mut x, mut y, d) = (curr[0], curr[1], curr[2]);
            for i in 0..dirs.len() - 1 {
                let (nxx, nyy) = (x + dirs[i], y + dirs[i + 1]);
                if nxx < 0 || nxx >= m || nyy < 0 || nyy >= n {
                    continue;
                }
                let (nx, ny) = (nxx as usize, nyy as usize);
                if grid[nx][ny] == 0 {
                    continue;
                }
                if grid[nx][ny] >= pricing[0] && grid[nx][ny] <= pricing[1] {
                    items.push(vec![d + 1, grid[nx][ny], nxx, nyy]);
                }
                q.push_back(vec![nxx, nyy, d + 1]);
                grid[nx][ny] = 0;
            }
        }
        items.sort();
        let mut ans =Vec::new();
        let cnt =items.len().min(k as usize);
        for i in 0..cnt{
            ans.push(vec![items[i][2],items[i][3]]);
        }
        ans
    }
}
// @lc code=end
