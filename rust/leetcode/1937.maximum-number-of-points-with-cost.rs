/*
 * @lc app=leetcode id=1937 lang=rust
 *
 * [1937] Maximum Number of Points with Cost
 */

// @lc code=start
impl Solution {
    pub fn max_points(points: Vec<Vec<i32>>) -> i64 {
        let (m, n) = (points.len(), points[0].len());
        let mut f = vec![0; n];
        for i in 0..m {
            let mut g = vec![0; n];
            let mut best = i64::MIN;
            for j in 0..n {
                best = best.max(f[j] + j as i64);
                g[j] = g[j].max(best + (points[i][j] as i64) - j as i64);
            }
            best = i64::MIN;
            for j in (0..n).rev() {
                best = best.max(f[j] - j as i64);
                g[j] = g[j].max(best + (points[i][j] as i64) + j as i64);
            }
            f = g;
        }
        *f.iter().max().unwrap()
    }
}
// @lc code=end
