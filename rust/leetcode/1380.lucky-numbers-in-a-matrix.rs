/*
 * @lc app=leetcode id=1380 lang=rust
 *
 * [1380] Lucky Numbers in a Matrix
 */

// @lc code=start
impl Solution {
    pub fn lucky_numbers(matrix: Vec<Vec<i32>>) -> Vec<i32> {
        let mut r = vec![i32::MAX; matrix.len()];
        let mut c = vec![0; matrix[0].len()];
        let mut ri = vec![matrix.len(); matrix.len()];
        let mut ci = vec![matrix[0].len(); matrix[0].len()];
        for i in 0..matrix.len() {
            for j in 0..matrix[i].len() {
                if matrix[i][j] < r[i] {
                    r[i] = matrix[i][j];
                    ri[i] = j;
                }
                if matrix[i][j] > c[j] {
                    c[j] = matrix[i][j];
                    ci[j] = i;
                }
            }
        }
        let mut ans = Vec::new();
        for i in 0..ri.len(){
                if ci[ri[i]] ==i{
                    ans.push(matrix[i][ri[i]]);
                } 
        }
        ans
    }
}
// @lc code=end
