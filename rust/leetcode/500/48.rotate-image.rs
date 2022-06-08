/*
 * @lc app=leetcode id=48 lang=rust
 *
 * [48] Rotate Image
 */

// @lc code=start
impl Solution {
    pub fn rotate(matrix: &mut Vec<Vec<i32>>) {
        let n = matrix.len();
        for i in 0..matrix.len()/2{
            for j in 0..(matrix.len()+1)/2{
                let t = matrix[i][j];
                 matrix[i][j]=matrix[n-j-1][i];
                 matrix[n-j-1][i]=matrix[n-i-1][n-j-1];
                 matrix[n-i-1][n-j-1] =matrix[j][n-i-1];
                 matrix[j][n-i-1]=t;
            }
        }
    }
}
// @lc code=end

