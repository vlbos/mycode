/*
 * @lc app=leetcode id=807 lang=rust
 *
 * [807] Max Increase to Keep City Skyline
 */

// @lc code=start
impl Solution {
    pub fn max_increase_keeping_skyline(grid: Vec<Vec<i32>>) -> i32 {
        let n = grid.len();
        let mut rowmax = vec![0; n];
        let mut colmax = vec![0; n];
        for i in 0..n {
            for j in 0..n {
                rowmax[i] = rowmax[i].max(grid[i][j]);
                colmax[j] = colmax[j].max(grid[i][j]);
            }
        }
        let mut ans = 0;
        for i in 0..n {
            for j in 0..n {
                ans += rowmax[i].min(colmax[j]) - grid[i][j];
            }
        }
        ans
    }
}
// @lc code=end
