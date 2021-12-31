/*
 * @lc app=leetcode id=498 lang=rust
 *
 * [498] Diagonal Traverse
 */

// @lc code=start
impl Solution {
    pub fn find_diagonal_order(mat: Vec<Vec<i32>>) -> Vec<i32> {
        let mut ans = Vec::new();
        let mut i = 0;
        let mut j = 0;
        let m = mat.len();
        let n = mat[0].len();
        let l = m * n;
        let mut signed = 1;
        let mm = m as i32;
        let nn = n as i32;
        while i < m && j < n {
            ans.push(mat[i][j]);
            let ii = i as i32 + signed * (-1);
            let jj = j as i32 + signed;

            if ii == mm || ii < 0 || jj == nn || jj < 0 {
                if signed > 0 {
                    i += if j == n - 1 { 1 } else { 0 };
                    j += if j < n - 1 { 1 } else { 0 };
                } else {
                    j += if i == m - 1 { 1 } else { 0 };
                    i += if i < m - 1 { 1 } else { 0 };
                }
                signed *= -1;
            } else {
                i = ii as usize;
                j = jj as usize;
            }
        }
        ans
    }
}
// @lc code=end
