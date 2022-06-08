/*
 * @lc app=leetcode id=1738 lang=rust
 *
 * [1738] Find Kth Largest XOR Coordinate Value
 */

// @lc code=start
impl Solution {
    pub fn kth_largest_value(matrix: Vec<Vec<i32>>, k: i32) -> i32 {
       let mut ans = Vec::new();
        let (m, n) = (matrix.len(), matrix[0].len());
        let mut pre = vec![vec![0; n + 1]; m + 1];
        for i in 1..=m {
            for j in 1..=n {
                pre[i][j] = pre[i][j - 1] ^ pre[i - 1][j] ^ pre[i - 1][j - 1]^matrix[i-1][j-1];
                ans.push(pre[i][j]);
            }
        }
        ans.sort_by(|a, b| b.cmp(&a));
        ans[k as usize - 1]
    }
}
// @lc code=end
