/*
 * @lc app=leetcode id=407 lang=rust
 *
 * [407] Trapping Rain Water II
 */

// @lc code=start
impl Solution {
    pub fn trap_rain_water(height_map: Vec<Vec<i32>>) -> i32 {
        let (m, n) = (height_map.len(), height_map[0].len());
        if m <= 2 || n <= 2 {
            return 0;
        }
        use std::cmp::Reverse;
        let mut q = std::collections::BinaryHeap::new();
        let mut visited = vec![vec![false; n]; m];
        for i in 0..m {
            for j in 0..n {
                if i == 0 || i == m - 1 || j == 0 || j == n - 1 {
                    q.push(Reverse((height_map[i][j], i, j)));
                    visited[i][j] = true;
                }
            }
        }
        let mut ans = 0;
        let dirs = [0, 1, 0, -1, 0];
        while let Some(Reverse((h, r, c))) = q.pop() {
            for k in 0..dirs.len() - 1 {
                let (nx, ny) = (r as i32 + dirs[k], c as i32 + dirs[k + 1]);
                if nx < 0 || nx >= m as i32 || ny < 0 || ny >= n as i32 || visited[nx as usize][ny as usize] {
                    continue;
                }
                let (i, j) = (nx as usize, ny as usize);
                if height_map[i][j] < h {
                    ans += h - height_map[i][j];
                }
                visited[i][j] = true;
                q.push(Reverse((height_map[i][j].max(h), i, j)));
            }
        }
        ans
    }
}
// @lc code=end
