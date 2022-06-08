/*
 * @lc app=leetcode id=1727 lang=rust
 *
 * [1727] Largest Submatrix With Rearrangements
 */

// @lc code=start
impl Solution {
    pub fn largest_submatrix(matrix: Vec<Vec<i32>>) -> i32 {
        let (m, n) = (matrix.len(), matrix[0].len());
        let mut dp = vec![0; n];
        let mut ans = 0;
        for i in 0..m {
            for j in 0..n {
                if matrix[i][j] == 0 {
                    dp[j] = 0;
                } else {
                    dp[j] += 1;
                }
            }
            let mut t = dp.clone();
            t.sort();
            for j in 0..n {
                ans = ans.max(t[j] * (n - j) as i32);
            }
        }
        ans
    }
}
// @lc code=end
