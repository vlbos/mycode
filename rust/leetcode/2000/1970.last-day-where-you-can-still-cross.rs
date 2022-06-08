/*
 * @lc app=leetcode id=1970 lang=rust
 *
 * [1970] Last Day Where You Can Still Cross
 */

// @lc code=start
impl Solution {
    pub fn latest_day_to_cross(row: i32, col: i32, cells: Vec<Vec<i32>>) -> i32 {
        let mut ans = 0;
        let (mut left, mut right) = (0, row * col);
        let dirs = [0, 1, 0, -1, 0];
        while left <= right {
            let mid = (left + right) / 2;
            let mut grid = vec![vec![1; col as usize]; row as usize];
            for cell in &cells[..mid as usize] {
                let (r, c) = (cell[0] as usize - 1, cell[1] as usize - 1);
                grid[r][c] = 0;
            }
            let mut q = std::collections::VecDeque::new();
            for i in 0..col {
                if grid[0][i as usize] > 0 {
                    q.push_back((0, i));
                    grid[0][i as usize] = 0;
                }
            }
            let mut found = false;
            while let Some((x, y)) = q.pop_front() {
                for d in 0..dirs.len() - 1 {
                    let (nx, ny) = (x + dirs[d], y + dirs[d + 1]);
                    if nx < 0
                        || nx >= row
                        || ny < 0
                        || ny >= col
                        || grid[nx as usize][ny as usize] == 0
                    {
                        continue;
                    }
                    if nx == row - 1 {
                        found = true;
                        break;
                    }
                    q.push_back((nx, ny));
                    grid[nx as usize][ny as usize] = 0;
                }
            }
            if found {
                ans = mid;
                left = mid + 1;
            } else {
                right = mid - 1;
            }
        }
        ans
    }
}
// @lc code=end
