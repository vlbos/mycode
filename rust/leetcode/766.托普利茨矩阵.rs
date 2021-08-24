/*
 * @lc app=leetcode.cn id=766 lang=rust
 *
 * [766] 托普利茨矩阵
 */

// @lc code=start
impl Solution {
    pub fn is_toeplitz_matrix(matrix: Vec<Vec<i32>>) -> bool {
        let mut x = 0;
        let mut y = 0;
        for i in 0..matrix[0].len(){
            x=0;
            y=i;
            while x<matrix.len() && y<matrix[0].len(){
                    if matrix[x][y]!=matrix[0][i]{
                    return false;
                    }
            x+=1;
            y+=1;
            }
        }
        for i in 1..matrix.len(){
            x=i;
            y=0;
            while x<matrix.len() && y<matrix[0].len(){
                    if matrix[x][y]!=matrix[i][0]{
                    return false;
                    }
            x+=1;
            y+=1;
            }
        }
        true
    }
}
// @lc code=end

