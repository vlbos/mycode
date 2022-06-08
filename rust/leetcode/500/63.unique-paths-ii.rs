/*
 * @lc app=leetcode id=63 lang=rust
 *
 * [63] Unique Paths II
 */

// @lc code=start
impl Solution {
    pub fn unique_paths_with_obstacles(obstacle_grid: Vec<Vec<i32>>) -> i32 {
        let mut o = vec![0; obstacle_grid[0].len()];
        o[0] = if obstacle_grid[0][0] == 0 { 1 } else { 0 };
        for i in 0..obstacle_grid.len() {
            for j in 0..obstacle_grid[i].len() {
                if obstacle_grid[i][j] == 1 {
                    o[j] = 0;
                    continue;
                }
                if j > 0 && obstacle_grid[i][j - 1] == 0 {
                    o[j] += o[j - 1];
                }
            }
        }
        *(o.last().unwrap())
    }
}
// @lc code=end
