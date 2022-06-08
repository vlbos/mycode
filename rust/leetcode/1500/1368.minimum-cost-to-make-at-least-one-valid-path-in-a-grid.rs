/*
 * @lc app=leetcode id=1368 lang=rust
 *
 * [1368] Minimum Cost to Make at Least One Valid Path in a Grid
 */

// @lc code=start
impl Solution {
    pub fn min_cost(grid: Vec<Vec<i32>>) -> i32 {
        let (m, n) = (grid.len() as i32, grid[0].len() as i32);
        let mut dist = vec![i32::MAX / 2; (m * n) as usize];
        dist[0] = 0;
        let mut seen = std::collections::HashSet::new();
        let mut q = std::collections::VecDeque::from([(0, 0)]);
        while let Some((x, y)) = q.pop_front() {
            if seen.contains(&(x, y)) {
                continue;
            }
            seen.insert((x, y));
            let cur_pos = x * n + y;
            for (i, &(nx, ny)) in [(x, y + 1), (x, y - 1), (x + 1, y), (x - 1, y)]
                .iter()
                .enumerate()
            {
                if nx < 0 || nx >= m || ny < 0 || ny >= n {
                    continue;
                }
                let new_pos = nx * n + ny;
                let new_dis = dist[cur_pos as usize]
                    + if grid[x as usize][y as usize] != i as i32 + 1 {
                        1
                    } else {
                        0
                    };
                if new_dis >= dist[new_pos as usize] {
                    continue;
                }
                dist[new_pos as usize] = new_dis;
                if grid[x as usize][y as usize] == i as i32 + 1 {
                    q.push_front((nx, ny));
                } else {
                    q.push_back((nx, ny));
                }
            }
        }
        *dist.last().unwrap()
    }
}
// @lc code=end
