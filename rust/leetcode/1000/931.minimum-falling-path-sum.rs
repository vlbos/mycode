/*
 * @lc app=leetcode id=931 lang=rust
 *
 * [931] Minimum Falling Path Sum
 */

// @lc code=start
impl Solution {
    pub fn min_falling_path_sum(matrix: Vec<Vec<i32>>) -> i32 {
         let mut matrix=matrix;
        let n = matrix.len();
        for i in (0..n-1).rev(){
            for j in 0..n{
                    let mut best = matrix[i+1][j];
                    if j>0{
                        best = best.min(matrix[i+1][j-1]);
                    }
                    if j+1<n{
                        best = best.min(matrix[i+1][j+1]);
                    }
                    matrix[i][j]+= best;
            }
        }
        *matrix[0].iter().min().unwrap()
    }
}
// @lc code=end

