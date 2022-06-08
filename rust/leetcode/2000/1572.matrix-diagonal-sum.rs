/*
 * @lc app=leetcode id=1572 lang=rust
 *
 * [1572] Matrix Diagonal Sum
 */

// @lc code=start
impl Solution {
    pub fn diagonal_sum(mat: Vec<Vec<i32>>) -> i32 {
        let mut ans = 0;
        let mut i = 0;
        let mut l = mat.len();
        for i in 0..l {
            ans += mat[i][i];
            ans += mat[l - i - 1][i];
        }
        if l % 2 != 0 {
            ans -= mat[l / 2][l / 2];
        }
        ans
    }
}
// @lc code=end
