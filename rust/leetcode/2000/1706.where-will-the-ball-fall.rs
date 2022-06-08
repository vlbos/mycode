/*
 * @lc app=leetcode id=1706 lang=rust
 *
 * [1706] Where Will the Ball Fall
 */

// @lc code=start
impl Solution {
    pub fn find_ball(grid: Vec<Vec<i32>>) -> Vec<i32> {
        let mut ans = Vec::new();
        fn dfs(grid: &Vec<Vec<i32>>, i: usize, j: usize) -> i32 {
            if i == grid.len() {
                return j as i32;
            }
            if j + 1 < grid[i].len() && grid[i][j] == 1 && grid[i][j + 1] == 1 {
                return dfs(grid, i + 1, j + 1);
            }
            if j > 0 && grid[i][j] == -1 && grid[i][j - 1] == -1 {
                return dfs(grid, i + 1, j - 1);
            }
            -1
        }
        for j in 0..grid[0].len() {
            ans.push(dfs(&grid, 0, j));
        }
        ans
    }
}
// @lc code=end
