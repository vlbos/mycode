/*
 * @lc app=leetcode id=73 lang=rust
 *
 * [73] Set Matrix Zeroes
 */

// @lc code=start
impl Solution {
    pub fn set_zeroes(matrix: &mut Vec<Vec<i32>>) {
        let mut n = vec![1; matrix.len()];
        let mut m = vec![1; matrix[0].len()];
        for i in 0..n.len() {
            for j in 0..m.len() {
                if matrix[i][j] == 0 {
                    n[i] = 0;
                    m[j] = 0;
                }
            }
        }
        for i in 0..n.len() {
            if n[i] == 0 {
                matrix[i] = vec![0; m.len()];
            }
        }
        for j in 0..m.len() {
            if m[j] == 0 {
                for i in 0..n.len() {
                    matrix[i][j] = 0;
                }
            }
        }
    }
}
// @lc code=end
