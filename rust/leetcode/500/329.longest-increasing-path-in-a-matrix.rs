/*
 * @lc app=leetcode id=329 lang=rust
 *
 * [329] Longest Increasing Path in a Matrix
 */

// @lc code=start
impl Solution {
    pub fn longest_increasing_path(matrix: Vec<Vec<i32>>) -> i32 {
        let (m, n) = (matrix.len(), matrix[0].len());
        let mut memo = vec![vec![0; n]; m];
        let mut ans = 0;
        for i in 0..m {
            for j in 0..n {
                ans = ans.max(dfs(&matrix, i, j, &mut memo));
            }
        }
        fn dfs(matrix: &Vec<Vec<i32>>, row: usize, col: usize, memo: &mut Vec<Vec<i32>>) -> i32 {
            if memo[row][col] != 0 {
                return memo[row][col];
            }
            memo[row][col] += 1;
            let (m, n) = (matrix.len() as i32, matrix[0].len() as i32);
            let dirs = [0, 1, 0, -1, 0];
            for i in 0..dirs.len() - 1 {
                let (r, c) = (row as i32 + dirs[i], col as i32 + dirs[i + 1]);
                if r < 0 || r >= m || c < 0 || c >= n {
                    continue;
                }
                let (rr, cc) = (r as usize, c as usize);
                if matrix[rr][cc] <= matrix[row][col] {
                    continue;
                }
                memo[row][col] = memo[row][col].max(dfs(&matrix, rr, cc, memo) + 1);
            }
            memo[row][col]
        }
        ans
    }
}
// @lc code=end
