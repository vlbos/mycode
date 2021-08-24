/*
 * @lc app=leetcode.cn id=867 lang=rust
 *
 * [867] 转置矩阵
 */

// @lc code=start
impl Solution {
    pub fn transpose(matrix: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut r = vec![vec![0;matrix.len()].to_vec();matrix[0].len()].to_vec();
        for i in 0..matrix.len(){
            for j in 0..matrix[0].len(){  
                r[j][i]=matrix[i][j];
            }
        }
        r
    }
}
// @lc code=end

