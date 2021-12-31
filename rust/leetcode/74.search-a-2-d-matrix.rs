/*
 * @lc app=leetcode id=74 lang=rust
 *
 * [74] Search a 2D Matrix
 */

// @lc code=start
impl Solution {
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        if matrix[0][0]==target || matrix[matrix.len()-1][matrix[0].len()-1]==target{
        return true;
        }
        if matrix[0][0]>target || matrix[matrix.len()-1][matrix[0].len()-1]<target{
        return false;
        }
        for i in 0..matrix.len(){
            if  matrix[i][0]==target || (i<matrix.len()-1 && matrix[i+1][0]==target){
                return true;
            }
            if (i<matrix.len()-1 && matrix[i+1][0]>target && matrix[i][0]<target)||(matrix[i][matrix[i].len()-1]>target&& matrix[i][0]<target){
                for j in 0..matrix[i].len(){
                    if matrix[i][j]==target||matrix[i][matrix[i].len()-1]==target{
                        return true;
                    }
                    if (j<matrix[i].len()-1 && matrix[i][j+1]>target && matrix[i][j]<target){ 
                        return false;
                    }
                }   
            }
        }
        false
    }
}
// @lc code=end

