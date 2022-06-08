/*
 * @lc app=leetcode id=1439 lang=rust
 *
 * [1439] Find the Kth Smallest Sum of a Matrix With Sorted Rows
 */

// @lc code=start
impl Solution {
    pub fn kth_smallest(mat: Vec<Vec<i32>>, k: i32) -> i32 {
        fn dfs(mid: i32, index: usize, sum: i32, num: &mut i32, k: i32, mat: &Vec<Vec<i32>>) {
            let (m, n) = (mat.len(), mat[0].len());
            if sum > mid || index == m || *num > k {
                return;
            }
            dfs(mid, index + 1, sum, num, k, mat);
            for i in 1..n {
                if sum + mat[index][i] - mat[index][0] > mid {
                    break;
                }
                *num += 1;
                dfs(
                    mid,
                    index + 1,
                    sum + mat[index][i] - mat[index][0],
                    num,
                    k,
                    mat,
                );
            }
        }
        let (m, n) = (mat.len(), mat[0].len());
        let (mut left, mut right) = (0, 0);
        for i in 0..m {
            left += mat[i][0];
            right += mat[i][n - 1];
        }
        let init = left;
        while left < right {
            let mid = (left + right) / 2;
            let mut num = 1;
            dfs(mid, 0, init, &mut num, k, &mat);
            if num >= k {
                right = mid;
            } else {
                left = mid + 1;
            }
        }
        left
    }
}
// @lc code=end
