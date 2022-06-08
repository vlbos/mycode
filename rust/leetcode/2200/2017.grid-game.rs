/*
 * @lc app=leetcode id=2017 lang=rust
 *
 * [2017] Grid Game
 */

// @lc code=start
impl Solution {
    pub fn grid_game(grid: Vec<Vec<i32>>) -> i64 {
       let n = grid[0].len();
        let mut p = vec![vec![0i64;  n + 1];2];
        for i in 1..=n {
            p[0][i] = p[0][i - 1] + grid[0][i - 1] as i64;
            p[1][i] = p[1][i - 1] + grid[1][i - 1]  as i64;
        }
        let mut ans = i64::MAX;
        for i in 1..=n {
            ans = ans.min(p[1][i - 1].max(p[0][n] - p[0][i]));
        }
        ans
    }
}
// @lc code=end
