/*
 * @lc app=leetcode id=240 lang=rust
 *
 * [240] Search a 2D Matrix II
 */

// @lc code=start
impl Solution {
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        let m = matrix.len();
        let n = matrix[0].len();
        let mut i = 0;
        let mut j = n - 1;
        while i < m && j >= 0 {
            if matrix[i][j] == target {
                return true;
            }
            if matrix[i][j] > target {
                if j > 0 {
                    j -= 1;
                } else {
                    break;
                }
            } else {
                i += 1;
            }
        }
        false
    }
}
// @lc code=end
