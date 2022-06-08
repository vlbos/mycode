/*
 * @lc app=leetcode id=1605 lang=rust
 *
 * [1605] Find Valid Matrix Given Row and Column Sums
 */

// @lc code=start
impl Solution {
    pub fn restore_matrix(row_sum: Vec<i32>, col_sum: Vec<i32>) -> Vec<Vec<i32>> {
       let (m, n) = (row_sum.len(), col_sum.len());
        let mut ans = vec![vec![0; n]; m];
         let mut row_sum=row_sum;
        let mut col_sum=col_sum;
        for i in 0..m {
            for j in 0..n {
                ans[i][j] = row_sum[i].min(col_sum[j]);
                row_sum[i] -= ans[i][j];
                col_sum[j] -= ans[i][j];
            }
        }
        ans
    }
}
// @lc code=end
