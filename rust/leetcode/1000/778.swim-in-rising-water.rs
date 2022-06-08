/*
 * @lc app=leetcode id=778 lang=rust
 *
 * [778] Swim in Rising Water
 */

// @lc code=start
impl Solution {
    pub fn swim_in_water(grid: Vec<Vec<i32>>) -> i32 {
        let nn = grid.len();
        let n = nn as i32;

        let check = |threshold: i32| -> bool {
            if grid[0][0] > threshold {
                return false;
            }
            let mut visited = vec![vec![false; nn]; nn];
            let mut q = std::collections::VecDeque::new();
            q.push_back((0, 0));
            visited[0][0] = true;
            let dirs = [0, 1, 0, -1, 0];
            while let Some((x, y)) = q.pop_front() {
                for d in 0..dirs.len() - 1 {
                    let (nx, ny) = (x as i32 + dirs[d], y  as i32+ dirs[d + 1]);
                    if nx < 0 || nx >= n || ny < 0 || ny >= n {
                        continue;
                    }
                    let (nx, ny) = (nx as usize, ny as usize);
                    if visited[nx][ny] || grid[nx][ny] > threshold {
                        continue;
                    }
                    visited[nx][ny] = true;
                    q.push_back((nx, ny));
                }
            }
            visited[nn - 1][nn - 1]
        };
        let (mut left, mut right) = (0, n * n - 1);
        while left < right {
            let mid = (left + right) / 2;
            if check(mid) {
                right = mid;
            } else {
                left = mid + 1;
            }
        }
        left
    }
}
// @lc code=end
