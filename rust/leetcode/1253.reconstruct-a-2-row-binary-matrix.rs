/*
 * @lc app=leetcode id=1253 lang=rust
 *
 * [1253] Reconstruct a 2-Row Binary Matrix
 */

// @lc code=start
impl Solution {
    pub fn reconstruct_matrix(upper: i32, lower: i32, colsum: Vec<i32>) -> Vec<Vec<i32>> {
        let n = colsum.len();
        let mut ans = vec![vec![0; n]; 2];
        let (mut u, mut l) = (upper, lower);
        for (i, &v) in colsum.iter().enumerate() {
            if v == 0 {
                continue;
            }
            if u == 0 && l == 0 {
                return Vec::new();
            }
            if v == 2 {
                u -= 1;
                l -= 1;
                ans[0][i] = 1;
                ans[1][i] = 1;
            } else if v == 1 {
                if u >= l {
                    ans[0][i] = 1;
                    u -= 1;
                } else {
                    ans[1][i] = 1;
                    l -= 1;
                }
            }
        }
        if u == 0 && l == 0 {
            ans
        } else {
            Vec::new()
        }
    }
}
// @lc code=end
