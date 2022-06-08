/*
 * @lc app=leetcode id=1293 lang=rust
 *
 * [1293] Shortest Path in a Grid with Obstacles Elimination
 */

// @lc code=start
impl Solution {
    pub fn shortest_path(grid: Vec<Vec<i32>>, k: i32) -> i32 {
        let (m, n) = (grid.len(), grid[0].len());
        if m == 1 && n == 1 {
            return 0;
        }
        let k = k.min((m + n ) as i32- 3);
        let mut visited = std::collections::HashSet::from([(0, 0, k)]);
        let mut q = std::collections::VecDeque::from([(0, 0, k)]);
        let mut step = 0;
        let dirs = [0, 1, 0, -1, 0];
        while !q.is_empty() {
            step += 1;
            let cnt = q.len();
            for _ in 0..cnt {
                let (x, y, rest) = q.pop_front().unwrap();
                for i in 0..dirs.len() - 1 {
                    let (nx, ny) = (x as i32 + dirs[i], y as i32 + dirs[i + 1]);
                    if nx < 0 || nx >= m as i32 || ny < 0 || ny >= n as i32 {
                        continue;
                    }
                    let (nx, ny) = (nx as usize, ny as usize);
                    if grid[nx][ny] == 0 && !visited.contains(&(nx, ny, rest)) {
                        if nx == m - 1 && ny == n - 1 {
                            return step;
                        }
                        q.push_back((nx, ny, rest));
                        visited.insert((nx, ny, rest));
                    } else if grid[nx][ny] == 1 && rest > 0 && !visited.contains(&(nx, ny, rest - 1))
                    {
                        q.push_back((nx, ny, rest - 1));
                        visited.insert((nx, ny, rest - 1));
                    }
                }
            }
        }

        -1
    }
}
// @lc code=end
