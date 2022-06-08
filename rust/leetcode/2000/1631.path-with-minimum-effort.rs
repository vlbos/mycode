/*
 * @lc app=leetcode id=1631 lang=rust
 *
 * [1631] Path With Minimum Effort
 */

// @lc code=start
impl Solution {
    pub fn minimum_effort_path(heights: Vec<Vec<i32>>) -> i32 {
        let (m, n) = (heights.len(), heights[0].len());
        let mut dist = vec![i32::MAX; m * n];
        dist[0] = 0;
        use std::cmp::Reverse;
        let mut q = std::collections::BinaryHeap::new();
        q.push(Reverse((0, 0, 0)));
        let mut seen = std::collections::HashSet::new();
        let (mm, nn) = (m as i32, n as i32);
        while let Some(Reverse((d, x, y))) = q.pop() {
            let id = x * n + y;
            if seen.contains(&id) {
                continue;
            }
            if (x, y) == (m - 1, n - 1) {
                break;
            }
            seen.insert(id);
            let (xx, yy) = (x as i32, y as i32);
            for (nx, ny) in [(xx - 1, yy), (xx + 1, yy), (xx, yy - 1), (xx, yy + 1)] {
                if nx >= 0
                    && nx < mm
                    && ny >= 0
                    && ny < nn
                    && d.max((heights[x][y] - heights[nx as usize][ny as usize]).abs())
                        <= dist[(nx * nn + ny) as usize]
                {
                    let (nxx, nyy) = (nx as usize, ny as usize);
                    dist[nxx * n + nyy] = d.max((heights[x][y] - heights[nxx][nyy]).abs());
                    q.push(Reverse((dist[nxx * n + nyy], nxx, nyy)));
                }
            }
        }
        dist[m * n - 1]
    }
}
// @lc code=end
