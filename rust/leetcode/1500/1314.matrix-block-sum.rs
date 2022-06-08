/*
 * @lc app=leetcode id=1314 lang=rust
 *
 * [1314] Matrix Block Sum
 */

// @lc code=start
impl Solution {
    pub fn matrix_block_sum(mat: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
        let (m, n) = (mat.len(), mat[0].len());
        let mut pre = vec![vec![0; n + 1]; m + 1];

        for i in 1..=m {
            for j in 1..=n {
                pre[i][j] = pre[i][j - 1] + pre[i - 1][j] - pre[i - 1][j - 1] + mat[i - 1][j - 1];
            }
        }
        let get = |i: i32, j: i32| -> i32 {
            let (x, y) = (
                i.min(m as i32).max(0) as usize,
                j.min(n as i32).max(0) as usize,
            );
            pre[x][y]
        };
        let mut ans = vec![vec![0; n]; m];
        for i in 0..m {
            for j in 0..n {
                let (ii, jj) = (i as i32, j as i32);
                ans[i][j] =
                    get(ii + k + 1, jj + k + 1) - get(ii + k + 1, jj - k) - get(ii - k, jj + k + 1)
                        + get(ii - k, jj - k);
            }
        }
        ans
    }
}
// @lc code=end
