/*
 * @lc app=leetcode id=1582 lang=rust
 *
 * [1582] Special Positions in a Binary Matrix
 */

// @lc code=start
impl Solution {
    pub fn num_special(mat: Vec<Vec<i32>>) -> i32 {
        let mut rows = vec![0; mat.len()];
        let mut cols = vec![0; mat[0].len()];
        let mut ones = vec![0; mat.len()];
        for i in 0..mat.len() {
            for j in 0..mat[i].len() {
                let m = mat[i][j];
                if m != 0 {
                    ones[i] = j;
                }
                rows[i] += m;
                cols[j] += m;
            }
        }

        let mut ans = 0;
        for i in 0..mat.len() {
            if rows[i] == 1 && cols[ones[i]] == 1 {
                ans += 1;
            }
        }
        ans
    }
}
// @lc code=end
