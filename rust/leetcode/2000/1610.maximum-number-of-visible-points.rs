/*
 * @lc app=leetcode id=1610 lang=rust
 *
 * [1610] Maximum Number of Visible Points
 */

// @lc code=start
impl Solution {
    pub fn visible_points(points: Vec<Vec<i32>>, angle: i32, location: Vec<i32>) -> i32 {
        let mut same_cnt = 0;
        let mut polar_degrees = Vec::new();
        for p in points {
            if p == location {
                same_cnt += 1;
            } else {
                polar_degrees
                    .push(((p[1] - location[1]) as f64).atan2((p[0] - location[0]) as f64));
            }
        }
        polar_degrees.sort_by(|a,b| a.partial_cmp(&b).unwrap());
        let n = polar_degrees.len();
        polar_degrees.extend(
            polar_degrees
                .clone()
                .into_iter()
                .map(|x| x + 2.0 * std::f64::consts::PI)
                .collect::<Vec<f64>>(),
        );
        let mut max_cnt = 0;
        let mut right = 0;
        let degree = (angle as f64) * std::f64::consts::PI / 180.0;
        for i in 0..n {
            while right < 2 * n && polar_degrees[right] <= polar_degrees[i] + degree {
                right += 1;
            }
            max_cnt = max_cnt.max((right - i) as i32);
        }
        same_cnt + max_cnt
    }
}
// @lc code=end
