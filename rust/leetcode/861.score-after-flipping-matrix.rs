/*
 * @lc app=leetcode id=861 lang=rust
 *
 * [861] Score After Flipping Matrix
 */

// @lc code=start
impl Solution {
    pub fn matrix_score(grid: Vec<Vec<i32>>) -> i32 {
        let m = grid.len();
        let n = grid[0].len();
        let mut ans = m  as i32 * (1 << (n - 1));
        for j in 1..n {
            let mut ones = 0;
            for i in 0..m {
                ones += if grid[i][0] == 1 {
                    grid[i][j]
                } else {
                    1 - grid[i][j]
                };
            }
            let k = ones.max(m as i32- ones);
            ans += k * (1 << (n - j - 1));
        }
        ans
    }
}
// @lc code=end
