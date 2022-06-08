/*
 * @lc app=leetcode id=1292 lang=rust
 *
 * [1292] Maximum Side Length of a Square with Sum Less than or Equal to Threshold
 */

// @lc code=start
impl Solution {
    pub fn max_side_length(mat: Vec<Vec<i32>>, threshold: i32) -> i32 {
        let (m, n) = (mat.len(), mat[0].len());
        let mut presum = vec![vec![0; n + 1]; m + 1];
             for i in 1..=m {
            for j in 1..=n {
                presum[i][j] =
                    presum[i][j - 1] + presum[i - 1][j] - presum[i - 1][j - 1] + mat[i - 1][j - 1];
            }
        }
        let get_rect = |i: usize, j: usize, i2: usize, j2: usize| -> i32 {
             presum[i2][j2]-presum[i-1][j2] - presum[i2][j-1] + presum[i-1][j-1] 
        };
   
        let mut ans = 0;
        let r = m.min(n);
        for i in 1..=m {
            for j in 1..=n {
                for c in ans + 1..=r {
                    if i + c - 1 <= m
                        && j + c - 1 <= n
                        && get_rect(i, j, i + c - 1, j + c - 1) <= threshold
                    {
                        ans += 1;
                    } else {
                        break;
                    }
                }
            }
        }
        ans as _
    }
}
// @lc code=end
