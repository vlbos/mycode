/*
 * @lc app=leetcode id=1504 lang=rust
 *
 * [1504] Count Submatrices With All Ones
 */

// @lc code=start
impl Solution {
    pub fn num_submat(mat: Vec<Vec<i32>>) -> i32 {
        let (m, n) = (mat.len(), mat[0].len());
        let mut row = vec![vec![0; n]; m];
        for i in 0..m {
            for j in 0..n {
                if j == 0 {
                    row[i][j] = mat[i][j];
                } else {
                    row[i][j] = if mat[i][j] == 0 { 0 } else { row[i][j - 1] + 1 };
                }
            }
        }
        let mut ans = 0;
        for j in 0..n {
            let mut q = Vec::<(i32,i32)>::new();
            let mut total = 0;
            for i in 0..m {
                let mut h = 1;
                while !q.is_empty() && q.last().unwrap().0 > row[i][j] {
                    let ql = q.last().unwrap();
                    total -= ql.1 * (ql.0 - row[i][j]);
                    h += ql.1;
                    q.pop();
                }
                total += row[i][j];
                ans += total;
                q.push((row[i][j], h));
            }
        }
        ans
    }
}
// @lc code=end
