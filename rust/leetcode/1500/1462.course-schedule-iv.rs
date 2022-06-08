/*
 * @lc app=leetcode id=1462 lang=rust
 *
 * [1462] Course Schedule IV
 */

// @lc code=start
impl Solution {
    pub fn check_if_prerequisite(
        num_courses: i32,
        prerequisites: Vec<Vec<i32>>,
        queries: Vec<Vec<i32>>,
    ) -> Vec<bool> {
        let n = num_courses as usize;
        let mut f = vec![vec![false; n]; n];
        for p in prerequisites {
            f[p[0] as usize][p[1] as usize] = true;
        }
        for k in 0..n {
            for i in 0..n {
                for j in 0..n {
                    f[i][j] |= f[i][k] && f[k][j];
                }
            }
        }
        queries
            .iter()
            .map(|p| f[p[0] as usize][p[1] as usize])
            .collect::<Vec<_>>()
    }
}
// @lc code=end
