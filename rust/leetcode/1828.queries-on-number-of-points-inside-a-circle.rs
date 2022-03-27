/*
 * @lc app=leetcode id=1828 lang=rust
 *
 * [1828] Queries on Number of Points Inside a Circle
 */

// @lc code=start
impl Solution {
    pub fn count_points(points: Vec<Vec<i32>>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let mut ans = Vec::new();
        for q in &queries {
            let mut cnt = 0;
            for p in &points {
                let (x, y) = (q[0] - p[0], q[1] - p[1]);
                if x * x + y * y <= q[2] * q[2] {
                    cnt += 1;
                }
            }
            ans.push(cnt);
        }
        ans
    }
}
// @lc code=end
