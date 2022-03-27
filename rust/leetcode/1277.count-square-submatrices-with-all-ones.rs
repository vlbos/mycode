/*
 * @lc app=leetcode id=1277 lang=rust
 *
 * [1277] Count Square Submatrices with All Ones
 */

// @lc code=start
impl Solution {
    pub fn count_squares(matrix: Vec<Vec<i32>>) -> i32 {
        let (m, n) = (matrix.len(), matrix[0].len());
        let mut f = vec![vec![0; n]; m];
        let mut ans = 0;
        for (i, r) in matrix.iter().enumerate() {
            for (j, &c) in r.iter().enumerate() {
                if i == 0 || j == 0 {
                    f[i][j] = c;
                } else if c == 0 {
                    f[i][j] = 0;
                } else {
                    f[i][j] = f[i - 1][j].min(f[i][j - 1]).min(f[i - 1][j - 1]) + 1;
                }
                ans += f[i][j];
            }
        }
        ans
    }
}
// @lc code=end
